#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_01002726(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002726); }
 let mut pc: u32 = 0x1002726;
 loop { match pc {
 0x1002726 => {
  { let v: u32 = cpu.r32(0x10e3f68u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10fd441u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10ee933u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x100273d); f_0100279d(cpu); if cpu.returned_to != 0x100273d { return; }
  if cpu.cf { pc = 0x1002789; continue; }
  pc = 0x100273f; continue;
 }
 0x100273f => {
  { let v: u8 = cpu.r8(0x10ee933u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x10fd441u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x10ee933u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1002775; continue; }
  pc = 0x1002766; continue;
 }
 0x1002766 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x10e3f68u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1002775 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f68u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd441u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee933u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1002789 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f68u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd441u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee933u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100279d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100279d); }
 let mut pc: u32 = 0x100279d;
 loop { match pc {
 0x100279d => {
  { let v: u32 = 0x10fd4d4u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x10027a7; continue;
 }
 0x10027a7 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x10027b6; continue; }
  pc = 0x10027af; continue;
 }
 0x10027af => {
  { let v: u32 = cpu.add32(cpu.esi, 0x14u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10027a7; continue; }
  pc = 0x10027b4; continue;
 }
 0x10027b4 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10027b6 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010027b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10027b8); }
 let mut pc: u32 = 0x10027b8;
 loop { match pc {
 0x10027b8 => {
  { let v: u32 = 0x10fd4d4u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x10027c2; continue;
 }
 0x10027c2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x14u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10027c2; continue; }
  pc = 0x10027cd; continue;
 }
 0x10027cd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010027ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10027ce); }
 let mut pc: u32 = 0x10027ce;
 loop { match pc {
 0x10027ce => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110c1c8u32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x110c1cau32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x110c1a8u32; cpu.esi = v; }
  pc = 0x10027eb; continue;
 }
 0x10027eb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x110c1c8u32)); }
  if !cpu.zf { pc = 0x100280c; continue; }
  pc = 0x1002803; continue;
 }
 0x1002803 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x110c1cau32)); }
  if cpu.zf { pc = 0x1002819; continue; }
  pc = 0x100280c; continue;
 }
 0x100280c => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10027eb; continue; }
  pc = 0x1002815; continue;
 }
 0x1002815 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1002819 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002827(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002827); }
 let mut pc: u32 = 0x1002827;
 loop { match pc {
 0x1002827 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002830(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002830); }
 let mut pc: u32 = 0x1002830;
 loop { match pc {
 0x1002830 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002832(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002832); }
 let mut pc: u32 = 0x1002832;
 loop { match pc {
 0x1002832 => {
  cpu.push(0x1002837); f_01002ca0(cpu); if cpu.returned_to != 0x1002837 { return; }
  if cpu.cf { pc = 0x1002934; continue; }
  pc = 0x100283d; continue;
 }
 0x100283d => {
  { cpu.sub8(cpu.r8(0x10fd43du32), 0x0u8); }
  if cpu.zf { pc = 0x1002934; continue; }
  pc = 0x100284a; continue;
 }
 0x100284a => {
  { cpu.sub8(cpu.r8(0x10fd43du32), 0x2u8); }
  if cpu.zf { pc = 0x1002934; continue; }
  pc = 0x1002857; continue;
 }
 0x1002857 => {
  cpu.push(0x100285c); f_01016940(cpu); if cpu.returned_to != 0x100285c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd441u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee933u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10e3f68u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10fd43du32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef37du32, v); }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if !cpu.zf { pc = 0x100289c; continue; }
  pc = 0x1002889; continue;
 }
 0x1002889 => {
  cpu.push(0x100288e); f_01016940(cpu); if cpu.returned_to != 0x100288e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112529fu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11c62fau32, v); }
  pc = 0x100289c; continue;
 }
 0x100289c => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10028ea; continue; }
  pc = 0x10028a5; continue;
 }
 0x10028a5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c5b92u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x10028c0; continue; }
  pc = 0x10028b1; continue;
 }
 0x10028b1 => {
  { let v: u32 = 0x11c5b8eu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c5b92u32, v); }
  pc = 0x10028c0; continue;
 }
 0x10028c0 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10028ea; continue; }
  pc = 0x10028cc; continue;
 }
 0x10028cc => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10028df); f_0108c1c5(cpu); if cpu.returned_to != 0x10028df { return; }
  { let v: u32 = cpu.r32(0x10fd441u32); cpu.eax = v; }
  cpu.push(0x10028e9); f_0101b5e4(cpu); if cpu.returned_to != 0x10028e9 { return; }
  { cpu.popad(); }
  pc = 0x10028ea; continue;
 }
 0x10028ea => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10028fd); f_01002c61(cpu); if cpu.returned_to != 0x10028fd { return; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x1002904); f_01002830(cpu); if cpu.returned_to != 0x1002904 { return; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x100290b); f_0103f696(cpu); if cpu.returned_to != 0x100290b { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x100291e); f_01002c39(cpu); if cpu.returned_to != 0x100291e { return; }
  cpu.push(0x1002923); f_01002726(cpu); if cpu.returned_to != 0x1002923 { return; }
  cpu.push(0x1002928); f_01002ca0(cpu); if cpu.returned_to != 0x1002928 { return; }
  if cpu.cf { pc = 0x1002934; continue; }
  pc = 0x100292a; continue;
 }
 0x100292a => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f68u32, v); }
  pc = 0x1002934; continue;
 }
 0x1002934 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002935(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002935); }
 let mut pc: u32 = 0x1002935;
 loop { match pc {
 0x1002935 => {
  cpu.push(0x100293a); f_01002ca0(cpu); if cpu.returned_to != 0x100293a { return; }
  if cpu.cf { pc = 0x1002a39; continue; }
  pc = 0x1002940; continue;
 }
 0x1002940 => {
  { cpu.sub8(cpu.r8(0x10fd43eu32), 0x0u8); }
  if cpu.zf { pc = 0x1002a39; continue; }
  pc = 0x100294d; continue;
 }
 0x100294d => {
  { cpu.sub8(cpu.r8(0x10fd43eu32), 0x2u8); }
  if cpu.zf { pc = 0x1002a39; continue; }
  pc = 0x100295a; continue;
 }
 0x100295a => {
  cpu.push(0x100295f); f_01016940(cpu); if cpu.returned_to != 0x100295f { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef37eu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd441u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10ee933u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10e3f68u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10fd43eu32, v); }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x1u8); }
  if !cpu.zf { pc = 0x100299f; continue; }
  pc = 0x100298c; continue;
 }
 0x100298c => {
  cpu.push(0x1002991); f_01016940(cpu); if cpu.returned_to != 0x1002991 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112529fu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11c62fau32, v); }
  pc = 0x100299f; continue;
 }
 0x100299f => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10029e8; continue; }
  pc = 0x10029a8; continue;
 }
 0x10029a8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c5b92u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x10029c3; continue; }
  pc = 0x10029b4; continue;
 }
 0x10029b4 => {
  { let v: u32 = 0x11c5b8eu32; cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c5b92u32, v); }
  pc = 0x10029c3; continue;
 }
 0x10029c3 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10029e8; continue; }
  pc = 0x10029cf; continue;
 }
 0x10029cf => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10029e2); f_0108c1c5(cpu); if cpu.returned_to != 0x10029e2 { return; }
  cpu.push(0x10029e7); f_0101b5e4(cpu); if cpu.returned_to != 0x10029e7 { return; }
  { cpu.popad(); }
  pc = 0x10029e8; continue;
 }
 0x10029e8 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10029ff); f_01002c61(cpu); if cpu.returned_to != 0x10029ff { return; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x1002a09); f_01002830(cpu); if cpu.returned_to != 0x1002a09 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1002a10); f_0103f696(cpu); if cpu.returned_to != 0x1002a10 { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1002a23); f_01002c39(cpu); if cpu.returned_to != 0x1002a23 { return; }
  cpu.push(0x1002a28); f_01002726(cpu); if cpu.returned_to != 0x1002a28 { return; }
  cpu.push(0x1002a2d); f_01002ca0(cpu); if cpu.returned_to != 0x1002a2d { return; }
  if cpu.cf { pc = 0x1002a39; continue; }
  pc = 0x1002a2f; continue;
 }
 0x1002a2f => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f68u32, v); }
  pc = 0x1002a39; continue;
 }
 0x1002a39 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002a3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002a3a); }
 let mut pc: u32 = 0x1002a3a;
 loop { match pc {
 0x1002a3a => {
  cpu.push(0x1002a3f); f_01002ca0(cpu); if cpu.returned_to != 0x1002a3f { return; }
  if cpu.cf { pc = 0x1002b37; continue; }
  pc = 0x1002a45; continue;
 }
 0x1002a45 => {
  { cpu.sub8(cpu.r8(0x10fd43fu32), 0x0u8); }
  if cpu.zf { pc = 0x1002b37; continue; }
  pc = 0x1002a52; continue;
 }
 0x1002a52 => {
  { cpu.sub8(cpu.r8(0x10fd43fu32), 0x2u8); }
  if cpu.zf { pc = 0x1002b37; continue; }
  pc = 0x1002a5f; continue;
 }
 0x1002a5f => {
  { let v: u8 = 0xffu8; cpu.w8(0x10ef37fu32, v); }
  cpu.push(0x1002a6b); f_01016940(cpu); if cpu.returned_to != 0x1002a6b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd441u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x10ee933u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10e3f68u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10fd43fu32, v); }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x2u8); }
  if !cpu.zf { pc = 0x1002aa4; continue; }
  pc = 0x1002a91; continue;
 }
 0x1002a91 => {
  cpu.push(0x1002a96); f_01016940(cpu); if cpu.returned_to != 0x1002a96 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x11c62fau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112529fu32, v); }
  pc = 0x1002aa4; continue;
 }
 0x1002aa4 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1002aed; continue; }
  pc = 0x1002aad; continue;
 }
 0x1002aad => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c5b92u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x1002ac8; continue; }
  pc = 0x1002ab9; continue;
 }
 0x1002ab9 => {
  { let v: u32 = 0x11c5b8eu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c5b92u32, v); }
  pc = 0x1002ac8; continue;
 }
 0x1002ac8 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1002aed; continue; }
  pc = 0x1002ad4; continue;
 }
 0x1002ad4 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1002ae7); f_0108c1c5(cpu); if cpu.returned_to != 0x1002ae7 { return; }
  cpu.push(0x1002aec); f_0101b5e4(cpu); if cpu.returned_to != 0x1002aec { return; }
  { cpu.popad(); }
  pc = 0x1002aed; continue;
 }
 0x1002aed => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1002b00); f_01002c61(cpu); if cpu.returned_to != 0x1002b00 { return; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x1002b07); f_01002830(cpu); if cpu.returned_to != 0x1002b07 { return; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1002b0e); f_0103f696(cpu); if cpu.returned_to != 0x1002b0e { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1002b21); f_01002c39(cpu); if cpu.returned_to != 0x1002b21 { return; }
  cpu.push(0x1002b26); f_01002726(cpu); if cpu.returned_to != 0x1002b26 { return; }
  cpu.push(0x1002b2b); f_01002ca0(cpu); if cpu.returned_to != 0x1002b2b { return; }
  if cpu.cf { pc = 0x1002b37; continue; }
  pc = 0x1002b2d; continue;
 }
 0x1002b2d => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f68u32, v); }
  pc = 0x1002b37; continue;
 }
 0x1002b37 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002b38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002b38); }
 let mut pc: u32 = 0x1002b38;
 loop { match pc {
 0x1002b38 => {
  cpu.push(0x1002b3d); f_01002ca0(cpu); if cpu.returned_to != 0x1002b3d { return; }
  if cpu.cf { pc = 0x1002c38; continue; }
  pc = 0x1002b43; continue;
 }
 0x1002b43 => {
  { cpu.sub8(cpu.r8(0x10fd440u32), 0x0u8); }
  if cpu.zf { pc = 0x1002c38; continue; }
  pc = 0x1002b50; continue;
 }
 0x1002b50 => {
  { cpu.sub8(cpu.r8(0x10fd440u32), 0x2u8); }
  if cpu.zf { pc = 0x1002c38; continue; }
  pc = 0x1002b5d; continue;
 }
 0x1002b5d => {
  { let v: u8 = 0xffu8; cpu.w8(0x10ef380u32, v); }
  cpu.push(0x1002b69); f_01016940(cpu); if cpu.returned_to != 0x1002b69 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fd441u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ee933u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10e3f68u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10fd440u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x3u8); }
  if !cpu.zf { pc = 0x1002ba2; continue; }
  pc = 0x1002b8f; continue;
 }
 0x1002b8f => {
  cpu.push(0x1002b94); f_01016940(cpu); if cpu.returned_to != 0x1002b94 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x11c62fau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112529fu32, v); }
  pc = 0x1002ba2; continue;
 }
 0x1002ba2 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1002beb; continue; }
  pc = 0x1002bab; continue;
 }
 0x1002bab => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c5b92u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x1002bc6; continue; }
  pc = 0x1002bb7; continue;
 }
 0x1002bb7 => {
  { let v: u32 = 0x11c5b8eu32; cpu.esi = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c5b92u32, v); }
  pc = 0x1002bc6; continue;
 }
 0x1002bc6 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1002beb; continue; }
  pc = 0x1002bd2; continue;
 }
 0x1002bd2 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1002be5); f_0108c1c5(cpu); if cpu.returned_to != 0x1002be5 { return; }
  cpu.push(0x1002bea); f_0101b5e4(cpu); if cpu.returned_to != 0x1002bea { return; }
  { cpu.popad(); }
  pc = 0x1002beb; continue;
 }
 0x1002beb => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1002bfe); f_01002c61(cpu); if cpu.returned_to != 0x1002bfe { return; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x1002c08); f_01002830(cpu); if cpu.returned_to != 0x1002c08 { return; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1002c0f); f_0103f696(cpu); if cpu.returned_to != 0x1002c0f { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1002c22); f_01002c39(cpu); if cpu.returned_to != 0x1002c22 { return; }
  cpu.push(0x1002c27); f_01002726(cpu); if cpu.returned_to != 0x1002c27 { return; }
  cpu.push(0x1002c2c); f_01002ca0(cpu); if cpu.returned_to != 0x1002c2c { return; }
  if cpu.cf { pc = 0x1002c38; continue; }
  pc = 0x1002c2e; continue;
 }
 0x1002c2e => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f68u32, v); }
  pc = 0x1002c38; continue;
 }
 0x1002c38 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002c39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002c39); }
 let mut pc: u32 = 0x1002c39;
 loop { match pc {
 0x1002c39 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x112ad0cu32, v); }
  cpu.push(0x1002c4c); f_01018e6e(cpu); if cpu.returned_to != 0x1002c4c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112ad0cu32, v); }
  if !cpu.cf { pc = 0x1002c5f; continue; }
  pc = 0x1002c55; continue;
 }
 0x1002c55 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1002c5f; continue;
 }
 0x1002c5f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002c61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002c61); }
 let mut pc: u32 = 0x1002c61;
 loop { match pc {
 0x1002c61 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1002c6a); f_0104f36d(cpu); if cpu.returned_to != 0x1002c6a { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4000u32); cpu.edi = v; }
  { let v: u32 = 0xbu32; cpu.edx = v; }
  pc = 0x1002c77; continue;
 }
 0x1002c77 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  pc = 0x1002c7e; continue;
 }
 0x1002c7e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x1002c8a; continue; }
  pc = 0x1002c88; continue;
 }
 0x1002c88 => {
  if !cpu.zf { pc = 0x1002c92; continue; }
  pc = 0x1002c8a; continue;
 }
 0x1002c8a => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x13u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1002c92; continue;
 }
 0x1002c92 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1002c7e; continue; }
  pc = 0x1002c95; continue;
 }
 0x1002c95 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1002c77; continue; }
  pc = 0x1002c9f; continue;
 }
 0x1002c9f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002ca0); }
 let mut pc: u32 = 0x1002ca0;
 loop { match pc {
 0x1002ca0 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1002cb4; continue; }
  pc = 0x1002ca9; continue;
 }
 0x1002ca9 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x1002cc0; continue; }
  pc = 0x1002cb2; continue;
 }
 0x1002cb2 => {
  pc = 0x1002cbe; continue;
 }
 0x1002cb4 => {
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x1002cc0; continue; }
  pc = 0x1002cbe; continue;
 }
 0x1002cbe => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1002cc0 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002cc2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002cc2); }
 let mut pc: u32 = 0x1002cc2;
 loop { match pc {
 0x1002cc2 => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43du32), 0x0u8); }
  if !cpu.zf { pc = 0x1002ce7; continue; }
  pc = 0x1002ce0; continue;
 }
 0x1002ce0 => {
  { let v: u32 = 0x69bu32; cpu.ecx = v; }
  pc = 0x1002cf7; continue;
 }
 0x1002ce7 => {
  { let v: u32 = cpu.r8(0x10ee92eu32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x699u32); cpu.ecx = v; }
  pc = 0x1002cf7; continue;
 }
 0x1002cf7 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x1002d0c); f_0106b882(cpu); if cpu.returned_to != 0x1002d0c { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1002d69; continue; }
  pc = 0x1002d15; continue;
 }
 0x1002d15 => {
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if (cpu.sf != cpu.of) { pc = 0x1002d69; continue; }
  pc = 0x1002d1f; continue;
 }
 0x1002d1f => {
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43fu32), 0x0u8); }
  if !cpu.zf { pc = 0x1002d44; continue; }
  pc = 0x1002d3d; continue;
 }
 0x1002d3d => {
  { let v: u32 = 0x69bu32; cpu.ecx = v; }
  pc = 0x1002d54; continue;
 }
 0x1002d44 => {
  { let v: u32 = cpu.r8(0x10ee92eu32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x699u32); cpu.ecx = v; }
  pc = 0x1002d54; continue;
 }
 0x1002d54 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x1002d69); f_0106b882(cpu); if cpu.returned_to != 0x1002d69 { return; }
  pc = 0x1002d69; continue;
 }
 0x1002d69 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002d6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002d6a); }
 let mut pc: u32 = 0x1002d6a;
 loop { match pc {
 0x1002d6a => {
  { let v: u16 = cpu.r16(0x10e690cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e690eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1002d98; continue; }
  pc = 0x1002d90; continue;
 }
 0x1002d90 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1002d98; continue;
 }
 0x1002d98 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002d99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002d99); }
 let mut pc: u32 = 0x1002d99;
 loop { match pc {
 0x1002d99 => {
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce7u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3ce9u32, v); }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x1002e08; continue; }
  pc = 0x1002dc9; continue;
 }
 0x1002dc9 => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x2u8); }
  if cpu.zf { pc = 0x1002dea; continue; }
  pc = 0x1002dd2; continue;
 }
 0x1002dd2 => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x3u8); }
  if cpu.zf { pc = 0x1002df9; continue; }
  pc = 0x1002ddb; continue;
 }
 0x1002ddb => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1002e08; continue;
 }
 0x1002dea => {
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1002e08; continue;
 }
 0x1002df9 => {
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1002e08; continue;
 }
 0x1002e08 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1002e21; continue; }
  pc = 0x1002e19; continue;
 }
 0x1002e19 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1002e21; continue;
 }
 0x1002e21 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3ce5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002e2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002e2f); }
 let mut pc: u32 = 0x1002e2f;
 loop { match pc {
 0x1002e2f => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10f7dd2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1002e53; continue; }
  pc = 0x1002e4c; continue;
 }
 0x1002e4c => {
  { let v: u16 = cpu.r16(0x10f7ddcu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1002e53; continue;
 }
 0x1002e53 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1002e5c); f_01002f4a(cpu); if cpu.returned_to != 0x1002e5c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1002e63); f_01002fb3(cpu); if cpu.returned_to != 0x1002e63 { return; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10f7dd2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43du32), 0x0u8); }
  if !cpu.zf { pc = 0x1002e87; continue; }
  pc = 0x1002e80; continue;
 }
 0x1002e80 => {
  { let v: u16 = cpu.r16(0x10f7ddcu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1002e87; continue;
 }
 0x1002e87 => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  cpu.push(0x1002e91); f_01002f4a(cpu); if cpu.returned_to != 0x1002e91 { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1002f49; continue; }
  pc = 0x1002e9e; continue;
 }
 0x1002e9e => {
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1002f49; continue; }
  pc = 0x1002eac; continue;
 }
 0x1002eac => {
  { cpu.sub8(cpu.r8(0x11de596u32), 0x3u8); }
  if cpu.cf { pc = 0x1002f49; continue; }
  pc = 0x1002eb9; continue;
 }
 0x1002eb9 => {
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10f7dd2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43fu32), 0x0u8); }
  if !cpu.zf { pc = 0x1002edd; continue; }
  pc = 0x1002ed6; continue;
 }
 0x1002ed6 => {
  { let v: u16 = cpu.r16(0x10f7ddcu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1002edd; continue;
 }
 0x1002edd => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1002ee9); f_01002f4a(cpu); if cpu.returned_to != 0x1002ee9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1002f49; continue; }
  pc = 0x1002ef5; continue;
 }
 0x1002ef5 => {
  { cpu.sub8(cpu.r8(0x11de596u32), 0x4u8); }
  if cpu.cf { pc = 0x1002f49; continue; }
  pc = 0x1002efe; continue;
 }
 0x1002efe => {
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if cpu.zf { pc = 0x1002f25; continue; }
  pc = 0x1002f15; continue;
 }
 0x1002f15 => {
  { let v: u16 = cpu.r16(0x10f7dd2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd440u32), 0x0u8); }
  if !cpu.zf { pc = 0x1002f2c; continue; }
  pc = 0x1002f25; continue;
 }
 0x1002f25 => {
  { let v: u16 = cpu.r16(0x10f7ddcu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1002f2c; continue;
 }
 0x1002f2c => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1002f38); f_01002f4a(cpu); if cpu.returned_to != 0x1002f38 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if cpu.zf { pc = 0x1002f49; continue; }
  pc = 0x1002f44; continue;
 }
 0x1002f44 => {
  cpu.push(0x1002f49); f_01002fe5(cpu); if cpu.returned_to != 0x1002f49 { return; }
  pc = 0x1002f49; continue;
 }
 0x1002f49 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002f4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002f4a); }
 let mut pc: u32 = 0x1002f4a;
 loop { match pc {
 0x1002f4a => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1002f53); f_01002f96(cpu); if cpu.returned_to != 0x1002f53 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1002f5e); f_01002f96(cpu); if cpu.returned_to != 0x1002f5e { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.edx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1002f70); f_01002f96(cpu); if cpu.returned_to != 0x1002f70 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1002f7b); f_01002f96(cpu); if cpu.returned_to != 0x1002f7b { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x70u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1002f8a); f_01002f96(cpu); if cpu.returned_to != 0x1002f8a { return; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x78u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1002f95); f_01002f96(cpu); if cpu.returned_to != 0x1002f95 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002f96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002f96); }
 let mut pc: u32 = 0x1002f96;
 loop { match pc {
 0x1002f96 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1002fae); cpu.call(t); if cpu.returned_to != 0x1002fae { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002fb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1002fb3); }
 let mut pc: u32 = 0x1002fb3;
 loop { match pc {
 0x1002fb3 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x90u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d55u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1002fe4; continue; }
  pc = 0x1002fc7; continue;
 }
 0x1002fc7 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10f7ddau32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1002fe4); cpu.call(t); if cpu.returned_to != 0x1002fe4 { return; }
  pc = 0x1002fe4; continue;
 }
 0x1002fe4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01002fe5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1002fe5); } r_01002fe5(cpu, 0x1002fe5); }
pub fn f_01003000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1003000); } r_01002fe5(cpu, 0x1003000); }
fn r_01002fe5(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1002fe5 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x90u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d55u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1003016; continue; }
  pc = 0x1002ff9; continue;
 }
 0x1002ff9 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10f7ddau32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1003016); cpu.call(t); if cpu.returned_to != 0x1003016 { return; }
  pc = 0x1003016; continue;
 }
 0x1003000 => {
  cpu.unimplemented(0x1003000, "Smsw");
  { let v: u32 = cpu.eax.wrapping_add(0x10e3f60u32); cpu.eax = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1003016); cpu.call(t); if cpu.returned_to != 0x1003016 { return; }
  pc = 0x1003016; continue;
 }
 0x1003016 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01003017(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1003017); } r_01003017(cpu, 0x1003017); }
pub fn f_01003400(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1003400); } r_01003017(cpu, 0x1003400); }
fn r_01003017(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1003017 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x100314d; continue; }
  pc = 0x1003024; continue;
 }
 0x1003024 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10e3f68u32)); cpu.w32(0x10e3f68u32, v); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x2u32); }
  if !cpu.zf { pc = 0x1003073; continue; }
  pc = 0x1003033; continue;
 }
 0x1003033 => {
  cpu.push(0x1003038); f_010129b5(cpu); if cpu.returned_to != 0x1003038 { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x10ee933u32)); }
  if !cpu.zf { pc = 0x1003056; continue; }
  pc = 0x100304a; continue;
 }
 0x100304a => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1003054); f_010355a5(cpu); if cpu.returned_to != 0x1003054 { return; }
  pc = 0x1003073; continue;
 }
 0x1003056 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1003069; continue; }
  pc = 0x100305f; continue;
 }
 0x100305f => {
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1003073; continue; }
  pc = 0x1003069; continue;
 }
 0x1003069 => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1003073); f_010354f3(cpu); if cpu.returned_to != 0x1003073 { return; }
  pc = 0x1003073; continue;
 }
 0x1003073 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee933u32), 0x1u8); }
  if cpu.zf { pc = 0x10030a2; continue; }
  pc = 0x1003081; continue;
 }
 0x1003081 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee933u32), 0x2u8); }
  if cpu.zf { pc = 0x10030a2; continue; }
  pc = 0x100308f; continue;
 }
 0x100308f => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee933u32), 0x3u8); }
  if cpu.zf { pc = 0x10030a2; continue; }
  pc = 0x100309d; continue;
 }
 0x100309d => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x10030a2; continue;
 }
 0x10030a2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x11d02bdu32, v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x10030bc; continue; }
  pc = 0x10030b0; continue;
 }
 0x10030b0 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10030d3; continue; }
  pc = 0x10030b5; continue;
 }
 0x10030b5 => {
  cpu.push(0x10030ba); f_0100a820(cpu); if cpu.returned_to != 0x10030ba { return; }
  pc = 0x10030d3; continue;
 }
 0x10030bc => {
  cpu.push(0x10030c1); f_0100a820(cpu); if cpu.returned_to != 0x10030c1 { return; }
  { let v: u32 = cpu.r32(0x11d02bdu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fd441u32); cpu.ebx = v; }
  cpu.push(0x10030d1); f_0100a747(cpu); if cpu.returned_to != 0x10030d1 { return; }
  pc = 0x10030d3; continue;
 }
 0x10030d3 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x10ee933u32), 0x2u8); }
  if cpu.zf { pc = 0x100314f; continue; }
  pc = 0x10030f2; continue;
 }
 0x10030f2 => {
  { cpu.sub8(cpu.r8(0x10ee933u32), 0x3u8); }
  if cpu.zf { pc = 0x1003163; continue; }
  pc = 0x10030fb; continue;
 }
 0x10030fb => {
  { cpu.sub8(cpu.r8(0x10ee933u32), 0x4u8); }
  if cpu.zf { pc = 0x1003172; continue; }
  pc = 0x1003104; continue;
 }
 0x1003104 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x2u32); }
  if cpu.zf { pc = 0x10031ef; continue; }
  pc = 0x1003111; continue;
 }
 0x1003111 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x3u32); }
  if cpu.zf { pc = 0x1003495; continue; }
  pc = 0x100311e; continue;
 }
 0x100311e => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0xfu32); }
  if cpu.zf { pc = 0x1003237; continue; }
  pc = 0x100312b; continue;
 }
 0x100312b => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x10u32); }
  if cpu.zf { pc = 0x1003327; continue; }
  pc = 0x1003138; continue;
 }
 0x1003138 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x11u32); }
  if cpu.zf { pc = 0x10031a9; continue; }
  pc = 0x1003141; continue;
 }
 0x1003141 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0xaau32); }
  if cpu.zf { pc = 0x1003181; continue; }
  pc = 0x100314d; continue;
 }
 0x100314d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100314f => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  cpu.poll(); pc = 0x1003104; continue;
 }
 0x1003163 => {
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1003104; continue;
 }
 0x1003172 => {
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1003104; continue;
 }
 0x1003181 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10031a0; continue; }
  pc = 0x100318a; continue;
 }
 0x100318a => {
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (cpu.sf != cpu.of) { pc = 0x10031a0; continue; }
  pc = 0x1003194; continue;
 }
 0x1003194 => {
  cpu.push(0x1003199); f_01003540(cpu); if cpu.returned_to != 0x1003199 { return; }
  cpu.push(0x100319e); f_01059ab7(cpu); if cpu.returned_to != 0x100319e { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10031a0 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b05u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10031a9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee933u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10031c4; continue; }
  pc = 0x10031ba; continue;
 }
 0x10031ba => {
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x10031cb; continue; }
  pc = 0x10031c4; continue;
 }
 0x10031c4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b05u32, v); }
  pc = 0x10031cb; continue;
 }
 0x10031cb => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd43du32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd439u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee932u32, v); }
  { let v: u8 = 0x14u8; cpu.w8(0x110d634u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x110d635u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10031ef => {
  cpu.push(0x10031f4); f_01057945(cpu); if cpu.returned_to != 0x10031f4 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef370u32, v); }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10ee933u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee91du32, v); }
  cpu.push(0x1003212); f_01002d99(cpu); if cpu.returned_to != 0x1003212 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee91du32, v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1003235; continue; }
  pc = 0x1003221; continue;
 }
 0x1003221 => {
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x1003235; continue; }
  pc = 0x100322b; continue;
 }
 0x100322b => {
  { let v: u32 = 0x10c07c8u32; cpu.esi = v; }
  cpu.push(0x1003235); f_01059ad5(cpu); if cpu.returned_to != 0x1003235 { return; }
  pc = 0x1003235; continue;
 }
 0x1003235 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1003237 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  cpu.push(0x1003243); f_01072d00(cpu); if cpu.returned_to != 0x1003243 { return; }
  if cpu.cf { pc = 0x1003276; continue; }
  pc = 0x1003245; continue;
 }
 0x1003245 => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1003276; continue; }
  pc = 0x100324e; continue;
 }
 0x100324e => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x100326e; continue; }
  pc = 0x1003264; continue;
 }
 0x1003264 => {
  cpu.push(0x1003269); f_01012780(cpu); if cpu.returned_to != 0x1003269 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x100326e; continue;
 }
 0x100326e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x1003276; continue;
 }
 0x1003276 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x1d9u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x10032be; continue; }
  pc = 0x10032a7; continue;
 }
 0x10032a7 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x10032c3; continue; }
  pc = 0x10032b0; continue;
 }
 0x10032b0 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x10032c8; continue; }
  pc = 0x10032b9; continue;
 }
 0x10032b9 => {
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  pc = 0x10032cb; continue;
 }
 0x10032be => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x10032cb; continue;
 }
 0x10032c3 => {
  { let v: u32 = cpu.eax.wrapping_add(0x6u32); cpu.eax = v; }
  pc = 0x10032cb; continue;
 }
 0x10032c8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x10032cb; continue;
 }
 0x10032cb => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10032ce; continue;
 }
 0x10032ce => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x10032d8); f_0105f215(cpu); if cpu.returned_to != 0x10032d8 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x10032df); f_0105f215(cpu); if cpu.returned_to != 0x10032df { return; }
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
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x1003320); f_0105ba41(cpu); if cpu.returned_to != 0x1003320 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10032ce; continue; }
  pc = 0x1003325; continue;
 }
 0x1003325 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1003327 => {
  cpu.push(0x100332c); f_01072d00(cpu); if cpu.returned_to != 0x100332c { return; }
  if cpu.cf { pc = 0x100335f; continue; }
  pc = 0x100332e; continue;
 }
 0x100332e => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x100335f; continue; }
  pc = 0x1003337; continue;
 }
 0x1003337 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1003357; continue; }
  pc = 0x100334d; continue;
 }
 0x100334d => {
  cpu.push(0x1003352); f_01012780(cpu); if cpu.returned_to != 0x1003352 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1003357; continue;
 }
 0x1003357 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x100335f; continue;
 }
 0x100335f => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x69cu16; cpu.w16(0x10e3ebfu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x10033a7; continue; }
  pc = 0x1003390; continue;
 }
 0x1003390 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x10033ac; continue; }
  pc = 0x1003399; continue;
 }
 0x1003399 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x10033b1; continue; }
  pc = 0x10033a2; continue;
 }
 0x10033a2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  pc = 0x10033b4; continue;
 }
 0x10033a7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x10033b4; continue;
 }
 0x10033ac => {
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  pc = 0x10033b4; continue;
 }
 0x10033b1 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x10033b4; continue;
 }
 0x10033b4 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10033b7; continue;
 }
 0x10033b7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10033c0); f_0105f215(cpu); if cpu.returned_to != 0x10033c0 { return; }
  { let v: u32 = cpu.eax & 0x301u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x10fd524u32.wrapping_add(cpu.esi.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10033dd); f_0105f215(cpu); if cpu.returned_to != 0x10033dd { return; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.push(0x10033e7); f_0105f215(cpu); if cpu.returned_to != 0x10033e7 { return; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x10033f5); f_01003474(cpu); if cpu.returned_to != 0x10033f5 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10033b7; continue; }
  pc = 0x10033fa; continue;
 }
 0x10033fa => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4d2u16; cpu.w16(0x10e3ebfu32, v); }
  { cpu.sub8(cpu.r8(0x10ee933u32), 0x1u8); }
  if cpu.zf { pc = 0x1003455; continue; }
  pc = 0x1003428; continue;
 }
 0x1003400 => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x10ef32du32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0xc6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x10ef32eu32); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x10ef342u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffc7u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffc7u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x10e3ebfu32); cpu.eax = v; }
  { let v: u8 = cpu.rol8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx & 0xff)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { cpu.sub32(cpu.eax, 0x10ee933u32); }
  { let v: u32 = cpu.r32(0x66u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(1))).wrapping_add(cpu.esi); cpu.w32(0x66u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u32 = 0x3d8004d6u32; cpu.w32(0x10e3ebfu32, v); }
  { let v: u32 = cpu.ebp ^ cpu.ecx; cpu.ebp = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  if cpu.zf { pc = 0x1003455; continue; }
  pc = 0x100343a; continue;
 }
 0x1003428 => {
  { let v: u16 = 0x4d6u16; cpu.w16(0x10e3ebfu32, v); }
  { cpu.sub8(cpu.r8(0x10ee933u32), 0x2u8); }
  if cpu.zf { pc = 0x1003455; continue; }
  pc = 0x100343a; continue;
 }
 0x100343a => {
  { let v: u16 = 0x4deu16; cpu.w16(0x10e3ebfu32, v); }
  { cpu.sub8(cpu.r8(0x10ee933u32), 0x3u8); }
  if cpu.zf { pc = 0x1003455; continue; }
  pc = 0x100344c; continue;
 }
 0x100344c => {
  { let v: u16 = 0x4e2u16; cpu.w16(0x10e3ebfu32, v); }
  pc = 0x1003455; continue;
 }
 0x1003455 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x53u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x3bu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  cpu.push(0x1003472); f_0105af34(cpu); if cpu.returned_to != 0x1003472 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1003495 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xa0u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), 0xa0u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11d02bdu32); cpu.esi = v; }
  cpu.push(0x10034b0); f_01053fa7(cpu); if cpu.returned_to != 0x10034b0 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01003474(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1003474); }
 let mut pc: u32 = 0x1003474;
 loop { match pc {
 0x1003474 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  cpu.push(0x1003493); f_0105af34(cpu); if cpu.returned_to != 0x1003493 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010034b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10034b2); }
 let mut pc: u32 = 0x10034b2;
 loop { match pc {
 0x10034b2 => {
  { cpu.sub8(cpu.r8(0x10fd439u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1003538; continue; }
  pc = 0x10034bb; continue;
 }
 0x10034bb => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd44cu32, v); }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x10034cc; continue;
 }
 0x10034cc => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x100352b; continue; }
  pc = 0x10034d4; continue;
 }
 0x10034d4 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x100352b; continue; }
  pc = 0x10034db; continue;
 }
 0x10034db => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x10034ee; continue; }
  pc = 0x10034e1; continue;
 }
 0x10034e1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10034e9); f_010504cc(cpu); if cpu.returned_to != 0x10034e9 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100352b; continue;
 }
 0x10034ee => {
  { let v: u8 = 0xf8u8; cpu.w8(0x10ef210u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1124e88u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x100351c); f_01036a75(cpu); if cpu.returned_to != 0x100351c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e88u32, v); }
  { cpu.popad(); }
  pc = 0x100352b; continue;
 }
 0x100352b => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10034cc; continue; }
  pc = 0x1003538; continue;
 }
 0x1003538 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd44cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01003540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1003540); }
 let mut pc: u32 = 0x1003540;
 loop { match pc {
 0x1003540 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f68u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee933u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01003552(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1003552); }
 let mut pc: u32 = 0x1003552;
 loop { match pc {
 0x1003552 => {
  { let v: u16 = 0x1u16; cpu.w16(0x10e3d57u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100355c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100355c); }
 let mut pc: u32 = 0x100355c;
 loop { match pc {
 0x100355c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100355d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100355d); }
 let mut pc: u32 = 0x100355d;
 loop { match pc {
 0x100355d => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1003579; continue; }
  pc = 0x1003566; continue;
 }
 0x1003566 => {
  { cpu.sub16(cpu.r16(0x10e3d55u32), 0x0u16); }
  if cpu.zf { pc = 0x1003579; continue; }
  pc = 0x1003570; continue;
 }
 0x1003570 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3d57u32, v); }
  pc = 0x1003579; continue;
 }
 0x1003579 => {
  cpu.push(0x100357e); f_0100357f(cpu); if cpu.returned_to != 0x100357e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100357f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100357f); }
 let mut pc: u32 = 0x100357f;
 loop { match pc {
 0x100357f => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1003589; continue;
 }
 0x1003589 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if !cpu.zf { pc = 0x10035cf; continue; }
  pc = 0x1003592; continue;
 }
 0x1003592 => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x10035cf; continue; }
  pc = 0x100359b; continue;
 }
 0x100359b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xc8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xf0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x10035c6); f_01018ffd(cpu); if cpu.returned_to != 0x10035c6 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x10035cf; continue; }
  pc = 0x10035c9; continue;
 }
 0x10035c9 => {
  cpu.push(0x10035ce); f_01003552(cpu); if cpu.returned_to != 0x10035ce { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10035cf => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1003589; continue; }
  pc = 0x10035d8; continue;
 }
 0x10035d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010035d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10035d9); }
 let mut pc: u32 = 0x10035d9;
 loop { match pc {
 0x10035d9 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x10035ec; continue; }
  pc = 0x10035e2; continue;
 }
 0x10035e2 => {
  { let v: u32 = 0x10e3d55u32; cpu.esi = v; }
  cpu.push(0x10035ec); f_010035ed(cpu); if cpu.returned_to != 0x10035ec { return; }
  pc = 0x10035ec; continue;
 }
 0x10035ec => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010035ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10035ed); }
 let mut pc: u32 = 0x10035ed;
 loop { match pc {
 0x10035ed => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1003604; continue; }
  pc = 0x10035f6; continue;
 }
 0x10035f6 => {
  { let v: u16 = cpu.add16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x1003608; continue; }
  pc = 0x10035fb; continue;
 }
 0x10035fb => {
  if cpu.sf { pc = 0x1003605; continue; }
  pc = 0x10035fd; continue;
 }
 0x10035fd => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x1003608; continue; }
  pc = 0x1003602; continue;
 }
 0x1003602 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1003605; continue; }
  pc = 0x1003604; continue;
 }
 0x1003604 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1003605 => {
  { let v: u16 = cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1003608; continue;
 }
 0x1003608 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01003614(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1003614); } r_01003614(cpu, 0x1003614); }
pub fn f_01003635(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1003635); } r_01003614(cpu, 0x1003635); }
pub fn f_01003643(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1003643); } r_01003614(cpu, 0x1003643); }
fn r_01003614(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1003614 => {
  cpu.push(0x1003619); f_01046c33(cpu); if cpu.returned_to != 0x1003619 { return; }
  { let v: u32 = 0x116cbc8u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x10ef358u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x100368e; continue; }
  pc = 0x1003628; continue;
 }
 0x1003628 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x11757deu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1003687; continue; }
  pc = 0x100363b; continue;
 }
 0x1003635 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x1003687; continue; }
  pc = 0x100363b; continue;
 }
 0x100363b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x1003687; continue; }
  pc = 0x1003644; continue;
 }
 0x1003643 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x1003644; continue;
 }
 0x1003644 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1003651); f_0100375f(cpu); if cpu.returned_to != 0x1003651 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x1003687; continue; }
  pc = 0x1003657; continue;
 }
 0x1003657 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x11757deu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1003687; continue; }
  pc = 0x100366a; continue;
 }
 0x100366a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x1003687; continue; }
  pc = 0x1003673; continue;
 }
 0x1003673 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1003680); f_01018cce(cpu); if cpu.returned_to != 0x1003680 { return; }
  if cpu.cf { pc = 0x1003687; continue; }
  pc = 0x1003682; continue;
 }
 0x1003682 => {
  cpu.push(0x1003687); f_0100368f(cpu); if cpu.returned_to != 0x1003687 { return; }
  pc = 0x1003687; continue;
 }
 0x1003687 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x14u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1003628; continue; }
  pc = 0x100368e; continue;
 }
 0x100368e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100368f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100368f); } r_0100368f(cpu, 0x100368f); }
pub fn f_01003749(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1003749); } r_0100368f(cpu, 0x1003749); }
fn r_0100368f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x100368f => {
  { cpu.pushad(); }
  cpu.push(0x1003695); f_0105f215(cpu); if cpu.returned_to != 0x1003695 { return; }
  { let v: u32 = cpu.eax & 0x38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11cec68u32, v); }
  { let v: u8 = 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x100369f; continue;
 }
 0x100369f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11cec68u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x110c1ccu32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x110c1d0u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10036cb); f_0100375f(cpu); if cpu.returned_to != 0x10036cb { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1003747; continue; }
  pc = 0x10036d1; continue;
 }
 0x10036d1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f6ab1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1003747; continue; }
  pc = 0x1003700; continue;
 }
 0x1003700 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f6ab1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1003747; continue; }
  pc = 0x100370f; continue;
 }
 0x100370f => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f6ab1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1003747; continue; }
  pc = 0x1003721; continue;
 }
 0x1003721 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f6ab1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1003747; continue; }
  pc = 0x1003733; continue;
 }
 0x1003733 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebx = v; }
  cpu.push(0x1003745); f_0106a04e(cpu); if cpu.returned_to != 0x1003745 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1003747 => {
  { let v: u32 = cpu.r32(0x11cec68u32).wrapping_add(0x8u32); cpu.w32(0x11cec68u32, v); }
  { let v: u32 = cpu.and32(cpu.r32(0x11cec68u32), 0x38u32); cpu.w32(0x11cec68u32, v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100369f; continue; }
  pc = 0x100375d; continue;
 }
 0x1003749 => {
  { let v = 0x8011cecu32; cpu.push(v); }
  { let v: u32 = cpu.and32(cpu.r32(0x11cec68u32), 0x38u32); cpu.w32(0x11cec68u32, v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100369f; continue; }
  pc = 0x100375d; continue;
 }
 0x100375d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100375f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100375f); }
 let mut pc: u32 = 0x100375f;
 loop { match pc {
 0x100375f => {
  { let v: u32 = 0x116cbc8u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x10ef358u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x100376a; continue;
 }
 0x100376a => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x10037ae; continue; }
  pc = 0x100376e; continue;
 }
 0x100376e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x1003785; continue; }
  pc = 0x1003775; continue;
 }
 0x1003775 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { pc = 0x10037ae; continue; }
  pc = 0x1003785; continue;
 }
 0x1003785 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x40u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x10037ae; continue; }
  pc = 0x10037ac; continue;
 }
 0x10037ac => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10037ae => {
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100376a; continue; }
  pc = 0x10037b5; continue;
 }
 0x10037b5 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01003848(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1003848); } r_01003848(cpu, 0x1003848); }
pub fn f_0100384a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100384a); } r_01003848(cpu, 0x100384a); }
fn r_01003848(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1003848 => {
  { cpu.push(cpu.segments[1] as u32); }
  pc = 0x100384a; continue;
 }
 0x100384a => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1194200u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esp); cpu.eax = v; }
  cpu.call(0x100385a); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100394e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100394e); }
 let mut pc: u32 = 0x100394e;
 loop { match pc {
 0x100394e => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edi); cpu.esi = v; }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01004541(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1004541); }
 let mut pc: u32 = 0x1004541;
 loop { match pc {
 0x1004541 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.ecx as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  {  }
  { cpu.cf = !cpu.cf; }
  cpu.call(0x1004547); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01007d03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1007d03); }
 let mut pc: u32 = 0x1007d03;
 loop { match pc {
 0x1007d03 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d935u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d93fu32, v); }
  { let v: u32 = 0x110d931u32; cpu.eax = v; }
  { let v: u32 = 0x119470au32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1194705u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x19u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1007d32); f_01001c47(cpu); if cpu.returned_to != 0x1007d32 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01007d93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1007d93); }
 let mut pc: u32 = 0x1007d93;
 loop { match pc {
 0x1007d93 => {
  { cpu.sub8(cpu.r8(0x1194709u32), 0x1u8); }
  if !cpu.zf { pc = 0x1007dc0; continue; }
  pc = 0x1007d9c; continue;
 }
 0x1007d9c => {
  { cpu.and8(cpu.r8(0x10ee92eu32), 0x4u8); }
  if cpu.zf { pc = 0x1007df5; continue; }
  pc = 0x1007da5; continue;
 }
 0x1007da5 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x2fu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1007dba); cpu.call(t); if cpu.returned_to != 0x1007dba { return; }
  cpu.push(0x1007dbf); f_01007df6(cpu); if cpu.returned_to != 0x1007dbf { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1007dc0 => {
  { cpu.sub8(cpu.r8(0x1194709u32), 0x2u8); }
  if (cpu.sf != cpu.of) { pc = 0x1007df5; continue; }
  pc = 0x1007dc9; continue;
 }
 0x1007dc9 => {
  { cpu.sub8(cpu.r8(0x1194709u32), 0x3u8); }
  if cpu.zf { pc = 0x1007ddb; continue; }
  pc = 0x1007dd2; continue;
 }
 0x1007dd2 => {
  { cpu.and8(cpu.r8(0x10ee92eu32), 0x4u8); }
  if cpu.zf { pc = 0x1007df5; continue; }
  pc = 0x1007ddb; continue;
 }
 0x1007ddb => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x30u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1007df0); cpu.call(t); if cpu.returned_to != 0x1007df0 { return; }
  cpu.push(0x1007df5); f_01007df6(cpu); if cpu.returned_to != 0x1007df5 { return; }
  pc = 0x1007df5; continue;
 }
 0x1007df5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01007df6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1007df6); }
 let mut pc: u32 = 0x1007df6;
 loop { match pc {
 0x1007df6 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x11b5447u32)); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1007e0a); f_01079abe(cpu); if cpu.returned_to != 0x1007e0a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x112d78fu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x112d793u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x112d78bu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x112d78bu32); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x1007e30); f_01059cfe(cpu); if cpu.returned_to != 0x1007e30 { return; }
  { let v: u32 = 0x1194208u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10ef397u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112d78fu32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x1007e51); f_01059cfe(cpu); if cpu.returned_to != 0x1007e51 { return; }
  { let v: u8 = cpu.r8(0x10ef397u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10ef398u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112d793u32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x1007e75); f_01059cfe(cpu); if cpu.returned_to != 0x1007e75 { return; }
  { let v: u16 = cpu.r16(0x10ef397u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0x130au32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x1007ea3); f_0104bd14(cpu); if cpu.returned_to != 0x1007ea3 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01007ea5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1007ea5); }
 let mut pc: u32 = 0x1007ea5;
 loop { match pc {
 0x1007ea5 => {
  { cpu.sub8(cpu.r8(0x110d637u32), 0x0u8); }
  if cpu.zf { pc = 0x1007eb3; continue; }
  pc = 0x1007eae; continue;
 }
 0x1007eae => {
  cpu.push(0x1007eb3); f_0105eaf1(cpu); if cpu.returned_to != 0x1007eb3 { return; }
  pc = 0x1007eb3; continue;
 }
 0x1007eb3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01007ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1007ee0); }
 let mut pc: u32 = 0x1007ee0;
 loop { match pc {
 0x1007ee0 => {
  cpu.push(0x1007ee5); f_010080c4(cpu); if cpu.returned_to != 0x1007ee5 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1007efe); f_010385ad(cpu); if cpu.returned_to != 0x1007efe { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1007f06); cpu.call(t); if cpu.returned_to != 0x1007f06 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1130f72u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1130f76u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if !cpu.zf { pc = 0x1007f31; continue; }
  pc = 0x1007f2a; continue;
 }
 0x1007f2a => {
  cpu.push(0x1007f2f); f_0107162e(cpu); if cpu.returned_to != 0x1007f2f { return; }
  pc = 0x1007f4f; continue;
 }
 0x1007f31 => {
  { let v: u32 = 0x1194208u32; cpu.eax = v; }
  cpu.push(0x1007f3b); f_0102dc2c(cpu); if cpu.returned_to != 0x1007f3b { return; }
  { let v: u32 = 0xa0u32; cpu.w32(0x1130f72u32, v); }
  { let v: u32 = 0x64u32; cpu.w32(0x1130f76u32, v); }
  pc = 0x1007f4f; continue;
 }
 0x1007f4f => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100809c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100809c); }
 let mut pc: u32 = 0x100809c;
 loop { match pc {
 0x100809c => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u8 = 0xfau8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x10080bc); f_01036a75(cpu); if cpu.returned_to != 0x10080bc { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010080c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10080c4); }
 let mut pc: u32 = 0x10080c4;
 loop { match pc {
 0x10080c4 => {
  cpu.push(0x10080c9); f_010129b5(cpu); if cpu.returned_to != 0x10080c9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d630u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d631u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d632u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d737u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d73bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100825f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100825f); } r_0100825f(cpu, 0x100825f); }
pub fn f_0100828b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100828b); } r_0100825f(cpu, 0x100828b); }
fn r_0100825f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x100825f => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b1bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b05u32, v); }
  cpu.push(0x1008272); f_010080c4(cpu); if cpu.returned_to != 0x1008272 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1130f72u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1130f76u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d61u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3db7u32, v); }
  cpu.push(0x10082a6); f_01007ee0(cpu); if cpu.returned_to != 0x10082a6 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd4d2u32, v); }
  cpu.push(0x10082b9); f_0107b7d2(cpu); if cpu.returned_to != 0x10082b9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b05u32, v); }
  { let v = cpu.flags(); cpu.push(v); }
  cpu.push(0x10082c6); f_01038ea5(cpu); if cpu.returned_to != 0x10082c6 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110566du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  cpu.push(0x10082e0); f_01059ab7(cpu); if cpu.returned_to != 0x10082e0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b05u32, v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  if !cpu.cf { pc = 0x1008302; continue; }
  pc = 0x10082f1; continue;
 }
 0x100828b => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffc7u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffc7u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x10e3d61u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3db7u32, v); }
  cpu.push(0x10082a6); f_01007ee0(cpu); if cpu.returned_to != 0x10082a6 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd4d2u32, v); }
  cpu.push(0x10082b9); f_0107b7d2(cpu); if cpu.returned_to != 0x10082b9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b05u32, v); }
  { let v = cpu.flags(); cpu.push(v); }
  cpu.push(0x10082c6); f_01038ea5(cpu); if cpu.returned_to != 0x10082c6 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110566du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  cpu.push(0x10082e0); f_01059ab7(cpu); if cpu.returned_to != 0x10082e0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b05u32, v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  if !cpu.cf { pc = 0x1008302; continue; }
  pc = 0x10082f1; continue;
 }
 0x10082f1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b1bu32, v); }
  { cpu.sub8(cpu.r8(0x11c5b24u32), 0x2u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1008302; continue; }
  pc = 0x1008301; continue;
 }
 0x1008301 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1008302 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b25u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100830a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100830a); }
 let mut pc: u32 = 0x100830a;
 loop { match pc {
 0x100830a => {
  { cpu.sub8(cpu.r8(0x11c5b1bu32), 0x0u8); }
  if cpu.zf { pc = 0x1008339; continue; }
  pc = 0x1008313; continue;
 }
 0x1008313 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x1008320); f_01008b97(cpu); if cpu.returned_to != 0x1008320 { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1008332; continue; }
  pc = 0x1008329; continue;
 }
 0x1008329 => {
  { cpu.sub8(cpu.r8(0x11c5abbu32), 0x0u8); }
  if !cpu.zf { pc = 0x1008339; continue; }
  pc = 0x1008332; continue;
 }
 0x1008332 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b1bu32, v); }
  pc = 0x1008339; continue;
 }
 0x1008339 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100833a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100833a); }
 let mut pc: u32 = 0x100833a;
 loop { match pc {
 0x100833a => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x100836a; continue; }
  pc = 0x1008343; continue;
 }
 0x1008343 => {
  { cpu.sub8(cpu.r8(0x11c5b25u32), 0x0u8); }
  if cpu.zf { pc = 0x100836a; continue; }
  pc = 0x100834c; continue;
 }
 0x100834c => {
  cpu.push(0x1008351); f_0100851a(cpu); if cpu.returned_to != 0x1008351 { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1008363; continue; }
  pc = 0x100835a; continue;
 }
 0x100835a => {
  { cpu.sub8(cpu.r8(0x11c5abbu32), 0x0u8); }
  if !cpu.zf { pc = 0x100836a; continue; }
  pc = 0x1008363; continue;
 }
 0x1008363 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b25u32, v); }
  pc = 0x100836a; continue;
 }
 0x100836a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100836b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100836b); }
 let mut pc: u32 = 0x100836b;
 loop { match pc {
 0x100836b => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1008375; continue; }
  pc = 0x1008374; continue;
 }
 0x1008374 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1008375 => {
  { cpu.sub8(cpu.r8(0x11c62fau32), 0x0u8); }
  if cpu.zf { pc = 0x10083b9; continue; }
  pc = 0x100837e; continue;
 }
 0x100837e => {
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xcu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10e3e0fu32, v); }
  { let v: u8 = 0xf1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x10083a0); f_01036a75(cpu); if cpu.returned_to != 0x10083a0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { cpu.sub8(cpu.r8(0x11c5abbu32), 0x0u8); }
  if !cpu.zf { pc = 0x10083e4; continue; }
  pc = 0x10083b0; continue;
 }
 0x10083b0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c62fau32, v); }
  pc = 0x10083e4; continue;
 }
 0x10083b9 => {
  { let v: u8 = cpu.r8(0x11c5b2bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10083e4; continue; }
  pc = 0x10083c6; continue;
 }
 0x10083c6 => {
  { let v: u32 = cpu.r32(0x11c62fcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10c6a5cu32), cpu.eax); }
  if cpu.cf { pc = 0x10083e4; continue; }
  pc = 0x10083d3; continue;
 }
 0x10083d3 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c62fau32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x13ecu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c62fcu32, v); }
  pc = 0x10083e4; continue;
 }
 0x10083e4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010083e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10083e5); }
 let mut pc: u32 = 0x10083e5;
 loop { match pc {
 0x10083e5 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x10083ef; continue; }
  pc = 0x10083ee; continue;
 }
 0x10083ee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10083ef => {
  { cpu.sub8(cpu.r8(0x11c633fu32), 0x0u8); }
  if cpu.zf { pc = 0x100842c; continue; }
  pc = 0x10083f8; continue;
 }
 0x10083f8 => {
  { let v: u32 = cpu.r32(0x11c5f42u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3e0fu32, v); }
  { let v: u8 = 0xf3u8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1008415); f_01036a75(cpu); if cpu.returned_to != 0x1008415 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { cpu.sub8(cpu.r8(0x11c5abbu32), 0x0u8); }
  if !cpu.zf { pc = 0x100842c; continue; }
  pc = 0x1008425; continue;
 }
 0x1008425 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c633fu32, v); }
  pc = 0x100842c; continue;
 }
 0x100842c => {
  { cpu.sub8(cpu.r8(0x11c5f46u32), 0x0u8); }
  if cpu.zf { pc = 0x1008481; continue; }
  pc = 0x1008435; continue;
 }
 0x1008435 => {
  cpu.push(0x100843a); f_0107b755(cpu); if cpu.returned_to != 0x100843a { return; }
  { cpu.sub8(cpu.r8(0x11c5f46u32), 0x0u8); }
  if !cpu.sf { pc = 0x100846d; continue; }
  pc = 0x1008443; continue;
 }
 0x1008443 => {
  { let v: u8 = 0xf2u8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1008456); f_01036a75(cpu); if cpu.returned_to != 0x1008456 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { cpu.sub8(cpu.r8(0x11c5abbu32), 0x0u8); }
  if !cpu.zf { pc = 0x1008481; continue; }
  pc = 0x1008466; continue;
 }
 0x1008466 => {
  { let v: u8 = 0x2u8; cpu.w8(0x11c5f46u32, v); }
  pc = 0x100846d; continue;
 }
 0x100846d => {
  { let v: u8 = cpu.dec8(cpu.r8(0x11c5f46u32)); cpu.w8(0x11c5f46u32, v); }
  if !cpu.zf { pc = 0x100847f; continue; }
  pc = 0x1008475; continue;
 }
 0x1008475 => {
  cpu.push(0x100847a); f_01038abf(cpu); if cpu.returned_to != 0x100847a { return; }
  cpu.push(0x100847f); f_01038f34(cpu); if cpu.returned_to != 0x100847f { return; }
  pc = 0x100847f; continue;
 }
 0x100847f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1008481 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008483(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008483); }
 let mut pc: u32 = 0x1008483;
 loop { match pc {
 0x1008483 => {
  { let v: u32 = cpu.r32(0x10e3e0fu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11c5f42u32)); }
  if cpu.zf { pc = 0x10084c6; continue; }
  pc = 0x1008490; continue;
 }
 0x1008490 => {
  { let v: u8 = 0x2u8; cpu.w8(0x11c5f46u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11c5f47u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x10084b0); f_010385ad(cpu); if cpu.returned_to != 0x10084b0 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x10084b8); cpu.call(t); if cpu.returned_to != 0x10084b8 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd361u32, v); }
  pc = 0x10084c6; continue;
 }
 0x10084c6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100851a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100851a); }
 let mut pc: u32 = 0x100851a;
 loop { match pc {
 0x100851a => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  cpu.push(0x100852c); f_0106256d(cpu); if cpu.returned_to != 0x100852c { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u8 = 0xf5u8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1008545); f_01036a75(cpu); if cpu.returned_to != 0x1008545 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100854d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100854d); }
 let mut pc: u32 = 0x100854d;
 loop { match pc {
 0x100854d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3e11u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x100855a); f_01062573(cpu); if cpu.returned_to != 0x100855a { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x11c5b24u32)); cpu.w8(0x11c5b24u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3e0fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x100856c); f_01075512(cpu); if cpu.returned_to != 0x100856c { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3e0fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b26u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x11c5b2bu32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1008592; continue; }
  pc = 0x1008582; continue;
 }
 0x1008582 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x11c5b26u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x1008582; continue; }
  pc = 0x100858d; continue;
 }
 0x100858d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11c5b2bu32, v); }
  pc = 0x1008592; continue;
 }
 0x1008592 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3e0fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x100859e); f_01026d81(cpu); if cpu.returned_to != 0x100859e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008808(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008808); }
 let mut pc: u32 = 0x1008808;
 loop { match pc {
 0x1008808 => {
  cpu.push(0x100880d); f_0100bede(cpu); if cpu.returned_to != 0x100880d { return; }
  cpu.push(0x1008812); f_0100be0f(cpu); if cpu.returned_to != 0x1008812 { return; }
  cpu.push(0x1008817); f_01016940(cpu); if cpu.returned_to != 0x1008817 { return; }
  cpu.push(0x100881c); f_010863ff(cpu); if cpu.returned_to != 0x100881c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100881d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100881d); }
 let mut pc: u32 = 0x100881d;
 loop { match pc {
 0x100881d => {
  cpu.push(0x1008822); f_0100bede(cpu); if cpu.returned_to != 0x1008822 { return; }
  cpu.push(0x1008827); f_0100b6f3(cpu); if cpu.returned_to != 0x1008827 { return; }
  cpu.push(0x100882c); f_01016940(cpu); if cpu.returned_to != 0x100882c { return; }
  cpu.push(0x1008831); f_010863ff(cpu); if cpu.returned_to != 0x1008831 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008832(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008832); }
 let mut pc: u32 = 0x1008832;
 loop { match pc {
 0x1008832 => {
  cpu.push(0x1008837); f_0100bede(cpu); if cpu.returned_to != 0x1008837 { return; }
  cpu.push(0x100883c); f_0100b87a(cpu); if cpu.returned_to != 0x100883c { return; }
  cpu.push(0x1008841); f_01016940(cpu); if cpu.returned_to != 0x1008841 { return; }
  cpu.push(0x1008846); f_010863ff(cpu); if cpu.returned_to != 0x1008846 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008847(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008847); }
 let mut pc: u32 = 0x1008847;
 loop { match pc {
 0x1008847 => {
  cpu.push(0x100884c); f_0100bede(cpu); if cpu.returned_to != 0x100884c { return; }
  cpu.push(0x1008851); f_0100ba01(cpu); if cpu.returned_to != 0x1008851 { return; }
  cpu.push(0x1008856); f_01016940(cpu); if cpu.returned_to != 0x1008856 { return; }
  cpu.push(0x100885b); f_010863ff(cpu); if cpu.returned_to != 0x100885b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008b97(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008b97); }
 let mut pc: u32 = 0x1008b97;
 loop { match pc {
 0x1008b97 => {
  { let v: u8 = 0x5au8; cpu.w8(0x11d02b8u32, v); }
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { cpu.sub8(cpu.r8(0x10fd43du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x1008bda; continue; }
  pc = 0x1008bb4; continue;
 }
 0x1008bb4 => {
  { let v: u16 = cpu.or16((cpu.ebx as u16), 0x8000u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u8 = 0xfdu8; cpu.w8(0x10ef210u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1008bd3); f_01036a75(cpu); if cpu.returned_to != 0x1008bd3 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  pc = 0x1008bda; continue;
 }
 0x1008bda => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008c6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008c6a); }
 let mut pc: u32 = 0x1008c6a;
 loop { match pc {
 0x1008c6a => {
  { cpu.sub8(cpu.r8(0x110d633u32), 0x0u8); }
  if cpu.zf { pc = 0x1008d25; continue; }
  pc = 0x1008c77; continue;
 }
 0x1008c77 => {
  { let v: u32 = cpu.r32(0x110d648u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1008d25; continue; }
  pc = 0x1008c85; continue;
 }
 0x1008c85 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1008c93; continue; }
  pc = 0x1008c90; continue;
 }
 0x1008c90 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x1008c93; continue;
 }
 0x1008c93 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1008d25; continue; }
  pc = 0x1008c9b; continue;
 }
 0x1008c9b => {
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1008d25; continue; }
  pc = 0x1008ca4; continue;
 }
 0x1008ca4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1008d1c; continue; }
  pc = 0x1008ccb; continue;
 }
 0x1008ccb => {
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0xfu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1008d1c; continue; }
  pc = 0x1008cdb; continue;
 }
 0x1008cdb => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1008d1c; continue; }
  pc = 0x1008ce1; continue;
 }
 0x1008ce1 => {
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0xfu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1008d1c; continue; }
  pc = 0x1008cf1; continue;
 }
 0x1008cf1 => {
  { cpu.pushad(); }
  { let v: u32 = 0xfu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xfu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  let t = cpu.r32(0x10e4318u32); cpu.push(0x1008d03); cpu.call(t); if cpu.returned_to != 0x1008d03 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  cpu.push(0x1008d11); f_01059cfe(cpu); if cpu.returned_to != 0x1008d11 { return; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  cpu.push(0x1008d1b); f_0104b988(cpu); if cpu.returned_to != 0x1008d1b { return; }
  { cpu.popad(); }
  pc = 0x1008d1c; continue;
 }
 0x1008d1c => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.poll(); pc = 0x1008c93; continue;
 }
 0x1008d25 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008d26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008d26); }
 let mut pc: u32 = 0x1008d26;
 loop { match pc {
 0x1008d26 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x10ea910u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x6u32); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008d40); }
 let mut pc: u32 = 0x1008d40;
 loop { match pc {
 0x1008d40 => {
  { let v: u32 = 0x10f7dcau32; cpu.esi = v; }
  cpu.push(0x1008d4a); f_0105a317(cpu); if cpu.returned_to != 0x1008d4a { return; }
  { let v: u32 = 0x10f7de2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x10f7dd2u32); cpu.ecx = v; }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10f7de2u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x10f7dd2u32; cpu.edi = v; }
  { cpu.movs(1, true); }
  cpu.push(0x1008d7a); f_0103ef35(cpu); if cpu.returned_to != 0x1008d7a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008de1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008de1); }
 let mut pc: u32 = 0x1008de1;
 loop { match pc {
 0x1008de1 => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x10ef37du32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1008e0f; continue; }
  pc = 0x1008df1; continue;
 }
 0x1008df1 => {
  { let v: u32 = cpu.r32(0x11de584u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  cpu.push(0x1008dfd); f_010586d4(cpu); if cpu.returned_to != 0x1008dfd { return; }
  { let v: u16 = cpu.r16(0x10e3ce3u32).wrapping_sub(0x40u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x10e3ce5u32), 0x20u16); cpu.w16(0x10e3ce5u32, v); }
  pc = 0x1008e3e; continue;
 }
 0x1008e0f => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1008e20; continue; }
  pc = 0x1008e19; continue;
 }
 0x1008e19 => {
  cpu.push(0x1008e1e); f_01059a31(cpu); if cpu.returned_to != 0x1008e1e { return; }
  pc = 0x1008e3e; continue;
 }
 0x1008e20 => {
  { cpu.sub16(cpu.r16(0x10e3debu32), 0x0u16); }
  if cpu.zf { pc = 0x1008e3e; continue; }
  pc = 0x1008e2a; continue;
 }
 0x1008e2a => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1008e3e; continue; }
  pc = 0x1008e33; continue;
 }
 0x1008e33 => {
  { let v: u16 = cpu.r16(0x10e3debu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1008e3e); f_01059200(cpu); if cpu.returned_to != 0x1008e3e { return; }
  pc = 0x1008e3e; continue;
 }
 0x1008e3e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008e5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008e5a); }
 let mut pc: u32 = 0x1008e5a;
 loop { match pc {
 0x1008e5a => {
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1008e7a); f_01018e6e(cpu); if cpu.returned_to != 0x1008e7a { return; }
  if !cpu.cf { pc = 0x1008e7d; continue; }
  pc = 0x1008e7c; continue;
 }
 0x1008e7c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1008e7d => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef350u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008e85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008e85); }
 let mut pc: u32 = 0x1008e85;
 loop { match pc {
 0x1008e85 => {
  cpu.push(0x1008e8a); f_0103ec28(cpu); if cpu.returned_to != 0x1008e8a { return; }
  { let v: u32 = 0x10c5364u32; cpu.esi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  pc = 0x1008e94; continue;
 }
 0x1008e94 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1008eb7; continue; }
  pc = 0x1008e9b; continue;
 }
 0x1008e9b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x25u8); }
  if cpu.zf { pc = 0x1008eb5; continue; }
  pc = 0x1008ead; continue;
 }
 0x1008ead => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x1008eb5); f_010396a0(cpu); if cpu.returned_to != 0x1008eb5 { return; }
  pc = 0x1008eb5; continue;
 }
 0x1008eb5 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1008eb7; continue;
 }
 0x1008eb7 => {
  { let v: u32 = cpu.add32(cpu.esi, 0xau32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1008e94; continue; }
  pc = 0x1008ebc; continue;
 }
 0x1008ebc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008ebd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008ebd); }
 let mut pc: u32 = 0x1008ebd;
 loop { match pc {
 0x1008ebd => {
  { cpu.sub8(cpu.r8(0x10ef35au32), 0x2u8); }
  if cpu.zf { pc = 0x1008ee4; continue; }
  pc = 0x1008ec6; continue;
 }
 0x1008ec6 => {
  { cpu.and8(cpu.r8(0x10ee968u32), 0x4u8); }
  if cpu.zf { pc = 0x1008ee4; continue; }
  pc = 0x1008ecf; continue;
 }
 0x1008ecf => {
  { cpu.sub8(cpu.r8(0x10ef35bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1008ee4; continue; }
  pc = 0x1008ed8; continue;
 }
 0x1008ed8 => {
  cpu.push(0x1008edd); f_01008de1(cpu); if cpu.returned_to != 0x1008edd { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef35bu32, v); }
  pc = 0x1008ee4; continue;
 }
 0x1008ee4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008ee5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008ee5); }
 let mut pc: u32 = 0x1008ee5;
 loop { match pc {
 0x1008ee5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef35cu32, v); }
  cpu.push(0x1008ef1); f_01068db3(cpu); if cpu.returned_to != 0x1008ef1 { return; }
  if cpu.cf { pc = 0x1008f0b; continue; }
  pc = 0x1008ef3; continue;
 }
 0x1008ef3 => {
  { let v: u16 = 0xf1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1008f00); f_01059200(cpu); if cpu.returned_to != 0x1008f00 { return; }
  cpu.push(0x1008f05); f_01057945(cpu); if cpu.returned_to != 0x1008f05 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1008f0b); f_01057a1d(cpu); if cpu.returned_to != 0x1008f0b { return; }
  pc = 0x1008f0b; continue;
 }
 0x1008f0b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01008fca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1008fca); }
 let mut pc: u32 = 0x1008fca;
 loop { match pc {
 0x1008fca => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x1008fd4; continue; }
  pc = 0x1008fd3; continue;
 }
 0x1008fd3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1008fd4 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x100900f; continue; }
  pc = 0x1008fdd; continue;
 }
 0x1008fdd => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x100900f; continue; }
  pc = 0x1008fe6; continue;
 }
 0x1008fe6 => {
  { let v: u32 = cpu.r32(0x10e4174u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { pc = 0x100900f; continue; }
  pc = 0x1008ff0; continue;
 }
 0x1008ff0 => {
  { cpu.sub8(cpu.r8(0x5du32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x100900f; continue; }
  pc = 0x1008ff6; continue;
 }
 0x1008ff6 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x11757b7u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100900f; continue; }
  pc = 0x1009009; continue;
 }
 0x1009009 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x100900f); f_01059200(cpu); if cpu.returned_to != 0x100900f { return; }
  pc = 0x100900f; continue;
 }
 0x100900f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01009010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1009010); }
 let mut pc: u32 = 0x1009010;
 loop { match pc {
 0x1009010 => {
  { cpu.sub8(cpu.r8(0x10ef37cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1009058; continue; }
  pc = 0x1009019; continue;
 }
 0x1009019 => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if !cpu.zf { pc = 0x1009058; continue; }
  pc = 0x1009022; continue;
 }
 0x1009022 => {
  { let v: u32 = cpu.r32(0x10e4174u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { pc = 0x1009058; continue; }
  pc = 0x100902c; continue;
 }
 0x100902c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1009032); f_01057945(cpu); if cpu.returned_to != 0x1009032 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x11757b7u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1009058; continue; }
  pc = 0x1009046; continue;
 }
 0x1009046 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x100904d); f_01059200(cpu); if cpu.returned_to != 0x100904d { return; }
  cpu.push(0x1009052); f_01057945(cpu); if cpu.returned_to != 0x1009052 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1009058); f_01057a1d(cpu); if cpu.returned_to != 0x1009058 { return; }
  pc = 0x1009058; continue;
 }
 0x1009058 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01009059(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1009059); }
 let mut pc: u32 = 0x1009059;
 loop { match pc {
 0x1009059 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1009097; continue; }
  pc = 0x1009062; continue;
 }
 0x1009062 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x110d6e0u32)); cpu.w32(0x110d6e0u32, v); }
  { cpu.sub32(cpu.r32(0x110d6e0u32), 0x22u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1009097; continue; }
  pc = 0x1009071; continue;
 }
 0x1009071 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1009097; continue; }
  pc = 0x100907b; continue;
 }
 0x100907b => {
  cpu.push(0x1009080); f_010576b2(cpu); if cpu.returned_to != 0x1009080 { return; }
  { let v = cpu.r32(0x11cfc8du32); cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11cfc8du32, v); }
  cpu.push(0x1009091); f_01009098(cpu); if cpu.returned_to != 0x1009091 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x11cfc8du32, v); }
  pc = 0x1009097; continue;
 }
 0x1009097 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01009098(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1009098); }
 let mut pc: u32 = 0x1009098;
 loop { match pc {
 0x1009098 => {
  { cpu.sub32(cpu.r32(0x11cfc8du32), 0x0u32); }
  if cpu.zf { pc = 0x1009234; continue; }
  pc = 0x10090a5; continue;
 }
 0x10090a5 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1009234; continue; }
  pc = 0x10090b2; continue;
 }
 0x10090b2 => {
  { let v: u32 = cpu.r32(0x11cfc8du32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x10090d4; continue; }
  pc = 0x10090c7; continue;
 }
 0x10090c7 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x1009234; continue; }
  pc = 0x10090d4; continue;
 }
 0x10090d4 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1009234; continue; }
  pc = 0x10090ed; continue;
 }
 0x10090ed => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c0u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x3u8); }
  if !cpu.zf { pc = 0x1009125; continue; }
  pc = 0x1009111; continue;
 }
 0x1009111 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x4d42u16); }
  if !cpu.zf { pc = 0x1009154; continue; }
  pc = 0x1009118; continue;
 }
 0x1009118 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.eax)), 0x54u8); }
  if !cpu.zf { pc = 0x1009154; continue; }
  pc = 0x100911e; continue;
 }
 0x100911e => {
  { let v: u32 = cpu.r32(0x1131394u32); cpu.eax = v; }
  pc = 0x1009154; continue;
 }
 0x1009125 => {
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x2u8); }
  if !cpu.zf { pc = 0x1009142; continue; }
  pc = 0x100912e; continue;
 }
 0x100912e => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x5456u16); }
  if !cpu.zf { pc = 0x1009154; continue; }
  pc = 0x1009135; continue;
 }
 0x1009135 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.eax)), 0x42u8); }
  if !cpu.zf { pc = 0x1009154; continue; }
  pc = 0x100913b; continue;
 }
 0x100913b => {
  { let v: u32 = cpu.r32(0x1131394u32); cpu.eax = v; }
  pc = 0x1009154; continue;
 }
 0x1009142 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x5041u16); }
  if !cpu.zf { pc = 0x1009154; continue; }
  pc = 0x1009149; continue;
 }
 0x1009149 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.eax)), 0x43u8); }
  if !cpu.zf { pc = 0x1009154; continue; }
  pc = 0x100914f; continue;
 }
 0x100914f => {
  { let v: u32 = cpu.r32(0x1131394u32); cpu.eax = v; }
  pc = 0x1009154; continue;
 }
 0x1009154 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x6d616c46u32); }
  if !cpu.zf { pc = 0x1009172; continue; }
  pc = 0x100915c; continue;
 }
 0x100915c => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.eax)), 0x7265u16); }
  if !cpu.zf { pc = 0x1009172; continue; }
  pc = 0x1009164; continue;
 }
 0x1009164 => {
  { let v: u32 = 0x6f727950u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x73u8; cpu.w8(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.eax), v); }
  pc = 0x1009172; continue;
 }
 0x1009172 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0xfffffff6u32; cpu.eax = v; }
  { let v: u32 = 0xfffffff5u32; cpu.ebx = v; }
  { let v: u32 = 0xeu32; cpu.w32(0x110d6dcu32, v); }
  { let v: u32 = 0x2ceu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10091ad; continue; }
  pc = 0x1009193; continue;
 }
 0x1009193 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x10091ad; continue; }
  pc = 0x1009199; continue;
 }
 0x1009199 => {
  { let v: u32 = 0x2ceu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffff8u32; cpu.eax = v; }
  { let v: u32 = 0x12u32; cpu.w32(0x110d6dcu32, v); }
  pc = 0x10091ad; continue;
 }
 0x10091ad => {
  { let v: u32 = cpu.r8(0x43u32.wrapping_add(cpu.edi)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10091d6); f_01009235(cpu); if cpu.returned_to != 0x10091d6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10091ea); f_0104bba6(cpu); if cpu.returned_to != 0x10091ea { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x110d6dcu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1009206); f_0104b9d0(cpu); if cpu.returned_to != 0x1009206 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x43u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2ceu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x1128de1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1128de3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1009234); cpu.call(t); if cpu.returned_to != 0x1009234 { return; }
  pc = 0x1009234; continue;
 }
 0x1009234 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01009235(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1009235); }
 let mut pc: u32 = 0x1009235;
 loop { match pc {
 0x1009235 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1009255; continue; }
  pc = 0x100923f; continue;
 }
 0x100923f => {
  { let v: u8 = cpu.r8(0x11294e8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x110d8fdu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x110d8ffu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1009255; continue;
 }
 0x1009255 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01009256(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1009256); }
 let mut pc: u32 = 0x1009256;
 loop { match pc {
 0x1009256 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10092e1; continue; }
  pc = 0x1009263; continue;
 }
 0x1009263 => {
  { cpu.sub32(cpu.r32(0x11cfc91u32), 0x0u32); }
  if cpu.zf { pc = 0x10092e1; continue; }
  pc = 0x1009270; continue;
 }
 0x1009270 => {
  { let v: u32 = cpu.r32(0x11cfc91u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), 0x20u8); }
  if cpu.zf { pc = 0x10092e1; continue; }
  pc = 0x100927f; continue;
 }
 0x100927f => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = 0xfffffff6u32; cpu.eax = v; }
  { let v: u32 = 0xfffffff5u32; cpu.ebx = v; }
  { let v: u32 = 0x2ceu32; cpu.ecx = v; }
  { let v: u32 = 0x1cu32; cpu.w32(0x110d6dcu32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10092c5); f_0104bba6(cpu); if cpu.returned_to != 0x10092c5 { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x110d6dcu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10092e1); f_0104b9d0(cpu); if cpu.returned_to != 0x10092e1 { return; }
  pc = 0x10092e1; continue;
 }
 0x10092e1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010092e2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10092e2); } r_010092e2(cpu, 0x10092e2); }
pub fn f_0100938d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100938d); } r_010092e2(cpu, 0x100938d); }
fn r_010092e2(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10092e2 => {
  { cpu.sub8(cpu.r8(0x11bd2f6u32), 0x0u8); }
  if cpu.zf { pc = 0x10092f3; continue; }
  pc = 0x10092eb; continue;
 }
 0x10092eb => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd361u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10092f3 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e41ecu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e41f0u32, v); }
  cpu.push(0x100930c); f_01057d72(cpu); if cpu.returned_to != 0x100930c { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e9bu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e9du32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d6ecu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x110d6eeu32, v); }
  cpu.push(0x100932b); f_01057d3c(cpu); if cpu.returned_to != 0x100932b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x110d6f7u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x110d6fbu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x110eab8u32), 0x0u32); }
  if cpu.zf { pc = 0x10093ce; continue; }
  pc = 0x1009345; continue;
 }
 0x1009345 => {
  { let v: u16 = cpu.r16(0x110d6ecu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110d6eeu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.w16(0x110ed8bu32, v); }
  { let v: u16 = 0xe8u16; cpu.w16(0x110ed8du32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x110ed8fu32, v); }
  { let v: u16 = 0x11u16; cpu.w16(0x110ed91u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10093a3; continue; }
  pc = 0x100937f; continue;
 }
 0x100937f => {
  { let v: u16 = 0xe0u16; cpu.w16(0x110ed8bu32, v); }
  { let v: u16 = 0x188u16; cpu.w16(0x110ed8du32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x110ed8fu32, v); }
  { let v: u16 = 0x11u16; cpu.w16(0x110ed91u32, v); }
  pc = 0x10093a3; continue;
 }
 0x100938d => {
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = 0x8u16; cpu.w16(0x110ed8fu32, v); }
  { let v: u16 = 0x11u16; cpu.w16(0x110ed91u32, v); }
  pc = 0x10093a3; continue;
 }
 0x10093a3 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x110ed8bu32)); }
  if (cpu.sf != cpu.of) { pc = 0x10093ce; continue; }
  pc = 0x10093ac; continue;
 }
 0x10093ac => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x110ed8du32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10093ce; continue; }
  pc = 0x10093b5; continue;
 }
 0x10093b5 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x110ed8fu32)); }
  if (cpu.sf != cpu.of) { pc = 0x10093ce; continue; }
  pc = 0x10093be; continue;
 }
 0x10093be => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x110ed91u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10093ce; continue; }
  pc = 0x10093c7; continue;
 }
 0x10093c7 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1009caf; continue;
 }
 0x10093ce => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3e9bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1009caf; continue; }
  pc = 0x10093e3; continue;
 }
 0x10093e3 => {
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1009caf; continue; }
  pc = 0x10093f7; continue;
 }
 0x10093f7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11cfc91u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d6e8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x110d6eau32, v); }
  cpu.push(0x1009423); f_010191d1(cpu); if cpu.returned_to != 0x1009423 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x100943b; continue; }
  pc = 0x1009427; continue;
 }
 0x1009427 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x145fu16); }
  if cpu.zf { pc = 0x100943b; continue; }
  pc = 0x100942e; continue;
 }
 0x100942e => {
  { let v: u32 = cpu.esi; cpu.w32(0x11cfc91u32, v); }
  { let v: u8 = 0x32u8; cpu.w8(0x11cfc95u32, v); }
  pc = 0x100943b; continue;
 }
 0x100943b => {
  { let v: u32 = 0x0u32; cpu.w32(0x11cfc89u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11cfc8du32, v); }
  cpu.push(0x1009454); f_01068db3(cpu); if cpu.returned_to != 0x1009454 { return; }
  if cpu.cf { pc = 0x100952b; continue; }
  pc = 0x100945a; continue;
 }
 0x100945a => {
  { let v: u32 = cpu.ebp; cpu.w32(0x11cfc89u32, v); }
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x100952b; continue; }
  pc = 0x100946d; continue;
 }
 0x100946d => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x100952b; continue; }
  pc = 0x1009477; continue;
 }
 0x1009477 => {
  cpu.push(0x100947c); f_01058cae(cpu); if cpu.returned_to != 0x100947c { return; }
  if cpu.cf { pc = 0x100952b; continue; }
  pc = 0x1009482; continue;
 }
 0x1009482 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x11cfc89u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11cfc8du32, v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x100952b; continue; }
  pc = 0x100949f; continue;
 }
 0x100949f => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1009caf; continue; }
  pc = 0x10094ad; continue;
 }
 0x10094ad => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  cpu.push(0x10094b4); f_010576b2(cpu); if cpu.returned_to != 0x10094b4 { return; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { pc = 0x100951a; continue; }
  pc = 0x10094bc; continue;
 }
 0x10094bc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x10094c4; continue; }
  pc = 0x10094c2; continue;
 }
 0x10094c2 => {
  pc = 0x10094e9; continue;
 }
 0x10094c4 => {
  cpu.push(0x10094c9); f_01053d3f(cpu); if cpu.returned_to != 0x10094c9 { return; }
  if cpu.cf { pc = 0x1009c83; continue; }
  pc = 0x10094cf; continue;
 }
 0x10094cf => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10094dd); f_010191d1(cpu); if cpu.returned_to != 0x10094dd { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1009cdb; continue; }
  pc = 0x10094e4; continue;
 }
 0x10094e4 => {
  pc = 0x1009c0c; continue;
 }
 0x10094e9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1009c83; continue; }
  pc = 0x10094f3; continue;
 }
 0x10094f3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x1009c83; continue; }
  pc = 0x1009506; continue;
 }
 0x1009506 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if !cpu.zf { pc = 0x1009cdb; continue; }
  pc = 0x1009515; continue;
 }
 0x1009515 => {
  pc = 0x1009c83; continue;
 }
 0x100951a => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1009c83; continue; }
  pc = 0x100952b; continue;
 }
 0x100952b => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1009caf; continue; }
  pc = 0x1009539; continue;
 }
 0x1009539 => {
  cpu.push(0x100953e); f_010576b2(cpu); if cpu.returned_to != 0x100953e { return; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x1009659; continue; }
  pc = 0x1009548; continue;
 }
 0x1009548 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x1009a2b; continue; }
  pc = 0x1009552; continue;
 }
 0x1009552 => {
  cpu.push(0x1009557); f_01068d2a(cpu); if cpu.returned_to != 0x1009557 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1009579; continue; }
  pc = 0x100955b; continue;
 }
 0x100955b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x100977b; continue; }
  pc = 0x1009565; continue;
 }
 0x1009565 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1009a2b; continue; }
  pc = 0x100956f; continue;
 }
 0x100956f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1009a2b; continue; }
  pc = 0x1009579; continue;
 }
 0x1009579 => {
  cpu.push(0x100957e); f_01057d3c(cpu); if cpu.returned_to != 0x100957e { return; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if !cpu.zf { pc = 0x1009659; continue; }
  pc = 0x1009588; continue;
 }
 0x1009588 => {
  cpu.push(0x100958d); f_0100bf29(cpu); if cpu.returned_to != 0x100958d { return; }
  if cpu.cf { pc = 0x1009d6e; continue; }
  pc = 0x1009593; continue;
 }
 0x1009593 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x110d6f7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d6fbu32); cpu.ebx = v; }
  cpu.push(0x10095a5); f_01018f3f(cpu); if cpu.returned_to != 0x10095a5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x10095c9; continue; }
  pc = 0x10095a9; continue;
 }
 0x10095a9 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1009c0c; continue; }
  pc = 0x10095c4; continue;
 }
 0x10095c4 => {
  pc = 0x1009e0b; continue;
 }
 0x10095c9 => {
  cpu.push(0x10095ce); f_0101acb0(cpu); if cpu.returned_to != 0x10095ce { return; }
  if !cpu.cf { pc = 0x10095f6; continue; }
  pc = 0x10095d0; continue;
 }
 0x10095d0 => {
  { let v: u32 = cpu.esi; cpu.w32(0x11b51aeu32, v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10095e4); f_0101acb0(cpu); if cpu.returned_to != 0x10095e4 { return; }
  { cpu.sub32(cpu.r32(0x11b51aeu32), cpu.esi); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x1009c0c; continue; }
  pc = 0x10095f1; continue;
 }
 0x10095f1 => {
  pc = 0x1009e0b; continue;
 }
 0x10095f6 => {
  { let v: u16 = cpu.r16(0x10e3e9bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1009616); f_01018baf(cpu); if cpu.returned_to != 0x1009616 { return; }
  if !cpu.cf { pc = 0x100963c; continue; }
  pc = 0x1009618; continue;
 }
 0x1009618 => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1009c0c; continue; }
  pc = 0x1009624; continue;
 }
 0x1009624 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x1009c0c; continue; }
  pc = 0x1009637; continue;
 }
 0x1009637 => {
  pc = 0x1009e0b; continue;
 }
 0x100963c => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x100964e); f_01058fff(cpu); if cpu.returned_to != 0x100964e { return; }
  if cpu.zf { pc = 0x1009c0c; continue; }
  pc = 0x1009654; continue;
 }
 0x1009654 => {
  pc = 0x1009d9f; continue;
 }
 0x1009659 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x10096eb; continue; }
  pc = 0x1009663; continue;
 }
 0x1009663 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x10096eb; continue; }
  pc = 0x100966d; continue;
 }
 0x100966d => {
  cpu.push(0x1009672); f_01057d3c(cpu); if cpu.returned_to != 0x1009672 { return; }
  cpu.push(0x1009677); f_01018e6e(cpu); if cpu.returned_to != 0x1009677 { return; }
  if !cpu.cf { pc = 0x10096a2; continue; }
  pc = 0x1009679; continue;
 }
 0x1009679 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10096a2; continue; }
  pc = 0x100968f; continue;
 }
 0x100968f => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1au32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1009c0c; continue; }
  pc = 0x100969d; continue;
 }
 0x100969d => {
  pc = 0x1009e3c; continue;
 }
 0x10096a2 => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x4u8; cpu.w8(0x10f78beu32, v); }
  cpu.push(0x10096bb); f_010654da(cpu); if cpu.returned_to != 0x10096bb { return; }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10f78beu32, v); }
  if cpu.cf { pc = 0x1009e3c; continue; }
  pc = 0x10096ce; continue;
 }
 0x10096ce => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10096e0); f_01058fff(cpu); if cpu.returned_to != 0x10096e0 { return; }
  if cpu.zf { pc = 0x1009c0c; continue; }
  pc = 0x10096e6; continue;
 }
 0x10096e6 => {
  pc = 0x1009d9f; continue;
 }
 0x10096eb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10096f2); f_010576b2(cpu); if cpu.returned_to != 0x10096f2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x1009761; continue; }
  pc = 0x10096fe; continue;
 }
 0x10096fe => {
  cpu.push(0x1009703); f_01039557(cpu); if cpu.returned_to != 0x1009703 { return; }
  if !cpu.cf { pc = 0x1009761; continue; }
  pc = 0x1009709; continue;
 }
 0x1009709 => {
  { let v: u32 = cpu.r32(0x110d6e4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x10c5a55u32); }
  if cpu.zf { pc = 0x100973e; continue; }
  pc = 0x1009718; continue;
 }
 0x1009718 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x10c5a6eu32); }
  if cpu.zf { pc = 0x100973e; continue; }
  pc = 0x1009721; continue;
 }
 0x1009721 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x1009761; continue; }
  pc = 0x1009727; continue;
 }
 0x1009727 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x1009761; continue; }
  pc = 0x100972d; continue;
 }
 0x100972d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1009761; continue; }
  pc = 0x1009733; continue;
 }
 0x1009733 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1009761; continue; }
  pc = 0x1009739; continue;
 }
 0x1009739 => {
  pc = 0x1009963; continue;
 }
 0x100973e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x1009754; continue; }
  pc = 0x1009744; continue;
 }
 0x1009744 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1009754; continue; }
  pc = 0x100974a; continue;
 }
 0x100974a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1009963; continue; }
  pc = 0x1009754; continue;
 }
 0x1009754 => {
  cpu.push(0x1009759); f_01068d2a(cpu); if cpu.returned_to != 0x1009759 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1009963; continue; }
  pc = 0x1009761; continue;
 }
 0x1009761 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcc47u32, v); }
  cpu.push(0x100976d); f_01059141(cpu); if cpu.returned_to != 0x100976d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fcc47u32, v); }
  if !cpu.cf { pc = 0x100977b; continue; }
  pc = 0x1009776; continue;
 }
 0x1009776 => {
  pc = 0x1009d9f; continue;
 }
 0x100977b => {
  { let v: u16 = cpu.r16(0x10e3e9bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x100979b); f_01018baf(cpu); if cpu.returned_to != 0x100979b { return; }
  if !cpu.cf { pc = 0x10097f8; continue; }
  pc = 0x10097a1; continue;
 }
 0x10097a1 => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10097ee; continue; }
  pc = 0x10097a9; continue;
 }
 0x10097a9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x10097f8; continue; }
  pc = 0x10097af; continue;
 }
 0x10097af => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.zf { pc = 0x1009e0b; continue; }
  pc = 0x10097c2; continue;
 }
 0x10097c2 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x1009a2b; continue; }
  pc = 0x10097cc; continue;
 }
 0x10097cc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x10097e9; continue; }
  pc = 0x10097d2; continue;
 }
 0x10097d2 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x10097e9; continue; }
  pc = 0x10097db; continue;
 }
 0x10097db => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x10097e9; continue; }
  pc = 0x10097e4; continue;
 }
 0x10097e4 => {
  pc = 0x1009a2b; continue;
 }
 0x10097e9 => {
  pc = 0x1009d0c; continue;
 }
 0x10097ee => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x1009a2b; continue; }
  pc = 0x10097f8; continue;
 }
 0x10097f8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1009803); f_01009e97(cpu); if cpu.returned_to != 0x1009803 { return; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x1009a2b; continue; }
  pc = 0x1009811; continue;
 }
 0x1009811 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x100981c); f_0101acb0(cpu); if cpu.returned_to != 0x100981c { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1009a2b; continue; }
  pc = 0x1009828; continue;
 }
 0x1009828 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1009868; continue; }
  pc = 0x100982e; continue;
 }
 0x100982e => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x11c59c5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x1009867; continue; }
  pc = 0x100983d; continue;
 }
 0x100983d => {
  cpu.push(0x1009842); f_0101ac6c(cpu); if cpu.returned_to != 0x1009842 { return; }
  if !cpu.cf { pc = 0x1009867; continue; }
  pc = 0x1009844; continue;
 }
 0x1009844 => {
  cpu.push(0x1009849); f_0101a1fc(cpu); if cpu.returned_to != 0x1009849 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x100985b); f_0104885a(cpu); if cpu.returned_to != 0x100985b { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1009c0c; continue; }
  pc = 0x1009862; continue;
 }
 0x1009862 => {
  pc = 0x1009d0c; continue;
 }
 0x1009867 => {
  { cpu.popad(); }
  pc = 0x1009868; continue;
 }
 0x1009868 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x110d6e8u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x110d6eau32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1009918; continue; }
  pc = 0x100988d; continue;
 }
 0x100988d => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x1009918; continue; }
  pc = 0x100989a; continue;
 }
 0x100989a => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x1u8); }
  if cpu.zf { pc = 0x10098f5; continue; }
  pc = 0x10098a3; continue;
 }
 0x10098a3 => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x2u8); }
  if cpu.zf { pc = 0x10098d2; continue; }
  pc = 0x10098ac; continue;
 }
 0x10098ac => {
  { cpu.sub8(cpu.r8(0x10fd440u32), 0xffu8); }
  if cpu.zf { pc = 0x10098c0; continue; }
  pc = 0x10098b5; continue;
 }
 0x10098b5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1009963; continue;
 }
 0x10098c0 => {
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1009936; continue;
 }
 0x10098d2 => {
  { cpu.sub8(cpu.r8(0x10fd43fu32), 0xffu8); }
  if cpu.zf { pc = 0x10098e6; continue; }
  pc = 0x10098db; continue;
 }
 0x10098db => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1009963; continue;
 }
 0x10098e6 => {
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1009936; continue;
 }
 0x10098f5 => {
  { cpu.sub8(cpu.r8(0x10fd43du32), 0xffu8); }
  if cpu.zf { pc = 0x1009909; continue; }
  pc = 0x10098fe; continue;
 }
 0x10098fe => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1009963; continue;
 }
 0x1009909 => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1009936; continue;
 }
 0x1009918 => {
  { cpu.sub8(cpu.r8(0x10fd43eu32), 0xffu8); }
  if cpu.zf { pc = 0x1009929; continue; }
  pc = 0x1009921; continue;
 }
 0x1009921 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1009963; continue;
 }
 0x1009929 => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1009936; continue;
 }
 0x1009936 => {
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x1009947); f_01018ffd(cpu); if cpu.returned_to != 0x1009947 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1009a2b; continue; }
  pc = 0x1009953; continue;
 }
 0x1009953 => {
  cpu.push(0x1009958); f_01057d3c(cpu); if cpu.returned_to != 0x1009958 { return; }
  cpu.push(0x100995d); f_0100258c(cpu); if cpu.returned_to != 0x100995d { return; }
  if cpu.cf { pc = 0x1009a2b; continue; }
  pc = 0x1009963; continue;
 }
 0x1009963 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x10099e9; continue; }
  pc = 0x100996d; continue;
 }
 0x100996d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x10099c5; continue; }
  pc = 0x1009973; continue;
 }
 0x1009973 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x10099e9; continue; }
  pc = 0x1009980; continue;
 }
 0x1009980 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x1009a2b; continue; }
  pc = 0x100998d; continue;
 }
 0x100998d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1009998); f_0101ac6c(cpu); if cpu.returned_to != 0x1009998 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x10099c5; continue; }
  pc = 0x10099a0; continue;
 }
 0x10099a0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112ad0cu32, v); }
  cpu.push(0x10099b2); f_01018e6e(cpu); if cpu.returned_to != 0x10099b2 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112ad0cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x1009a2b; continue; }
  pc = 0x10099c5; continue;
 }
 0x10099c5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x1009a2b; continue; }
  pc = 0x10099cb; continue;
 }
 0x10099cb => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x1009b09; continue; }
  pc = 0x10099d5; continue;
 }
 0x10099d5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x1009b09; continue; }
  pc = 0x10099df; continue;
 }
 0x10099df => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x1009b09; continue; }
  pc = 0x10099e9; continue;
 }
 0x10099e9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1009d0c; continue; }
  pc = 0x10099f3; continue;
 }
 0x10099f3 => {
  { let v: u16 = cpu.r16(0x10e3e9bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x1009a20); f_0104885a(cpu); if cpu.returned_to != 0x1009a20 { return; }
  if cpu.cf { pc = 0x1009c0c; continue; }
  pc = 0x1009a26; continue;
 }
 0x1009a26 => {
  pc = 0x1009d0c; continue;
 }
 0x1009a2b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x1009bcd; continue; }
  pc = 0x1009a35; continue;
 }
 0x1009a35 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1009a4b; continue; }
  pc = 0x1009a3b; continue;
 }
 0x1009a3b => {
  cpu.push(0x1009a40); f_0100bf29(cpu); if cpu.returned_to != 0x1009a40 { return; }
  if cpu.cf { pc = 0x1009d6e; continue; }
  pc = 0x1009a46; continue;
 }
 0x1009a46 => {
  pc = 0x1009adf; continue;
 }
 0x1009a4b => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10e3e97u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.r32(0x11cfc89u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1009b09; continue; }
  pc = 0x1009a7b; continue;
 }
 0x1009a7b => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if cpu.zf { pc = 0x1009b09; continue; }
  pc = 0x1009a8c; continue;
 }
 0x1009a8c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1009d0c; continue; }
  pc = 0x1009a96; continue;
 }
 0x1009a96 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3e93u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3e95u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3e97u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  if (cpu.cf || cpu.zf) { pc = 0x1009adf; continue; }
  pc = 0x1009add; continue;
 }
 0x1009add => {
  pc = 0x1009b09; continue;
 }
 0x1009adf => {
  { let v: u32 = cpu.r32(0x11cfc89u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x1009b09; continue; }
  pc = 0x1009ae9; continue;
 }
 0x1009ae9 => {
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1009b09; continue; }
  pc = 0x1009af2; continue;
 }
 0x1009af2 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x1009b09; continue; }
  pc = 0x1009af8; continue;
 }
 0x1009af8 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1009d0c; continue; }
  pc = 0x1009b09; continue;
 }
 0x1009b09 => {
  cpu.push(0x1009b0e); f_010576b2(cpu); if cpu.returned_to != 0x1009b0e { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1009bc2; continue; }
  pc = 0x1009b18; continue;
 }
 0x1009b18 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1009c38; continue; }
  pc = 0x1009b22; continue;
 }
 0x1009b22 => {
  { let v: u16 = cpu.r16(0x10e3e9bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x110d769u32, v); }
  cpu.push(0x1009b49); f_010191d1(cpu); if cpu.returned_to != 0x1009b49 { return; }
  if !cpu.cf { pc = 0x1009bc2; continue; }
  pc = 0x1009b4f; continue;
 }
 0x1009b4f => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d769u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x50u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x1009b84); f_01018ffd(cpu); if cpu.returned_to != 0x1009b84 { return; }
  if !cpu.cf { pc = 0x1009b8f; continue; }
  pc = 0x1009b86; continue;
 }
 0x1009b86 => {
  { let v: u16 = 0x1u16; cpu.w16(0x10e3e93u32, v); }
  pc = 0x1009b8f; continue;
 }
 0x1009b8f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10c5974u32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f746fu32.wrapping_add(cpu.eax)), 0x1u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1009d6e; continue; }
  pc = 0x1009bb2; continue;
 }
 0x1009bb2 => {
  if !cpu.zf { pc = 0x1009bc2; continue; }
  pc = 0x1009bb4; continue;
 }
 0x1009bb4 => {
  { cpu.sub8(cpu.r8(0x10e3e93u32), 0x0u8); }
  if !cpu.zf { pc = 0x1009c0c; continue; }
  pc = 0x1009bbd; continue;
 }
 0x1009bbd => {
  pc = 0x1009d3d; continue;
 }
 0x1009bc2 => {
  cpu.push(0x1009bc7); f_0100bf29(cpu); if cpu.returned_to != 0x1009bc7 { return; }
  if cpu.cf { pc = 0x1009d6e; continue; }
  pc = 0x1009bcd; continue;
 }
 0x1009bcd => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1009caf; continue; }
  pc = 0x1009bdb; continue;
 }
 0x1009bdb => {
  cpu.push(0x1009be0); f_010576b2(cpu); if cpu.returned_to != 0x1009be0 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1009c0c; continue; }
  pc = 0x1009be6; continue;
 }
 0x1009be6 => {
  { let v: u16 = cpu.r16(0x10e3e9bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1009c06); f_01058fff(cpu); if cpu.returned_to != 0x1009c06 { return; }
  if !cpu.cf { pc = 0x1009d9f; continue; }
  pc = 0x1009c0c; continue;
 }
 0x1009c0c => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x6u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0xae4u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0xfffau16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0xfffau16; cpu.w16(0x10e3d61u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009c38 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { cpu.poll(); pc = 0x1009c0c; continue; }
  pc = 0x1009c3e; continue;
 }
 0x1009c3e => {
  cpu.push(0x1009c43); f_01057d3c(cpu); if cpu.returned_to != 0x1009c43 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x1009c55); f_0104885a(cpu); if cpu.returned_to != 0x1009c55 { return; }
  if cpu.cf { cpu.poll(); pc = 0x1009c0c; continue; }
  pc = 0x1009c57; continue;
 }
 0x1009c57 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0xaecu16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x10e3d61u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009c83 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x7u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0xae8u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d61u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009caf => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d61u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009cdb => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x5u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0xfff9u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3d61u32, v); }
  cpu.push(0x1009d0b); f_0104eefa(cpu); if cpu.returned_to != 0x1009d0b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009d0c => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x10e3d61u32, v); }
  cpu.push(0x1009d3c); f_0104eefa(cpu); if cpu.returned_to != 0x1009d3c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009d3d => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0x6u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0xfff9u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0xfff5u16; cpu.w16(0x10e3d61u32, v); }
  cpu.push(0x1009d6d); f_0104eefa(cpu); if cpu.returned_to != 0x1009d6d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009d6e => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0x2du16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0xfff7u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0xfff9u16; cpu.w16(0x10e3d61u32, v); }
  cpu.push(0x1009d9e); f_0104eefa(cpu); if cpu.returned_to != 0x1009d9e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009d9f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x110d6f7u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x110d6fbu32) as u32; cpu.edi = v; }
  cpu.push(0x1009db4); f_01019090(cpu); if cpu.returned_to != 0x1009db4 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x1009c0c; continue; }
  pc = 0x1009dbc; continue;
 }
 0x1009dbc => {
  { let v: u32 = cpu.r32(0x110d6f7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d6fbu32); cpu.ebx = v; }
  cpu.push(0x1009dcc); f_01018cce(cpu); if cpu.returned_to != 0x1009dcc { return; }
  if !cpu.cf { pc = 0x1009dda; continue; }
  pc = 0x1009dce; continue;
 }
 0x1009dce => {
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.ecx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x1009c0c; continue; }
  pc = 0x1009dda; continue;
 }
 0x1009dda => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x10e3d61u32, v); }
  cpu.push(0x1009e0a); f_0104eefa(cpu); if cpu.returned_to != 0x1009e0a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009e0b => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0x2bu16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0xfffau16; cpu.w16(0x10e3d61u32, v); }
  cpu.push(0x1009e3b); f_0104eefa(cpu); if cpu.returned_to != 0x1009e3b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009e3c => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x10e3d5du32, v); }
  { let v: u16 = 0x2cu16; cpu.w16(0x10e3db7u32, v); }
  { let v: u16 = 0xfffbu16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0xfff5u16; cpu.w16(0x10e3d61u32, v); }
  cpu.push(0x1009e6c); f_0104eefa(cpu); if cpu.returned_to != 0x1009e6c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01009e6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1009e6d); }
 let mut pc: u32 = 0x1009e6d;
 loop { match pc {
 0x1009e6d => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11cfc89u32); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x1009e95; continue; }
  pc = 0x1009e93; continue;
 }
 0x1009e93 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009e95 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01009e97(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1009e97); }
 let mut pc: u32 = 0x1009e97;
 loop { match pc {
 0x1009e97 => {
  cpu.push(0x1009e9c); f_0104f36d(cpu); if cpu.returned_to != 0x1009e9c { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d6bau32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11c59c5u32, v); }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110d64cu32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x1009eda; continue; }
  pc = 0x1009ebe; continue;
 }
 0x1009ebe => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x1009ef3; continue; }
  pc = 0x1009ec4; continue;
 }
 0x1009ec4 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1009ed3; continue; }
  pc = 0x1009eca; continue;
 }
 0x1009eca => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x1009ee5; continue; }
  pc = 0x1009ed3; continue;
 }
 0x1009ed3 => {
  { let v: u8 = cpu.r8(0x10f71d1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009eda => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1009ee0); f_01068d2a(cpu); if cpu.returned_to != 0x1009ee0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x1009eec; continue; }
  pc = 0x1009ee5; continue;
 }
 0x1009ee5 => {
  { let v: u8 = cpu.r8(0x10f7214u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009eec => {
  { let v: u8 = cpu.r8(0x10f729au32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1009ef3 => {
  { let v: u8 = cpu.r8(0x10f7257u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01009f1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1009f1b); }
 let mut pc: u32 = 0x1009f1b;
 loop { match pc {
 0x1009f1b => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d736u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef35du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef35eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef35fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d739u32, v); }
  cpu.push(0x1009f43); f_0100a1d1(cpu); if cpu.returned_to != 0x1009f43 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01009f44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1009f44); }
 let mut pc: u32 = 0x1009f44;
 loop { match pc {
 0x1009f44 => {
  { cpu.sub8(cpu.r8(0x10ef35eu32), 0x0u8); }
  if cpu.zf { pc = 0x1009f4f; continue; }
  pc = 0x1009f4d; continue;
 }
 0x1009f4d => {
  pc = 0x1009f78; continue;
 }
 0x1009f4f => {
  { cpu.sub8(cpu.r8(0x110d631u32), 0x0u8); }
  if cpu.zf { pc = 0x1009fbf; continue; }
  pc = 0x1009f5c; continue;
 }
 0x1009f5c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110d731u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x10fcc06u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1009f6f); f_01012930(cpu); if cpu.returned_to != 0x1009f6f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d631u32, v); }
  pc = 0x1009fbf; continue;
 }
 0x1009f78 => {
  { cpu.sub8(cpu.r8(0x110d631u32), 0x0u8); }
  if !cpu.zf { pc = 0x1009fbf; continue; }
  pc = 0x1009f81; continue;
 }
 0x1009f81 => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1009fbf; continue; }
  pc = 0x1009f8a; continue;
 }
 0x1009f8a => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x1009fa5; continue; }
  pc = 0x1009f9e; continue;
 }
 0x1009f9e => {
  { let v: u32 = 0x76u32; cpu.eax = v; }
  pc = 0x1009faa; continue;
 }
 0x1009fa5 => {
  { let v: u32 = 0x78u32; cpu.eax = v; }
  pc = 0x1009faa; continue;
 }
 0x1009faa => {
  cpu.push(0x1009faf); f_01012780(cpu); if cpu.returned_to != 0x1009faf { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1009fbf; continue; }
  pc = 0x1009fb3; continue;
 }
 0x1009fb3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d731u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d631u32, v); }
  pc = 0x1009fbf; continue;
 }
 0x1009fbf => {
  { cpu.sub8(cpu.r8(0x110d739u32), 0x0u8); }
  if cpu.zf { pc = 0x1009fca; continue; }
  pc = 0x1009fc8; continue;
 }
 0x1009fc8 => {
  pc = 0x1009ff3; continue;
 }
 0x1009fca => {
  { cpu.sub8(cpu.r8(0x110d73bu32), 0x0u8); }
  if cpu.zf { pc = 0x100a039; continue; }
  pc = 0x1009fd7; continue;
 }
 0x1009fd7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110d73au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2u8; cpu.w8(0x10fcc06u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1009fea); f_01012930(cpu); if cpu.returned_to != 0x1009fea { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d73bu32, v); }
  pc = 0x100a039; continue;
 }
 0x1009ff3 => {
  { cpu.sub8(cpu.r8(0x110d73bu32), 0x0u8); }
  if !cpu.zf { pc = 0x100a039; continue; }
  pc = 0x1009ffc; continue;
 }
 0x1009ffc => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x100a039; continue; }
  pc = 0x100a005; continue;
 }
 0x100a005 => {
  { cpu.sub8(cpu.r8(0x110d739u32), 0x1u8); }
  if cpu.zf { pc = 0x100a01a; continue; }
  pc = 0x100a00e; continue;
 }
 0x100a00e => {
  { let v: u32 = 0x2fu32; cpu.eax = v; }
  cpu.push(0x100a018); f_01012780(cpu); if cpu.returned_to != 0x100a018 { return; }
  pc = 0x100a024; continue;
 }
 0x100a01a => {
  { let v: u32 = 0x2du32; cpu.eax = v; }
  cpu.push(0x100a024); f_01012780(cpu); if cpu.returned_to != 0x100a024 { return; }
  pc = 0x100a024; continue;
 }
 0x100a024 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100a039; continue; }
  pc = 0x100a028; continue;
 }
 0x100a028 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d73au32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x110d739u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d73bu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100a039; continue;
 }
 0x100a039 => {
  { cpu.sub8(cpu.r8(0x10ef35du32), 0x0u8); }
  if cpu.zf { pc = 0x100a044; continue; }
  pc = 0x100a042; continue;
 }
 0x100a042 => {
  pc = 0x100a077; continue;
 }
 0x100a044 => {
  { cpu.sub8(cpu.r8(0x110d630u32), 0x0u8); }
  if cpu.zf { pc = 0x100a10b; continue; }
  pc = 0x100a051; continue;
 }
 0x100a051 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110d730u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.w8(0x10fcc06u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x100a064); f_01012930(cpu); if cpu.returned_to != 0x100a064 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d630u32, v); }
  { let v: u8 = 0x2au8; cpu.w8(0x110d730u32, v); }
  pc = 0x100a10b; continue;
 }
 0x100a077 => {
  { cpu.sub8(cpu.r8(0x110d630u32), 0x0u8); }
  if cpu.zf { pc = 0x100a0b0; continue; }
  pc = 0x100a080; continue;
 }
 0x100a080 => {
  { let v: u8 = cpu.r8(0x10ef35du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x110d630u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x100a0df; continue; }
  pc = 0x100a08d; continue;
 }
 0x100a08d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110d730u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.w8(0x10fcc06u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x100a0a0); f_01012930(cpu); if cpu.returned_to != 0x100a0a0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d630u32, v); }
  { let v: u8 = 0x2au8; cpu.w8(0x110d730u32, v); }
  pc = 0x100a0b0; continue;
 }
 0x100a0b0 => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x100a10b; continue; }
  pc = 0x100a0b9; continue;
 }
 0x100a0b9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ef35du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x110d84bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x100a0cc); f_01012780(cpu); if cpu.returned_to != 0x100a0cc { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100a10b; continue; }
  pc = 0x100a0d0; continue;
 }
 0x100a0d0 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d730u32, v); }
  { let v: u8 = cpu.r8(0x10ef35du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d630u32, v); }
  pc = 0x100a0df; continue;
 }
 0x100a0df => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x110d730u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x100a10b; continue; }
  pc = 0x100a0ec; continue;
 }
 0x100a0ec => {
  { let v: u16 = cpu.r16(0x110d732u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xffu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x100a0ff; continue; }
  pc = 0x100a0fa; continue;
 }
 0x100a0fa => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  pc = 0x100a0ff; continue;
 }
 0x100a0ff => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x112c96eu32, v); }
  cpu.push(0x100a10b); f_010356fe(cpu); if cpu.returned_to != 0x100a10b { return; }
  pc = 0x100a10b; continue;
 }
 0x100a10b => {
  { cpu.sub8(cpu.r8(0x110d736u32), 0x0u8); }
  if cpu.zf { pc = 0x100a116; continue; }
  pc = 0x100a114; continue;
 }
 0x100a114 => {
  pc = 0x100a142; continue;
 }
 0x100a116 => {
  { cpu.sub8(cpu.r8(0x110d737u32), 0x0u8); }
  if cpu.zf { pc = 0x100a1cd; continue; }
  pc = 0x100a123; continue;
 }
 0x100a123 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110d738u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x5u8; cpu.w8(0x10fcc06u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x100a136); f_01012930(cpu); if cpu.returned_to != 0x100a136 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d737u32, v); }
  pc = 0x100a1cd; continue;
 }
 0x100a142 => {
  { cpu.sub8(cpu.r8(0x110d737u32), 0x0u8); }
  if cpu.zf { pc = 0x100a16f; continue; }
  pc = 0x100a14b; continue;
 }
 0x100a14b => {
  { let v: u8 = cpu.r8(0x110d736u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x110d737u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x100a1cd; continue; }
  pc = 0x100a15c; continue;
 }
 0x100a15c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110d738u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.w8(0x10fcc06u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x100a16f); f_01012930(cpu); if cpu.returned_to != 0x100a16f { return; }
  pc = 0x100a16f; continue;
 }
 0x100a16f => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x100a1cd; continue; }
  pc = 0x100a178; continue;
 }
 0x100a178 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110d736u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x110d86fu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x100a18b); f_01012780(cpu); if cpu.returned_to != 0x100a18b { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100a1cd; continue; }
  pc = 0x100a18f; continue;
 }
 0x100a18f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d738u32, v); }
  { let v: u8 = cpu.r8(0x110d736u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d737u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x110d738u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { cpu.poll(); pc = 0x100a10b; continue; }
  pc = 0x100a1af; continue;
 }
 0x100a1af => {
  { let v: u16 = cpu.r16(0x110d734u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xffu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x100a1c1; continue; }
  pc = 0x100a1bd; continue;
 }
 0x100a1bd => {
  { let v: u16 = 0xffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x100a1c1; continue;
 }
 0x100a1c1 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x112c96eu32, v); }
  cpu.push(0x100a1cd); f_010356fe(cpu); if cpu.returned_to != 0x100a1cd { return; }
  pc = 0x100a1cd; continue;
 }
 0x100a1cd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a1d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a1d0); }
 let mut pc: u32 = 0x100a1d0;
 loop { match pc {
 0x100a1d0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a1d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a1d1); }
 let mut pc: u32 = 0x100a1d1;
 loop { match pc {
 0x100a1d1 => {
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x100a1e3); f_0104f36d(cpu); if cpu.returned_to != 0x100a1e3 { return; }
  { let v: u32 = 0x4000u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10e4108u32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x10e4104u32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x100a218; continue; }
  pc = 0x100a214; continue;
 }
 0x100a214 => {
  { let v: u8 = 0x28u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x100a21c; continue;
 }
 0x100a218 => {
  { let v: u8 = 0x1du8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x100a21c; continue;
 }
 0x100a21c => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.scas(1, 2); }
  if cpu.zf { pc = 0x100a235; continue; }
  pc = 0x100a222; continue;
 }
 0x100a222 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100a21c; continue; }
  pc = 0x100a22d; continue;
 }
 0x100a22d => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef35eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100a235 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef35eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a23d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a23d); }
 let mut pc: u32 = 0x100a23d;
 loop { match pc {
 0x100a23d => {
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x100a280; continue; }
  pc = 0x100a246; continue;
 }
 0x100a246 => {
  { let v = cpu.r16(0x10e3cdbu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3cddu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3cdbu32, v); }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3cddu32, v); }
  cpu.push(0x100a271); f_0103d074(cpu); if cpu.returned_to != 0x100a271 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3cddu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3cdbu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100a280 => {
  cpu.push(0x100a285); f_0103d074(cpu); if cpu.returned_to != 0x100a285 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a286(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a286); }
 let mut pc: u32 = 0x100a286;
 loop { match pc {
 0x100a286 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x100a33c; continue; }
  pc = 0x100a293; continue;
 }
 0x100a293 => {
  { cpu.sub8(cpu.r8(0x10fd439u32), 0x0u8); }
  if cpu.zf { pc = 0x100a2a9; continue; }
  pc = 0x100a29c; continue;
 }
 0x100a29c => {
  { cpu.sub8(cpu.r8(0x10ee934u32), 0x0u8); }
  if cpu.zf { pc = 0x100a30e; continue; }
  pc = 0x100a2a9; continue;
 }
 0x100a2a9 => {
  { cpu.sub8(cpu.r8(0x10fd43au32), 0x0u8); }
  if cpu.zf { pc = 0x100a2bf; continue; }
  pc = 0x100a2b2; continue;
 }
 0x100a2b2 => {
  { cpu.sub8(cpu.r8(0x10ee935u32), 0x0u8); }
  if cpu.zf { pc = 0x100a31a; continue; }
  pc = 0x100a2bf; continue;
 }
 0x100a2bf => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x100a33c; continue; }
  pc = 0x100a2cc; continue;
 }
 0x100a2cc => {
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x100a33c; continue; }
  pc = 0x100a2da; continue;
 }
 0x100a2da => {
  { cpu.sub8(cpu.r8(0x10fd43bu32), 0x0u8); }
  if cpu.zf { pc = 0x100a2ec; continue; }
  pc = 0x100a2e3; continue;
 }
 0x100a2e3 => {
  { cpu.sub8(cpu.r8(0x10ee936u32), 0x0u8); }
  if cpu.zf { pc = 0x100a326; continue; }
  pc = 0x100a2ec; continue;
 }
 0x100a2ec => {
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x100a33c; continue; }
  pc = 0x100a2fa; continue;
 }
 0x100a2fa => {
  { cpu.sub8(cpu.r8(0x10fd43cu32), 0x0u8); }
  if cpu.zf { pc = 0x100a33c; continue; }
  pc = 0x100a303; continue;
 }
 0x100a303 => {
  { cpu.sub8(cpu.r8(0x10ee937u32), 0x0u8); }
  if cpu.zf { pc = 0x100a332; continue; }
  pc = 0x100a30c; continue;
 }
 0x100a30c => {
  pc = 0x100a33c; continue;
 }
 0x100a30e => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x100a318); f_01002832(cpu); if cpu.returned_to != 0x100a318 { return; }
  pc = 0x100a33c; continue;
 }
 0x100a31a => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x100a324); f_01002935(cpu); if cpu.returned_to != 0x100a324 { return; }
  pc = 0x100a33c; continue;
 }
 0x100a326 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x100a330); f_01002a3a(cpu); if cpu.returned_to != 0x100a330 { return; }
  pc = 0x100a33c; continue;
 }
 0x100a332 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x100a33c); f_01002b38(cpu); if cpu.returned_to != 0x100a33c { return; }
  pc = 0x100a33c; continue;
 }
 0x100a33c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a33d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a33d); }
 let mut pc: u32 = 0x100a33d;
 loop { match pc {
 0x100a33d => {
  { let v: u32 = cpu.esi.wrapping_add(0x1efu32); cpu.esi = v; }
  { let v: u32 = 0x11cfc96u32; cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x11cfc96u32; cpu.esi = v; }
  { let v: u32 = 0x11cfcd5u32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ebp = v; }
  pc = 0x100a35e; continue;
 }
 0x100a35e => {
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x13u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x3u32); cpu.edi = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.add32(cpu.esi, 0x3u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100a35e; continue; }
  pc = 0x100a379; continue;
 }
 0x100a379 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a37a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a37a); }
 let mut pc: u32 = 0x100a37a;
 loop { match pc {
 0x100a37a => {
  { let v: u8 = cpu.r8(0x110d635u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x110d634u32), (cpu.eax as u8)); cpu.w8(0x110d634u32, v); }
  if !cpu.sf { pc = 0x100a395; continue; }
  pc = 0x100a387; continue;
 }
 0x100a387 => {
  { let v: u8 = cpu.neg8(cpu.r8(0x110d635u32)); cpu.w8(0x110d635u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d634u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100a395 => {
  { cpu.sub8(cpu.r8(0x110d634u32), 0x14u8); }
  if cpu.cf { pc = 0x100a3a4; continue; }
  pc = 0x100a39e; continue;
 }
 0x100a39e => {
  { let v: u8 = cpu.neg8(cpu.r8(0x110d635u32)); cpu.w8(0x110d635u32, v); }
  pc = 0x100a3a4; continue;
 }
 0x100a3a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a747(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a747); }
 let mut pc: u32 = 0x100a747;
 loop { match pc {
 0x100a747 => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x100a751; continue;
 }
 0x100a751 => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x100a772; continue; }
  pc = 0x100a758; continue;
 }
 0x100a758 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8((cpu.ebx as u8), 0x4u8); }
  if !cpu.zf { pc = 0x100a76d; continue; }
  pc = 0x100a760; continue;
 }
 0x100a760 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0xfu8); }
  if cpu.zf { pc = 0x100a772; continue; }
  pc = 0x100a766; continue;
 }
 0x100a766 => {
  cpu.push(0x100a76b); f_0100a792(cpu); if cpu.returned_to != 0x100a76b { return; }
  pc = 0x100a772; continue;
 }
 0x100a76d => {
  cpu.push(0x100a772); f_0100a7e6(cpu); if cpu.returned_to != 0x100a772 { return; }
  pc = 0x100a772; continue;
 }
 0x100a772 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100a751; continue; }
  pc = 0x100a778; continue;
 }
 0x100a778 => {
  cpu.push(0x100a77d); f_01088003(cpu); if cpu.returned_to != 0x100a77d { return; }
  cpu.push(0x100a782); f_0108bb0f(cpu); if cpu.returned_to != 0x100a782 { return; }
  cpu.push(0x100a787); f_0101b533(cpu); if cpu.returned_to != 0x100a787 { return; }
  cpu.push(0x100a78c); f_0101b263(cpu); if cpu.returned_to != 0x100a78c { return; }
  cpu.push(0x100a791); f_010268b8(cpu); if cpu.returned_to != 0x100a791 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a792(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a792); }
 let mut pc: u32 = 0x100a792;
 loop { match pc {
 0x100a792 => {
  { cpu.pushad(); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x100a79d; continue;
 }
 0x100a79d => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100a7de; continue; }
  pc = 0x100a7a4; continue;
 }
 0x100a7a4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1105e68u32, v); }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x100a7b6; continue; }
  pc = 0x100a7af; continue;
 }
 0x100a7af => {
  { let v: u8 = 0x1u8; cpu.w8(0x1105e68u32, v); }
  pc = 0x100a7b6; continue;
 }
 0x100a7b6 => {
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.ebp)), cpu.esi); }
  if !cpu.zf { pc = 0x100a7de; continue; }
  pc = 0x100a7bb; continue;
 }
 0x100a7bb => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x100a7d7; continue; }
  pc = 0x100a7c3; continue;
 }
 0x100a7c3 => {
  { cpu.pushad(); }
  cpu.push(0x100a7c9); f_0102ea87(cpu); if cpu.returned_to != 0x100a7c9 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1105e68u32, v); }
  pc = 0x100a7d7; continue;
 }
 0x100a7d7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x45u32.wrapping_add(cpu.ebp), v); }
  pc = 0x100a7de; continue;
 }
 0x100a7de => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100a79d; continue; }
  pc = 0x100a7e4; continue;
 }
 0x100a7e4 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a7e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a7e6); }
 let mut pc: u32 = 0x100a7e6;
 loop { match pc {
 0x100a7e6 => {
  { cpu.pushad(); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x100a7f1; continue;
 }
 0x100a7f1 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100a818; continue; }
  pc = 0x100a7f8; continue;
 }
 0x100a7f8 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x100a818; continue; }
  pc = 0x100a7fc; continue;
 }
 0x100a7fc => {
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.ebp)), cpu.esi); }
  if !cpu.zf { pc = 0x100a818; continue; }
  pc = 0x100a801; continue;
 }
 0x100a801 => {
  { cpu.pushad(); }
  cpu.push(0x100a807); f_0102ea87(cpu); if cpu.returned_to != 0x100a807 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x17du32.wrapping_add(cpu.ebp), v); }
  pc = 0x100a818; continue;
 }
 0x100a818 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100a7f1; continue; }
  pc = 0x100a81e; continue;
 }
 0x100a81e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a820); }
 let mut pc: u32 = 0x100a820;
 loop { match pc {
 0x100a820 => {
  { cpu.sub8(cpu.r8(0x10fd439u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x100a87f; continue; }
  pc = 0x100a829; continue;
 }
 0x100a829 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd44cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd439u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x10fd436u32)); cpu.w16(0x10fd436u32, v); }
  { cpu.sub16(cpu.r16(0x10fd436u32), 0x1u16); }
  if !cpu.zf { pc = 0x100a852; continue; }
  pc = 0x100a848; continue;
 }
 0x100a848 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10ef37du32, v); }
  pc = 0x100a852; continue;
 }
 0x100a852 => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100a85c; continue;
 }
 0x100a85c => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x100a876; continue; }
  pc = 0x100a864; continue;
 }
 0x100a864 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x100a876; continue; }
  pc = 0x100a86b; continue;
 }
 0x100a86b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x100a873); f_010504cc(cpu); if cpu.returned_to != 0x100a873 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100a876; continue;
 }
 0x100a876 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100a85c; continue; }
  pc = 0x100a87f; continue;
 }
 0x100a87f => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd44cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a887(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a887); }
 let mut pc: u32 = 0x100a887;
 loop { match pc {
 0x100a887 => {
  { cpu.sub8(cpu.r8(0x110d634u32), 0x0u8); }
  if cpu.sf { pc = 0x100a8f0; continue; }
  pc = 0x100a890; continue;
 }
 0x100a890 => {
  { let v: u32 = 0x15u32; cpu.eax = v; }
  { let v = cpu.r8(0x110d634u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x11cfc96u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.esi = v; }
  { let v: u32 = 0x11ebf98u32; cpu.edi = v; }
  { let v: u8 = 0xa5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x100a8b0; continue;
 }
 0x100a8b0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x100a8be); f_0107270b(cpu); if cpu.returned_to != 0x100a8be { return; }
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
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100a8b0; continue; }
  pc = 0x100a8f0; continue;
 }
 0x100a8f0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a8f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a8f1); }
 let mut pc: u32 = 0x100a8f1;
 loop { match pc {
 0x100a8f1 => {
  { cpu.sub8(cpu.r8(0x10ee932u32), 0x0u8); }
  if cpu.zf { pc = 0x100a8ff; continue; }
  pc = 0x100a8fa; continue;
 }
 0x100a8fa => {
  cpu.push(0x100a8ff); f_0100a37a(cpu); if cpu.returned_to != 0x100a8ff { return; }
  pc = 0x100a8ff; continue;
 }
 0x100a8ff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a900); }
 let mut pc: u32 = 0x100a900;
 loop { match pc {
 0x100a900 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee92eu32)); cpu.w8(0x10ee92eu32, v); }
  { let v: u8 = cpu.r8(0x10ee92eu32) & 0x7u8; cpu.w8(0x10ee92eu32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11b5420u32)); cpu.w32(0x11b5420u32, v); }
  { let v: u32 = cpu.r32(0x11b5420u32) & 0xfu32; cpu.w32(0x11b5420u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11b5424u32)); cpu.w32(0x11b5424u32, v); }
  { let v: u32 = cpu.r32(0x11b5424u32) & 0x1fu32; cpu.w32(0x11b5424u32, v); }
  { let v: u8 = !cpu.r8(0x10ee91bu32); cpu.w8(0x10ee91bu32, v); }
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x100a945; continue; }
  pc = 0x100a936; continue;
 }
 0x100a936 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3d07u32)); cpu.w16(0x10e3d07u32, v); }
  { let v: u16 = cpu.r16(0x10e3d07u32) & 0x3u16; cpu.w16(0x10e3d07u32, v); }
  pc = 0x100a945; continue;
 }
 0x100a945 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee988u32)); cpu.w8(0x10ee988u32, v); }
  if !cpu.sf { pc = 0x100a954; continue; }
  pc = 0x100a94d; continue;
 }
 0x100a94d => {
  { let v: u8 = 0x16u8; cpu.w8(0x10ee988u32, v); }
  pc = 0x100a954; continue;
 }
 0x100a954 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee92du32)); cpu.w8(0x10ee92du32, v); }
  { cpu.sub8(cpu.r8(0x10ee92du32), 0x6u8); }
  if !cpu.zf { pc = 0x100a96a; continue; }
  pc = 0x100a963; continue;
 }
 0x100a963 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee92du32, v); }
  pc = 0x100a96a; continue;
 }
 0x100a96a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a96b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a96b); }
 let mut pc: u32 = 0x100a96b;
 loop { match pc {
 0x100a96b => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd47du32, v); }
  { let v: u32 = cpu.r32(0x10e3f4cu32); cpu.ecx = v; }
  { let v: u32 = 0x114eed4u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  if !cpu.sf { pc = 0x100a98e; continue; }
  pc = 0x100a98c; continue;
 }
 0x100a98c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x100a98e; continue;
 }
 0x100a98e => {
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e4108u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x20u32); cpu.ebx = v; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3d19u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x100a9b0; continue; }
  pc = 0x100a9a9; continue;
 }
 0x100a9a9 => {
  { let v: u16 = cpu.r16(0x10e3d19u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x100a9b0; continue;
 }
 0x100a9b0 => {
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 3u32); cpu.ebx = v; }
  pc = 0x100a9b9; continue;
 }
 0x100a9b9 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x100a9e7; continue; }
  pc = 0x100a9c2; continue;
 }
 0x100a9c2 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x100a9e2; continue; }
  pc = 0x100a9c8; continue;
 }
 0x100a9c8 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x100a9e2; continue; }
  pc = 0x100a9ce; continue;
 }
 0x100a9ce => {
  { cpu.sub8(cpu.r8(0xcu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x100a9d9; continue; }
  pc = 0x100a9d4; continue;
 }
 0x100a9d4 => {
  { let v: u8 = cpu.dec8(cpu.r8(0xcu32.wrapping_add(cpu.ebp))); cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x100a9e2; continue;
 }
 0x100a9d9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x100a9e0); f_0100a9e8(cpu); if cpu.returned_to != 0x100a9e0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100a9e2; continue;
 }
 0x100a9e2 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x10u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100a9b9; continue; }
  pc = 0x100a9e7; continue;
 }
 0x100a9e7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100a9e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100a9e8); }
 let mut pc: u32 = 0x100a9e8;
 loop { match pc {
 0x100a9e8 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x100aa13; continue; }
  pc = 0x100a9ee; continue;
 }
 0x100a9ee => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x100ab96; continue; }
  pc = 0x100a9fb; continue;
 }
 0x100a9fb => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5u32.wrapping_add(cpu.ebp))); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x100ab96; continue; }
  pc = 0x100aa04; continue;
 }
 0x100aa04 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x100aa70; continue;
 }
 0x100aa13 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x100ab96; continue; }
  pc = 0x100aa1c; continue;
 }
 0x100aa1c => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x6u32.wrapping_add(cpu.ebp))); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.cf { pc = 0x100aa4b; continue; }
  pc = 0x100aa2b; continue;
 }
 0x100aa2b => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if !cpu.zf { pc = 0x100aa70; continue; }
  pc = 0x100aa41; continue;
 }
 0x100aa41 => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x100ab96; continue; }
  pc = 0x100aa4b; continue;
 }
 0x100aa4b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x100aa5f; continue; }
  pc = 0x100aa56; continue;
 }
 0x100aa56 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x100aa5c); f_0105f178(cpu); if cpu.returned_to != 0x100aa5c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100aa5f; continue;
 }
 0x100aa5f => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x100ab96; continue; }
  pc = 0x100aa6e; continue;
 }
 0x100aa6e => {
  cpu.poll(); pc = 0x100aa2b; continue;
 }
 0x100aa70 => {
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e6910u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10ea910u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x10ea910u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10ea910u32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x10ea910u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x100aa95); f_0104f35c(cpu); if cpu.returned_to != 0x100aa95 { return; }
  cpu.push(0x100aa9a); f_01072d00(cpu); if cpu.returned_to != 0x100aa9a { return; }
  if cpu.cf { pc = 0x100ab14; continue; }
  pc = 0x100aaa0; continue;
 }
 0x100aaa0 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x100ab11; continue; }
  pc = 0x100aabf; continue;
 }
 0x100aabf => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u8 = cpu.r8(0x10ea910u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if !cpu.zf { pc = 0x100ab11; continue; }
  pc = 0x100aad0; continue;
 }
 0x100aad0 => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x100ab11; continue; }
  pc = 0x100aad6; continue;
 }
 0x100aad6 => {
  { cpu.sub32(cpu.r32(0x10e4138u32), 0x4u32); }
  if !cpu.zf { pc = 0x100aae6; continue; }
  pc = 0x100aadf; continue;
 }
 0x100aadf => {
  cpu.push(0x100aae4); f_0100ab97(cpu); if cpu.returned_to != 0x100aae4 { return; }
  pc = 0x100ab11; continue;
 }
 0x100aae6 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x76u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x100ab06; continue; }
  pc = 0x100aafc; continue;
 }
 0x100aafc => {
  cpu.push(0x100ab01); f_01012780(cpu); if cpu.returned_to != 0x100ab01 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x100ab06; continue;
 }
 0x100ab06 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x100ab11); f_010356fe(cpu); if cpu.returned_to != 0x100ab11 { return; }
  pc = 0x100ab11; continue;
 }
 0x100ab11 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100ab17; continue;
 }
 0x100ab14 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100ab17; continue;
 }
 0x100ab17 => {
  { cpu.sub8(cpu.r8(0xbu32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x100ab3a; continue; }
  pc = 0x100ab1d; continue;
 }
 0x100ab1d => {
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x100ab57; continue; }
  pc = 0x100ab23; continue;
 }
 0x100ab23 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x14u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e6990u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10ea990u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x10ea990u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x10ea990u32.wrapping_add(cpu.edi)), ((cpu.eax >> 8) as u8)); cpu.w8(0x10ea990u32.wrapping_add(cpu.edi), v); }
  pc = 0x100ab96; continue;
 }
 0x100ab3a => {
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x100ab96; continue; }
  pc = 0x100ab40; continue;
 }
 0x100ab40 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e6911u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10ea911u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x10ea911u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x10ea911u32.wrapping_add(cpu.edi)), ((cpu.eax >> 8) as u8)); cpu.w8(0x10ea911u32.wrapping_add(cpu.edi), v); }
  pc = 0x100ab96; continue;
 }
 0x100ab57 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e6911u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10ea911u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x10ea911u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10ea911u32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x10ea911u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x13u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e6990u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10ea990u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x10ea990u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10ea990u32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x10ea990u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e6991u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10ea991u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x10ea991u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x10ea991u32.wrapping_add(cpu.edi)), ((cpu.eax >> 8) as u8)); cpu.w8(0x10ea991u32.wrapping_add(cpu.edi), v); }
  pc = 0x100ab96; continue;
 }
 0x100ab96 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ab97(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ab97); }
 let mut pc: u32 = 0x100ab97;
 loop { match pc {
 0x100ab97 => {
  { cpu.pushad(); }
  cpu.push(0x100ab9d); f_0105f178(cpu); if cpu.returned_to != 0x100ab9d { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x100abbd; continue; }
  pc = 0x100aba5; continue;
 }
 0x100aba5 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x100abb6; continue; }
  pc = 0x100abaa; continue;
 }
 0x100abaa => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x100abd7; continue; }
  pc = 0x100abaf; continue;
 }
 0x100abaf => {
  { let v: u32 = 0x69u32; cpu.eax = v; }
  pc = 0x100abc2; continue;
 }
 0x100abb6 => {
  { let v: u32 = 0x75u32; cpu.eax = v; }
  pc = 0x100abc2; continue;
 }
 0x100abbd => {
  { let v: u32 = 0x76u32; cpu.eax = v; }
  pc = 0x100abc2; continue;
 }
 0x100abc2 => {
  cpu.push(0x100abc7); f_01012780(cpu); if cpu.returned_to != 0x100abc7 { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100abd7; continue; }
  pc = 0x100abcb; continue;
 }
 0x100abcb => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110d731u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d631u32, v); }
  pc = 0x100abd7; continue;
 }
 0x100abd7 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100abd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100abd9); }
 let mut pc: u32 = 0x100abd9;
 loop { match pc {
 0x100abd9 => {
  { let v: u32 = 0x10e438cu32; cpu.esi = v; }
  cpu.push(0x100abe3); f_0100ac15(cpu); if cpu.returned_to != 0x100abe3 { return; }
  cpu.push(0x100abe8); f_01071bba(cpu); if cpu.returned_to != 0x100abe8 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x11b51b9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100abf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100abf0); }
 let mut pc: u32 = 0x100abf0;
 loop { match pc {
 0x100abf0 => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x100ac14; continue; }
  pc = 0x100abf9; continue;
 }
 0x100abf9 => {
  cpu.push(0x100abfe); f_01071890(cpu); if cpu.returned_to != 0x100abfe { return; }
  { let v: u32 = 0x10e442cu32; cpu.esi = v; }
  cpu.push(0x100ac08); f_0100ac15(cpu); if cpu.returned_to != 0x100ac08 { return; }
  cpu.push(0x100ac0d); f_01071b25(cpu); if cpu.returned_to != 0x100ac0d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11b51b9u32, v); }
  pc = 0x100ac14; continue;
 }
 0x100ac14 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ac15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ac15); }
 let mut pc: u32 = 0x100ac15;
 loop { match pc {
 0x100ac15 => {
  { let v: u32 = 0x10e442cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x10e438cu32); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = 0x10e42ecu32; cpu.edi = v; }
  { cpu.interrupts_off = true; }
  { cpu.movs(4, true); }
  { cpu.interrupts_off = false; }
  let t = cpu.r32(0x10e434cu32); cpu.push(0x100ac32); cpu.call(t); if cpu.returned_to != 0x100ac32 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ac34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ac34); }
 let mut pc: u32 = 0x100ac34;
 loop { match pc {
 0x100ac34 => {
  { let v: u32 = 0x1194608u32; cpu.edi = v; }
  { let v: u32 = 0xfcu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11b5171u32); cpu.ebx = v; }
  pc = 0x100ac44; continue;
 }
 0x100ac44 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x100ac4e; continue; }
  pc = 0x100ac47; continue;
 }
 0x100ac47 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.ebx); }
  if cpu.zf { pc = 0x100ac4f; continue; }
  pc = 0x100ac4b; continue;
 }
 0x100ac4b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x100ac44; continue;
 }
 0x100ac4e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ac4f => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd40eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11bd2f6u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10fd3f3u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11302acu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ac6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ac6e); }
 let mut pc: u32 = 0x100ac6e;
 loop { match pc {
 0x100ac6e => {
  { let v: u32 = 0x1194608u32; cpu.edi = v; }
  { let v: u32 = 0xfcu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11b5153u32); cpu.ebx = v; }
  pc = 0x100ac7e; continue;
 }
 0x100ac7e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x100ac88; continue; }
  pc = 0x100ac81; continue;
 }
 0x100ac81 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.ebx); }
  if cpu.zf { pc = 0x100ac89; continue; }
  pc = 0x100ac85; continue;
 }
 0x100ac85 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x100ac7e; continue;
 }
 0x100ac88 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ac89 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x1105eb8u32; cpu.edi = v; }
  { let v: u32 = 0x11302b3u32; cpu.edx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11cec63u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x100ac88; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100aca7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100aca7); }
 let mut pc: u32 = 0x100aca7;
 loop { match pc {
 0x100aca7 => {
  { let v: u32 = 0x1194608u32; cpu.edi = v; }
  { let v: u32 = 0xfcu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11b514eu32); cpu.ebx = v; }
  pc = 0x100acb7; continue;
 }
 0x100acb7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x100acc1; continue; }
  pc = 0x100acba; continue;
 }
 0x100acba => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.ebx); }
  if cpu.zf { pc = 0x100acc2; continue; }
  pc = 0x100acbe; continue;
 }
 0x100acbe => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x100acb7; continue;
 }
 0x100acc1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100acc2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x11cea9cu32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x100acd1; continue;
 }
 0x100acd1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x100acc1; continue; }
  pc = 0x100acdb; continue;
 }
 0x100acdb => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { cpu.poll(); pc = 0x100acc1; continue; }
  pc = 0x100acdf; continue;
 }
 0x100acdf => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100acd1; continue; }
  pc = 0x100ace2; continue;
 }
 0x100ace2 => {
  cpu.poll(); pc = 0x100acc1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ace5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ace5); }
 let mut pc: u32 = 0x100ace5;
 loop { match pc {
 0x100ace5 => {
  { let v: u32 = 0x1194608u32; cpu.edi = v; }
  { let v: u32 = 0xfcu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11b516cu32); cpu.ebx = v; }
  pc = 0x100acf5; continue;
 }
 0x100acf5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x100acff; continue; }
  pc = 0x100acf8; continue;
 }
 0x100acf8 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.ebx); }
  if cpu.zf { pc = 0x100ad00; continue; }
  pc = 0x100acfc; continue;
 }
 0x100acfc => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x100acf5; continue;
 }
 0x100acff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ad00 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b5428u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ad08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ad08); }
 let mut pc: u32 = 0x100ad08;
 loop { match pc {
 0x100ad08 => {
  { let v: u32 = 0x1194608u32; cpu.edi = v; }
  { let v: u32 = 0xfcu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11b5162u32); cpu.ebx = v; }
  pc = 0x100ad18; continue;
 }
 0x100ad18 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x100ad22; continue; }
  pc = 0x100ad1b; continue;
 }
 0x100ad1b => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.ebx); }
  if cpu.zf { pc = 0x100ad23; continue; }
  pc = 0x100ad1f; continue;
 }
 0x100ad1f => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x100ad18; continue;
 }
 0x100ad22 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ad23 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11302acu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ad2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ad2b); }
 let mut pc: u32 = 0x100ad2b;
 loop { match pc {
 0x100ad2b => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b5176u32, v); }
  { let v: u32 = 0x1194608u32; cpu.edi = v; }
  { let v: u32 = 0xfcu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11b515du32); cpu.ebx = v; }
  pc = 0x100ad42; continue;
 }
 0x100ad42 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x100ad4c; continue; }
  pc = 0x100ad45; continue;
 }
 0x100ad45 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.ebx); }
  if cpu.zf { pc = 0x100ad4d; continue; }
  pc = 0x100ad49; continue;
 }
 0x100ad49 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x100ad42; continue;
 }
 0x100ad4c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ad4d => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x31u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef36du32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11b5176u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ad65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ad65); }
 let mut pc: u32 = 0x100ad65;
 loop { match pc {
 0x100ad65 => {
  { let v: u32 = 0x1194608u32; cpu.edi = v; }
  { let v: u32 = 0xfcu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11b5167u32); cpu.ebx = v; }
  pc = 0x100ad75; continue;
 }
 0x100ad75 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x100ad7f; continue; }
  pc = 0x100ad78; continue;
 }
 0x100ad78 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.ebx); }
  if cpu.zf { pc = 0x100ad80; continue; }
  pc = 0x100ad7c; continue;
 }
 0x100ad7c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x100ad75; continue;
 }
 0x100ad7f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ad80 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11b5187u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ad91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ad91); }
 let mut pc: u32 = 0x100ad91;
 loop { match pc {
 0x100ad91 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd44bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11b5187u32, v); }
  { let v: u32 = 0x1194608u32; cpu.edi = v; }
  { let v: u32 = 0xfcu32; cpu.ecx = v; }
  pc = 0x100ada9; continue;
 }
 0x100ada9 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x61u8); }
  if cpu.cf { pc = 0x100adb9; continue; }
  pc = 0x100adae; continue;
 }
 0x100adae => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x100adb9; continue; }
  pc = 0x100adb3; continue;
 }
 0x100adb3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_sub(0x61u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x41u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x100adb9; continue;
 }
 0x100adb9 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100ada9; continue; }
  pc = 0x100adbc; continue;
 }
 0x100adbc => {
  { let v: u32 = 0x10f7b65u32; cpu.esi = v; }
  cpu.push(0x100adc6); f_0100ae4f(cpu); if cpu.returned_to != 0x100adc6 { return; }
  if !cpu.cf { pc = 0x100addb; continue; }
  pc = 0x100adc8; continue;
 }
 0x100adc8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b5187u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd44bu32, v); }
  pc = 0x100ae4e; continue;
 }
 0x100addb => {
  { let v: u32 = 0x10f7b6au32; cpu.esi = v; }
  cpu.push(0x100ade5); f_0100ae4f(cpu); if cpu.returned_to != 0x100ade5 { return; }
  if !cpu.cf { pc = 0x100adfa; continue; }
  pc = 0x100ade7; continue;
 }
 0x100ade7 => {
  { let v: u8 = 0x2u8; cpu.w8(0x11b5187u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10fd44bu32, v); }
  pc = 0x100ae4e; continue;
 }
 0x100adfa => {
  { let v: u32 = 0x10f7b6fu32; cpu.esi = v; }
  cpu.push(0x100ae04); f_0100ae4f(cpu); if cpu.returned_to != 0x100ae04 { return; }
  if !cpu.cf { pc = 0x100ae16; continue; }
  pc = 0x100ae06; continue;
 }
 0x100ae06 => {
  { let v: u8 = 0x3u8; cpu.w8(0x11b5187u32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x10fd44bu32, v); }
  pc = 0x100ae4e; continue;
 }
 0x100ae16 => {
  { let v: u32 = 0x10f7b74u32; cpu.esi = v; }
  cpu.push(0x100ae20); f_0100ae4f(cpu); if cpu.returned_to != 0x100ae20 { return; }
  if !cpu.cf { pc = 0x100ae32; continue; }
  pc = 0x100ae22; continue;
 }
 0x100ae22 => {
  { let v: u8 = 0x4u8; cpu.w8(0x11b5187u32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10fd44bu32, v); }
  pc = 0x100ae4e; continue;
 }
 0x100ae32 => {
  { let v: u32 = 0x10f7b79u32; cpu.esi = v; }
  cpu.push(0x100ae3c); f_0100ae4f(cpu); if cpu.returned_to != 0x100ae3c { return; }
  if !cpu.cf { pc = 0x100ae4e; continue; }
  pc = 0x100ae3e; continue;
 }
 0x100ae3e => {
  { let v: u8 = 0x5u8; cpu.w8(0x11b5187u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10fd44bu32, v); }
  pc = 0x100ae4e; continue;
 }
 0x100ae4e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ae4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ae4f); }
 let mut pc: u32 = 0x100ae4f;
 loop { match pc {
 0x100ae4f => {
  { let v: u32 = 0x1194608u32; cpu.edi = v; }
  { let v: u32 = 0xfcu32; cpu.ecx = v; }
  pc = 0x100ae59; continue;
 }
 0x100ae59 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x100ae5b; continue;
 }
 0x100ae5b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x100ae71; continue; }
  pc = 0x100ae5e; continue;
 }
 0x100ae5e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 0); }
  if cpu.zf { pc = 0x100ae69; continue; }
  pc = 0x100ae63; continue;
 }
 0x100ae63 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x100ae59; continue; }
  pc = 0x100ae67; continue;
 }
 0x100ae67 => {
  cpu.poll(); pc = 0x100ae5b; continue;
 }
 0x100ae69 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x100ae5b; continue; }
  pc = 0x100ae6f; continue;
 }
 0x100ae6f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ae71 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ae73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ae73); }
 let mut pc: u32 = 0x100ae73;
 loop { match pc {
 0x100ae73 => {
  { cpu.sub8(cpu.r8(0x11b5176u32), 0x0u8); }
  if cpu.zf { pc = 0x100ae7d; continue; }
  pc = 0x100ae7c; continue;
 }
 0x100ae7c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ae7d => {
  { let v: u8 = 0x2u8; cpu.w8(0x10ef36du32, v); }
  { cpu.sub8(cpu.r8(0x1129914u32), 0x3u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x100ae93; continue; }
  pc = 0x100ae8d; continue;
 }
 0x100ae8d => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef36du32)); cpu.w8(0x10ef36du32, v); }
  pc = 0x100ae93; continue;
 }
 0x100ae93 => {
  { cpu.sub16(cpu.r16(0x1129916u32), 0x1eu16); }
  if !cpu.cf { pc = 0x100aea3; continue; }
  pc = 0x100ae9d; continue;
 }
 0x100ae9d => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef36du32)); cpu.w8(0x10ef36du32, v); }
  pc = 0x100aea3; continue;
 }
 0x100aea3 => {
  { let v: u8 = cpu.r8(0x10ef36fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(cpu.r8(0x10ef36du32), (cpu.eax as u8)); cpu.w8(0x10ef36du32, v); }
  if !cpu.sf { pc = 0x100aeb9; continue; }
  pc = 0x100aeb2; continue;
 }
 0x100aeb2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef36du32, v); }
  pc = 0x100aeb9; continue;
 }
 0x100aeb9 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x3u8); }
  if (cpu.cf || cpu.zf) { pc = 0x100aec9; continue; }
  pc = 0x100aec2; continue;
 }
 0x100aec2 => {
  { let v: u8 = 0x3u8; cpu.w8(0x10ef36du32, v); }
  pc = 0x100aec9; continue;
 }
 0x100aec9 => {
  cpu.push(0x100aece); f_0100aecf(cpu); if cpu.returned_to != 0x100aece { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100aecf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100aecf); }
 let mut pc: u32 = 0x100aecf;
 loop { match pc {
 0x100aecf => {
  { let v: u32 = 0x7d0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ef36du32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e41acu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100aeea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100aeea); }
 let mut pc: u32 = 0x100aeea;
 loop { match pc {
 0x100aeea => {
  { cpu.sub8(cpu.r8(0x110d638u32), 0x0u8); }
  if cpu.zf { pc = 0x100b05b; continue; }
  pc = 0x100aef7; continue;
 }
 0x100aef7 => {
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100af01; continue;
 }
 0x100af01 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x100b051; continue; }
  pc = 0x100af0b; continue;
 }
 0x100af0b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x100af14); f_010868d2(cpu); if cpu.returned_to != 0x100af14 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1128dd7u32, v); }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x16u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1128dd6u32, v); }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x100b04f; continue; }
  pc = 0x100af5d; continue;
 }
 0x100af5d => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x100b04f; continue; }
  pc = 0x100af67; continue;
 }
 0x100af67 => {
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x30u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x100b04f; continue; }
  pc = 0x100af7b; continue;
 }
 0x100af7b => {
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x25u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x100b04f; continue; }
  pc = 0x100af8f; continue;
 }
 0x100af8f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x100afa1); f_01059c7e(cpu); if cpu.returned_to != 0x100afa1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x9u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x100b04f; continue; }
  pc = 0x100afb6; continue;
 }
 0x100afb6 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x100afbc); f_0108a641(cpu); if cpu.returned_to != 0x100afbc { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x100b04f; continue; }
  pc = 0x100afc5; continue;
 }
 0x100afc5 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x100afe2; continue; }
  pc = 0x100afcb; continue;
 }
 0x100afcb => {
  { let v: u32 = 0x110d5c7u32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x100afda); f_01034211(cpu); if cpu.returned_to != 0x100afda { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x9u32); cpu.ebx = v; }
  pc = 0x100aff7; continue;
 }
 0x100afe2 => {
  { let v: u32 = 0x110d5cbu32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x100aff1); f_01034211(cpu); if cpu.returned_to != 0x100aff1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x9u32); cpu.ebx = v; }
  pc = 0x100aff7; continue;
 }
 0x100aff7 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x110d538u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x100b00f); f_01034211(cpu); if cpu.returned_to != 0x100b00f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x9u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x100b04f; continue; }
  pc = 0x100b01d; continue;
 }
 0x100b01d => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x100b04f; continue; }
  pc = 0x100b022; continue;
 }
 0x100b022 => {
  { cpu.sub32(cpu.ecx, 0x116de88u32); }
  if (cpu.cf || cpu.zf) { pc = 0x100b04f; continue; }
  pc = 0x100b02a; continue;
 }
 0x100b02a => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x116de88u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = 0x1eu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  cpu.push(0x100b04f); f_01059c7e(cpu); if cpu.returned_to != 0x100b04f { return; }
  pc = 0x100b04f; continue;
 }
 0x100b04f => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x100b051; continue;
 }
 0x100b051 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100af01; continue; }
  pc = 0x100b05b; continue;
 }
 0x100b05b => {
  { let v: u8 = 0x0u8; cpu.w8(0x1128dd6u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b063(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b063); }
 let mut pc: u32 = 0x100b063;
 loop { match pc {
 0x100b063 => {
  { cpu.sub8(cpu.r8(0x10ef378u32), 0x0u8); }
  if cpu.zf { pc = 0x100b08b; continue; }
  pc = 0x100b06c; continue;
 }
 0x100b06c => {
  { let v: u32 = cpu.r32(0x10f7e46u32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  cpu.push(0x100b08b); f_01059c7e(cpu); if cpu.returned_to != 0x100b08b { return; }
  pc = 0x100b08b; continue;
 }
 0x100b08b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b08c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b08c); }
 let mut pc: u32 = 0x100b08c;
 loop { match pc {
 0x100b08c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b08f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b08f); }
 let mut pc: u32 = 0x100b08f;
 loop { match pc {
 0x100b08f => {
  { cpu.and8(cpu.r8(0x10ee968u32), 0x2u8); }
  if cpu.zf { pc = 0x100b0c4; continue; }
  pc = 0x100b098; continue;
 }
 0x100b098 => {
  { cpu.sub32(cpu.r32(0x110d534u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x100b0c3; continue; }
  pc = 0x100b0a1; continue;
 }
 0x100b0a1 => {
  { let v: u32 = cpu.r32(0x10e3f78u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110d534u32, v); }
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d644u32, v); }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d646u32, v); }
  pc = 0x100b0c3; continue;
 }
 0x100b0c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b0c4 => {
  { cpu.sub32(cpu.r32(0x110d534u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x100b12f; continue; }
  pc = 0x100b0d1; continue;
 }
 0x100b0d1 => {
  { let v: u32 = cpu.r32(0x10e3f78u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d534u32)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d534u32, v); }
  { cpu.sub32(cpu.eax, 0xfu32); }
  if cpu.cf { pc = 0x100b12f; continue; }
  pc = 0x100b0eb; continue;
 }
 0x100b0eb => {
  { let v: u16 = cpu.r16(0x110d644u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x110d646u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x100b12f; continue; }
  pc = 0x100b120; continue;
 }
 0x100b120 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x100b12f; continue; }
  pc = 0x100b12a; continue;
 }
 0x100b12a => {
  cpu.push(0x100b12f); f_01059a31(cpu); if cpu.returned_to != 0x100b12f { return; }
  pc = 0x100b12f; continue;
 }
 0x100b12f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b130); }
 let mut pc: u32 = 0x100b130;
 loop { match pc {
 0x100b130 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100b16d; continue; }
  pc = 0x100b136; continue;
 }
 0x100b136 => {
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x100b16d; continue; }
  pc = 0x100b13b; continue;
 }
 0x100b13b => {
  { cpu.sub32(cpu.eax, 0x10c7adbu32); }
  if cpu.cf { pc = 0x100b16d; continue; }
  pc = 0x100b142; continue;
 }
 0x100b142 => {
  { cpu.sub32(cpu.eax, 0x10d10dcu32); }
  if !cpu.cf { pc = 0x100b16d; continue; }
  pc = 0x100b149; continue;
 }
 0x100b149 => {
  { let v: u32 = 0x1152e54u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100b153; continue;
 }
 0x100b153 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x100b162; continue; }
  pc = 0x100b159; continue;
 }
 0x100b159 => {
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if cpu.zf { pc = 0x100b162; continue; }
  pc = 0x100b15d; continue;
 }
 0x100b15d => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if cpu.zf { pc = 0x100b16d; continue; }
  pc = 0x100b162; continue;
 }
 0x100b162 => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100b153; continue; }
  pc = 0x100b16b; continue;
 }
 0x100b16b => {
  pc = 0x100b16e; continue;
 }
 0x100b16d => {
  {  }
  pc = 0x100b16e; continue;
 }
 0x100b16e => {
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b173(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b173); }
 let mut pc: u32 = 0x100b173;
 loop { match pc {
 0x100b173 => {
  { let v: u32 = 0x110c26cu32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x4b0u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x10c7adbu32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x100b190; continue;
 }
 0x100b190 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100b1ad; continue; }
  pc = 0x100b198; continue;
 }
 0x100b198 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x100b1b4; continue; }
  pc = 0x100b19c; continue;
 }
 0x100b19c => {
  { let v: u32 = cpu.eax.wrapping_sub(0x10c7adbu32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x110c26cu32.wrapping_add(cpu.eax), v); }
  pc = 0x100b1b4; continue;
 }
 0x100b1ad => {
  { let v: u8 = 0x1u8; cpu.w8(0x110c26cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x100b1b4; continue;
 }
 0x100b1b4 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100b190; continue; }
  pc = 0x100b1bb; continue;
 }
 0x100b1bb => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100b1c5; continue;
 }
 0x100b1c5 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x100b1e6; continue; }
  pc = 0x100b1cb; continue;
 }
 0x100b1cb => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x100b1e6; continue; }
  pc = 0x100b1d2; continue;
 }
 0x100b1d2 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100b1e6; continue; }
  pc = 0x100b1d7; continue;
 }
 0x100b1d7 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x10c7adbu32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x110c26cu32.wrapping_add(cpu.eax), v); }
  pc = 0x100b1e6; continue;
 }
 0x100b1e6 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100b1c5; continue; }
  pc = 0x100b1ef; continue;
 }
 0x100b1ef => {
  { let v: u32 = 0x10c7adbu32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  { let v: u32 = 0x110c26cu32; cpu.ebx = v; }
  pc = 0x100b1fe; continue;
 }
 0x100b1fe => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x100b20a; continue; }
  pc = 0x100b203; continue;
 }
 0x100b203 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100b20a; continue;
 }
 0x100b20a => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100b1fe; continue; }
  pc = 0x100b211; continue;
 }
 0x100b211 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b212(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b212); }
 let mut pc: u32 = 0x100b212;
 loop { match pc {
 0x100b212 => {
  { cpu.sub8(cpu.r8(0x110d636u32), 0x0u8); }
  if cpu.zf { pc = 0x100b2b2; continue; }
  pc = 0x100b21f; continue;
 }
 0x100b21f => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if !cpu.zf { pc = 0x100b2b2; continue; }
  pc = 0x100b22c; continue;
 }
 0x100b22c => {
  { let v: u32 = 0x1194208u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x116e72cu32); cpu.eax = v; }
  { let v = cpu.r32(0x10e3d81u32); cpu.mul32(v); }
  { let v: u32 = 0xddu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x100b250); f_01059d5a(cpu); if cpu.returned_to != 0x100b250 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x20u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x113385cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x116e72cu32)); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x100b29c; continue; }
  pc = 0x100b273; continue;
 }
 0x100b273 => {
  { let v: u32 = cpu.r32(0x116e72cu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x100b291); f_01059d5a(cpu); if cpu.returned_to != 0x100b291 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x100b29c; continue;
 }
 0x100b29c => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x100b2b2); f_01034211(cpu); if cpu.returned_to != 0x100b2b2 { return; }
  pc = 0x100b2b2; continue;
 }
 0x100b2b2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b2b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b2b3); }
 let mut pc: u32 = 0x100b2b3;
 loop { match pc {
 0x100b2b3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11bce8du32, v); }
  { let v: u32 = 0x11bceacu32; cpu.ebp = v; }
  { let v: u32 = 0x1152e54u32; cpu.esi = v; }
  { let v: u32 = 0x11bcd6du32; cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x100b2ce; continue;
 }
 0x100b2ce => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x0u32); }
  if cpu.zf { pc = 0x100b362; continue; }
  pc = 0x100b2da; continue;
 }
 0x100b2da => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x100b358; continue; }
  pc = 0x100b2eb; continue;
 }
 0x100b2eb => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x100b358; continue; }
  pc = 0x100b2f5; continue;
 }
 0x100b2f5 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x100b358; continue; }
  pc = 0x100b2fb; continue;
 }
 0x100b2fb => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x100b309); f_0104f36d(cpu); if cpu.returned_to != 0x100b309 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x100b31c; continue; }
  pc = 0x100b316; continue;
 }
 0x100b316 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x100b31c; continue; }
  pc = 0x100b31a; continue;
 }
 0x100b31a => {
  pc = 0x100b358; continue;
 }
 0x100b31c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6f7u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6fbu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x11bce8du32)); cpu.w8(0x11bce8du32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x100b358; continue;
 }
 0x100b358 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100b2ce; continue; }
  pc = 0x100b362; continue;
 }
 0x100b362 => {
  { let v: u32 = 0x11bcd6du32; cpu.w32(0x110d728u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11bce8eu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x110d72cu32, v); }
  { cpu.sub8(cpu.r8(0x11bce8du32), 0x1u8); }
  if cpu.zf { pc = 0x100b395; continue; }
  pc = 0x100b382; continue;
 }
 0x100b382 => {
  { cpu.sub8(cpu.r8(0x11bce8du32), 0x0u8); }
  if !cpu.zf { pc = 0x100b396; continue; }
  pc = 0x100b38b; continue;
 }
 0x100b38b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d728u32, v); }
  pc = 0x100b395; continue;
 }
 0x100b395 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b396 => {
  { let v: u8 = cpu.r8(0x11bce8du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x100b39c; continue;
 }
 0x100b39c => {
  { let v: u32 = 0x11bcd6du32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x11bce8du32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x100b3a9; continue;
 }
 0x100b3a9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x100b3be; continue; }
  pc = 0x100b3b1; continue;
 }
 0x100b3b1 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100b3be; continue;
 }
 0x100b3be => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x100b3a9; continue; }
  pc = 0x100b3c5; continue;
 }
 0x100b3c5 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100b39c; continue; }
  pc = 0x100b3c9; continue;
 }
 0x100b3c9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b3ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b3ca); }
 let mut pc: u32 = 0x100b3ca;
 loop { match pc {
 0x100b3ca => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x100b3d4; continue; }
  pc = 0x100b3d3; continue;
 }
 0x100b3d3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b3d4 => {
  { cpu.sub8(cpu.r8(0x11bce8eu32), 0x0u8); }
  if cpu.zf { pc = 0x100b3e2; continue; }
  pc = 0x100b3dd; continue;
 }
 0x100b3dd => {
  cpu.push(0x100b3e2); f_0100b2b3(cpu); if cpu.returned_to != 0x100b3e2 { return; }
  pc = 0x100b3e2; continue;
 }
 0x100b3e2 => {
  { let v: u32 = cpu.r32(0x110d728u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x100b45f; continue; }
  pc = 0x100b3f1; continue;
 }
 0x100b3f1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d3u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x100b42e; continue; }
  pc = 0x100b421; continue;
 }
 0x100b421 => {
  if cpu.sf { pc = 0x100b42e; continue; }
  pc = 0x100b423; continue;
 }
 0x100b423 => {
  { cpu.sub32(cpu.eax, 0xb800u32); }
  if (cpu.sf == cpu.of) { pc = 0x100b42e; continue; }
  pc = 0x100b42a; continue;
 }
 0x100b42a => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x100b42e; continue;
 }
 0x100b42e => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x100b439); f_01059200(cpu); if cpu.returned_to != 0x100b439 { return; }
  { let v: u32 = cpu.r32(0x110d728u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x110d72cu32), cpu.edi); }
  if !cpu.zf { pc = 0x100b457; continue; }
  pc = 0x100b44a; continue;
 }
 0x100b44a => {
  { let v: u32 = 0x11bcd6du32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x110d728u32, v); }
  pc = 0x100b45d; continue;
 }
 0x100b457 => {
  { let v: u32 = cpu.edi; cpu.w32(0x110d728u32, v); }
  pc = 0x100b45d; continue;
 }
 0x100b45d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b45f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b461(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b461); }
 let mut pc: u32 = 0x100b461;
 loop { match pc {
 0x100b461 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11bce93u32, v); }
  { let v: u32 = 0x11bceacu32; cpu.ebp = v; }
  { let v: u32 = 0x1152e54u32; cpu.esi = v; }
  { let v: u32 = 0x11bcc4du32; cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x100b47c; continue;
 }
 0x100b47c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x0u32); }
  if cpu.zf { pc = 0x100b4f6; continue; }
  pc = 0x100b488; continue;
 }
 0x100b488 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x100b4f0; continue; }
  pc = 0x100b495; continue;
 }
 0x100b495 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x100b4f0; continue; }
  pc = 0x100b49b; continue;
 }
 0x100b49b => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x100b4f0; continue; }
  pc = 0x100b4a1; continue;
 }
 0x100b4a1 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x100b4af); f_01018e6e(cpu); if cpu.returned_to != 0x100b4af { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x100b4b4; continue; }
  pc = 0x100b4b2; continue;
 }
 0x100b4b2 => {
  pc = 0x100b4f0; continue;
 }
 0x100b4b4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6f7u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6fbu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x11bce93u32)); cpu.w8(0x11bce93u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x100b4f0; continue;
 }
 0x100b4f0 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100b47c; continue; }
  pc = 0x100b4f6; continue;
 }
 0x100b4f6 => {
  { let v: u32 = 0x11bcc4du32; cpu.w32(0x110d714u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d5d2u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x110d71cu32, v); }
  { cpu.sub8(cpu.r8(0x11bce93u32), 0x1u8); }
  if cpu.zf { pc = 0x100b529; continue; }
  pc = 0x100b516; continue;
 }
 0x100b516 => {
  { cpu.sub8(cpu.r8(0x11bce93u32), 0x0u8); }
  if !cpu.zf { pc = 0x100b52a; continue; }
  pc = 0x100b51f; continue;
 }
 0x100b51f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d714u32, v); }
  pc = 0x100b529; continue;
 }
 0x100b529 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b52a => {
  { let v: u8 = cpu.r8(0x11bce93u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x100b530; continue;
 }
 0x100b530 => {
  { let v: u32 = 0x11bcc4du32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x11bce93u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x100b53d; continue;
 }
 0x100b53d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x100b552; continue; }
  pc = 0x100b545; continue;
 }
 0x100b545 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100b552; continue;
 }
 0x100b552 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x100b53d; continue; }
  pc = 0x100b559; continue;
 }
 0x100b559 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100b530; continue; }
  pc = 0x100b55d; continue;
 }
 0x100b55d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b55e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b55e); }
 let mut pc: u32 = 0x100b55e;
 loop { match pc {
 0x100b55e => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x100b568; continue; }
  pc = 0x100b567; continue;
 }
 0x100b567 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b568 => {
  { cpu.sub8(cpu.r8(0x110d5d2u32), 0x0u8); }
  if cpu.zf { pc = 0x100b576; continue; }
  pc = 0x100b571; continue;
 }
 0x100b571 => {
  cpu.push(0x100b576); f_0100b461(cpu); if cpu.returned_to != 0x100b576 { return; }
  pc = 0x100b576; continue;
 }
 0x100b576 => {
  { let v: u32 = cpu.r32(0x110d714u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x100b5f3; continue; }
  pc = 0x100b585; continue;
 }
 0x100b585 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11bce8eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d3u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x100b5c2; continue; }
  pc = 0x100b5b5; continue;
 }
 0x100b5b5 => {
  if cpu.sf { pc = 0x100b5c2; continue; }
  pc = 0x100b5b7; continue;
 }
 0x100b5b7 => {
  { cpu.sub32(cpu.eax, 0xb800u32); }
  if (cpu.sf == cpu.of) { pc = 0x100b5c2; continue; }
  pc = 0x100b5be; continue;
 }
 0x100b5be => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x100b5c2; continue;
 }
 0x100b5c2 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x100b5cd); f_01059200(cpu); if cpu.returned_to != 0x100b5cd { return; }
  { let v: u32 = cpu.r32(0x110d714u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x110d71cu32), cpu.edi); }
  if !cpu.zf { pc = 0x100b5eb; continue; }
  pc = 0x100b5de; continue;
 }
 0x100b5de => {
  { let v: u32 = 0x11bcc4du32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x110d714u32, v); }
  pc = 0x100b5f1; continue;
 }
 0x100b5eb => {
  { let v: u32 = cpu.edi; cpu.w32(0x110d714u32, v); }
  pc = 0x100b5f1; continue;
 }
 0x100b5f1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b5f3 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b5f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b5f5); }
 let mut pc: u32 = 0x100b5f5;
 loop { match pc {
 0x100b5f5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d6ffu32, v); }
  { let v: u32 = 0x1152e54u32; cpu.esi = v; }
  { let v: u32 = 0x11b544du32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100b60b; continue;
 }
 0x100b60b => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x100b682; continue; }
  pc = 0x100b61c; continue;
 }
 0x100b61c => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x100b682; continue; }
  pc = 0x100b626; continue;
 }
 0x100b626 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x100b682; continue; }
  pc = 0x100b62c; continue;
 }
 0x100b62c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x100b646; continue; }
  pc = 0x100b632; continue;
 }
 0x100b632 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x100b646; continue; }
  pc = 0x100b638; continue;
 }
 0x100b638 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0xfu8); }
  if cpu.zf { pc = 0x100b646; continue; }
  pc = 0x100b63e; continue;
 }
 0x100b63e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x100b646; continue; }
  pc = 0x100b644; continue;
 }
 0x100b644 => {
  pc = 0x100b682; continue;
 }
 0x100b646 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6f7u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6fbu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x110d6ffu32)); cpu.w8(0x110d6ffu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x100b682; continue;
 }
 0x100b682 => {
  { let v: u32 = cpu.esi.wrapping_add(0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100b60b; continue; }
  pc = 0x100b68b; continue;
 }
 0x100b68b => {
  { let v: u32 = 0x11b544du32; cpu.w32(0x110d700u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x110d704u32, v); }
  { cpu.sub8(cpu.r8(0x110d6ffu32), 0x1u8); }
  if cpu.zf { pc = 0x100b6be; continue; }
  pc = 0x100b6ab; continue;
 }
 0x100b6ab => {
  { cpu.sub8(cpu.r8(0x110d6ffu32), 0x0u8); }
  if !cpu.zf { pc = 0x100b6bf; continue; }
  pc = 0x100b6b4; continue;
 }
 0x100b6b4 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d700u32, v); }
  pc = 0x100b6be; continue;
 }
 0x100b6be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b6bf => {
  { let v: u8 = cpu.r8(0x110d6ffu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x100b6c5; continue;
 }
 0x100b6c5 => {
  { let v: u32 = 0x11b544du32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x110d6ffu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x100b6d2; continue;
 }
 0x100b6d2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x100b6e7; continue; }
  pc = 0x100b6da; continue;
 }
 0x100b6da => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100b6e7; continue;
 }
 0x100b6e7 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x100b6d2; continue; }
  pc = 0x100b6ee; continue;
 }
 0x100b6ee => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100b6c5; continue; }
  pc = 0x100b6f2; continue;
 }
 0x100b6f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b6f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b6f3); }
 let mut pc: u32 = 0x100b6f3;
 loop { match pc {
 0x100b6f3 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x100b6fd; continue; }
  pc = 0x100b6fc; continue;
 }
 0x100b6fc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b6fd => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x100b714; continue; }
  pc = 0x100b713; continue;
 }
 0x100b713 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b714 => {
  { cpu.sub8(cpu.r8(0x110d5cfu32), 0x0u8); }
  if cpu.zf { pc = 0x100b722; continue; }
  pc = 0x100b71d; continue;
 }
 0x100b71d => {
  cpu.push(0x100b722); f_0100b5f5(cpu); if cpu.returned_to != 0x100b722 { return; }
  pc = 0x100b722; continue;
 }
 0x100b722 => {
  { let v: u32 = cpu.r32(0x110d700u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x100b797; continue; }
  pc = 0x100b731; continue;
 }
 0x100b731 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11bce8eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d3u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x100b76e; continue; }
  pc = 0x100b761; continue;
 }
 0x100b761 => {
  if cpu.sf { pc = 0x100b76e; continue; }
  pc = 0x100b763; continue;
 }
 0x100b763 => {
  { cpu.sub32(cpu.eax, 0xb800u32); }
  if (cpu.sf == cpu.of) { pc = 0x100b76e; continue; }
  pc = 0x100b76a; continue;
 }
 0x100b76a => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x100b76e; continue;
 }
 0x100b76e => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x100b779); f_01059200(cpu); if cpu.returned_to != 0x100b779 { return; }
  { let v: u32 = cpu.r32(0x110d700u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x110d704u32), cpu.edi); }
  if !cpu.zf { pc = 0x100b78f; continue; }
  pc = 0x100b78a; continue;
 }
 0x100b78a => {
  { let v: u32 = 0x11b544du32; cpu.edi = v; }
  pc = 0x100b78f; continue;
 }
 0x100b78f => {
  { let v: u32 = cpu.edi; cpu.w32(0x110d700u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b797 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b799(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100b799); } r_0100b799(cpu, 0x100b799); }
pub fn f_0100b834(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100b834); } r_0100b799(cpu, 0x100b834); }
pub fn f_0100b866(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100b866); } r_0100b799(cpu, 0x100b866); }
fn r_0100b799(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x100b799 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d6ffu32, v); }
  { let v: u32 = 0x1152e54u32; cpu.esi = v; }
  { let v: u32 = 0x11b724du32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100b7af; continue;
 }
 0x100b7af => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x100b80a; continue; }
  pc = 0x100b7bc; continue;
 }
 0x100b7bc => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x100b80a; continue; }
  pc = 0x100b7c2; continue;
 }
 0x100b7c2 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x100b80a; continue; }
  pc = 0x100b7c8; continue;
 }
 0x100b7c8 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x100b80a; continue; }
  pc = 0x100b7ce; continue;
 }
 0x100b7ce => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6f7u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6fbu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x110d6ffu32)); cpu.w8(0x110d6ffu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x100b80a; continue;
 }
 0x100b80a => {
  { let v: u32 = cpu.esi.wrapping_add(0xb8u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100b7af; continue; }
  pc = 0x100b812; continue;
 }
 0x100b812 => {
  { let v: u32 = 0x11b724du32; cpu.w32(0x110d708u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x110d70cu32, v); }
  { cpu.sub8(cpu.r8(0x110d6ffu32), 0x1u8); }
  if cpu.zf { pc = 0x100b845; continue; }
  pc = 0x100b832; continue;
 }
 0x100b832 => {
  { cpu.sub8(cpu.r8(0x110d6ffu32), 0x0u8); }
  if !cpu.zf { pc = 0x100b846; continue; }
  pc = 0x100b83b; continue;
 }
 0x100b834 => {
  let t = cpu.esi; cpu.push(0x100b836); cpu.call(t); if cpu.returned_to != 0x100b836 { return; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0xbu32.wrapping_add(cpu.ebp)), ((cpu.edx >> 8) as u8)); cpu.w8(0xbu32.wrapping_add(cpu.ebp), v); }
  pc = 0x100b83b; continue;
 }
 0x100b83b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d708u32, v); }
  pc = 0x100b845; continue;
 }
 0x100b845 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b846 => {
  { let v: u8 = cpu.r8(0x110d6ffu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x100b84c; continue;
 }
 0x100b84c => {
  { let v: u32 = 0x11b724du32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x110d6ffu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x100b859; continue;
 }
 0x100b859 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x100b86e; continue; }
  pc = 0x100b861; continue;
 }
 0x100b861 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100b86e; continue;
 }
 0x100b866 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100b86e; continue;
 }
 0x100b86e => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x100b859; continue; }
  pc = 0x100b875; continue;
 }
 0x100b875 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100b84c; continue; }
  pc = 0x100b879; continue;
 }
 0x100b879 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b87a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b87a); }
 let mut pc: u32 = 0x100b87a;
 loop { match pc {
 0x100b87a => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x100b884; continue; }
  pc = 0x100b883; continue;
 }
 0x100b883 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b884 => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x100b89b; continue; }
  pc = 0x100b89a; continue;
 }
 0x100b89a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b89b => {
  { cpu.sub8(cpu.r8(0x110d5d1u32), 0x0u8); }
  if cpu.zf { pc = 0x100b8a9; continue; }
  pc = 0x100b8a4; continue;
 }
 0x100b8a4 => {
  cpu.push(0x100b8a9); f_0100b799(cpu); if cpu.returned_to != 0x100b8a9 { return; }
  pc = 0x100b8a9; continue;
 }
 0x100b8a9 => {
  { let v: u32 = cpu.r32(0x110d708u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x100b91e; continue; }
  pc = 0x100b8b8; continue;
 }
 0x100b8b8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11bce8eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d3u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x100b8f5; continue; }
  pc = 0x100b8e8; continue;
 }
 0x100b8e8 => {
  if cpu.sf { pc = 0x100b8f5; continue; }
  pc = 0x100b8ea; continue;
 }
 0x100b8ea => {
  { cpu.sub32(cpu.eax, 0xb800u32); }
  if (cpu.sf == cpu.of) { pc = 0x100b8f5; continue; }
  pc = 0x100b8f1; continue;
 }
 0x100b8f1 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x100b8f5; continue;
 }
 0x100b8f5 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x100b900); f_01059200(cpu); if cpu.returned_to != 0x100b900 { return; }
  { let v: u32 = cpu.r32(0x110d708u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x110d70cu32), cpu.edi); }
  if !cpu.zf { pc = 0x100b916; continue; }
  pc = 0x100b911; continue;
 }
 0x100b911 => {
  { let v: u32 = 0x11b724du32; cpu.edi = v; }
  pc = 0x100b916; continue;
 }
 0x100b916 => {
  { let v: u32 = cpu.edi; cpu.w32(0x110d708u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b91e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100b920(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100b920); }
 let mut pc: u32 = 0x100b920;
 loop { match pc {
 0x100b920 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d6ffu32, v); }
  { let v: u32 = 0x1152e54u32; cpu.esi = v; }
  { let v: u32 = 0x11b904du32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100b936; continue;
 }
 0x100b936 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x100b991; continue; }
  pc = 0x100b943; continue;
 }
 0x100b943 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x100b991; continue; }
  pc = 0x100b949; continue;
 }
 0x100b949 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x100b991; continue; }
  pc = 0x100b94f; continue;
 }
 0x100b94f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x100b991; continue; }
  pc = 0x100b955; continue;
 }
 0x100b955 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6f7u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6fbu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x110d6ffu32)); cpu.w8(0x110d6ffu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x100b991; continue;
 }
 0x100b991 => {
  { let v: u32 = cpu.esi.wrapping_add(0xb8u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100b936; continue; }
  pc = 0x100b999; continue;
 }
 0x100b999 => {
  { let v: u32 = 0x11b904du32; cpu.w32(0x110d710u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x110d718u32, v); }
  { cpu.sub8(cpu.r8(0x110d6ffu32), 0x1u8); }
  if cpu.zf { pc = 0x100b9cc; continue; }
  pc = 0x100b9b9; continue;
 }
 0x100b9b9 => {
  { cpu.sub8(cpu.r8(0x110d6ffu32), 0x0u8); }
  if !cpu.zf { pc = 0x100b9cd; continue; }
  pc = 0x100b9c2; continue;
 }
 0x100b9c2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d710u32, v); }
  pc = 0x100b9cc; continue;
 }
 0x100b9cc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100b9cd => {
  { let v: u8 = cpu.r8(0x110d6ffu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x100b9d3; continue;
 }
 0x100b9d3 => {
  { let v: u32 = 0x11b904du32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x110d6ffu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x100b9e0; continue;
 }
 0x100b9e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x100b9f5; continue; }
  pc = 0x100b9e8; continue;
 }
 0x100b9e8 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100b9f5; continue;
 }
 0x100b9f5 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x100b9e0; continue; }
  pc = 0x100b9fc; continue;
 }
 0x100b9fc => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100b9d3; continue; }
  pc = 0x100ba00; continue;
 }
 0x100ba00 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ba01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ba01); }
 let mut pc: u32 = 0x100ba01;
 loop { match pc {
 0x100ba01 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x100ba0b; continue; }
  pc = 0x100ba0a; continue;
 }
 0x100ba0a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ba0b => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x100ba22; continue; }
  pc = 0x100ba21; continue;
 }
 0x100ba21 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ba22 => {
  { cpu.sub8(cpu.r8(0x110d5d0u32), 0x0u8); }
  if cpu.zf { pc = 0x100ba30; continue; }
  pc = 0x100ba2b; continue;
 }
 0x100ba2b => {
  cpu.push(0x100ba30); f_0100b920(cpu); if cpu.returned_to != 0x100ba30 { return; }
  pc = 0x100ba30; continue;
 }
 0x100ba30 => {
  { let v: u32 = cpu.r32(0x110d710u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x100baa5; continue; }
  pc = 0x100ba3f; continue;
 }
 0x100ba3f => {
  { let v: u8 = 0x1u8; cpu.w8(0x11bce8eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d3u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x100ba7c; continue; }
  pc = 0x100ba6f; continue;
 }
 0x100ba6f => {
  if cpu.sf { pc = 0x100ba7c; continue; }
  pc = 0x100ba71; continue;
 }
 0x100ba71 => {
  { cpu.sub32(cpu.eax, 0xb800u32); }
  if (cpu.sf == cpu.of) { pc = 0x100ba7c; continue; }
  pc = 0x100ba78; continue;
 }
 0x100ba78 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x100ba7c; continue;
 }
 0x100ba7c => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x100ba87); f_01059200(cpu); if cpu.returned_to != 0x100ba87 { return; }
  { let v: u32 = cpu.r32(0x110d710u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x110d718u32), cpu.edi); }
  if !cpu.zf { pc = 0x100ba9d; continue; }
  pc = 0x100ba98; continue;
 }
 0x100ba98 => {
  { let v: u32 = 0x11b904du32; cpu.edi = v; }
  pc = 0x100ba9d; continue;
 }
 0x100ba9d => {
  { let v: u32 = cpu.edi; cpu.w32(0x110d710u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100baa5 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100baa7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100baa7); } r_0100baa7(cpu, 0x100baa7); }
pub fn f_0100bb01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100bb01); } r_0100baa7(cpu, 0x100bb01); }
fn r_0100baa7(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x100baa7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d6ffu32, v); }
  { let v: u32 = 0x116c0d8u32; cpu.esi = v; }
  { let v: u32 = 0x11bcc4du32; cpu.edi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x100babd; continue;
 }
 0x100babd => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100bb43; continue; }
  pc = 0x100bac8; continue;
 }
 0x100bac8 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x100bb43; continue; }
  pc = 0x100bad0; continue;
 }
 0x100bad0 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x100bae1; continue; }
  pc = 0x100bad4; continue;
 }
 0x100bad4 => {
  { cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100bb43; continue; }
  pc = 0x100bae1; continue;
 }
 0x100bae1 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100bb43; continue; }
  pc = 0x100bae9; continue;
 }
 0x100bae9 => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x100bb43; continue; }
  pc = 0x100baf0; continue;
 }
 0x100baf0 => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x100bb43; continue; }
  pc = 0x100bafa; continue;
 }
 0x100bafa => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x100bb43; continue; }
  pc = 0x100bb02; continue;
 }
 0x100bb01 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x100bb02; continue;
 }
 0x100bb02 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x100bb43; continue; }
  pc = 0x100bb08; continue;
 }
 0x100bb08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6f7u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6fbu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x110d6ffu32)); cpu.w8(0x110d6ffu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x100bb43; continue;
 }
 0x100bb43 => {
  { let v: u32 = cpu.esi.wrapping_add(0x50u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100babd; continue; }
  pc = 0x100bb4d; continue;
 }
 0x100bb4d => {
  { let v: u32 = 0x11bcc4du32; cpu.w32(0x110d714u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d5d2u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x110d71cu32, v); }
  { cpu.sub8(cpu.r8(0x110d6ffu32), 0x1u8); }
  if cpu.zf { pc = 0x100bb80; continue; }
  pc = 0x100bb6d; continue;
 }
 0x100bb6d => {
  { cpu.sub8(cpu.r8(0x110d6ffu32), 0x0u8); }
  if !cpu.zf { pc = 0x100bb81; continue; }
  pc = 0x100bb76; continue;
 }
 0x100bb76 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d714u32, v); }
  pc = 0x100bb80; continue;
 }
 0x100bb80 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100bb81 => {
  { let v: u8 = cpu.r8(0x110d6ffu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x100bb87; continue;
 }
 0x100bb87 => {
  { let v: u32 = 0x11bcc4du32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x110d6ffu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x100bb94; continue;
 }
 0x100bb94 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x100bba9; continue; }
  pc = 0x100bb9c; continue;
 }
 0x100bb9c => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100bba9; continue;
 }
 0x100bba9 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x100bbb2; continue; }
  pc = 0x100bbb0; continue;
 }
 0x100bbb0 => {
  cpu.poll(); pc = 0x100bb94; continue;
 }
 0x100bbb2 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100bb87; continue; }
  pc = 0x100bbb6; continue;
 }
 0x100bbb6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100bbb7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100bbb7); }
 let mut pc: u32 = 0x100bbb7;
 loop { match pc {
 0x100bbb7 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x100bbc1; continue; }
  pc = 0x100bbc0; continue;
 }
 0x100bbc0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100bbc1 => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x100bbd8; continue; }
  pc = 0x100bbd7; continue;
 }
 0x100bbd7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100bbd8 => {
  { cpu.sub8(cpu.r8(0x110d5d2u32), 0x0u8); }
  if cpu.zf { pc = 0x100bbe6; continue; }
  pc = 0x100bbe1; continue;
 }
 0x100bbe1 => {
  cpu.push(0x100bbe6); f_0100baa7(cpu); if cpu.returned_to != 0x100bbe6 { return; }
  pc = 0x100bbe6; continue;
 }
 0x100bbe6 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11bce8eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d3u32, v); }
  { let v: u32 = cpu.r32(0x110d714u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x100bc71; continue; }
  pc = 0x100bc18; continue;
 }
 0x100bc18 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x110eab4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed82u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed83u32, v); }
  cpu.push(0x100bc34); f_010863ff(cpu); if cpu.returned_to != 0x100bc34 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x100bc3a); f_010586d4(cpu); if cpu.returned_to != 0x100bc3a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3ce3u32).wrapping_sub(0x40u16); cpu.w16(0x10e3ce3u32, v); }
  { let v: u16 = cpu.r16(0x10e3ce5u32).wrapping_sub(0x20u16); cpu.w16(0x10e3ce5u32, v); }
  cpu.push(0x100bc50); f_01015c29(cpu); if cpu.returned_to != 0x100bc50 { return; }
  cpu.push(0x100bc55); f_01057945(cpu); if cpu.returned_to != 0x100bc55 { return; }
  { let v: u32 = cpu.r32(0x110d714u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x110d71cu32), cpu.edi); }
  if !cpu.zf { pc = 0x100bc6b; continue; }
  pc = 0x100bc66; continue;
 }
 0x100bc66 => {
  { let v: u32 = 0x11bcc4du32; cpu.edi = v; }
  pc = 0x100bc6b; continue;
 }
 0x100bc6b => {
  { let v: u32 = cpu.edi; cpu.w32(0x110d714u32, v); }
  pc = 0x100bc71; continue;
 }
 0x100bc71 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100bc72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100bc72); }
 let mut pc: u32 = 0x100bc72;
 loop { match pc {
 0x100bc72 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d6ffu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11b5436u32, v); }
  cpu.push(0x100bc88); f_0100bddf(cpu); if cpu.returned_to != 0x100bc88 { return; }
  { let v: u32 = 0x1152e54u32; cpu.esi = v; }
  { let v: u32 = 0x11bae4du32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100bc97; continue;
 }
 0x100bc97 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x100bd0b; continue; }
  pc = 0x100bca8; continue;
 }
 0x100bca8 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x100bd0b; continue; }
  pc = 0x100bcae; continue;
 }
 0x100bcae => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x100bd0b; continue; }
  pc = 0x100bcb4; continue;
 }
 0x100bcb4 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x100bcbc); f_0100bdf5(cpu); if cpu.returned_to != 0x100bcbc { return; }
  if cpu.cf { pc = 0x100bd0b; continue; }
  pc = 0x100bcbe; continue;
 }
 0x100bcbe => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x11d01c1u32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6f7u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6fbu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x110d6ffu32)); cpu.w8(0x110d6ffu32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11b5436u32)); cpu.w32(0x11b5436u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x100bd0b; continue;
 }
 0x100bd0b => {
  { let v: u32 = cpu.esi.wrapping_add(0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100bc97; continue; }
  pc = 0x100bd14; continue;
 }
 0x100bd14 => {
  { let v: u32 = 0x11bae4du32; cpu.w32(0x110d720u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110d5d3u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x110d724u32, v); }
  { cpu.sub8(cpu.r8(0x110d6ffu32), 0x1u8); }
  if cpu.zf { pc = 0x100bd47; continue; }
  pc = 0x100bd34; continue;
 }
 0x100bd34 => {
  { cpu.sub8(cpu.r8(0x110d6ffu32), 0x0u8); }
  if !cpu.zf { pc = 0x100bd48; continue; }
  pc = 0x100bd3d; continue;
 }
 0x100bd3d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d720u32, v); }
  pc = 0x100bd47; continue;
 }
 0x100bd47 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100bd48 => {
  { let v: u8 = cpu.r8(0x110d6ffu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x100bd4e; continue;
 }
 0x100bd4e => {
  { let v: u32 = 0x11bae4du32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x110d6ffu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x100bd5b; continue;
 }
 0x100bd5b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x100bd70; continue; }
  pc = 0x100bd63; continue;
 }
 0x100bd63 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100bd70; continue;
 }
 0x100bd70 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x100bd5b; continue; }
  pc = 0x100bd77; continue;
 }
 0x100bd77 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100bd4e; continue; }
  pc = 0x100bd7b; continue;
 }
 0x100bd7b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100bd7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100bd7c); }
 let mut pc: u32 = 0x100bd7c;
 loop { match pc {
 0x100bd7c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100bd96; continue; }
  pc = 0x100bd87; continue;
 }
 0x100bd87 => {
  { let v: u32 = 0x11d01c1u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x100bd98; continue; }
  pc = 0x100bd94; continue;
 }
 0x100bd94 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100bd96 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100bd98; continue;
 }
 0x100bd98 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100bd9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100bd9a); }
 let mut pc: u32 = 0x100bd9a;
 loop { match pc {
 0x100bd9a => {
  { cpu.pushad(); }
  { let v: u32 = 0x1152e54u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100bda5; continue;
 }
 0x100bda5 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x100bdcd; continue; }
  pc = 0x100bdb2; continue;
 }
 0x100bdb2 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x100bdcd; continue; }
  pc = 0x100bdb8; continue;
 }
 0x100bdb8 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x100bdcd; continue; }
  pc = 0x100bdbe; continue;
 }
 0x100bdbe => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcbf5u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x11c5b0fu32, v); }
  pc = 0x100bddd; continue;
 }
 0x100bdcd => {
  { let v: u32 = cpu.add32(cpu.esi, 0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100bda5; continue; }
  pc = 0x100bdd6; continue;
 }
 0x100bdd6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fcbf5u32, v); }
  pc = 0x100bddd; continue;
 }
 0x100bddd => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100bddf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100bddf); }
 let mut pc: u32 = 0x100bddf;
 loop { match pc {
 0x100bddf => {
  { let v: u32 = 0x11d01c1u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x100bdee; continue;
 }
 0x100bdee => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100bdee; continue; }
  pc = 0x100bdf4; continue;
 }
 0x100bdf4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100bdf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100bdf5); }
 let mut pc: u32 = 0x100bdf5;
 loop { match pc {
 0x100bdf5 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100be0d; continue; }
  pc = 0x100bdf9; continue;
 }
 0x100bdf9 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x11d01c1u32; cpu.edi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x100be0d; continue; }
  pc = 0x100be0b; continue;
 }
 0x100be0b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100be0d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100be0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100be0f); }
 let mut pc: u32 = 0x100be0f;
 loop { match pc {
 0x100be0f => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x100be19; continue; }
  pc = 0x100be18; continue;
 }
 0x100be18 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100be19 => {
  { cpu.sub8(cpu.r8(0x11bd2f6u32), 0x0u8); }
  if !cpu.zf { pc = 0x100bed2; continue; }
  pc = 0x100be26; continue;
 }
 0x100be26 => {
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x100be3d; continue; }
  pc = 0x100be3c; continue;
 }
 0x100be3c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100be3d => {
  { cpu.sub8(cpu.r8(0x110d5d3u32), 0x0u8); }
  if cpu.zf { pc = 0x100be4b; continue; }
  pc = 0x100be46; continue;
 }
 0x100be46 => {
  cpu.push(0x100be4b); f_0100bc72(cpu); if cpu.returned_to != 0x100be4b { return; }
  pc = 0x100be4b; continue;
 }
 0x100be4b => {
  { let v: u32 = cpu.r32(0x110d720u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x100bedc; continue; }
  pc = 0x100be5a; continue;
 }
 0x100be5a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11bce8eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x100bebc; continue; }
  pc = 0x100be8e; continue;
 }
 0x100be8e => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x100bebc; continue; }
  pc = 0x100be94; continue;
 }
 0x100be94 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.eax)), 0x80u8); }
  if cpu.zf { pc = 0x100bebc; continue; }
  pc = 0x100be9a; continue;
 }
 0x100be9a => {
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x100beb1; continue; }
  pc = 0x100bea4; continue;
 }
 0x100bea4 => {
  if cpu.sf { pc = 0x100beda; continue; }
  pc = 0x100bea6; continue;
 }
 0x100bea6 => {
  { cpu.sub32(cpu.eax, 0xac80u32); }
  if (cpu.sf == cpu.of) { pc = 0x100beb1; continue; }
  pc = 0x100bead; continue;
 }
 0x100bead => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  pc = 0x100beb1; continue;
 }
 0x100beb1 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x100bebc); f_01059200(cpu); if cpu.returned_to != 0x100bebc { return; }
  pc = 0x100bebc; continue;
 }
 0x100bebc => {
  { let v: u32 = cpu.r32(0x110d720u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x110d724u32), cpu.edi); }
  if !cpu.zf { pc = 0x100bed2; continue; }
  pc = 0x100becd; continue;
 }
 0x100becd => {
  { let v: u32 = 0x11bae4du32; cpu.edi = v; }
  pc = 0x100bed2; continue;
 }
 0x100bed2 => {
  { let v: u32 = cpu.edi; cpu.w32(0x110d720u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100beda => {
  cpu.poll(); pc = 0x100bebc; continue;
 }
 0x100bedc => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100bede(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100bede); }
 let mut pc: u32 = 0x100bede;
 loop { match pc {
 0x100bede => {
  { let v: u32 = cpu.r32(0x10e3f78u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xaau32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110c265u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100beee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100beee); }
 let mut pc: u32 = 0x100beee;
 loop { match pc {
 0x100beee => {
  { let v: u32 = cpu.r32(0x10e3f78u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x110c265u32), cpu.eax); }
  if !cpu.zf { pc = 0x100bf28; continue; }
  pc = 0x100befb; continue;
 }
 0x100befb => {
  { let v: u32 = cpu.add32(cpu.eax, 0xaau32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110c265u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11bce8eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5cfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d1u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  pc = 0x100bf28; continue;
 }
 0x100bf28 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100bf29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100bf29); }
 let mut pc: u32 = 0x100bf29;
 loop { match pc {
 0x100bf29 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3e9bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x110d769u32, v); }
  cpu.push(0x100bf5a); f_0101b70a(cpu); if cpu.returned_to != 0x100bf5a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d769u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100bf63(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100bf63); }
 let mut pc: u32 = 0x100bf63;
 loop { match pc {
 0x100bf63 => {
  { cpu.sub8(cpu.r8(0x11bd2f6u32), 0x0u8); }
  if !cpu.zf { pc = 0x100bf79; continue; }
  pc = 0x100bf6c; continue;
 }
 0x100bf6c => {
  { cpu.and8(cpu.r8(0x10ee968u32), 0x1u8); }
  if cpu.zf { pc = 0x100bfc9; continue; }
  pc = 0x100bf79; continue;
 }
 0x100bf79 => {
  { cpu.sub8(cpu.r8(0x110d76eu32), 0xffu8); }
  if cpu.zf { pc = 0x100bfd3; continue; }
  pc = 0x100bf86; continue;
 }
 0x100bf86 => {
  { let v: u32 = cpu.r32(0x110d76fu32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x110d78fu32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x110d777u32.wrapping_add(cpu.ecx.wrapping_mul(2)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x110d76eu32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x100bfc9; continue; }
  pc = 0x100bfa7; continue;
 }
 0x100bfa7 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x110d76fu32)); cpu.w32(0x110d76fu32, v); }
  { let v: u32 = cpu.r32(0x110d76fu32); cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x110d78fu32.wrapping_add(cpu.ecx.wrapping_mul(2))), 0x223du16); }
  if !cpu.zf { pc = 0x100bfd3; continue; }
  pc = 0x100bfbf; continue;
 }
 0x100bfbf => {
  { let v: u32 = 0x1u32; cpu.w32(0x110d751u32, v); }
  pc = 0x100bfc9; continue;
 }
 0x100bfc9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110d76fu32, v); }
  pc = 0x100bfd3; continue;
 }
 0x100bfd3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100bfd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100bfd4); }
 let mut pc: u32 = 0x100bfd4;
 loop { match pc {
 0x100bfd4 => {
  { cpu.sub32(cpu.r32(0x110d751u32), 0x0u32); }
  if !cpu.zf { pc = 0x100bfde; continue; }
  pc = 0x100bfdd; continue;
 }
 0x100bfdd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100bfde => {
  { let v: u32 = 0x110d801u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d75eu32, v); }
  { let v: u16 = 0x20u16; cpu.w16(0x110d75au32, v); }
  { let v: u16 = 0x20u16; cpu.w16(0x110d75cu32, v); }
  pc = 0x100bffc; continue;
 }
 0x100bffc => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0x824u16); }
  if cpu.zf { pc = 0x100c045; continue; }
  pc = 0x100c008; continue;
 }
 0x100c008 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x110d75eu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x110d75fu32.wrapping_add(cpu.ecx.wrapping_mul(2)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x110d75eu32)); cpu.w8(0x110d75eu32, v); }
  { let v: u8 = cpu.r8(0x110d75eu32) & 0x3u8; cpu.w8(0x110d75eu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110d75au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110d75cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x100c03a); f_0104ba91(cpu); if cpu.returned_to != 0x100c03a { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d75au32, v); }
  cpu.poll(); pc = 0x100bffc; continue;
 }
 0x100c045 => {
  { let v: u32 = 0x1003dcfu32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x110d75au32).wrapping_add(0x20u16); cpu.w16(0x110d75au32, v); }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  pc = 0x100c057; continue;
 }
 0x100c057 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110d75au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110d75cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x100c072); f_0104ba91(cpu); if cpu.returned_to != 0x100c072 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d75au32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100c057; continue; }
  pc = 0x100c07b; continue;
 }
 0x100c07b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100c07c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100c07c); }
 let mut pc: u32 = 0x100c07c;
 loop { match pc {
 0x100c07c => {
  { cpu.sub8(cpu.r8(0x11bd2f6u32), 0x0u8); }
  if !cpu.zf { pc = 0x100c092; continue; }
  pc = 0x100c085; continue;
 }
 0x100c085 => {
  { cpu.and8(cpu.r8(0x10ee968u32), 0x1u8); }
  if cpu.zf { pc = 0x100c0e2; continue; }
  pc = 0x100c092; continue;
 }
 0x100c092 => {
  { cpu.sub8(cpu.r8(0x110d76eu32), 0xffu8); }
  if cpu.zf { pc = 0x100c0ec; continue; }
  pc = 0x100c09f; continue;
 }
 0x100c09f => {
  { let v: u32 = cpu.r32(0x110d773u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x110d7adu32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x110d787u32.wrapping_add(cpu.ecx.wrapping_mul(2)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x110d76eu32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x100c0e2; continue; }
  pc = 0x100c0c0; continue;
 }
 0x100c0c0 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x110d773u32)); cpu.w32(0x110d773u32, v); }
  { let v: u32 = cpu.r32(0x110d773u32); cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x110d7adu32.wrapping_add(cpu.ecx.wrapping_mul(2))), 0x223du16); }
  if !cpu.zf { pc = 0x100c0ec; continue; }
  pc = 0x100c0d8; continue;
 }
 0x100c0d8 => {
  { let v: u32 = 0x1u32; cpu.w32(0x110d755u32, v); }
  pc = 0x100c0e2; continue;
 }
 0x100c0e2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110d773u32, v); }
  pc = 0x100c0ec; continue;
 }
 0x100c0ec => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100c173(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100c173); }
 let mut pc: u32 = 0x100c173;
 loop { match pc {
 0x100c173 => {
  { let v: u16 = cpu.r16(0x10fd3f3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16(cpu.r16(0x10fd3f3u32)); cpu.w16(0x10fd3f3u32, v); }
  { cpu.sub16(cpu.r16(0x10fd3f3u32), 0x5u16); }
  if !cpu.zf { pc = 0x100c194; continue; }
  pc = 0x100c18b; continue;
 }
 0x100c18b => {
  { let v: u16 = 0x0u16; cpu.w16(0x10fd3f3u32, v); }
  pc = 0x100c194; continue;
 }
 0x100c194 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100c195(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100c195); }
 let mut pc: u32 = 0x100c195;
 loop { match pc {
 0x100c195 => {
  { cpu.sub16(cpu.r16(0x10fd3f3u32), 0x0u16); }
  if !cpu.zf { pc = 0x100c1a8; continue; }
  pc = 0x100c19f; continue;
 }
 0x100c19f => {
  { let v: u16 = 0x3ffu16; cpu.w16(0x10fd3f3u32, v); }
  pc = 0x100c1a8; continue;
 }
 0x100c1a8 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  cpu.push(0x100c1b7); f_0105f2ec(cpu); if cpu.returned_to != 0x100c1b7 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x100c1cd; continue; }
  pc = 0x100c1c8; continue;
 }
 0x100c1c8 => {
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100c1c8; continue; }
  pc = 0x100c1cd; continue;
 }
 0x100c1cd => {
  { cpu.and16(cpu.r16(0x10fd3f3u32), (cpu.eax as u16)); }
  if cpu.zf { f_0100c173(cpu); return; }
  pc = 0x100c1d6; continue;
 }
 0x100c1d6 => {
  { let v: u16 = !(cpu.eax as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16(cpu.r16(0x10fd3f3u32), (cpu.eax as u16)); cpu.w16(0x10fd3f3u32, v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100c328(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100c328); }
 let mut pc: u32 = 0x100c328;
 loop { match pc {
 0x100c328 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1105881u32; cpu.esi = v; }
  { let v: u32 = 0x11058dau32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x110587du32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x100c350; continue; }
  pc = 0x100c33e; continue;
 }
 0x100c33e => {
  { let v: u32 = cpu.r32(0x11c5bb8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x100c347; continue;
 }
 0x100c347 => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100c34e; continue; }
  pc = 0x100c34d; continue;
 }
 0x100c34d => {
  { cpu.stos(1, false); }
  pc = 0x100c34e; continue;
 }
 0x100c34e => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100c347; continue; }
  pc = 0x100c350; continue;
 }
 0x100c350 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100c355(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100c355); }
 let mut pc: u32 = 0x100c355;
 loop { match pc {
 0x100c355 => {
  { cpu.pushad(); }
  { let v: u32 = 0x11b48a1u32; cpu.edi = v; }
  { let v: u16 = 0x34u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xbau16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x100c379; continue; }
  pc = 0x100c36c; continue;
 }
 0x100c36c => {
  { let v: u16 = 0x68u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x1cu32); cpu.ebx = v; }
  pc = 0x100c379; continue;
 }
 0x100c379 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x11b5187u32) as u32; cpu.eax = v; }
  cpu.push(0x100c38d); f_0100c328(cpu); if cpu.returned_to != 0x100c38d { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x11058dau32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x1u16; cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x1121308u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x4u16); }
  if cpu.zf { pc = 0x100c3c6; continue; }
  pc = 0x100c3b2; continue;
 }
 0x100c3b2 => {
  { let v: u32 = 0x1121314u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if cpu.zf { pc = 0x100c3c6; continue; }
  pc = 0x100c3c1; continue;
 }
 0x100c3c1 => {
  { let v: u32 = 0x112131du32; cpu.ebp = v; }
  pc = 0x100c3c6; continue;
 }
 0x100c3c6 => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x10ee91du32)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.sf { pc = 0x100c3ee; continue; }
  pc = 0x100c3de; continue;
 }
 0x100c3de => {
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x100c3ee; continue; }
  pc = 0x100c3e6; continue;
 }
 0x100c3e6 => {
  { let v: u8 = cpu.r8(0x110c249u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x100c3ee; continue;
 }
 0x100c3ee => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100c3c6; continue; }
  pc = 0x100c3f1; continue;
 }
 0x100c3f1 => {
  { let v: u8 = cpu.r8(0x110c24du32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x0u8); }
  if cpu.zf { pc = 0x100c44d; continue; }
  pc = 0x100c400; continue;
 }
 0x100c400 => {
  { let v: u32 = 0x110c261u32; cpu.eax = v; }
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x100c40d; continue; }
  pc = 0x100c409; continue;
 }
 0x100c409 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x100c40d; continue;
 }
 0x100c40d => {
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x100c415; continue; }
  pc = 0x100c411; continue;
 }
 0x100c411 => {
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x100c415; continue;
 }
 0x100c415 => {
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x100c41d; continue; }
  pc = 0x100c419; continue;
 }
 0x100c419 => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x100c41d; continue;
 }
 0x100c41d => {
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x100c425; continue; }
  pc = 0x100c421; continue;
 }
 0x100c421 => {
  { let v: u8 = 0x3u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x100c425; continue;
 }
 0x100c425 => {
  { let v: u32 = 0x110c261u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110c25du32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e3f78u32); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if !cpu.zf { pc = 0x100c43c; continue; }
  pc = 0x100c43b; continue;
 }
 0x100c43b => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x100c43c; continue;
 }
 0x100c43c => {
  { cpu.sub8((cpu.ebx as u8), (cpu.edx as u8)); }
  if (cpu.sf != cpu.of) { pc = 0x100c442; continue; }
  pc = 0x100c440; continue;
 }
 0x100c440 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x100c442; continue;
 }
 0x100c442 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x110c25du32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x100c44f; continue;
 }
 0x100c44d => {
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x100c44f; continue;
 }
 0x100c44f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100c454(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100c454); }
 let mut pc: u32 = 0x100c454;
 loop { match pc {
 0x100c454 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x110587bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x110587du32, v); }
  { let v: u32 = 0x1105881u32; cpu.edi = v; }
  { let v: u32 = 0x3du32; cpu.ecx = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x11b48a1u32; cpu.edi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100c486(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100c486); }
 let mut pc: u32 = 0x100c486;
 loop { match pc {
 0x100c486 => {
  { cpu.pushad(); }
  { let v: u32 = 0x19u32; cpu.w32(0x110587du32, v); }
  { let v: u32 = 0x1105881u32; cpu.edi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = 0x0u8; cpu.w8(0x110587bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x110587du32, v); }
  { let v: u32 = 0x1105881u32; cpu.edi = v; }
  { let v: u32 = 0x3du32; cpu.ecx = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x11b48a1u32; cpu.edi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100c4d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100c4d3); }
 let mut pc: u32 = 0x100c4d3;
 loop { match pc {
 0x100c4d3 => {
  { cpu.sub32(cpu.r32(0x110587du32), 0x0u32); }
  if cpu.zf { pc = 0x100c58e; continue; }
  pc = 0x100c4e0; continue;
 }
 0x100c4e0 => {
  { let v: u32 = 0x1105937u32; cpu.edi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = 0x1105881u32; cpu.esi = v; }
  { let v: u32 = 0x110593au32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x110587du32); cpu.ecx = v; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r8(0x11b5187u32) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x11c5bb8u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x100c515; continue;
 }
 0x100c515 => {
  { cpu.lods(1, false); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))), 0xffu8); }
  if !cpu.zf { pc = 0x100c51e; continue; }
  pc = 0x100c51c; continue;
 }
 0x100c51c => {
  { let v: u8 = 0x39u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x100c51e; continue;
 }
 0x100c51e => {
  { cpu.stos(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100c515; continue; }
  pc = 0x100c521; continue;
 }
 0x100c521 => {
  { let v: u8 = 0xfdu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = 0x1105937u32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x1121308u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x4u16); }
  if cpu.zf { pc = 0x100c554; continue; }
  pc = 0x100c540; continue;
 }
 0x100c540 => {
  { let v: u32 = 0x1121314u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if cpu.zf { pc = 0x100c554; continue; }
  pc = 0x100c54f; continue;
 }
 0x100c54f => {
  { let v: u32 = 0x112131du32; cpu.ebp = v; }
  pc = 0x100c554; continue;
 }
 0x100c554 => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x10ee91du32)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.sf { pc = 0x100c57c; continue; }
  pc = 0x100c56c; continue;
 }
 0x100c56c => {
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x100c57c; continue; }
  pc = 0x100c574; continue;
 }
 0x100c574 => {
  { let v: u8 = cpu.r8(0x110c249u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x100c57c; continue;
 }
 0x100c57c => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100c554; continue; }
  pc = 0x100c57f; continue;
 }
 0x100c57f => {
  cpu.push(0x100c584); f_0100c486(cpu); if cpu.returned_to != 0x100c584 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x100c58e); f_010379f8(cpu); if cpu.returned_to != 0x100c58e { return; }
  pc = 0x100c58e; continue;
 }
 0x100c58e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100c58f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100c58f); }
 let mut pc: u32 = 0x100c58f;
 loop { match pc {
 0x100c58f => {
  { let v: u32 = cpu.r32(0x10e3ff8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0xfffffffdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x110c249u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), (cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x100c5f7; continue; }
  pc = 0x100c5ac; continue;
 }
 0x100c5ac => {
  { let v: u32 = cpu.r8(0xfffffffeu32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11c5bb8u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = 0x110c20cu32; cpu.edi = v; }
  pc = 0x100c5c0; continue;
 }
 0x100c5c0 => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x100c5c0; continue; }
  pc = 0x100c5c5; continue;
 }
 0x100c5c5 => {
  { let v: u32 = cpu.r8(0x1124ee5u32) as u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x100c5e5; continue; }
  pc = 0x100c5d5; continue;
 }
 0x100c5d5 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x100c5e5; continue; }
  pc = 0x100c5e3; continue;
 }
 0x100c5e3 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  pc = 0x100c5e5; continue;
 }
 0x100c5e5 => {
  { let v: u32 = 0x110c20cu32; cpu.esi = v; }
  { let v: u16 = 0x40u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x100c5f7); f_01033375(cpu); if cpu.returned_to != 0x100c5f7 { return; }
  pc = 0x100c5f7; continue;
 }
 0x100c5f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100c5f8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100c5f8); } r_0100c5f8(cpu, 0x100c5f8); }
pub fn f_0100c6e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100c6e8); } r_0100c5f8(cpu, 0x100c6e8); }
fn r_0100c5f8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x100c5f8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x100c60d; continue; }
  pc = 0x100c601; continue;
 }
 0x100c601 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x110da60u32; cpu.edx = v; }
  pc = 0x100c619; continue;
 }
 0x100c60d => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d963u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  pc = 0x100c619; continue;
 }
 0x100c619 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11cc9b1u32, v); }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d948u32, v); }
  cpu.push(0x100c634); f_01001d70(cpu); if cpu.returned_to != 0x100c634 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.sf { pc = 0x100d190; continue; }
  pc = 0x100c63c; continue;
 }
 0x100c63c => {
  { let v: u32 = cpu.eax; cpu.w32(0x110d948u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d91u32, v); }
  { let v: u32 = 0x110da68u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c65d); f_0105e75e(cpu); if cpu.returned_to != 0x100c65d { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c669; continue;
 }
 0x100c669 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c67c); f_0105e75e(cpu); if cpu.returned_to != 0x100c67c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c688; continue;
 }
 0x100c688 => {
  { let v: u32 = 0x10e3d63u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c69e); f_0105e75e(cpu); if cpu.returned_to != 0x100c69e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c6aa; continue;
 }
 0x100c6aa => {
  { let v: u32 = 0x1003dcfu32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c6c0); f_0105e75e(cpu); if cpu.returned_to != 0x100c6c0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c6cc; continue;
 }
 0x100c6cc => {
  { let v: u32 = 0x10e3d85u32; cpu.ecx = v; }
  { let v: u32 = 0x10e3cdbu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c6e8); f_0105e75e(cpu); if cpu.returned_to != 0x100c6e8 { return; }
  pc = 0x100c6e8; continue;
 }
 0x100c6e8 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c6f4; continue;
 }
 0x100c6f4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e3f7cu32).wrapping_sub(cpu.eax); cpu.w32(0x10e3f7cu32, v); }
  { let v: u32 = cpu.r32(0x10e3f6cu32).wrapping_sub(cpu.eax); cpu.w32(0x10e3f6cu32, v); }
  { let v: u32 = cpu.r32(0x10e3f80u32).wrapping_sub(cpu.eax); cpu.w32(0x10e3f80u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x10e3f84u32; cpu.ecx = v; }
  { let v: u32 = 0x10e3f4cu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c729); f_0105e75e(cpu); if cpu.returned_to != 0x100c729 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c735; continue;
 }
 0x100c735 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e3f7cu32).wrapping_add(cpu.eax); cpu.w32(0x10e3f7cu32, v); }
  { let v: u32 = cpu.r32(0x10e3f6cu32).wrapping_add(cpu.eax); cpu.w32(0x10e3f6cu32, v); }
  { let v: u32 = cpu.r32(0x10e3f80u32).wrapping_add(cpu.eax); cpu.w32(0x10e3f80u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x10ee958u32; cpu.ecx = v; }
  { let v: u32 = 0x10ee917u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c76a); f_0105e75e(cpu); if cpu.returned_to != 0x100c76a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c776; continue;
 }
 0x100c776 => {
  { let v: u32 = 0x10ef352u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c78e); f_0105e75e(cpu); if cpu.returned_to != 0x100c78e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c79a; continue;
 }
 0x100c79a => {
  { let v: u32 = 0x10e68efu32; cpu.edx = v; }
  { let v: u32 = 0x8021u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c7b0); f_0105e75e(cpu); if cpu.returned_to != 0x100c7b0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c7bc; continue;
 }
 0x100c7bc => {
  { let v: u32 = 0x10ef70cu32; cpu.edx = v; }
  { let v: u32 = 0x6000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c7d2); f_0105e75e(cpu); if cpu.returned_to != 0x100c7d2 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c7de; continue;
 }
 0x100c7de => {
  { let v: u32 = 0x113964cu32; cpu.edx = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c7f4); f_0105e75e(cpu); if cpu.returned_to != 0x100c7f4 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c800; continue;
 }
 0x100c800 => {
  { let v: u32 = 0x115e424u32; cpu.edx = v; }
  { let v: u32 = 0xfa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c816); f_0105e75e(cpu); if cpu.returned_to != 0x100c816 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c822; continue;
 }
 0x100c822 => {
  { let v: u32 = 0x115f3c4u32; cpu.edx = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c838); f_0105e75e(cpu); if cpu.returned_to != 0x100c838 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c844; continue;
 }
 0x100c844 => {
  { let v: u32 = 0x115f5b8u32; cpu.edx = v; }
  { let v: u32 = 0x8214u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c85a); f_0105e75e(cpu); if cpu.returned_to != 0x100c85a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c866; continue;
 }
 0x100c866 => {
  { let v: u32 = 0x1167ed4u32; cpu.edx = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c87c); f_0105e75e(cpu); if cpu.returned_to != 0x100c87c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c888; continue;
 }
 0x100c888 => {
  { let v: u32 = 0x1167f74u32; cpu.edx = v; }
  { let v: u32 = 0x2940u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c89e); f_0105e75e(cpu); if cpu.returned_to != 0x100c89e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c8aa; continue;
 }
 0x100c8aa => {
  { let v: u32 = 0x116a8b4u32; cpu.edx = v; }
  { let v: u32 = 0x12cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c8c0); f_0105e75e(cpu); if cpu.returned_to != 0x100c8c0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c8cc; continue;
 }
 0x100c8cc => {
  { let v: u32 = 0x116a9e0u32; cpu.edx = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c8e2); f_0105e75e(cpu); if cpu.returned_to != 0x100c8e2 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c8ee; continue;
 }
 0x100c8ee => {
  { let v: u32 = 0x116b520u32; cpu.edx = v; }
  { let v: u32 = 0xbb8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c904); f_0105e75e(cpu); if cpu.returned_to != 0x100c904 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c910; continue;
 }
 0x100c910 => {
  { let v: u32 = 0x116fbd4u32; cpu.edx = v; }
  { let v: u32 = 0x180u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c926); f_0105e75e(cpu); if cpu.returned_to != 0x100c926 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c932; continue;
 }
 0x100c932 => {
  { let v: u32 = 0x117e804u32; cpu.edx = v; }
  { let v: u32 = 0xe4c0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c948); f_0105e75e(cpu); if cpu.returned_to != 0x100c948 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c954; continue;
 }
 0x100c954 => {
  { let v: u32 = 0x1174854u32; cpu.edx = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c96a); f_0105e75e(cpu); if cpu.returned_to != 0x100c96a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c976; continue;
 }
 0x100c976 => {
  { let v: u32 = 0x116fd54u32; cpu.edx = v; }
  { let v: u32 = 0x15e0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c98c); f_0105e75e(cpu); if cpu.returned_to != 0x100c98c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c998; continue;
 }
 0x100c998 => {
  { let v: u32 = 0x1179b24u32; cpu.edx = v; }
  { let v: u32 = 0x4ce0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c9ae); f_0105e75e(cpu); if cpu.returned_to != 0x100c9ae { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c9ba; continue;
 }
 0x100c9ba => {
  { let v: u32 = 0x118ccc4u32; cpu.edx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c9d0); f_0105e75e(cpu); if cpu.returned_to != 0x100c9d0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c9dc; continue;
 }
 0x100c9dc => {
  { let v: u32 = 0x118cd14u32; cpu.edx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100c9f2); f_0105e75e(cpu); if cpu.returned_to != 0x100c9f2 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100c9fe; continue;
 }
 0x100c9fe => {
  { let v: u32 = 0x10f7dd2u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ca14); f_0105e75e(cpu); if cpu.returned_to != 0x100ca14 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ca20; continue;
 }
 0x100ca20 => {
  { let v: u32 = 0x10f7625u32; cpu.edx = v; }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ca36); f_0105e75e(cpu); if cpu.returned_to != 0x100ca36 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ca42; continue;
 }
 0x100ca42 => {
  { let v: u32 = 0x10f782au32; cpu.edx = v; }
  { let v: u32 = 0x48u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ca58); f_0105e75e(cpu); if cpu.returned_to != 0x100ca58 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ca64; continue;
 }
 0x100ca64 => {
  { let v: u32 = 0x10c5944u32; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ca7a); f_0105e75e(cpu); if cpu.returned_to != 0x100ca7a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ca86; continue;
 }
 0x100ca86 => {
  { let v: u32 = 0x10c797du32; cpu.edx = v; }
  { let v: u32 = 0xbeu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ca9c); f_0105e75e(cpu); if cpu.returned_to != 0x100ca9c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100caa8; continue;
 }
 0x100caa8 => {
  { let v: u32 = 0x10c7a3bu32; cpu.edx = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cabe); f_0105e75e(cpu); if cpu.returned_to != 0x100cabe { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100caca; continue;
 }
 0x100caca => {
  { let v: u32 = 0x10c7a5bu32; cpu.edx = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cae0); f_0105e75e(cpu); if cpu.returned_to != 0x100cae0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100caec; continue;
 }
 0x100caec => {
  { let v: u32 = 0x10c09ccu32; cpu.edx = v; }
  { let v: u32 = 0x2a8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cb02); f_0105e75e(cpu); if cpu.returned_to != 0x100cb02 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cb0e; continue;
 }
 0x100cb0e => {
  { let v: u32 = 0x11233fau32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cb24); f_0105e75e(cpu); if cpu.returned_to != 0x100cb24 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cb30; continue;
 }
 0x100cb30 => {
  { let v: u32 = 0x10c6a58u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cb46); f_0105e75e(cpu); if cpu.returned_to != 0x100cb46 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cb52; continue;
 }
 0x100cb52 => {
  { let v: u32 = 0x10c7969u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cb68); f_0105e75e(cpu); if cpu.returned_to != 0x100cb68 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cb74; continue;
 }
 0x100cb74 => {
  { let v: u32 = 0x10c6a4cu32; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cb8a); f_0105e75e(cpu); if cpu.returned_to != 0x100cb8a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cb96; continue;
 }
 0x100cb96 => {
  { let v: u32 = 0x10c6a5au32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cbac); f_0105e75e(cpu); if cpu.returned_to != 0x100cbac { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cbb8; continue;
 }
 0x100cbb8 => {
  { let v: u32 = 0x10c6a5cu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cbce); f_0105e75e(cpu); if cpu.returned_to != 0x100cbce { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cbda; continue;
 }
 0x100cbda => {
  { let v: u32 = 0x1105e60u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cbf0); f_0105e75e(cpu); if cpu.returned_to != 0x100cbf0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cbfc; continue;
 }
 0x100cbfc => {
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x2630u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cc15); f_0105e75e(cpu); if cpu.returned_to != 0x100cc15 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cc21; continue;
 }
 0x100cc21 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11de584u32).wrapping_sub(cpu.eax); cpu.w32(0x11de584u32, v); }
  { let v: u32 = cpu.r32(0x11de588u32).wrapping_sub(cpu.eax); cpu.w32(0x11de588u32, v); }
  { let v: u32 = cpu.r32(0x11de58cu32).wrapping_sub(cpu.eax); cpu.w32(0x11de58cu32, v); }
  { let v: u32 = cpu.r32(0x11de590u32).wrapping_sub(cpu.eax); cpu.w32(0x11de590u32, v); }
  { let v: u32 = cpu.r32(0x110eac4u32).wrapping_sub(cpu.eax); cpu.w32(0x110eac4u32, v); }
  { let v: u32 = cpu.r32(0x110eac8u32).wrapping_sub(cpu.eax); cpu.w32(0x110eac8u32, v); }
  { let v: u32 = cpu.r32(0x110eaccu32).wrapping_sub(cpu.eax); cpu.w32(0x110eaccu32, v); }
  { let v: u32 = cpu.r32(0x110ead0u32).wrapping_sub(cpu.eax); cpu.w32(0x110ead0u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x10ef37du32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cc6e); f_0105e75e(cpu); if cpu.returned_to != 0x100cc6e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cc7a; continue;
 }
 0x100cc7a => {
  { let v: u32 = 0x11de584u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cc90); f_0105e75e(cpu); if cpu.returned_to != 0x100cc90 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cc9c; continue;
 }
 0x100cc9c => {
  { let v: u32 = 0x110eac4u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ccb2); f_0105e75e(cpu); if cpu.returned_to != 0x100ccb2 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ccbe; continue;
 }
 0x100ccbe => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11de584u32).wrapping_add(cpu.eax); cpu.w32(0x11de584u32, v); }
  { let v: u32 = cpu.r32(0x11de588u32).wrapping_add(cpu.eax); cpu.w32(0x11de588u32, v); }
  { let v: u32 = cpu.r32(0x11de58cu32).wrapping_add(cpu.eax); cpu.w32(0x11de58cu32, v); }
  { let v: u32 = cpu.r32(0x11de590u32).wrapping_add(cpu.eax); cpu.w32(0x11de590u32, v); }
  { let v: u32 = cpu.r32(0x110eac4u32).wrapping_add(cpu.eax); cpu.w32(0x110eac4u32, v); }
  { let v: u32 = cpu.r32(0x110eac8u32).wrapping_add(cpu.eax); cpu.w32(0x110eac8u32, v); }
  { let v: u32 = cpu.r32(0x110eaccu32).wrapping_add(cpu.eax); cpu.w32(0x110eaccu32, v); }
  { let v: u32 = cpu.r32(0x110ead0u32).wrapping_add(cpu.eax); cpu.w32(0x110ead0u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x11aa104u32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cd0b); f_0105e75e(cpu); if cpu.returned_to != 0x100cd0b { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cd17; continue;
 }
 0x100cd17 => {
  { let v: u32 = 0x11ac904u32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cd2d); f_0105e75e(cpu); if cpu.returned_to != 0x100cd2d { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cd39; continue;
 }
 0x100cd39 => {
  { let v: u32 = 0x11af104u32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cd4f); f_0105e75e(cpu); if cpu.returned_to != 0x100cd4f { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cd5b; continue;
 }
 0x100cd5b => {
  { let v: u32 = 0x11b1904u32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cd71); f_0105e75e(cpu); if cpu.returned_to != 0x100cd71 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cd7d; continue;
 }
 0x100cd7d => {
  { let v: u32 = 0x1133838u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cd93); f_0105e75e(cpu); if cpu.returned_to != 0x100cd93 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cd9f; continue;
 }
 0x100cd9f => {
  { let v: u32 = 0x113383cu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cdb5); f_0105e75e(cpu); if cpu.returned_to != 0x100cdb5 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cdc1; continue;
 }
 0x100cdc1 => {
  { let v: u32 = 0x1133840u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cdd7); f_0105e75e(cpu); if cpu.returned_to != 0x100cdd7 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cde3; continue;
 }
 0x100cde3 => {
  { let v: u32 = 0x1133844u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cdf9); f_0105e75e(cpu); if cpu.returned_to != 0x100cdf9 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ce05; continue;
 }
 0x100ce05 => {
  { let v: u32 = 0x1133848u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ce1b); f_0105e75e(cpu); if cpu.returned_to != 0x100ce1b { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ce27; continue;
 }
 0x100ce27 => {
  { let v: u32 = 0x113384cu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ce3d); f_0105e75e(cpu); if cpu.returned_to != 0x100ce3d { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ce49; continue;
 }
 0x100ce49 => {
  { let v: u32 = 0x1133858u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ce5f); f_0105e75e(cpu); if cpu.returned_to != 0x100ce5f { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ce6b; continue;
 }
 0x100ce6b => {
  { let v: u32 = 0x113385cu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ce81); f_0105e75e(cpu); if cpu.returned_to != 0x100ce81 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ce8d; continue;
 }
 0x100ce8d => {
  cpu.push(0x100ce92); f_0100e444(cpu); if cpu.returned_to != 0x100ce92 { return; }
  { let v: u32 = 0x116de88u32; cpu.edx = v; }
  { let v: u32 = 0x8a9u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cea8); f_0105e75e(cpu); if cpu.returned_to != 0x100cea8 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ceb4; continue;
 }
 0x100ceb4 => {
  { let v: u32 = 0x116e734u32; cpu.edx = v; }
  { let v: u32 = 0x14a0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ceca); f_0105e75e(cpu); if cpu.returned_to != 0x100ceca { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100ced6; continue;
 }
 0x100ced6 => {
  cpu.push(0x100cedb); f_0100e4a6(cpu); if cpu.returned_to != 0x100cedb { return; }
  cpu.push(0x100cee0); f_0100e392(cpu); if cpu.returned_to != 0x100cee0 { return; }
  { let v: u32 = 0x10c6a68u32; cpu.edx = v; }
  { let v: u32 = 0xf00u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cef6); f_0105e75e(cpu); if cpu.returned_to != 0x100cef6 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cf02; continue;
 }
 0x100cf02 => {
  { let v: u32 = 0x10c0c74u32; cpu.edx = v; }
  { let v: u32 = 0xf00u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cf18); f_0105e75e(cpu); if cpu.returned_to != 0x100cf18 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cf24; continue;
 }
 0x100cf24 => {
  cpu.push(0x100cf29); f_0100e3eb(cpu); if cpu.returned_to != 0x100cf29 { return; }
  cpu.push(0x100cf2e); f_0100e508(cpu); if cpu.returned_to != 0x100cf2e { return; }
  { let v: u32 = 0x116c0d8u32; cpu.edx = v; }
  { let v: u32 = 0xaf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cf44); f_0105e75e(cpu); if cpu.returned_to != 0x100cf44 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cf50; continue;
 }
 0x100cf50 => {
  cpu.push(0x100cf55); f_0100e589(cpu); if cpu.returned_to != 0x100cf55 { return; }
  cpu.push(0x100cf5a); f_0100e60a(cpu); if cpu.returned_to != 0x100cf5a { return; }
  { let v: u32 = 0x116aa58u32; cpu.edx = v; }
  { let v: u32 = 0xac8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cf70); f_0105e75e(cpu); if cpu.returned_to != 0x100cf70 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cf7c; continue;
 }
 0x100cf7c => {
  { let v: u32 = cpu.r32(0x10e40a0u32); cpu.edx = v; }
  { let v: u32 = 0x9c40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cf93); f_0105e75e(cpu); if cpu.returned_to != 0x100cf93 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cf9f; continue;
 }
 0x100cf9f => {
  cpu.push(0x100cfa4); f_0100e635(cpu); if cpu.returned_to != 0x100cfa4 { return; }
  cpu.push(0x100cfa9); f_0100e992(cpu); if cpu.returned_to != 0x100cfa9 { return; }
  { let v: u32 = 0x1139714u32; cpu.edx = v; }
  { let v: u32 = 0x157c0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cfbf); f_0105e75e(cpu); if cpu.returned_to != 0x100cfbf { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cfcb; continue;
 }
 0x100cfcb => {
  cpu.push(0x100cfd0); f_0100e9e2(cpu); if cpu.returned_to != 0x100cfd0 { return; }
  cpu.push(0x100cfd5); f_0100e93e(cpu); if cpu.returned_to != 0x100cfd5 { return; }
  { let v: u32 = 0x114eed4u32; cpu.edx = v; }
  { let v: u32 = 0x3e80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100cfeb); f_0105e75e(cpu); if cpu.returned_to != 0x100cfeb { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100cff7; continue;
 }
 0x100cff7 => {
  cpu.push(0x100cffc); f_0100e968(cpu); if cpu.returned_to != 0x100cffc { return; }
  cpu.push(0x100d001); f_0100e6ba(cpu); if cpu.returned_to != 0x100d001 { return; }
  { let v: u32 = 0x1152e54u32; cpu.edx = v; }
  { let v: u32 = 0xac80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100d017); f_0105e75e(cpu); if cpu.returned_to != 0x100d017 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100d023; continue;
 }
 0x100d023 => {
  cpu.push(0x100d028); f_0100e7fc(cpu); if cpu.returned_to != 0x100d028 { return; }
  cpu.push(0x100d02d); f_0100e660(cpu); if cpu.returned_to != 0x100d02d { return; }
  { let v: u32 = 0x115db8cu32; cpu.edx = v; }
  { let v: u32 = 0x898u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100d043); f_0105e75e(cpu); if cpu.returned_to != 0x100d043 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100d04f; continue;
 }
 0x100d04f => {
  cpu.push(0x100d054); f_0100e68d(cpu); if cpu.returned_to != 0x100d054 { return; }
  cpu.push(0x100d059); f_0100ea32(cpu); if cpu.returned_to != 0x100d059 { return; }
  { let v: u32 = 0x10c7adbu32; cpu.edx = v; }
  { let v: u32 = 0x12c04u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100d06f); f_0105e75e(cpu); if cpu.returned_to != 0x100d06f { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100d07b; continue;
 }
 0x100d07b => {
  cpu.push(0x100d080); f_0100ea8b(cpu); if cpu.returned_to != 0x100d080 { return; }
  cpu.push(0x100d085); f_0100e292(cpu); if cpu.returned_to != 0x100d085 { return; }
  { let v: u32 = 0x1172f54u32; cpu.edx = v; }
  { let v: u32 = 0x1900u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100d09b); f_0105e75e(cpu); if cpu.returned_to != 0x100d09b { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100d0a7; continue;
 }
 0x100d0a7 => {
  cpu.push(0x100d0ac); f_0100e2aa(cpu); if cpu.returned_to != 0x100d0ac { return; }
  cpu.push(0x100d0b1); f_0100e32a(cpu); if cpu.returned_to != 0x100d0b1 { return; }
  { let v: u32 = 0x1174a48u32; cpu.edx = v; }
  { let v: u32 = 0x780u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100d0c7); f_0105e75e(cpu); if cpu.returned_to != 0x100d0c7 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100d0d3; continue;
 }
 0x100d0d3 => {
  cpu.push(0x100d0d8); f_0100e35e(cpu); if cpu.returned_to != 0x100d0d8 { return; }
  cpu.push(0x100d0dd); f_0100e2c2(cpu); if cpu.returned_to != 0x100d0dd { return; }
  { let v: u32 = 0x1171334u32; cpu.edx = v; }
  { let v: u32 = 0x1c20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100d0f3); f_0105e75e(cpu); if cpu.returned_to != 0x100d0f3 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100d0ff; continue;
 }
 0x100d0ff => {
  cpu.push(0x100d104); f_0100e2f6(cpu); if cpu.returned_to != 0x100d104 { return; }
  cpu.push(0x100d109); f_0100e21f(cpu); if cpu.returned_to != 0x100d109 { return; }
  { let v: u32 = 0x11757a4u32; cpu.edx = v; }
  { let v: u32 = 0x4380u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100d11f); f_0105e75e(cpu); if cpu.returned_to != 0x100d11f { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100d12b; continue;
 }
 0x100d12b => {
  cpu.push(0x100d130); f_0100e257(cpu); if cpu.returned_to != 0x100d130 { return; }
  cpu.push(0x100d135); f_0100e18d(cpu); if cpu.returned_to != 0x100d135 { return; }
  { let v: u32 = 0x11a188cu32; cpu.edx = v; }
  { let v: u32 = 0x1f40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100d14b); f_0105e75e(cpu); if cpu.returned_to != 0x100d14b { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100d153; continue;
 }
 0x100d153 => {
  cpu.push(0x100d158); f_0100e1d6(cpu); if cpu.returned_to != 0x100d158 { return; }
  cpu.push(0x100d15d); f_0100e133(cpu); if cpu.returned_to != 0x100d15d { return; }
  { let v: u32 = 0x10ef3d4u32; cpu.edx = v; }
  { let v: u32 = 0x270u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100d173); f_0105e75e(cpu); if cpu.returned_to != 0x100d173 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100d17b; continue;
 }
 0x100d17b => {
  cpu.push(0x100d180); f_0100e160(cpu); if cpu.returned_to != 0x100d180 { return; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100d18a); f_01001e06(cpu); if cpu.returned_to != 0x100d18a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x100d190; continue; }
  pc = 0x100d18e; continue;
 }
 0x100d18e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100d190 => {
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100d19f; continue; }
  pc = 0x100d19a; continue;
 }
 0x100d19a => {
  cpu.push(0x100d19f); f_01001e06(cpu); if cpu.returned_to != 0x100d19f { return; }
  pc = 0x100d19f; continue;
 }
 0x100d19f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100d1a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100d1a1); }
 let mut pc: u32 = 0x100d1a1;
 loop { match pc {
 0x100d1a1 => {
  cpu.push(0x100d1a6); f_01083df3(cpu); if cpu.returned_to != 0x100d1a6 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x100d1ba); f_010385ad(cpu); if cpu.returned_to != 0x100d1ba { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x100d1c2); cpu.call(t); if cpu.returned_to != 0x100d1c2 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x110d98bu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x100d1e9; continue; }
  pc = 0x100d1db; continue;
 }
 0x100d1db => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d9e4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  pc = 0x100d1f5; continue;
 }
 0x100d1e9 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d963u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  pc = 0x100d1f5; continue;
 }
 0x100d1f5 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d94cu32, v); }
  cpu.push(0x100d20c); f_010018d7(cpu); if cpu.returned_to != 0x100d20c { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8fu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.sf { pc = 0x100ddbd; continue; }
  pc = 0x100d222; continue;
 }
 0x100d222 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x110d94cu32, v); }
  { let v: u32 = 0x110da68u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d245); f_0105e746(cpu); if cpu.returned_to != 0x100d245 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d251; continue;
 }
 0x100d251 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d264); f_0105e746(cpu); if cpu.returned_to != 0x100d264 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d270; continue;
 }
 0x100d270 => {
  { let v: u32 = 0x10e3d63u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d286); f_0105e746(cpu); if cpu.returned_to != 0x100d286 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d292; continue;
 }
 0x100d292 => {
  { let v: u32 = 0x110d98cu32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d2a8); f_0105e746(cpu); if cpu.returned_to != 0x100d2a8 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d2b4; continue;
 }
 0x100d2b4 => {
  { let v: u32 = 0x10e3d85u32; cpu.ecx = v; }
  { let v: u32 = 0x10e3cdbu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d2d0); f_0105e746(cpu); if cpu.returned_to != 0x100d2d0 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d2dc; continue;
 }
 0x100d2dc => {
  { let v: u32 = 0x10e3f84u32; cpu.ecx = v; }
  { let v: u32 = 0x10e3f4cu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d2f8); f_0105e746(cpu); if cpu.returned_to != 0x100d2f8 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d304; continue;
 }
 0x100d304 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e3f7cu32).wrapping_add(cpu.eax); cpu.w32(0x10e3f7cu32, v); }
  { let v: u32 = cpu.r32(0x10e3f6cu32).wrapping_add(cpu.eax); cpu.w32(0x10e3f6cu32, v); }
  { let v: u32 = cpu.r32(0x10e3f80u32).wrapping_add(cpu.eax); cpu.w32(0x10e3f80u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x10ee958u32; cpu.ecx = v; }
  { let v: u32 = 0x10ee917u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d339); f_0105e746(cpu); if cpu.returned_to != 0x100d339 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d345; continue;
 }
 0x100d345 => {
  { let v: u32 = 0x11b543au32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d35d); f_0105e746(cpu); if cpu.returned_to != 0x100d35d { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d369; continue;
 }
 0x100d369 => {
  { let v: u32 = 0x10e68efu32; cpu.edx = v; }
  { let v: u32 = 0x8021u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d37f); f_0105e746(cpu); if cpu.returned_to != 0x100d37f { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d38b; continue;
 }
 0x100d38b => {
  { let v: u32 = 0x10ef70cu32; cpu.edx = v; }
  { let v: u32 = 0x6000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d3a1); f_0105e746(cpu); if cpu.returned_to != 0x100d3a1 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d3ad; continue;
 }
 0x100d3ad => {
  { let v: u32 = 0x113964cu32; cpu.edx = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d3c3); f_0105e746(cpu); if cpu.returned_to != 0x100d3c3 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d3cf; continue;
 }
 0x100d3cf => {
  { let v: u32 = 0x115e424u32; cpu.edx = v; }
  { let v: u32 = 0xfa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d3e5); f_0105e746(cpu); if cpu.returned_to != 0x100d3e5 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d3f1; continue;
 }
 0x100d3f1 => {
  { let v: u32 = 0x115f3c4u32; cpu.edx = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d407); f_0105e746(cpu); if cpu.returned_to != 0x100d407 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d413; continue;
 }
 0x100d413 => {
  { let v: u32 = 0x115f5b8u32; cpu.edx = v; }
  { let v: u32 = 0x8214u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d429); f_0105e746(cpu); if cpu.returned_to != 0x100d429 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d435; continue;
 }
 0x100d435 => {
  { let v: u32 = 0x1167ed4u32; cpu.edx = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d44b); f_0105e746(cpu); if cpu.returned_to != 0x100d44b { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d457; continue;
 }
 0x100d457 => {
  { let v: u32 = 0x1167f74u32; cpu.edx = v; }
  { let v: u32 = 0x2940u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d46d); f_0105e746(cpu); if cpu.returned_to != 0x100d46d { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d479; continue;
 }
 0x100d479 => {
  { let v: u32 = 0x116a8b4u32; cpu.edx = v; }
  { let v: u32 = 0x12cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d48f); f_0105e746(cpu); if cpu.returned_to != 0x100d48f { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d49b; continue;
 }
 0x100d49b => {
  { let v: u32 = 0x116a9e0u32; cpu.edx = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d4b1); f_0105e746(cpu); if cpu.returned_to != 0x100d4b1 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d4bd; continue;
 }
 0x100d4bd => {
  { let v: u32 = 0x116b520u32; cpu.edx = v; }
  { let v: u32 = 0xbb8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d4d3); f_0105e746(cpu); if cpu.returned_to != 0x100d4d3 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d4df; continue;
 }
 0x100d4df => {
  { let v: u32 = 0x116fbd4u32; cpu.edx = v; }
  { let v: u32 = 0x180u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d4f5); f_0105e746(cpu); if cpu.returned_to != 0x100d4f5 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d501; continue;
 }
 0x100d501 => {
  { let v: u32 = 0x117e804u32; cpu.edx = v; }
  { let v: u32 = 0xe4c0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d517); f_0105e746(cpu); if cpu.returned_to != 0x100d517 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d523; continue;
 }
 0x100d523 => {
  { let v: u32 = 0x1174854u32; cpu.edx = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d539); f_0105e746(cpu); if cpu.returned_to != 0x100d539 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d545; continue;
 }
 0x100d545 => {
  { let v: u32 = 0x116fd54u32; cpu.edx = v; }
  { let v: u32 = 0x15e0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d55b); f_0105e746(cpu); if cpu.returned_to != 0x100d55b { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d567; continue;
 }
 0x100d567 => {
  { let v: u32 = 0x1179b24u32; cpu.edx = v; }
  { let v: u32 = 0x4ce0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d57d); f_0105e746(cpu); if cpu.returned_to != 0x100d57d { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d589; continue;
 }
 0x100d589 => {
  { let v: u32 = 0x118ccc4u32; cpu.edx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d59f); f_0105e746(cpu); if cpu.returned_to != 0x100d59f { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d5ab; continue;
 }
 0x100d5ab => {
  { let v: u32 = 0x118cd14u32; cpu.edx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d5c1); f_0105e746(cpu); if cpu.returned_to != 0x100d5c1 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d5cd; continue;
 }
 0x100d5cd => {
  { let v: u32 = 0x10f7dd2u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d5e3); f_0105e746(cpu); if cpu.returned_to != 0x100d5e3 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d5ef; continue;
 }
 0x100d5ef => {
  { let v: u32 = 0x10f7625u32; cpu.edx = v; }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d605); f_0105e746(cpu); if cpu.returned_to != 0x100d605 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d611; continue;
 }
 0x100d611 => {
  { let v: u32 = 0x10f782au32; cpu.edx = v; }
  { let v: u32 = 0x48u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d627); f_0105e746(cpu); if cpu.returned_to != 0x100d627 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d633; continue;
 }
 0x100d633 => {
  { let v: u32 = 0x10c5944u32; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d649); f_0105e746(cpu); if cpu.returned_to != 0x100d649 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d655; continue;
 }
 0x100d655 => {
  { let v: u32 = 0x10c797du32; cpu.edx = v; }
  { let v: u32 = 0xbeu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d66b); f_0105e746(cpu); if cpu.returned_to != 0x100d66b { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d677; continue;
 }
 0x100d677 => {
  { let v: u32 = 0x10c7a3bu32; cpu.edx = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d68d); f_0105e746(cpu); if cpu.returned_to != 0x100d68d { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d699; continue;
 }
 0x100d699 => {
  { let v: u32 = 0x10c7a5bu32; cpu.edx = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d6af); f_0105e746(cpu); if cpu.returned_to != 0x100d6af { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d6bb; continue;
 }
 0x100d6bb => {
  { let v: u32 = 0x10c09ccu32; cpu.edx = v; }
  { let v: u32 = 0x2a8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d6d1); f_0105e746(cpu); if cpu.returned_to != 0x100d6d1 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d6dd; continue;
 }
 0x100d6dd => {
  { let v: u32 = 0x11233fau32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d6f3); f_0105e746(cpu); if cpu.returned_to != 0x100d6f3 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d6ff; continue;
 }
 0x100d6ff => {
  { let v: u32 = 0x10c6a58u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d715); f_0105e746(cpu); if cpu.returned_to != 0x100d715 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d721; continue;
 }
 0x100d721 => {
  { let v: u32 = 0x10c7969u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d737); f_0105e746(cpu); if cpu.returned_to != 0x100d737 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d743; continue;
 }
 0x100d743 => {
  { let v: u32 = 0x10c6a4cu32; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d759); f_0105e746(cpu); if cpu.returned_to != 0x100d759 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d765; continue;
 }
 0x100d765 => {
  { let v: u32 = 0x10c6a5au32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d77b); f_0105e746(cpu); if cpu.returned_to != 0x100d77b { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d787; continue;
 }
 0x100d787 => {
  { let v: u32 = 0x10c6a5cu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d79d); f_0105e746(cpu); if cpu.returned_to != 0x100d79d { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d7a9; continue;
 }
 0x100d7a9 => {
  { let v: u32 = 0x1105e60u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d7bf); f_0105e746(cpu); if cpu.returned_to != 0x100d7bf { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d7cb; continue;
 }
 0x100d7cb => {
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x2630u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d7e4); f_0105e746(cpu); if cpu.returned_to != 0x100d7e4 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d7f0; continue;
 }
 0x100d7f0 => {
  { let v: u32 = 0x10ef37du32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d806); f_0105e746(cpu); if cpu.returned_to != 0x100d806 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d812; continue;
 }
 0x100d812 => {
  { let v: u32 = 0x11de584u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d828); f_0105e746(cpu); if cpu.returned_to != 0x100d828 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d834; continue;
 }
 0x100d834 => {
  { let v: u32 = 0x110eac4u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d84a); f_0105e746(cpu); if cpu.returned_to != 0x100d84a { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d856; continue;
 }
 0x100d856 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11de584u32).wrapping_add(cpu.eax); cpu.w32(0x11de584u32, v); }
  { let v: u32 = cpu.r32(0x11de588u32).wrapping_add(cpu.eax); cpu.w32(0x11de588u32, v); }
  { let v: u32 = cpu.r32(0x11de58cu32).wrapping_add(cpu.eax); cpu.w32(0x11de58cu32, v); }
  { let v: u32 = cpu.r32(0x11de590u32).wrapping_add(cpu.eax); cpu.w32(0x11de590u32, v); }
  { let v: u32 = cpu.r32(0x110eac4u32).wrapping_add(cpu.eax); cpu.w32(0x110eac4u32, v); }
  { let v: u32 = cpu.r32(0x110eac8u32).wrapping_add(cpu.eax); cpu.w32(0x110eac8u32, v); }
  { let v: u32 = cpu.r32(0x110eaccu32).wrapping_add(cpu.eax); cpu.w32(0x110eaccu32, v); }
  { let v: u32 = cpu.r32(0x110ead0u32).wrapping_add(cpu.eax); cpu.w32(0x110ead0u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x11aa104u32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d8a3); f_0105e746(cpu); if cpu.returned_to != 0x100d8a3 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d8af; continue;
 }
 0x100d8af => {
  { let v: u32 = 0x11ac904u32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d8c5); f_0105e746(cpu); if cpu.returned_to != 0x100d8c5 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d8d1; continue;
 }
 0x100d8d1 => {
  { let v: u32 = 0x11af104u32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d8e7); f_0105e746(cpu); if cpu.returned_to != 0x100d8e7 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d8f3; continue;
 }
 0x100d8f3 => {
  { let v: u32 = 0x11b1904u32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d909); f_0105e746(cpu); if cpu.returned_to != 0x100d909 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d915; continue;
 }
 0x100d915 => {
  { let v: u32 = 0x1133838u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d92b); f_0105e746(cpu); if cpu.returned_to != 0x100d92b { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d937; continue;
 }
 0x100d937 => {
  { let v: u32 = 0x113383cu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d94d); f_0105e746(cpu); if cpu.returned_to != 0x100d94d { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d959; continue;
 }
 0x100d959 => {
  { let v: u32 = 0x1133840u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d96f); f_0105e746(cpu); if cpu.returned_to != 0x100d96f { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d97b; continue;
 }
 0x100d97b => {
  { let v: u32 = 0x1133844u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d991); f_0105e746(cpu); if cpu.returned_to != 0x100d991 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d99d; continue;
 }
 0x100d99d => {
  { let v: u32 = 0x1133848u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d9b3); f_0105e746(cpu); if cpu.returned_to != 0x100d9b3 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d9bf; continue;
 }
 0x100d9bf => {
  { let v: u32 = 0x113384cu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d9d5); f_0105e746(cpu); if cpu.returned_to != 0x100d9d5 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100d9e1; continue;
 }
 0x100d9e1 => {
  { let v: u32 = 0x1133858u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100d9f7); f_0105e746(cpu); if cpu.returned_to != 0x100d9f7 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100da03; continue;
 }
 0x100da03 => {
  { let v: u32 = 0x113385cu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100da19); f_0105e746(cpu); if cpu.returned_to != 0x100da19 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100da25; continue;
 }
 0x100da25 => {
  { let v: u32 = 0x116de88u32; cpu.edx = v; }
  { let v: u32 = 0x8a9u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100da3b); f_0105e746(cpu); if cpu.returned_to != 0x100da3b { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100da47; continue;
 }
 0x100da47 => {
  { let v: u32 = 0x116e734u32; cpu.edx = v; }
  { let v: u32 = 0x14a0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100da5d); f_0105e746(cpu); if cpu.returned_to != 0x100da5d { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100da69; continue;
 }
 0x100da69 => {
  cpu.push(0x100da6e); f_0100e4a6(cpu); if cpu.returned_to != 0x100da6e { return; }
  { let v: u32 = 0x10c6a68u32; cpu.edx = v; }
  { let v: u32 = 0xf00u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100da84); f_0105e746(cpu); if cpu.returned_to != 0x100da84 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100da90; continue;
 }
 0x100da90 => {
  { let v: u32 = 0x10c0c74u32; cpu.edx = v; }
  { let v: u32 = 0xf00u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100daa6); f_0105e746(cpu); if cpu.returned_to != 0x100daa6 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dab2; continue;
 }
 0x100dab2 => {
  cpu.push(0x100dab7); f_0100e3eb(cpu); if cpu.returned_to != 0x100dab7 { return; }
  { let v: u32 = 0x116c0d8u32; cpu.edx = v; }
  { let v: u32 = 0xaf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100dacd); f_0105e746(cpu); if cpu.returned_to != 0x100dacd { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dad9; continue;
 }
 0x100dad9 => {
  cpu.push(0x100dade); f_0100e589(cpu); if cpu.returned_to != 0x100dade { return; }
  { let v: u32 = 0x116aa58u32; cpu.edx = v; }
  { let v: u32 = 0xac8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100daf4); f_0105e746(cpu); if cpu.returned_to != 0x100daf4 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100db00; continue;
 }
 0x100db00 => {
  { let v: u32 = cpu.r32(0x10e40a0u32); cpu.edx = v; }
  { let v: u32 = 0x9c40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100db17); f_0105e746(cpu); if cpu.returned_to != 0x100db17 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100db23; continue;
 }
 0x100db23 => {
  cpu.push(0x100db28); f_0100e635(cpu); if cpu.returned_to != 0x100db28 { return; }
  { let v: u32 = 0x1139714u32; cpu.edx = v; }
  { let v: u32 = 0x157c0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100db3e); f_0105e746(cpu); if cpu.returned_to != 0x100db3e { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100db4a; continue;
 }
 0x100db4a => {
  cpu.push(0x100db4f); f_0100e9e2(cpu); if cpu.returned_to != 0x100db4f { return; }
  { let v: u32 = 0x114eed4u32; cpu.edx = v; }
  { let v: u32 = 0x3e80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100db65); f_0105e746(cpu); if cpu.returned_to != 0x100db65 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100db71; continue;
 }
 0x100db71 => {
  cpu.push(0x100db76); f_0100e968(cpu); if cpu.returned_to != 0x100db76 { return; }
  { let v: u32 = 0x1152e54u32; cpu.edx = v; }
  { let v: u32 = 0xac80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100db8c); f_0105e746(cpu); if cpu.returned_to != 0x100db8c { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100db98; continue;
 }
 0x100db98 => {
  cpu.push(0x100db9d); f_0100e7fc(cpu); if cpu.returned_to != 0x100db9d { return; }
  { let v: u32 = 0x115db8cu32; cpu.edx = v; }
  { let v: u32 = 0x898u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100dbb3); f_0105e746(cpu); if cpu.returned_to != 0x100dbb3 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dbbf; continue;
 }
 0x100dbbf => {
  cpu.push(0x100dbc4); f_0100e68d(cpu); if cpu.returned_to != 0x100dbc4 { return; }
  { let v: u32 = 0x10c7adbu32; cpu.edx = v; }
  { let v: u32 = 0x12c04u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100dbda); f_0105e746(cpu); if cpu.returned_to != 0x100dbda { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dbe6; continue;
 }
 0x100dbe6 => {
  cpu.push(0x100dbeb); f_0100ea8b(cpu); if cpu.returned_to != 0x100dbeb { return; }
  { let v: u32 = 0x1172f54u32; cpu.edx = v; }
  { let v: u32 = 0x1900u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100dc01); f_0105e746(cpu); if cpu.returned_to != 0x100dc01 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dc0d; continue;
 }
 0x100dc0d => {
  cpu.push(0x100dc12); f_0100e2aa(cpu); if cpu.returned_to != 0x100dc12 { return; }
  { let v: u32 = 0x1174a48u32; cpu.edx = v; }
  { let v: u32 = 0x780u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100dc28); f_0105e746(cpu); if cpu.returned_to != 0x100dc28 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dc34; continue;
 }
 0x100dc34 => {
  cpu.push(0x100dc39); f_0100e35e(cpu); if cpu.returned_to != 0x100dc39 { return; }
  { let v: u32 = 0x1171334u32; cpu.edx = v; }
  { let v: u32 = 0x1c20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100dc4f); f_0105e746(cpu); if cpu.returned_to != 0x100dc4f { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dc5b; continue;
 }
 0x100dc5b => {
  cpu.push(0x100dc60); f_0100e2f6(cpu); if cpu.returned_to != 0x100dc60 { return; }
  { let v: u32 = 0x11757a4u32; cpu.edx = v; }
  { let v: u32 = 0x4380u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100dc76); f_0105e746(cpu); if cpu.returned_to != 0x100dc76 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dc82; continue;
 }
 0x100dc82 => {
  cpu.push(0x100dc87); f_0100e257(cpu); if cpu.returned_to != 0x100dc87 { return; }
  { let v: u32 = 0x11a188cu32; cpu.edx = v; }
  { let v: u32 = 0x1f40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100dc9d); f_0105e746(cpu); if cpu.returned_to != 0x100dc9d { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dca9; continue;
 }
 0x100dca9 => {
  cpu.push(0x100dcae); f_0100e1d6(cpu); if cpu.returned_to != 0x100dcae { return; }
  { let v: u32 = 0x10ef3d4u32; cpu.edx = v; }
  { let v: u32 = 0x270u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100dcc4); f_0105e746(cpu); if cpu.returned_to != 0x100dcc4 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dcd0; continue;
 }
 0x100dcd0 => {
  cpu.push(0x100dcd5); f_0100e160(cpu); if cpu.returned_to != 0x100dcd5 { return; }
  { cpu.pushad(); }
  cpu.push(0x100dcdb); f_0103ecd6(cpu); if cpu.returned_to != 0x100dcdb { return; }
  let t = cpu.r32(0x10e4334u32); cpu.push(0x100dce1); cpu.call(t); if cpu.returned_to != 0x100dce1 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x100dce9); f_01033b46(cpu); if cpu.returned_to != 0x100dce9 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x100dcf7); f_0105a0b6(cpu); if cpu.returned_to != 0x100dcf7 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x10c5944u32; cpu.esi = v; }
  cpu.push(0x100dd02); f_0105a2f3(cpu); if cpu.returned_to != 0x100dd02 { return; }
  cpu.push(0x100dd07); f_01034db3(cpu); if cpu.returned_to != 0x100dd07 { return; }
  cpu.push(0x100dd0c); f_010714bc(cpu); if cpu.returned_to != 0x100dd0c { return; }
  { let v: u32 = 0x11a37f0u32; cpu.esi = v; }
  { let v: u32 = 0x11a3af0u32; cpu.edi = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x0u32; cpu.w32(0x110eab8u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100dd32); f_01001e06(cpu); if cpu.returned_to != 0x100dd32 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x100ddbd; continue; }
  pc = 0x100dd3a; continue;
 }
 0x100dd3a => {
  cpu.push(0x100dd3f); f_01059ee0(cpu); if cpu.returned_to != 0x100dd3f { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x11e21bcu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10fcdd5u32, v); }
  { cpu.sub8(cpu.r8(0x10fd40eu32), 0x0u8); }
  if !cpu.zf { pc = 0x100dd56; continue; }
  pc = 0x100dd56; continue;
 }
 0x100dd56 => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ccce0u32, v); }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ccce2u32, v); }
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ccce4u32, v); }
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ccce6u32, v); }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ccce8u32, v); }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ccceau32, v); }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11cccecu32, v); }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ccceeu32, v); }
  cpu.push(0x100ddbb); f_0105ec4c(cpu); if cpu.returned_to != 0x100ddbb { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100ddbd => {
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100ddcc; continue; }
  pc = 0x100ddc7; continue;
 }
 0x100ddc7 => {
  cpu.push(0x100ddcc); f_01001e06(cpu); if cpu.returned_to != 0x100ddcc { return; }
  pc = 0x100ddcc; continue;
 }
 0x100ddcc => {
  { cpu.sub8(cpu.r8(0x10fd40eu32), 0x0u8); }
  if !cpu.zf { pc = 0x100de0c; continue; }
  pc = 0x100ddd5; continue;
 }
 0x100ddd5 => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x10e3d89u32); cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3d85u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d89u32, v); }
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xbeu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  { let v: u8 = 0x5au8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x100de05); f_01034211(cpu); if cpu.returned_to != 0x100de05 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3d89u32, v); }
  { cpu.popad(); }
  pc = 0x100de0c; continue;
 }
 0x100de0c => {
  cpu.push(0x100de11); f_0105ec4c(cpu); if cpu.returned_to != 0x100de11 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100de13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100de13); }
 let mut pc: u32 = 0x100de13;
 loop { match pc {
 0x100de13 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d98bu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d948u32, v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d963u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x100de40); f_01001d70(cpu); if cpu.returned_to != 0x100de40 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.sf { pc = 0x100debe; continue; }
  pc = 0x100de48; continue;
 }
 0x100de48 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d91u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x110d948u32, v); }
  { let v: u32 = 0x110da69u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100de69); f_0105e75e(cpu); if cpu.returned_to != 0x100de69 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100debe; continue; }
  pc = 0x100de75; continue;
 }
 0x100de75 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100de88); f_0105e75e(cpu); if cpu.returned_to != 0x100de88 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100debe; continue; }
  pc = 0x100de90; continue;
 }
 0x100de90 => {
  { let v: u32 = 0x110da69u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100dea6); f_0105e75e(cpu); if cpu.returned_to != 0x100dea6 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100debe; continue; }
  pc = 0x100deae; continue;
 }
 0x100deae => {
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100deb8); f_01001e06(cpu); if cpu.returned_to != 0x100deb8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x100debe; continue; }
  pc = 0x100debc; continue;
 }
 0x100debc => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100debe => {
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100decd; continue; }
  pc = 0x100dec8; continue;
 }
 0x100dec8 => {
  cpu.push(0x100decd); f_01001e06(cpu); if cpu.returned_to != 0x100decd { return; }
  pc = 0x100decd; continue;
 }
 0x100decd => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100decf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100decf); }
 let mut pc: u32 = 0x100decf;
 loop { match pc {
 0x100decf => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d98bu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d963u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d94cu32, v); }
  cpu.push(0x100defb); f_010019bc(cpu); if cpu.returned_to != 0x100defb { return; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x100df92; continue; }
  pc = 0x100df06; continue;
 }
 0x100df06 => {
  { cpu.pushad(); }
  cpu.push(0x100df0c); f_010018d7(cpu); if cpu.returned_to != 0x100df0c { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8fu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.sf { pc = 0x100df92; continue; }
  pc = 0x100df22; continue;
 }
 0x100df22 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x110d94cu32, v); }
  { let v: u32 = 0x110da6bu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100df45); f_0105e746(cpu); if cpu.returned_to != 0x100df45 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100df92; continue; }
  pc = 0x100df4d; continue;
 }
 0x100df4d => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100df60); f_0105e746(cpu); if cpu.returned_to != 0x100df60 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100df92; continue; }
  pc = 0x100df68; continue;
 }
 0x100df68 => {
  { let v: u32 = 0x110da6cu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100df7e); f_0105e746(cpu); if cpu.returned_to != 0x100df7e { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100df92; continue; }
  pc = 0x100df86; continue;
 }
 0x100df86 => {
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100df90); f_01001e06(cpu); if cpu.returned_to != 0x100df90 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100df92 => {
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100dfa1; continue; }
  pc = 0x100df9c; continue;
 }
 0x100df9c => {
  cpu.push(0x100dfa1); f_01001e06(cpu); if cpu.returned_to != 0x100dfa1 { return; }
  pc = 0x100dfa1; continue;
 }
 0x100dfa1 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100dfa3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100dfa3); }
 let mut pc: u32 = 0x100dfa3;
 loop { match pc {
 0x100dfa3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d98bu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d9e4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { cpu.pushad(); }
  cpu.push(0x100dfc5); f_010019bc(cpu); if cpu.returned_to != 0x100dfc5 { return; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x100e066; continue; }
  pc = 0x100dfd0; continue;
 }
 0x100dfd0 => {
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d94cu32, v); }
  cpu.push(0x100dfe0); f_010018d7(cpu); if cpu.returned_to != 0x100dfe0 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8fu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.sf { pc = 0x100e066; continue; }
  pc = 0x100dff6; continue;
 }
 0x100dff6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x110d94cu32, v); }
  { let v: u32 = 0x110da6bu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100e019); f_0105e746(cpu); if cpu.returned_to != 0x100e019 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100e066; continue; }
  pc = 0x100e021; continue;
 }
 0x100e021 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100e034); f_0105e746(cpu); if cpu.returned_to != 0x100e034 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100e066; continue; }
  pc = 0x100e03c; continue;
 }
 0x100e03c => {
  { let v: u32 = 0x110da6cu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100e052); f_0105e746(cpu); if cpu.returned_to != 0x100e052 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100e066; continue; }
  pc = 0x100e05a; continue;
 }
 0x100e05a => {
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100e064); f_01001e06(cpu); if cpu.returned_to != 0x100e064 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100e066 => {
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100e075; continue; }
  pc = 0x100e070; continue;
 }
 0x100e070 => {
  cpu.push(0x100e075); f_01001e06(cpu); if cpu.returned_to != 0x100e075 { return; }
  pc = 0x100e075; continue;
 }
 0x100e075 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e077(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e077); }
 let mut pc: u32 = 0x100e077;
 loop { match pc {
 0x100e077 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d98bu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110d9e4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d948u32, v); }
  cpu.push(0x100e0a4); f_01001d70(cpu); if cpu.returned_to != 0x100e0a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.sf { pc = 0x100e122; continue; }
  pc = 0x100e0ac; continue;
 }
 0x100e0ac => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d91u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x110d948u32, v); }
  { let v: u32 = 0x110da69u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100e0cd); f_0105e75e(cpu); if cpu.returned_to != 0x100e0cd { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100e122; continue; }
  pc = 0x100e0d9; continue;
 }
 0x100e0d9 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100e0ec); f_0105e75e(cpu); if cpu.returned_to != 0x100e0ec { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100e122; continue; }
  pc = 0x100e0f4; continue;
 }
 0x100e0f4 => {
  { let v: u32 = 0x110da69u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100e10a); f_0105e75e(cpu); if cpu.returned_to != 0x100e10a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100e122; continue; }
  pc = 0x100e112; continue;
 }
 0x100e112 => {
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100e11c); f_01001e06(cpu); if cpu.returned_to != 0x100e11c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x100e122; continue; }
  pc = 0x100e120; continue;
 }
 0x100e120 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100e122 => {
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100e131; continue; }
  pc = 0x100e12c; continue;
 }
 0x100e12c => {
  cpu.push(0x100e131); f_01001e06(cpu); if cpu.returned_to != 0x100e131 { return; }
  pc = 0x100e131; continue;
 }
 0x100e131 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e133(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e133); }
 let mut pc: u32 = 0x100e133;
 loop { match pc {
 0x100e133 => {
  { let v: u32 = 0x10ef3d4u32; cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  pc = 0x100e13d; continue;
 }
 0x100e13d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x15u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e158; continue; }
  pc = 0x100e14c; continue;
 }
 0x100e14c => {
  { cpu.sub32(cpu.r32(0x15u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e158; continue; }
  pc = 0x100e155; continue;
 }
 0x100e155 => {
  { let v: u32 = cpu.r32(0x15u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x15u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e158; continue;
 }
 0x100e158 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1au32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e13d; continue; }
  pc = 0x100e15f; continue;
 }
 0x100e15f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e160); }
 let mut pc: u32 = 0x100e160;
 loop { match pc {
 0x100e160 => {
  { let v: u32 = 0x10ef3d4u32; cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  pc = 0x100e16a; continue;
 }
 0x100e16a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x15u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e185; continue; }
  pc = 0x100e179; continue;
 }
 0x100e179 => {
  { cpu.sub32(cpu.r32(0x15u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e185; continue; }
  pc = 0x100e182; continue;
 }
 0x100e182 => {
  { let v: u32 = cpu.r32(0x15u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x15u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e185; continue;
 }
 0x100e185 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1au32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e16a; continue; }
  pc = 0x100e18c; continue;
 }
 0x100e18c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e18d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e18d); }
 let mut pc: u32 = 0x100e18d;
 loop { match pc {
 0x100e18d => {
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x100e197; continue;
 }
 0x100e197 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e1b2; continue; }
  pc = 0x100e1a6; continue;
 }
 0x100e1a6 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e1b2; continue; }
  pc = 0x100e1af; continue;
 }
 0x100e1af => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e1b2; continue;
 }
 0x100e1b2 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e1ce; continue; }
  pc = 0x100e1c2; continue;
 }
 0x100e1c2 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e1ce; continue; }
  pc = 0x100e1cb; continue;
 }
 0x100e1cb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e1ce; continue;
 }
 0x100e1ce => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e197; continue; }
  pc = 0x100e1d5; continue;
 }
 0x100e1d5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e1d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e1d6); }
 let mut pc: u32 = 0x100e1d6;
 loop { match pc {
 0x100e1d6 => {
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x100e1e0; continue;
 }
 0x100e1e0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e1fb; continue; }
  pc = 0x100e1ef; continue;
 }
 0x100e1ef => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e1fb; continue; }
  pc = 0x100e1f8; continue;
 }
 0x100e1f8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e1fb; continue;
 }
 0x100e1fb => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e217; continue; }
  pc = 0x100e20b; continue;
 }
 0x100e20b => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e217; continue; }
  pc = 0x100e214; continue;
 }
 0x100e214 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e217; continue;
 }
 0x100e217 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e1e0; continue; }
  pc = 0x100e21e; continue;
 }
 0x100e21e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e21f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e21f); }
 let mut pc: u32 = 0x100e21f;
 loop { match pc {
 0x100e21f => {
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100e229; continue;
 }
 0x100e229 => {
  { let v: u32 = cpu.r32(0x10e3f94u32); cpu.eax = v; }
  { let v: u32 = 0x1179b24u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e23e; continue; }
  pc = 0x100e23b; continue;
 }
 0x100e23b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e23e; continue;
 }
 0x100e23e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.sf { pc = 0x100e250; continue; }
  pc = 0x100e246; continue;
 }
 0x100e246 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) | 0x80000000u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e250; continue;
 }
 0x100e250 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e229; continue; }
  pc = 0x100e256; continue;
 }
 0x100e256 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e257(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e257); }
 let mut pc: u32 = 0x100e257;
 loop { match pc {
 0x100e257 => {
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100e261; continue;
 }
 0x100e261 => {
  { let v: u32 = cpu.r32(0x10e3f94u32); cpu.eax = v; }
  { let v: u32 = 0x1179b24u32; cpu.ebx = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x80000000u32); }
  if cpu.zf { pc = 0x100e280; continue; }
  pc = 0x100e273; continue;
 }
 0x100e273 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) & 0x7fffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e28b; continue;
 }
 0x100e280 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e28b; continue; }
  pc = 0x100e288; continue;
 }
 0x100e288 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e28b; continue;
 }
 0x100e28b => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e261; continue; }
  pc = 0x100e291; continue;
 }
 0x100e291 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e292(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e292); }
 let mut pc: u32 = 0x100e292;
 loop { match pc {
 0x100e292 => {
  { let v: u32 = 0x1172f54u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x100e29c; continue;
 }
 0x100e29c => {
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.esi)).wrapping_sub(0x1075ad4u32); cpu.w32(0x12u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e29c; continue; }
  pc = 0x100e2a9; continue;
 }
 0x100e2a9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e2aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e2aa); }
 let mut pc: u32 = 0x100e2aa;
 loop { match pc {
 0x100e2aa => {
  { let v: u32 = 0x1172f54u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x100e2b4; continue;
 }
 0x100e2b4 => {
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.esi)).wrapping_add(0x1075ad4u32); cpu.w32(0x12u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e2b4; continue; }
  pc = 0x100e2c1; continue;
 }
 0x100e2c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e2c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e2c2); }
 let mut pc: u32 = 0x100e2c2;
 loop { match pc {
 0x100e2c2 => {
  { let v: u32 = 0x1171334u32; cpu.esi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  pc = 0x100e2cc; continue;
 }
 0x100e2cc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_sub(0x101bdb4u32); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e2ee; continue; }
  pc = 0x100e2e2; continue;
 }
 0x100e2e2 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e2ee; continue; }
  pc = 0x100e2eb; continue;
 }
 0x100e2eb => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e2ee; continue;
 }
 0x100e2ee => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x30u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e2cc; continue; }
  pc = 0x100e2f5; continue;
 }
 0x100e2f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e2f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e2f6); }
 let mut pc: u32 = 0x100e2f6;
 loop { match pc {
 0x100e2f6 => {
  { let v: u32 = 0x1171334u32; cpu.esi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  pc = 0x100e300; continue;
 }
 0x100e300 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_add(0x101bdb4u32); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e322; continue; }
  pc = 0x100e316; continue;
 }
 0x100e316 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e322; continue; }
  pc = 0x100e31f; continue;
 }
 0x100e31f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e322; continue;
 }
 0x100e322 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x30u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e300; continue; }
  pc = 0x100e329; continue;
 }
 0x100e329 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e32a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e32a); }
 let mut pc: u32 = 0x100e32a;
 loop { match pc {
 0x100e32a => {
  { let v: u32 = 0x1174a48u32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x100e334; continue;
 }
 0x100e334 => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esi)).wrapping_sub(0x1075ad4u32); cpu.w32(0xau32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xeu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e356; continue; }
  pc = 0x100e34a; continue;
 }
 0x100e34a => {
  { cpu.sub32(cpu.r32(0xeu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e356; continue; }
  pc = 0x100e353; continue;
 }
 0x100e353 => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0xeu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e356; continue;
 }
 0x100e356 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x3cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e334; continue; }
  pc = 0x100e35d; continue;
 }
 0x100e35d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e35e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e35e); }
 let mut pc: u32 = 0x100e35e;
 loop { match pc {
 0x100e35e => {
  { let v: u32 = 0x1174a48u32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x100e368; continue;
 }
 0x100e368 => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esi)).wrapping_add(0x1075ad4u32); cpu.w32(0xau32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xeu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e38a; continue; }
  pc = 0x100e37e; continue;
 }
 0x100e37e => {
  { cpu.sub32(cpu.r32(0xeu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e38a; continue; }
  pc = 0x100e387; continue;
 }
 0x100e387 => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xeu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e38a; continue;
 }
 0x100e38a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x3cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e368; continue; }
  pc = 0x100e391; continue;
 }
 0x100e391 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e392(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e392); }
 let mut pc: u32 = 0x100e392;
 loop { match pc {
 0x100e392 => {
  { let v: u32 = 0x10c6a68u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100e39c; continue;
 }
 0x100e39c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e3b7; continue; }
  pc = 0x100e3ab; continue;
 }
 0x100e3ab => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e3b7; continue; }
  pc = 0x100e3b4; continue;
 }
 0x100e3b4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e3b7; continue;
 }
 0x100e3b7 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e39c; continue; }
  pc = 0x100e3be; continue;
 }
 0x100e3be => {
  { let v: u32 = 0x10c0c74u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100e3c8; continue;
 }
 0x100e3c8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e3e3; continue; }
  pc = 0x100e3d7; continue;
 }
 0x100e3d7 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e3e3; continue; }
  pc = 0x100e3e0; continue;
 }
 0x100e3e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e3e3; continue;
 }
 0x100e3e3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e3c8; continue; }
  pc = 0x100e3ea; continue;
 }
 0x100e3ea => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e3eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e3eb); }
 let mut pc: u32 = 0x100e3eb;
 loop { match pc {
 0x100e3eb => {
  { let v: u32 = 0x10c6a68u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100e3f5; continue;
 }
 0x100e3f5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e410; continue; }
  pc = 0x100e404; continue;
 }
 0x100e404 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e410; continue; }
  pc = 0x100e40d; continue;
 }
 0x100e40d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e410; continue;
 }
 0x100e410 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e3f5; continue; }
  pc = 0x100e417; continue;
 }
 0x100e417 => {
  { let v: u32 = 0x10c0c74u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100e421; continue;
 }
 0x100e421 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e43c; continue; }
  pc = 0x100e430; continue;
 }
 0x100e430 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e43c; continue; }
  pc = 0x100e439; continue;
 }
 0x100e439 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e43c; continue;
 }
 0x100e43c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e421; continue; }
  pc = 0x100e443; continue;
 }
 0x100e443 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e444(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e444); }
 let mut pc: u32 = 0x100e444;
 loop { match pc {
 0x100e444 => {
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100e44e; continue;
 }
 0x100e44e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e466; continue; }
  pc = 0x100e45c; continue;
 }
 0x100e45c => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e466; continue; }
  pc = 0x100e464; continue;
 }
 0x100e464 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e466; continue;
 }
 0x100e466 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e482; continue; }
  pc = 0x100e476; continue;
 }
 0x100e476 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e482; continue; }
  pc = 0x100e47f; continue;
 }
 0x100e47f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e482; continue;
 }
 0x100e482 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e49e; continue; }
  pc = 0x100e492; continue;
 }
 0x100e492 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e49e; continue; }
  pc = 0x100e49b; continue;
 }
 0x100e49b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e49e; continue;
 }
 0x100e49e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e44e; continue; }
  pc = 0x100e4a5; continue;
 }
 0x100e4a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e4a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e4a6); }
 let mut pc: u32 = 0x100e4a6;
 loop { match pc {
 0x100e4a6 => {
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100e4b0; continue;
 }
 0x100e4b0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e4c8; continue; }
  pc = 0x100e4be; continue;
 }
 0x100e4be => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e4c8; continue; }
  pc = 0x100e4c6; continue;
 }
 0x100e4c6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e4c8; continue;
 }
 0x100e4c8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e4e4; continue; }
  pc = 0x100e4d8; continue;
 }
 0x100e4d8 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e4e4; continue; }
  pc = 0x100e4e1; continue;
 }
 0x100e4e1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e4e4; continue;
 }
 0x100e4e4 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e500; continue; }
  pc = 0x100e4f4; continue;
 }
 0x100e4f4 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e500; continue; }
  pc = 0x100e4fd; continue;
 }
 0x100e4fd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e500; continue;
 }
 0x100e500 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e4b0; continue; }
  pc = 0x100e507; continue;
 }
 0x100e507 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e508(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e508); }
 let mut pc: u32 = 0x100e508;
 loop { match pc {
 0x100e508 => {
  { let v: u32 = 0x116c0d8u32; cpu.esi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x100e512; continue;
 }
 0x100e512 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x16u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e52d; continue; }
  pc = 0x100e521; continue;
 }
 0x100e521 => {
  { cpu.sub32(cpu.r32(0x16u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e52d; continue; }
  pc = 0x100e52a; continue;
 }
 0x100e52a => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x16u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e52d; continue;
 }
 0x100e52d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e549; continue; }
  pc = 0x100e53d; continue;
 }
 0x100e53d => {
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e549; continue; }
  pc = 0x100e546; continue;
 }
 0x100e546 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x1eu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e549; continue;
 }
 0x100e549 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2eu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e565; continue; }
  pc = 0x100e559; continue;
 }
 0x100e559 => {
  { cpu.sub32(cpu.r32(0x2eu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e565; continue; }
  pc = 0x100e562; continue;
 }
 0x100e562 => {
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x2eu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e565; continue;
 }
 0x100e565 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e581; continue; }
  pc = 0x100e575; continue;
 }
 0x100e575 => {
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e581; continue; }
  pc = 0x100e57e; continue;
 }
 0x100e57e => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x3cu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e581; continue;
 }
 0x100e581 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e512; continue; }
  pc = 0x100e588; continue;
 }
 0x100e588 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e589(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e589); }
 let mut pc: u32 = 0x100e589;
 loop { match pc {
 0x100e589 => {
  { let v: u32 = 0x116c0d8u32; cpu.esi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x100e593; continue;
 }
 0x100e593 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x16u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e5ae; continue; }
  pc = 0x100e5a2; continue;
 }
 0x100e5a2 => {
  { cpu.sub32(cpu.r32(0x16u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e5ae; continue; }
  pc = 0x100e5ab; continue;
 }
 0x100e5ab => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x16u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e5ae; continue;
 }
 0x100e5ae => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e5ca; continue; }
  pc = 0x100e5be; continue;
 }
 0x100e5be => {
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e5ca; continue; }
  pc = 0x100e5c7; continue;
 }
 0x100e5c7 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x1eu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e5ca; continue;
 }
 0x100e5ca => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2eu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e5e6; continue; }
  pc = 0x100e5da; continue;
 }
 0x100e5da => {
  { cpu.sub32(cpu.r32(0x2eu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e5e6; continue; }
  pc = 0x100e5e3; continue;
 }
 0x100e5e3 => {
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x2eu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e5e6; continue;
 }
 0x100e5e6 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e602; continue; }
  pc = 0x100e5f6; continue;
 }
 0x100e5f6 => {
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e602; continue; }
  pc = 0x100e5ff; continue;
 }
 0x100e5ff => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x3cu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e602; continue;
 }
 0x100e602 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e593; continue; }
  pc = 0x100e609; continue;
 }
 0x100e609 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e60a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e60a); }
 let mut pc: u32 = 0x100e60a;
 loop { match pc {
 0x100e60a => {
  { let v: u32 = cpu.r32(0x10e40a0u32); cpu.esi = v; }
  { let v: u32 = 0x1388u32; cpu.ecx = v; }
  pc = 0x100e615; continue;
 }
 0x100e615 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e62d; continue; }
  pc = 0x100e623; continue;
 }
 0x100e623 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e62d; continue; }
  pc = 0x100e62b; continue;
 }
 0x100e62b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e62d; continue;
 }
 0x100e62d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e615; continue; }
  pc = 0x100e634; continue;
 }
 0x100e634 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e635(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e635); }
 let mut pc: u32 = 0x100e635;
 loop { match pc {
 0x100e635 => {
  { let v: u32 = cpu.r32(0x10e40a0u32); cpu.esi = v; }
  { let v: u32 = 0x1388u32; cpu.ecx = v; }
  pc = 0x100e640; continue;
 }
 0x100e640 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e658; continue; }
  pc = 0x100e64e; continue;
 }
 0x100e64e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e658; continue; }
  pc = 0x100e656; continue;
 }
 0x100e656 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e658; continue;
 }
 0x100e658 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e640; continue; }
  pc = 0x100e65f; continue;
 }
 0x100e65f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e660); }
 let mut pc: u32 = 0x100e660;
 loop { match pc {
 0x100e660 => {
  { let v: u32 = 0x115db8cu32; cpu.esi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x100e66a; continue;
 }
 0x100e66a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e685; continue; }
  pc = 0x100e679; continue;
 }
 0x100e679 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e685; continue; }
  pc = 0x100e682; continue;
 }
 0x100e682 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e685; continue;
 }
 0x100e685 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e66a; continue; }
  pc = 0x100e68c; continue;
 }
 0x100e68c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e68d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e68d); }
 let mut pc: u32 = 0x100e68d;
 loop { match pc {
 0x100e68d => {
  { let v: u32 = 0x115db8cu32; cpu.esi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x100e697; continue;
 }
 0x100e697 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e6b2; continue; }
  pc = 0x100e6a6; continue;
 }
 0x100e6a6 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e6b2; continue; }
  pc = 0x100e6af; continue;
 }
 0x100e6af => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e6b2; continue;
 }
 0x100e6b2 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e697; continue; }
  pc = 0x100e6b9; continue;
 }
 0x100e6b9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e6ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e6ba); }
 let mut pc: u32 = 0x100e6ba;
 loop { match pc {
 0x100e6ba => {
  { let v: u32 = 0x1152e54u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100e6c4; continue;
 }
 0x100e6c4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e6df; continue; }
  pc = 0x100e6d3; continue;
 }
 0x100e6d3 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e6df; continue; }
  pc = 0x100e6dc; continue;
 }
 0x100e6dc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e6df; continue;
 }
 0x100e6df => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e6fb; continue; }
  pc = 0x100e6ef; continue;
 }
 0x100e6ef => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e6fb; continue; }
  pc = 0x100e6f8; continue;
 }
 0x100e6f8 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e6fb; continue;
 }
 0x100e6fb => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e717; continue; }
  pc = 0x100e70b; continue;
 }
 0x100e70b => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e717; continue; }
  pc = 0x100e714; continue;
 }
 0x100e714 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e717; continue;
 }
 0x100e717 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e733; continue; }
  pc = 0x100e727; continue;
 }
 0x100e727 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e733; continue; }
  pc = 0x100e730; continue;
 }
 0x100e730 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e733; continue;
 }
 0x100e733 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e74f; continue; }
  pc = 0x100e743; continue;
 }
 0x100e743 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e74f; continue; }
  pc = 0x100e74c; continue;
 }
 0x100e74c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e74f; continue;
 }
 0x100e74f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e76b; continue; }
  pc = 0x100e75f; continue;
 }
 0x100e75f => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e76b; continue; }
  pc = 0x100e768; continue;
 }
 0x100e768 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e76b; continue;
 }
 0x100e76b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e787; continue; }
  pc = 0x100e77b; continue;
 }
 0x100e77b => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e787; continue; }
  pc = 0x100e784; continue;
 }
 0x100e784 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e787; continue;
 }
 0x100e787 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xa6u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e7ac; continue; }
  pc = 0x100e79a; continue;
 }
 0x100e79a => {
  { cpu.sub32(cpu.r32(0xa6u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e7ac; continue; }
  pc = 0x100e7a6; continue;
 }
 0x100e7a6 => {
  { let v: u32 = cpu.r32(0xa6u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0xa6u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e7ac; continue;
 }
 0x100e7ac => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e7c8; continue; }
  pc = 0x100e7bc; continue;
 }
 0x100e7bc => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e7c8; continue; }
  pc = 0x100e7c5; continue;
 }
 0x100e7c5 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e7c8; continue;
 }
 0x100e7c8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x2084u32); }
  if cpu.zf { pc = 0x100e7ee; continue; }
  pc = 0x100e7d2; continue;
 }
 0x100e7d2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e7ed; continue; }
  pc = 0x100e7e1; continue;
 }
 0x100e7e1 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e7ed; continue; }
  pc = 0x100e7ea; continue;
 }
 0x100e7ea => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e7ed; continue;
 }
 0x100e7ed => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100e7ee; continue;
 }
 0x100e7ee => {
  { let v: u32 = cpu.add32(cpu.esi, 0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e6c4; continue; }
  pc = 0x100e7fb; continue;
 }
 0x100e7fb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e7fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e7fc); }
 let mut pc: u32 = 0x100e7fc;
 loop { match pc {
 0x100e7fc => {
  { let v: u32 = 0x1152e54u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x100e806; continue;
 }
 0x100e806 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e821; continue; }
  pc = 0x100e815; continue;
 }
 0x100e815 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e821; continue; }
  pc = 0x100e81e; continue;
 }
 0x100e81e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e821; continue;
 }
 0x100e821 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e83d; continue; }
  pc = 0x100e831; continue;
 }
 0x100e831 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e83d; continue; }
  pc = 0x100e83a; continue;
 }
 0x100e83a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e83d; continue;
 }
 0x100e83d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e859; continue; }
  pc = 0x100e84d; continue;
 }
 0x100e84d => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e859; continue; }
  pc = 0x100e856; continue;
 }
 0x100e856 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e859; continue;
 }
 0x100e859 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e875; continue; }
  pc = 0x100e869; continue;
 }
 0x100e869 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e875; continue; }
  pc = 0x100e872; continue;
 }
 0x100e872 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e875; continue;
 }
 0x100e875 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e891; continue; }
  pc = 0x100e885; continue;
 }
 0x100e885 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e891; continue; }
  pc = 0x100e88e; continue;
 }
 0x100e88e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e891; continue;
 }
 0x100e891 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e8ad; continue; }
  pc = 0x100e8a1; continue;
 }
 0x100e8a1 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e8ad; continue; }
  pc = 0x100e8aa; continue;
 }
 0x100e8aa => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x100e8ad; continue;
 }
 0x100e8ad => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e8c9; continue; }
  pc = 0x100e8bd; continue;
 }
 0x100e8bd => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e8c9; continue; }
  pc = 0x100e8c6; continue;
 }
 0x100e8c6 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e8c9; continue;
 }
 0x100e8c9 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xa6u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e8ee; continue; }
  pc = 0x100e8dc; continue;
 }
 0x100e8dc => {
  { cpu.sub32(cpu.r32(0xa6u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e8ee; continue; }
  pc = 0x100e8e8; continue;
 }
 0x100e8e8 => {
  { let v: u32 = cpu.r32(0xa6u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xa6u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e8ee; continue;
 }
 0x100e8ee => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e90a; continue; }
  pc = 0x100e8fe; continue;
 }
 0x100e8fe => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e90a; continue; }
  pc = 0x100e907; continue;
 }
 0x100e907 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e90a; continue;
 }
 0x100e90a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x2084u32); }
  if cpu.zf { pc = 0x100e930; continue; }
  pc = 0x100e914; continue;
 }
 0x100e914 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e92f; continue; }
  pc = 0x100e923; continue;
 }
 0x100e923 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e92f; continue; }
  pc = 0x100e92c; continue;
 }
 0x100e92c => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e92f; continue;
 }
 0x100e92f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100e930; continue;
 }
 0x100e930 => {
  { let v: u32 = cpu.add32(cpu.esi, 0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e806; continue; }
  pc = 0x100e93d; continue;
 }
 0x100e93d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e93e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e93e); }
 let mut pc: u32 = 0x100e93e;
 loop { match pc {
 0x100e93e => {
  { let v: u32 = 0x114eed4u32; cpu.esi = v; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  pc = 0x100e948; continue;
 }
 0x100e948 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e960; continue; }
  pc = 0x100e956; continue;
 }
 0x100e956 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e960; continue; }
  pc = 0x100e95e; continue;
 }
 0x100e95e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e960; continue;
 }
 0x100e960 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e948; continue; }
  pc = 0x100e967; continue;
 }
 0x100e967 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e968(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e968); }
 let mut pc: u32 = 0x100e968;
 loop { match pc {
 0x100e968 => {
  { let v: u32 = 0x114eed4u32; cpu.esi = v; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  pc = 0x100e972; continue;
 }
 0x100e972 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e98a; continue; }
  pc = 0x100e980; continue;
 }
 0x100e980 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e98a; continue; }
  pc = 0x100e988; continue;
 }
 0x100e988 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e98a; continue;
 }
 0x100e98a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e972; continue; }
  pc = 0x100e991; continue;
 }
 0x100e991 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e992(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e992); }
 let mut pc: u32 = 0x100e992;
 loop { match pc {
 0x100e992 => {
  { let v: u32 = 0x1139714u32; cpu.esi = v; }
  { let v: u32 = 0x7d0u32; cpu.ecx = v; }
  pc = 0x100e99c; continue;
 }
 0x100e99c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)).wrapping_sub(0x105a73au32); cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e9be; continue; }
  pc = 0x100e9b2; continue;
 }
 0x100e9b2 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e9be; continue; }
  pc = 0x100e9bb; continue;
 }
 0x100e9bb => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e9be; continue;
 }
 0x100e9be => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100e9da; continue; }
  pc = 0x100e9ce; continue;
 }
 0x100e9ce => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100e9da; continue; }
  pc = 0x100e9d7; continue;
 }
 0x100e9d7 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  pc = 0x100e9da; continue;
 }
 0x100e9da => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e99c; continue; }
  pc = 0x100e9e1; continue;
 }
 0x100e9e1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100e9e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100e9e2); }
 let mut pc: u32 = 0x100e9e2;
 loop { match pc {
 0x100e9e2 => {
  { let v: u32 = 0x1139714u32; cpu.esi = v; }
  { let v: u32 = 0x7d0u32; cpu.ecx = v; }
  pc = 0x100e9ec; continue;
 }
 0x100e9ec => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)).wrapping_add(0x105a73au32); cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100ea0e; continue; }
  pc = 0x100ea02; continue;
 }
 0x100ea02 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100ea0e; continue; }
  pc = 0x100ea0b; continue;
 }
 0x100ea0b => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x100ea0e; continue;
 }
 0x100ea0e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100ea2a; continue; }
  pc = 0x100ea1e; continue;
 }
 0x100ea1e => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100ea2a; continue; }
  pc = 0x100ea27; continue;
 }
 0x100ea27 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  pc = 0x100ea2a; continue;
 }
 0x100ea2a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100e9ec; continue; }
  pc = 0x100ea31; continue;
 }
 0x100ea31 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ea32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ea32); }
 let mut pc: u32 = 0x100ea32;
 loop { match pc {
 0x100ea32 => {
  { let v: u32 = 0x10c7adbu32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x100ea3c; continue;
 }
 0x100ea3c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100ea57; continue; }
  pc = 0x100ea4b; continue;
 }
 0x100ea4b => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100ea57; continue; }
  pc = 0x100ea54; continue;
 }
 0x100ea54 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100ea57; continue;
 }
 0x100ea57 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100ea3c; continue; }
  pc = 0x100ea5e; continue;
 }
 0x100ea5e => {
  { let v: u32 = 0x10d10dbu32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x100ea68; continue;
 }
 0x100ea68 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100ea83; continue; }
  pc = 0x100ea77; continue;
 }
 0x100ea77 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100ea83; continue; }
  pc = 0x100ea80; continue;
 }
 0x100ea80 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100ea83; continue;
 }
 0x100ea83 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100ea68; continue; }
  pc = 0x100ea8a; continue;
 }
 0x100ea8a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100ea8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100ea8b); }
 let mut pc: u32 = 0x100ea8b;
 loop { match pc {
 0x100ea8b => {
  { let v: u32 = 0x10c7adbu32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x100ea95; continue;
 }
 0x100ea95 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100eab0; continue; }
  pc = 0x100eaa4; continue;
 }
 0x100eaa4 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100eab0; continue; }
  pc = 0x100eaad; continue;
 }
 0x100eaad => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100eab0; continue;
 }
 0x100eab0 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100ea95; continue; }
  pc = 0x100eab7; continue;
 }
 0x100eab7 => {
  { let v: u32 = 0x10d10dbu32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x100eac1; continue;
 }
 0x100eac1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f98u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x100eadc; continue; }
  pc = 0x100ead0; continue;
 }
 0x100ead0 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x100eadc; continue; }
  pc = 0x100ead9; continue;
 }
 0x100ead9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x100eadc; continue;
 }
 0x100eadc => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x100eac1; continue; }
  pc = 0x100eae3; continue;
 }
 0x100eae3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100eae4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100eae4); }
 let mut pc: u32 = 0x100eae4;
 loop { match pc {
 0x100eae4 => {
  { cpu.sub8(cpu.r8(0x110d954u32), 0x0u8); }
  if cpu.zf { pc = 0x100efe9; continue; }
  pc = 0x100eaf1; continue;
 }
 0x100eaf1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x110d956u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d948u32, v); }
  cpu.push(0x100eb10); f_01001d70(cpu); if cpu.returned_to != 0x100eb10 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.sf { pc = 0x100efd8; continue; }
  pc = 0x100eb18; continue;
 }
 0x100eb18 => {
  { let v: u32 = cpu.eax; cpu.w32(0x110d948u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d91u32, v); }
  { let v: u8 = 0x32u8; cpu.w8(0x110d962u32, v); }
  { let v: u32 = 0x110d962u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100eb40); f_0105e75e(cpu); if cpu.returned_to != 0x100eb40 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100eb4c; continue;
 }
 0x100eb4c => {
  { let v: u32 = 0x1105737u32; cpu.edx = v; }
  { let v: u32 = 0x34u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100eb62); f_0105e75e(cpu); if cpu.returned_to != 0x100eb62 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100eb6e; continue;
 }
 0x100eb6e => {
  { let v: u32 = 0x112c97cu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100eb84); f_0105e75e(cpu); if cpu.returned_to != 0x100eb84 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100eb90; continue;
 }
 0x100eb90 => {
  { let v: u32 = 0x112c97du32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100eba6); f_0105e75e(cpu); if cpu.returned_to != 0x100eba6 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ebb2; continue;
 }
 0x100ebb2 => {
  { let v: u32 = 0x10e3d7fu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ebc8); f_0105e75e(cpu); if cpu.returned_to != 0x100ebc8 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ebd4; continue;
 }
 0x100ebd4 => {
  { let v: u32 = 0x112639eu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ebea); f_0105e75e(cpu); if cpu.returned_to != 0x100ebea { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ebf6; continue;
 }
 0x100ebf6 => {
  { let v: u32 = 0x11263c7u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ec0c); f_0105e75e(cpu); if cpu.returned_to != 0x100ec0c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ec18; continue;
 }
 0x100ec18 => {
  { let v: u32 = 0x10e3e81u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ec2e); f_0105e75e(cpu); if cpu.returned_to != 0x100ec2e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ec3a; continue;
 }
 0x100ec3a => {
  { let v: u32 = 0x1124bfdu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ec50); f_0105e75e(cpu); if cpu.returned_to != 0x100ec50 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ec5c; continue;
 }
 0x100ec5c => {
  { let v: u32 = 0x112446du32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ec72); f_0105e75e(cpu); if cpu.returned_to != 0x100ec72 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ec7e; continue;
 }
 0x100ec7e => {
  { let v: u32 = 0x112446fu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ec94); f_0105e75e(cpu); if cpu.returned_to != 0x100ec94 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100eca0; continue;
 }
 0x100eca0 => {
  { let v: u32 = 0x1124469u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ecb6); f_0105e75e(cpu); if cpu.returned_to != 0x100ecb6 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ecc2; continue;
 }
 0x100ecc2 => {
  { let v: u32 = 0x112446bu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ecd8); f_0105e75e(cpu); if cpu.returned_to != 0x100ecd8 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ece4; continue;
 }
 0x100ece4 => {
  { let v: u32 = 0x11302acu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ecfa); f_0105e75e(cpu); if cpu.returned_to != 0x100ecfa { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ed06; continue;
 }
 0x100ed06 => {
  { let v: u32 = 0x1130f1du32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ed1c); f_0105e75e(cpu); if cpu.returned_to != 0x100ed1c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ed28; continue;
 }
 0x100ed28 => {
  { let v: u32 = 0x1125281u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ed3e); f_0105e75e(cpu); if cpu.returned_to != 0x100ed3e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ed4a; continue;
 }
 0x100ed4a => {
  { let v: u32 = 0x11263a0u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ed60); f_0105e75e(cpu); if cpu.returned_to != 0x100ed60 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ed6c; continue;
 }
 0x100ed6c => {
  { let v: u32 = 0x1124c5fu32; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ed82); f_0105e75e(cpu); if cpu.returned_to != 0x100ed82 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ed8e; continue;
 }
 0x100ed8e => {
  { let v: u32 = 0x10f7acdu32; cpu.edx = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100eda4); f_0105e75e(cpu); if cpu.returned_to != 0x100eda4 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100edb0; continue;
 }
 0x100edb0 => {
  { let v: u32 = 0x112c98au32; cpu.edx = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100edc6); f_0105e75e(cpu); if cpu.returned_to != 0x100edc6 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100edd2; continue;
 }
 0x100edd2 => {
  { let v: u32 = 0x10ef383u32; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ede8); f_0105e75e(cpu); if cpu.returned_to != 0x100ede8 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100edf4; continue;
 }
 0x100edf4 => {
  { let v: u32 = 0x11263cfu32; cpu.edx = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ee0a); f_0105e75e(cpu); if cpu.returned_to != 0x100ee0a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ee16; continue;
 }
 0x100ee16 => {
  { let v: u32 = 0x10fd374u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ee2c); f_0105e75e(cpu); if cpu.returned_to != 0x100ee2c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ee38; continue;
 }
 0x100ee38 => {
  { let v: u32 = 0x1124e8au32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ee4e); f_0105e75e(cpu); if cpu.returned_to != 0x100ee4e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ee5a; continue;
 }
 0x100ee5a => {
  { let v: u32 = 0x1105eaau32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ee70); f_0105e75e(cpu); if cpu.returned_to != 0x100ee70 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ee7c; continue;
 }
 0x100ee7c => {
  { let v: u32 = 0x10ef36du32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ee92); f_0105e75e(cpu); if cpu.returned_to != 0x100ee92 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ee9e; continue;
 }
 0x100ee9e => {
  { let v: u32 = 0x10f7fc8u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100eeb4); f_0105e75e(cpu); if cpu.returned_to != 0x100eeb4 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100eec0; continue;
 }
 0x100eec0 => {
  { let v: u32 = 0x11c5b21u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100eed6); f_0105e75e(cpu); if cpu.returned_to != 0x100eed6 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100eee2; continue;
 }
 0x100eee2 => {
  { let v: u32 = 0x11c5b22u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100eef8); f_0105e75e(cpu); if cpu.returned_to != 0x100eef8 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ef04; continue;
 }
 0x100ef04 => {
  { let v: u32 = 0x10ef352u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ef1a); f_0105e75e(cpu); if cpu.returned_to != 0x100ef1a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ef26; continue;
 }
 0x100ef26 => {
  { let v: u32 = 0x11c5b8cu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ef3c); f_0105e75e(cpu); if cpu.returned_to != 0x100ef3c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ef48; continue;
 }
 0x100ef48 => {
  { let v: u32 = 0x11c5b8du32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ef5e); f_0105e75e(cpu); if cpu.returned_to != 0x100ef5e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ef6a; continue;
 }
 0x100ef6a => {
  { let v: u32 = 0x111e9f4u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100ef80); f_0105e75e(cpu); if cpu.returned_to != 0x100ef80 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100ef8c; continue;
 }
 0x100ef8c => {
  { let v: u32 = 0x111e9f6u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100efa2); f_0105e75e(cpu); if cpu.returned_to != 0x100efa2 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100efaa; continue;
 }
 0x100efaa => {
  { let v: u32 = 0x111e9f5u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100efc0); f_0105e75e(cpu); if cpu.returned_to != 0x100efc0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x11c5afeu32)); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100efc8; continue;
 }
 0x100efc8 => {
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  cpu.push(0x100efd2); f_01001e06(cpu); if cpu.returned_to != 0x100efd2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x100efd8; continue; }
  pc = 0x100efd6; continue;
 }
 0x100efd6 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100efd8 => {
  { let v: u32 = cpu.r32(0x110d948u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100efe7; continue; }
  pc = 0x100efe2; continue;
 }
 0x100efe2 => {
  cpu.push(0x100efe7); f_01001e06(cpu); if cpu.returned_to != 0x100efe7 { return; }
  pc = 0x100efe7; continue;
 }
 0x100efe7 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100efe9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100efea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100efea); } r_0100efea(cpu, 0x100efea); }
pub fn f_0100f47d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100f47d); } r_0100efea(cpu, 0x100f47d); }
fn r_0100efea(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x100efea => {
  { cpu.pushad(); }
  cpu.push(0x100eff0); f_0105ec4c(cpu); if cpu.returned_to != 0x100eff0 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c59c4u32, v); }
  { cpu.sub8(cpu.r8(0x110d954u32), 0x0u8); }
  if cpu.zf { pc = 0x100f58c; continue; }
  pc = 0x100f005; continue;
 }
 0x100f005 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x110d956u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { cpu.pushad(); }
  cpu.push(0x100f019); f_010019bc(cpu); if cpu.returned_to != 0x100f019 { return; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x100f568; continue; }
  pc = 0x100f024; continue;
 }
 0x100f024 => {
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x110d94cu32, v); }
  cpu.push(0x100f034); f_010018d7(cpu); if cpu.returned_to != 0x100f034 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8fu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.sf { pc = 0x100f57b; continue; }
  pc = 0x100f04a; continue;
 }
 0x100f04a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x110d94cu32, v); }
  { let v: u32 = 0x110d962u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f06d); f_0105e746(cpu); if cpu.returned_to != 0x100f06d { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f079; continue;
 }
 0x100f079 => {
  { cpu.sub8(cpu.r8(0x110d962u32), 0x32u8); }
  if !cpu.zf { pc = 0x100f568; continue; }
  pc = 0x100f086; continue;
 }
 0x100f086 => {
  { let v: u32 = 0x1105737u32; cpu.edx = v; }
  { let v: u32 = 0x34u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f09c); f_0105e746(cpu); if cpu.returned_to != 0x100f09c { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f0a8; continue;
 }
 0x100f0a8 => {
  { let v: u32 = 0x112c97cu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f0be); f_0105e746(cpu); if cpu.returned_to != 0x100f0be { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f0ca; continue;
 }
 0x100f0ca => {
  { let v: u32 = 0x112c97du32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f0e0); f_0105e746(cpu); if cpu.returned_to != 0x100f0e0 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f0ec; continue;
 }
 0x100f0ec => {
  { let v: u32 = 0x10e3d7fu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f102); f_0105e746(cpu); if cpu.returned_to != 0x100f102 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f10e; continue;
 }
 0x100f10e => {
  { let v: u32 = 0x112639eu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f124); f_0105e746(cpu); if cpu.returned_to != 0x100f124 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f130; continue;
 }
 0x100f130 => {
  { let v: u32 = 0x11263c7u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f146); f_0105e746(cpu); if cpu.returned_to != 0x100f146 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f152; continue;
 }
 0x100f152 => {
  { let v: u32 = 0x10e3e81u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f168); f_0105e746(cpu); if cpu.returned_to != 0x100f168 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f174; continue;
 }
 0x100f174 => {
  { let v: u32 = 0x1124bfdu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f18a); f_0105e746(cpu); if cpu.returned_to != 0x100f18a { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f196; continue;
 }
 0x100f196 => {
  { let v: u32 = 0x112446du32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f1ac); f_0105e746(cpu); if cpu.returned_to != 0x100f1ac { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f1b8; continue;
 }
 0x100f1b8 => {
  { let v: u32 = 0x112446fu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f1ce); f_0105e746(cpu); if cpu.returned_to != 0x100f1ce { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f1da; continue;
 }
 0x100f1da => {
  { let v: u32 = 0x1124469u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f1f0); f_0105e746(cpu); if cpu.returned_to != 0x100f1f0 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f1fc; continue;
 }
 0x100f1fc => {
  { let v: u32 = 0x112446bu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f212); f_0105e746(cpu); if cpu.returned_to != 0x100f212 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f21e; continue;
 }
 0x100f21e => {
  { let v: u32 = 0x11302acu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f234); f_0105e746(cpu); if cpu.returned_to != 0x100f234 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f240; continue;
 }
 0x100f240 => {
  { let v: u32 = 0x1130f1du32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f256); f_0105e746(cpu); if cpu.returned_to != 0x100f256 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f262; continue;
 }
 0x100f262 => {
  { let v: u32 = 0x1125281u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f278); f_0105e746(cpu); if cpu.returned_to != 0x100f278 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f284; continue;
 }
 0x100f284 => {
  { let v: u32 = 0x11263a0u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f29a); f_0105e746(cpu); if cpu.returned_to != 0x100f29a { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f2a6; continue;
 }
 0x100f2a6 => {
  { let v: u32 = 0x11263cfu32; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f2bc); f_0105e746(cpu); if cpu.returned_to != 0x100f2bc { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f2c8; continue;
 }
 0x100f2c8 => {
  { let v: u32 = 0x10f7acdu32; cpu.edx = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f2de); f_0105e746(cpu); if cpu.returned_to != 0x100f2de { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f2ea; continue;
 }
 0x100f2ea => {
  { let v: u32 = 0x112c98au32; cpu.edx = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f300); f_0105e746(cpu); if cpu.returned_to != 0x100f300 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f30c; continue;
 }
 0x100f30c => {
  { let v: u32 = 0x10ef383u32; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f322); f_0105e746(cpu); if cpu.returned_to != 0x100f322 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f32e; continue;
 }
 0x100f32e => {
  { let v: u32 = 0x11263cfu32; cpu.edx = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f344); f_0105e746(cpu); if cpu.returned_to != 0x100f344 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f350; continue;
 }
 0x100f350 => {
  { let v: u32 = 0x10fd374u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f366); f_0105e746(cpu); if cpu.returned_to != 0x100f366 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f372; continue;
 }
 0x100f372 => {
  { let v: u32 = 0x1124e8au32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f388); f_0105e746(cpu); if cpu.returned_to != 0x100f388 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f394; continue;
 }
 0x100f394 => {
  { let v: u32 = 0x1105eaau32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f3aa); f_0105e746(cpu); if cpu.returned_to != 0x100f3aa { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f3b6; continue;
 }
 0x100f3b6 => {
  { let v: u32 = 0x10ef36du32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f3cc); f_0105e746(cpu); if cpu.returned_to != 0x100f3cc { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f3d8; continue;
 }
 0x100f3d8 => {
  { let v: u32 = 0x10f7fc8u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f3ee); f_0105e746(cpu); if cpu.returned_to != 0x100f3ee { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f3fa; continue;
 }
 0x100f3fa => {
  { let v: u32 = 0x11c5b21u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f410); f_0105e746(cpu); if cpu.returned_to != 0x100f410 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f41c; continue;
 }
 0x100f41c => {
  { let v: u32 = 0x11c5b22u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f432); f_0105e746(cpu); if cpu.returned_to != 0x100f432 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f43e; continue;
 }
 0x100f43e => {
  { let v: u32 = 0x10ef352u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f454); f_0105e746(cpu); if cpu.returned_to != 0x100f454 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f460; continue;
 }
 0x100f460 => {
  { let v: u32 = 0x11c5b8cu32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f476); f_0105e746(cpu); if cpu.returned_to != 0x100f476 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f482; continue;
 }
 0x100f47d => {
  { cpu.and32(cpu.ecx, cpu.edi); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x11c5b8du32.wrapping_add(cpu.edx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x11c5b8du32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f498); f_0105e746(cpu); if cpu.returned_to != 0x100f498 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f4a4; continue;
 }
 0x100f482 => {
  { let v: u32 = 0x11c5b8du32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f498); f_0105e746(cpu); if cpu.returned_to != 0x100f498 { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f4a4; continue;
 }
 0x100f4a4 => {
  { let v: u32 = 0x111e9f4u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f4ba); f_0105e746(cpu); if cpu.returned_to != 0x100f4ba { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f4c6; continue;
 }
 0x100f4c6 => {
  { let v: u32 = 0x111e9f6u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f4dc); f_0105e746(cpu); if cpu.returned_to != 0x100f4dc { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f4e8; continue;
 }
 0x100f4e8 => {
  { let v: u32 = 0x111e9f5u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f4fe); f_0105e746(cpu); if cpu.returned_to != 0x100f4fe { return; }
  { cpu.sub32(cpu.r32(0x10e3ff0u32), cpu.ecx); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f50a; continue;
 }
 0x100f50a => {
  { let v: u32 = 0x1bu32; cpu.w32(0x10e4128u32, v); }
  { let v: u8 = cpu.r8(0x11c5b21u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef36du32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x100f531; continue; }
  pc = 0x100f527; continue;
 }
 0x100f527 => {
  { let v: u8 = cpu.r8(0x11c5b22u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef36du32, v); }
  pc = 0x100f531; continue;
 }
 0x100f531 => {
  { let v: u8 = cpu.r8(0x112446du32) & 0x7fu8; cpu.w8(0x112446du32, v); }
  { let v: u8 = cpu.r8(0x112446fu32) & 0x7fu8; cpu.w8(0x112446fu32, v); }
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  cpu.push(0x100f549); f_01001e06(cpu); if cpu.returned_to != 0x100f549 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x100f57b; continue; }
  pc = 0x100f54d; continue;
 }
 0x100f54d => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11263c7u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd52eu32.wrapping_add(cpu.ecx)), 0xffu8); }
  if !cpu.zf { pc = 0x100f566; continue; }
  pc = 0x100f561; continue;
 }
 0x100f561 => {
  cpu.push(0x100f566); f_0104091e(cpu); if cpu.returned_to != 0x100f566 { return; }
  pc = 0x100f566; continue;
 }
 0x100f566 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100f568 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c59c4u32, v); }
  { let v: u32 = cpu.r32(0x1131078u32); cpu.edx = v; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  cpu.poll(); pc = 0x100f531; continue;
 }
 0x100f57b => {
  { let v: u32 = cpu.r32(0x110d94cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x100f58a; continue; }
  pc = 0x100f585; continue;
 }
 0x100f585 => {
  cpu.push(0x100f58a); f_01001e06(cpu); if cpu.returned_to != 0x100f58a { return; }
  pc = 0x100f58a; continue;
 }
 0x100f58a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100f58c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100f590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100f590); }
 let mut pc: u32 = 0x100f590;
 loop { match pc {
 0x100f590 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110db64u32, v); }
  cpu.push(0x100f5a0); f_01047466(cpu); if cpu.returned_to != 0x100f5a0 { return; }
  { let v = cpu.r32(0x42u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u16 = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x100f5ad); f_0100f5c1(cpu); if cpu.returned_to != 0x100f5ad { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x42u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110db64u32, v); }
  { let v: u8 = 0x38u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100f5c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100f5c1); }
 let mut pc: u32 = 0x100f5c1;
 loop { match pc {
 0x100f5c1 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x100f6b7; continue; }
  pc = 0x100f5cb; continue;
 }
 0x100f5cb => {
  cpu.push(0x100f5d0); f_0100fd0c(cpu); if cpu.returned_to != 0x100f5d0 { return; }
  cpu.push(0x100f5d5); f_0100fd23(cpu); if cpu.returned_to != 0x100f5d5 { return; }
  cpu.push(0x100f5da); f_0100fd3a(cpu); if cpu.returned_to != 0x100f5da { return; }
  cpu.push(0x100f5df); f_0100fd51(cpu); if cpu.returned_to != 0x100f5df { return; }
  { let v: u8 = cpu.r8(0x110db65u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x110db66u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x110db67u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x110db68u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x100f6b7; continue; }
  pc = 0x100f5fe; continue;
 }
 0x100f5fe => {
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x100f610; continue; }
  pc = 0x100f60d; continue;
 }
 0x100f60d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x100f610; continue;
 }
 0x100f610 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x11757b7u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x11757b6u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x11b544bu32, v); }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x100f630; continue;
 }
 0x100f630 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x15u8); }
  if (cpu.sf == cpu.of) { pc = 0x100f6b6; continue; }
  pc = 0x100f637; continue;
 }
 0x100f637 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))), 0xffu8); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.zf { pc = 0x100f650; continue; }
  pc = 0x100f64c; continue;
 }
 0x100f64c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x100f630; continue;
 }
 0x100f650 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.mul8(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0xb0u32.wrapping_add(cpu.eax))); cpu.w8(0xb0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xb0u32.wrapping_add(cpu.eax)), 0xfu8); cpu.w8(0xb0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x100f66f; continue; }
  pc = 0x100f66f; continue;
 }
 0x100f66f => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x100f681; continue; }
  pc = 0x100f678; continue;
 }
 0x100f678 => {
  { cpu.sub8(cpu.r8(0xb0u32.wrapping_add(cpu.eax)), 0xau8); }
  if (cpu.sf == cpu.of) { pc = 0x100f6a3; continue; }
  pc = 0x100f681; continue;
 }
 0x100f681 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x11b544bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x100f68d); f_0100f7c7(cpu); if cpu.returned_to != 0x100f68d { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11b544bu32)); cpu.w8(0x11b544bu32, v); }
  { cpu.sub8(cpu.r8(0x11b544bu32), 0xcu8); }
  if cpu.zf { pc = 0x100f6a3; continue; }
  pc = 0x100f69f; continue;
 }
 0x100f69f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x100f681; continue; }
  pc = 0x100f6a3; continue;
 }
 0x100f6a3 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11b544bu32), 0xcu8); }
  if cpu.zf { pc = 0x100f6b6; continue; }
  pc = 0x100f6ae; continue;
 }
 0x100f6ae => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100f630; continue; }
  pc = 0x100f6b6; continue;
 }
 0x100f6b6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x100f6b7; continue;
 }
 0x100f6b7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100f6b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100f6b8); }
 let mut pc: u32 = 0x100f6b8;
 loop { match pc {
 0x100f6b8 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x100f7c6; continue; }
  pc = 0x100f6c2; continue;
 }
 0x100f6c2 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x110dd2eu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x110dd32u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  cpu.push(0x100f6e5); f_01049cd8(cpu); if cpu.returned_to != 0x100f6e5 { return; }
  { let v: u8 = 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x110de36u32, v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x100f6f1; continue;
 }
 0x100f6f1 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x100f718; continue; }
  pc = 0x100f6f5; continue;
 }
 0x100f6f5 => {
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x100f718; continue; }
  pc = 0x100f6fe; continue;
 }
 0x100f6fe => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x100f718; continue; }
  pc = 0x100f708; continue;
 }
 0x100f708 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.inc8(cpu.r8(0x110de36u32)); cpu.w8(0x110de36u32, v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100f6f1; continue; }
  pc = 0x100f718; continue;
 }
 0x100f718 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110de37u32, v); }
  { let v: u8 = cpu.r8(0x110de36u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x11757b7u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x11757b6u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x100f73a; continue;
 }
 0x100f73a => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0xb0u32.wrapping_add(cpu.eax))); cpu.w8(0xb0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xb0u32.wrapping_add(cpu.eax)), 0xfu8); cpu.w8(0xb0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x100f757; continue; }
  pc = 0x100f757; continue;
 }
 0x100f757 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x100f769; continue; }
  pc = 0x100f760; continue;
 }
 0x100f760 => {
  { cpu.sub8(cpu.r8(0xb0u32.wrapping_add(cpu.eax)), 0x4u8); }
  if (cpu.sf == cpu.of) { pc = 0x100f7ac; continue; }
  pc = 0x100f769; continue;
 }
 0x100f769 => {
  { let v = cpu.r32(0x110dd2eu32); cpu.push(v); }
  { let v = cpu.r32(0x110dd32u32); cpu.push(v); }
  cpu.push(0x100f77a); f_0105f215(cpu); if cpu.returned_to != 0x100f77a { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110dd2eu32).wrapping_add(cpu.eax); cpu.w32(0x110dd2eu32, v); }
  cpu.push(0x100f78b); f_0105f215(cpu); if cpu.returned_to != 0x100f78b { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110dd32u32).wrapping_add(cpu.eax); cpu.w32(0x110dd32u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.push(0x100f79f); f_0100f7c7(cpu); if cpu.returned_to != 0x100f79f { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x110dd32u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x110dd2eu32, v); }
  pc = 0x100f7ac; continue;
 }
 0x100f7ac => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x100f73a; continue; }
  pc = 0x100f7b3; continue;
 }
 0x100f7b3 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x110de37u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x100f7c6; continue;
 }
 0x100f7c6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100f7c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100f7c7); }
 let mut pc: u32 = 0x100f7c7;
 loop { match pc {
 0x100f7c7 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x100f946; continue; }
  pc = 0x100f7d0; continue;
 }
 0x100f7d0 => {
  { cpu.sub8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x100f946; continue; }
  pc = 0x100f7d9; continue;
 }
 0x100f7d9 => {
  { cpu.sub8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x100f946; continue; }
  pc = 0x100f7e2; continue;
 }
 0x100f7e2 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x100f828; continue; }
  pc = 0x100f7e7; continue;
 }
 0x100f7e7 => {
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if cpu.zf { pc = 0x100f828; continue; }
  pc = 0x100f7ec; continue;
 }
 0x100f7ec => {
  { cpu.sub8((cpu.ecx as u8), 0x9u8); }
  if cpu.zf { pc = 0x100f828; continue; }
  pc = 0x100f7f1; continue;
 }
 0x100f7f1 => {
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x100f9d9; continue; }
  pc = 0x100f7fa; continue;
 }
 0x100f7fa => {
  { cpu.sub8((cpu.ecx as u8), 0x6u8); }
  if cpu.zf { pc = 0x100f9d9; continue; }
  pc = 0x100f803; continue;
 }
 0x100f803 => {
  { cpu.sub8((cpu.ecx as u8), 0xau8); }
  if cpu.zf { pc = 0x100f9d9; continue; }
  pc = 0x100f80c; continue;
 }
 0x100f80c => {
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if cpu.zf { pc = 0x100f8b7; continue; }
  pc = 0x100f815; continue;
 }
 0x100f815 => {
  { cpu.sub8((cpu.ecx as u8), 0x7u8); }
  if cpu.zf { pc = 0x100f8b7; continue; }
  pc = 0x100f81e; continue;
 }
 0x100f81e => {
  { cpu.sub8((cpu.ecx as u8), 0xbu8); }
  if cpu.zf { pc = 0x100f8b7; continue; }
  pc = 0x100f827; continue;
 }
 0x100f827 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100f828 => {
  { cpu.sub8(cpu.r8(0x110de37u32), 0x0u8); }
  if !cpu.zf { pc = 0x100f83a; continue; }
  pc = 0x100f831; continue;
 }
 0x100f831 => {
  { cpu.sub8(cpu.r8(0x110db65u32), 0xffu8); }
  if cpu.zf { pc = 0x100f8b6; continue; }
  pc = 0x100f83a; continue;
 }
 0x100f83a => {
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
  { let v: u16 = cpu.r16(0x110dbccu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110dbcau32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x110de37u32), 0x0u8); }
  if cpu.zf { pc = 0x100f89b; continue; }
  pc = 0x100f881; continue;
 }
 0x100f881 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x100fa68; continue; }
  pc = 0x100f88e; continue;
 }
 0x100f88e => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x100fafe; continue; }
  pc = 0x100f89b; continue;
 }
 0x100f89b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x110db65u32); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x100f8b3); f_0100fb44(cpu); if cpu.returned_to != 0x100f8b3 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x100f8b6; continue;
 }
 0x100f8b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100f8b7 => {
  { cpu.sub8(cpu.r8(0x110de37u32), 0x0u8); }
  if !cpu.zf { pc = 0x100f8c9; continue; }
  pc = 0x100f8c0; continue;
 }
 0x100f8c0 => {
  { cpu.sub8(cpu.r8(0x110db66u32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x100f8b6; continue; }
  pc = 0x100f8c9; continue;
 }
 0x100f8c9 => {
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
  { let v: u16 = cpu.r16(0x110db6cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110db6au32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x110de37u32), 0x0u8); }
  if cpu.zf { pc = 0x100f92a; continue; }
  pc = 0x100f910; continue;
 }
 0x100f910 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x100fa68; continue; }
  pc = 0x100f91d; continue;
 }
 0x100f91d => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x100fafe; continue; }
  pc = 0x100f92a; continue;
 }
 0x100f92a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x110db66u32); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x100f942); f_0100fb44(cpu); if cpu.returned_to != 0x100f942 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100f946 => {
  { cpu.sub8(cpu.r8(0x110de37u32), 0x0u8); }
  if !cpu.zf { pc = 0x100f95c; continue; }
  pc = 0x100f94f; continue;
 }
 0x100f94f => {
  { cpu.sub8(cpu.r8(0x110db67u32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x100f8b6; continue; }
  pc = 0x100f95c; continue;
 }
 0x100f95c => {
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
  { let v: u16 = cpu.r16(0x110dc2cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110dc2au32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x110de37u32), 0x0u8); }
  if cpu.zf { pc = 0x100f9bd; continue; }
  pc = 0x100f9a3; continue;
 }
 0x100f9a3 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x100fa68; continue; }
  pc = 0x100f9b0; continue;
 }
 0x100f9b0 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x100fafe; continue; }
  pc = 0x100f9bd; continue;
 }
 0x100f9bd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x110db67u32); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x100f9d5); f_0100fb44(cpu); if cpu.returned_to != 0x100f9d5 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100f9d9 => {
  { cpu.sub8(cpu.r8(0x110de37u32), 0x0u8); }
  if !cpu.zf { pc = 0x100f9ef; continue; }
  pc = 0x100f9e2; continue;
 }
 0x100f9e2 => {
  { cpu.sub8(cpu.r8(0x110db68u32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x100f8b6; continue; }
  pc = 0x100f9ef; continue;
 }
 0x100f9ef => {
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
  { let v: u16 = cpu.r16(0x110dc8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110dc8au32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x110de37u32), 0x0u8); }
  if cpu.zf { pc = 0x100fa4c; continue; }
  pc = 0x100fa36; continue;
 }
 0x100fa36 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x100fa68; continue; }
  pc = 0x100fa3f; continue;
 }
 0x100fa3f => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x100fafe; continue; }
  pc = 0x100fa4c; continue;
 }
 0x100fa4c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x110db68u32); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x100fa64); f_0100fb44(cpu); if cpu.returned_to != 0x100fa64 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100fa68 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x110db69u32, v); }
  cpu.push(0x100fa7f); f_01072d00(cpu); if cpu.returned_to != 0x100fa7f { return; }
  if cpu.cf { pc = 0x100fab2; continue; }
  pc = 0x100fa81; continue;
 }
 0x100fa81 => {
  { cpu.sub8(cpu.r8(0x10f7dc9u32), 0x0u8); }
  if !cpu.zf { pc = 0x100fab2; continue; }
  pc = 0x100fa8a; continue;
 }
 0x100fa8a => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x100faaa; continue; }
  pc = 0x100faa0; continue;
 }
 0x100faa0 => {
  cpu.push(0x100faa5); f_01012780(cpu); if cpu.returned_to != 0x100faa5 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x100faaa; continue;
 }
 0x100faaa => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc9u32, v); }
  pc = 0x100fab2; continue;
 }
 0x100fab2 => {
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x63u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.r32(0x110dd2eu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x110dd32u32); cpu.edx = v; }
  { let v: u8 = 0x1du8; cpu.w8(0x1128d9fu32, v); }
  cpu.push(0x100faf7); f_01049561(cpu); if cpu.returned_to != 0x100faf7 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.poll(); pc = 0x100f8b6; continue;
 }
 0x100fafe => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x110db69u32, v); }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.r32(0x110dd2eu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x110dd32u32); cpu.edx = v; }
  cpu.push(0x100fb3d); f_01049729(cpu); if cpu.returned_to != 0x100fb3d { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.poll(); pc = 0x100f8b6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100fb44(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100fb44); } r_0100fb44(cpu, 0x100fb44); }
pub fn f_0100fc04(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100fc04); } r_0100fb44(cpu, 0x100fc04); }
fn r_0100fb44(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x100fb44 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x110db69u32, v); }
  cpu.push(0x100fb52); f_01072d00(cpu); if cpu.returned_to != 0x100fb52 { return; }
  if cpu.cf { pc = 0x100fbb1; continue; }
  pc = 0x100fb54; continue;
 }
 0x100fb54 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x100fb80; continue; }
  pc = 0x100fb5d; continue;
 }
 0x100fb5d => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x100fb7d; continue; }
  pc = 0x100fb73; continue;
 }
 0x100fb73 => {
  cpu.push(0x100fb78); f_01012780(cpu); if cpu.returned_to != 0x100fb78 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x100fb7d; continue;
 }
 0x100fb7d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x100fbb1; continue;
 }
 0x100fb80 => {
  { cpu.sub8(cpu.r8(0x10f7dc9u32), 0x0u8); }
  if !cpu.zf { pc = 0x100fbb1; continue; }
  pc = 0x100fb89; continue;
 }
 0x100fb89 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x100fba9; continue; }
  pc = 0x100fb9f; continue;
 }
 0x100fb9f => {
  cpu.push(0x100fba4); f_01012780(cpu); if cpu.returned_to != 0x100fba4 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x100fba9; continue;
 }
 0x100fba9 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc9u32, v); }
  pc = 0x100fbb1; continue;
 }
 0x100fbb1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x100fbbe); f_0106426e(cpu); if cpu.returned_to != 0x100fbbe { return; }
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
  { let v: u8 = cpu.r8(0x63u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x100fc46; continue; }
  pc = 0x100fbf4; continue;
 }
 0x100fbf4 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x100fc85; continue; }
  pc = 0x100fc01; continue;
 }
 0x100fc01 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x100fc19; continue; }
  pc = 0x100fc0a; continue;
 }
 0x100fc04 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x100fc19; continue; }
  pc = 0x100fc0a; continue;
 }
 0x100fc0a => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128d9du32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x7u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x100fc19; continue;
 }
 0x100fc19 => {
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x1128d44u32, v); }
  cpu.push(0x100fc29); f_0104a611(cpu); if cpu.returned_to != 0x100fc29 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x1128d9du32, v); }
  { let v: u32 = cpu.r32(0x1128d44u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x100fc3d; continue; }
  pc = 0x100fc3a; continue;
 }
 0x100fc3a => {
  { let v: u8 = cpu.inc8(cpu.r8(0xbu32.wrapping_add(cpu.eax))); cpu.w8(0xbu32.wrapping_add(cpu.eax), v); }
  pc = 0x100fc3d; continue;
 }
 0x100fc3d => {
  { let v: u8 = cpu.r8(0x110db69u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100fc46 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x63u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1du8; cpu.w8(0x1128d9fu32, v); }
  cpu.push(0x100fc82); f_01049561(cpu); if cpu.returned_to != 0x100fc82 { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x100fc3d; continue;
 }
 0x100fc85 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110db69u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x110dd0cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110dd0au32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e93u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.add16(cpu.r16(0x10e3e95u32), (cpu.ebx as u16)); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x100fcca); f_01049729(cpu); if cpu.returned_to != 0x100fcca { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x100fc3d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100fcff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100fcff); } r_0100fcff(cpu, 0x100fcff); }
pub fn f_0100fd0c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100fd0c); } r_0100fcff(cpu, 0x100fd0c); }
fn r_0100fcff(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x100fcff => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xdu32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffff83u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffff83u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x5c6c3cfu32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5]), v); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x100fd1a); f_0100fd68(cpu); if cpu.returned_to != 0x100fd1a { return; }
  if !cpu.cf { pc = 0x100fd22; continue; }
  pc = 0x100fd1c; continue;
 }
 0x100fd0c => {
  { let v: u8 = 0xffu8; cpu.w8(0x110db65u32, v); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x100fd1a); f_0100fd68(cpu); if cpu.returned_to != 0x100fd1a { return; }
  if !cpu.cf { pc = 0x100fd22; continue; }
  pc = 0x100fd1c; continue;
 }
 0x100fd1c => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x110db65u32, v); }
  pc = 0x100fd22; continue;
 }
 0x100fd22 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100fd23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100fd23); }
 let mut pc: u32 = 0x100fd23;
 loop { match pc {
 0x100fd23 => {
  { let v: u8 = 0xffu8; cpu.w8(0x110db66u32, v); }
  { let v: u8 = 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x100fd31); f_0100fd68(cpu); if cpu.returned_to != 0x100fd31 { return; }
  if !cpu.cf { pc = 0x100fd39; continue; }
  pc = 0x100fd33; continue;
 }
 0x100fd33 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x110db66u32, v); }
  pc = 0x100fd39; continue;
 }
 0x100fd39 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100fd3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100fd3a); }
 let mut pc: u32 = 0x100fd3a;
 loop { match pc {
 0x100fd3a => {
  { let v: u8 = 0xffu8; cpu.w8(0x110db67u32, v); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x100fd48); f_0100fd68(cpu); if cpu.returned_to != 0x100fd48 { return; }
  if !cpu.cf { pc = 0x100fd50; continue; }
  pc = 0x100fd4a; continue;
 }
 0x100fd4a => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x110db67u32, v); }
  pc = 0x100fd50; continue;
 }
 0x100fd50 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100fd51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100fd51); }
 let mut pc: u32 = 0x100fd51;
 loop { match pc {
 0x100fd51 => {
  { let v: u8 = 0xffu8; cpu.w8(0x110db68u32, v); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x100fd5f); f_0100fd68(cpu); if cpu.returned_to != 0x100fd5f { return; }
  if !cpu.cf { pc = 0x100fd67; continue; }
  pc = 0x100fd61; continue;
 }
 0x100fd61 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x110db68u32, v); }
  pc = 0x100fd67; continue;
 }
 0x100fd67 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100fd68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100fd68); }
 let mut pc: u32 = 0x100fd68;
 loop { match pc {
 0x100fd68 => {
  cpu.push(0x100fd6d); f_01067515(cpu); if cpu.returned_to != 0x100fd6d { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x100fdd0; continue; }
  pc = 0x100fd72; continue;
 }
 0x100fd72 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x10e3e93u32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3e95u32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x1152e54u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x100fd9c); f_01065050(cpu); if cpu.returned_to != 0x100fd9c { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x100fdaf); f_0104885a(cpu); if cpu.returned_to != 0x100fdaf { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x100fdd0; continue; }
  pc = 0x100fdc0; continue;
 }
 0x100fdc0 => {
  cpu.push(0x100fdc5); f_0108a349(cpu); if cpu.returned_to != 0x100fdc5 { return; }
  if cpu.cf { pc = 0x100fdd0; continue; }
  pc = 0x100fdc7; continue;
 }
 0x100fdc7 => {
  cpu.push(0x100fdcc); f_0108c35e(cpu); if cpu.returned_to != 0x100fdcc { return; }
  if cpu.cf { pc = 0x100fdd0; continue; }
  pc = 0x100fdce; continue;
 }
 0x100fdce => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x100fdd0 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0100fdd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100fdd2); }
 let mut pc: u32 = 0x100fdd2;
 loop { match pc {
 0x100fdd2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  cpu.push(0x100fde5); f_01012f7b(cpu); if cpu.returned_to != 0x100fde5 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11d02f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x11d02dcu32), 0x0u32); }
  if cpu.zf { pc = 0x100fdfc; continue; }
  pc = 0x100fdf9; continue;
 }
 0x100fdf9 => {
  { let v: u32 = cpu.shl32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 1u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x100fdfc; continue;
 }
 0x100fdfc => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x3e8u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1e6u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_0100fe2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100fe2d); }
 let mut pc: u32 = 0x100fe2d;
 loop { match pc {
 0x100fe2d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10e3f13u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x11d02e0u32)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x3e8u32); cpu.edx = v; }
  { let v: u32 = 0x46u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x11d02e4u32)); cpu.edx = v; }
  { let v: u32 = 0x3e8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_0100fe81(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x100fe81); }
 let mut pc: u32 = 0x100fe81;
 loop { match pc {
 0x100fe81 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11d02fcu32), 0x0u32); }
  if cpu.zf { pc = 0x100fea2; continue; }
  pc = 0x100fe98; continue;
 }
 0x100fe98 => {
  cpu.push(0x100fe9d); f_0100fdd2(cpu); if cpu.returned_to != 0x100fe9d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x100feaa; continue;
 }
 0x100fea2 => {
  cpu.push(0x100fea7); f_0100fe2d(cpu); if cpu.returned_to != 0x100fea7 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x100feaa; continue;
 }
 0x100feaa => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_0100feb6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100feb6); } r_0100feb6(cpu, 0x100feb6); }
pub fn f_0100ff02(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100ff02); } r_0100feb6(cpu, 0x100ff02); }
fn r_0100feb6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x100feb6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xa0000u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x100fedd); f_01010954(cpu); if cpu.returned_to != 0x100fedd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x100feed; continue; }
  pc = 0x100fee1; continue;
 }
 0x100fee1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x100ff6c; continue;
 }
 0x100feed => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x100fef5); f_01014856(cpu); if cpu.returned_to != 0x100fef5 { return; }
  cpu.push(0x100fefa); f_01010dde(cpu); if cpu.returned_to != 0x100fefa { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02c8u32, v); }
  cpu.push(0x100ff04); f_01010e25(cpu); if cpu.returned_to != 0x100ff04 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02f4u32, v); }
  { cpu.sub32(cpu.r32(0x11d02f4u32), 0x0u32); }
  if !cpu.zf { pc = 0x100ff1c; continue; }
  pc = 0x100ff12; continue;
 }
 0x100ff02 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02f4u32, v); }
  { cpu.sub32(cpu.r32(0x11d02f4u32), 0x0u32); }
  if !cpu.zf { pc = 0x100ff1c; continue; }
  pc = 0x100ff12; continue;
 }
 0x100ff12 => {
  { let v: u32 = 0x2b11u32; cpu.w32(0x11d02f4u32, v); }
  pc = 0x100ff1c; continue;
 }
 0x100ff1c => {
  cpu.push(0x100ff21); f_01010e47(cpu); if cpu.returned_to != 0x100ff21 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02dcu32, v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x100ff38; continue; }
  pc = 0x100ff2c; continue;
 }
 0x100ff2c => {
  cpu.push(0x100ff31); f_01010e70(cpu); if cpu.returned_to != 0x100ff31 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02ccu32, v); }
  pc = 0x100ff40; continue;
 }
 0x100ff38 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02ccu32, v); }
  pc = 0x100ff40; continue;
 }
 0x100ff40 => {
  { let v: u32 = 0x11d02f0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11d02f8u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11d02e8u32; cpu.ecx = v; }
  { let v: u32 = 0x11d02d0u32; cpu.ebx = v; }
  { let v: u32 = 0x11d02ecu32; cpu.edx = v; }
  { let v: u32 = 0x11d02d8u32; cpu.eax = v; }
  cpu.push(0x100ff65); f_01010f1d(cpu); if cpu.returned_to != 0x100ff65 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x100ff6c; continue;
 }
 0x100ff6c => {
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
pub fn f_0100ff75(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100ff75); } r_0100ff75(cpu, 0x100ff75); }
pub fn f_0100fff1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100fff1); } r_0100ff75(cpu, 0x100fff1); }
pub fn f_0100fff4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x100fff4); } r_0100ff75(cpu, 0x100fff4); }
pub fn f_01010000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010000); } r_0100ff75(cpu, 0x1010000); }
pub fn f_01010001(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010001); } r_0100ff75(cpu, 0x1010001); }
pub fn f_01010004(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010004); } r_0100ff75(cpu, 0x1010004); }
pub fn f_01010008(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010008); } r_0100ff75(cpu, 0x1010008); }
pub fn f_01010014(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010014); } r_0100ff75(cpu, 0x1010014); }
pub fn f_0101001b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101001b); } r_0100ff75(cpu, 0x101001b); }
pub fn f_01010028(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010028); } r_0100ff75(cpu, 0x1010028); }
pub fn f_01010036(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010036); } r_0100ff75(cpu, 0x1010036); }
pub fn f_01010050(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010050); } r_0100ff75(cpu, 0x1010050); }
pub fn f_01010059(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010059); } r_0100ff75(cpu, 0x1010059); }
pub fn f_010100ff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10100ff); } r_0100ff75(cpu, 0x10100ff); }
pub fn f_01010100(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010100); } r_0100ff75(cpu, 0x1010100); }
pub fn f_01010101(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010101); } r_0100ff75(cpu, 0x1010101); }
pub fn f_01010102(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010102); } r_0100ff75(cpu, 0x1010102); }
pub fn f_01010105(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010105); } r_0100ff75(cpu, 0x1010105); }
pub fn f_0101010c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101010c); } r_0100ff75(cpu, 0x101010c); }
pub fn f_01010201(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010201); } r_0100ff75(cpu, 0x1010201); }
pub fn f_01010202(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010202); } r_0100ff75(cpu, 0x1010202); }
pub fn f_01010300(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010300); } r_0100ff75(cpu, 0x1010300); }
pub fn f_01010303(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010303); } r_0100ff75(cpu, 0x1010303); }
pub fn f_01010305(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010305); } r_0100ff75(cpu, 0x1010305); }
fn r_0100ff75(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x100ff75 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x3cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02e4u32, v); }
  cpu.push(0x100ffaa); f_01013a46(cpu); if cpu.returned_to != 0x100ffaa { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x100ffb7; continue; }
  pc = 0x100ffae; continue;
 }
 0x100ffae => {
  cpu.push(0x100ffb3); f_01010e25(cpu); if cpu.returned_to != 0x100ffb3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x100ffb9; continue; }
  pc = 0x100ffb7; continue;
 }
 0x100ffb7 => {
  pc = 0x100ffc2; continue;
 }
 0x100ffb9 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x100ffc9; continue;
 }
 0x100ffc2 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x100ffc9; continue;
 }
 0x100ffc9 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02fcu32, v); }
  { let v: u32 = cpu.r32(0x11d02ccu32); cpu.eax = v; }
  cpu.push(0x100ffdb); f_01012c1c(cpu); if cpu.returned_to != 0x100ffdb { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d02d4u32, v); }
  { cpu.sub32(cpu.r32(0x11d02f8u32), 0x57u32); }
  if !cpu.zf { pc = 0x1010004; continue; }
  pc = 0x100ffee; continue;
 }
 0x100ffee => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = 0x3fu32; cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x1010002); f_0108f030(cpu); if cpu.returned_to != 0x1010002 { return; }
  pc = 0x1010015; continue;
 }
 0x100fff1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x3fu32; cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x1010002); f_0108f030(cpu); if cpu.returned_to != 0x1010002 { return; }
  pc = 0x1010015; continue;
 }
 0x100fff4 => {
  cpu.unimplemented(0x100fff4, "Aas");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x11d0c68u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x11d0c68u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1010002); f_0108f030(cpu); if cpu.returned_to != 0x1010002 { return; }
  pc = 0x1010015; continue;
 }
 0x1010000 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  pc = 0x1010001; continue;
 }
 0x1010001 => {
  { let v: u8 = cpu.add8((cpu.ebx as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.r32(0x300u32.wrapping_add(cpu.ebx)), cpu.edi); cpu.w32(0x300u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x1010015); f_0108f030(cpu); if cpu.returned_to != 0x1010015 { return; }
  pc = 0x1010015; continue;
 }
 0x1010004 => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x1010015); f_0108f030(cpu); if cpu.returned_to != 0x1010015 { return; }
  pc = 0x1010015; continue;
 }
 0x1010008 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.sar8(cpu.r8(0x11d0c68u32.wrapping_add(cpu.eax)), (cpu.ecx & 0xff)); cpu.w8(0x11d0c68u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x1010015); f_0108f030(cpu); if cpu.returned_to != 0x1010015 { return; }
  pc = 0x1010015; continue;
 }
 0x1010014 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 14u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x101001b; continue;
 }
 0x1010015 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x101001c); f_0101042a(cpu); if cpu.returned_to != 0x101001c { return; }
  { let v: u32 = cpu.r32(0x11d02dcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d02f4u32); cpu.eax = v; }
  cpu.push(0x101002c); f_01012ccb(cpu); if cpu.returned_to != 0x101002c { return; }
  { let v: u32 = cpu.r32(0x10e3f13u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02e0u32, v); }
  pc = 0x1010036; continue;
 }
 0x101001b => {
  { let v: u8 = cpu.r8(0x1d02dc15u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x1d02dc15u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x11d02f4u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.esp); cpu.w32(0x11d02f4u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x101002c); f_01012ccb(cpu); if cpu.returned_to != 0x101002c { return; }
  { let v: u32 = cpu.r32(0x10e3f13u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02e0u32, v); }
  pc = 0x1010036; continue;
 }
 0x1010028 => {
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10e3f13u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x10e3f13u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02e0u32, v); }
  pc = 0x1010036; continue;
 }
 0x1010036 => {
  { let v: u32 = cpu.r32(0x11d02c8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101003e; continue;
 }
 0x101003e => {
  cpu.push(0x1010043); f_010104a4(cpu); if cpu.returned_to != 0x1010043 { return; }
  cpu.push(0x1010048); f_01010fa3(cpu); if cpu.returned_to != 0x1010048 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1010059; continue; }
  pc = 0x101004c; continue;
 }
 0x101004c => {
  cpu.push(0x1010051); f_01011070(cpu); if cpu.returned_to != 0x1010051 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10103dc; continue;
 }
 0x1010050 => {
  { let v: u8 = cpu.r8(0x83e9fc45u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x83e9fc45u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x45f7d045u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x45f7d045u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.rol8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))), 1u32); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x1010123; continue; }
  pc = 0x101006e; continue;
 }
 0x1010059 => {
  cpu.push(0x101005e); f_010110a2(cpu); if cpu.returned_to != 0x101005e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)), 0x4u32); }
  if cpu.zf { pc = 0x1010123; continue; }
  pc = 0x101006e; continue;
 }
 0x101006e => {
  { let v: u32 = cpu.r32(0x11d0fb0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11d0fa8u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0fa4u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10100bb; continue; }
  pc = 0x1010081; continue;
 }
 0x1010081 => {
  { let v: u32 = cpu.r32(0x11d0fa8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d0fb8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0fb0u32); cpu.eax = v; }
  cpu.push(0x1010097); f_0108ef05(cpu); if cpu.returned_to != 0x1010097 { return; }
  { let v: u32 = cpu.r32(0x11d0fa8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0fb0u32).wrapping_add(cpu.eax); cpu.w32(0x11d0fb0u32, v); }
  { let v: u32 = cpu.r32(0x11d0fb0u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0fa4u32)); }
  if !cpu.zf { pc = 0x10100b9; continue; }
  pc = 0x10100af; continue;
 }
 0x10100af => {
  { let v: u32 = cpu.r32(0x11d0fb4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0fb0u32, v); }
  pc = 0x10100b9; continue;
 }
 0x10100b9 => {
  pc = 0x101010d; continue;
 }
 0x10100bb => {
  { let v: u32 = cpu.r32(0x11d0fa4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11d0fb0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11d0fa8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d0fb8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0fb0u32); cpu.eax = v; }
  cpu.push(0x10100e7); f_0108ef05(cpu); if cpu.returned_to != 0x10100e7 { return; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d0fb8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0fb4u32); cpu.eax = v; }
  cpu.push(0x10100fd); f_0108ef05(cpu); if cpu.returned_to != 0x10100fd { return; }
  { let v: u32 = cpu.r32(0x11d0fb4u32); cpu.eax = v; }
  pc = 0x1010102; continue;
 }
 0x10100ff => {
  cpu.unimplemented(0x10100ff, "Reservednop");
  pc = 0x1010102; continue;
 }
 0x1010100 => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0xf0558b01u32); cpu.eax = v; }
  pc = 0x1010105; continue;
 }
 0x1010101 => {
  { let v: u32 = cpu.r32(0xc201f055u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ecx); cpu.w32(0xc201f055u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11d0fb0u32, v); }
  pc = 0x101010d; continue;
 }
 0x1010102 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x1010105; continue;
 }
 0x1010105 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11d0fb0u32, v); }
  pc = 0x101010d; continue;
 }
 0x101010c => {
  { let v: u32 = cpu.add32(cpu.r32(0x7400d47du32.wrapping_add(cpu.ebx)), cpu.eax); cpu.w32(0x7400d47du32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x1d0fa415u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0x1d0fa415u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x11d0fb4u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.esp); cpu.w32(0x11d0fb4u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x1010123); f_01012d21(cpu); if cpu.returned_to != 0x1010123 { return; }
  pc = 0x1010123; continue;
 }
 0x101010d => {
  { cpu.sub32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1010123; continue; }
  pc = 0x1010113; continue;
 }
 0x1010113 => {
  { let v: u32 = cpu.r32(0x11d0fa4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0fb4u32); cpu.eax = v; }
  cpu.push(0x1010123); f_01012d21(cpu); if cpu.returned_to != 0x1010123 { return; }
  pc = 0x1010123; continue;
 }
 0x1010123 => {
  { cpu.and32(cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if cpu.zf { pc = 0x1010160; continue; }
  pc = 0x101012c; continue;
 }
 0x101012c => {
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x1010140); f_01014921(cpu); if cpu.returned_to != 0x1010140 { return; }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1010159; continue; }
  pc = 0x1010146; continue;
 }
 0x1010146 => {
  { let v: u32 = cpu.r32(0x11d02d8u32); cpu.eax = v; }
  cpu.push(0x1010150); f_0101042a(cpu); if cpu.returned_to != 0x1010150 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010160; continue;
 }
 0x1010159 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x1010160); f_0101042a(cpu); if cpu.returned_to != 0x1010160 { return; }
  pc = 0x1010160; continue;
 }
 0x1010160 => {
  { cpu.sub32(cpu.r32(0x11d02d0u32), 0x0u32); }
  if !cpu.zf { pc = 0x1010172; continue; }
  pc = 0x1010169; continue;
 }
 0x1010169 => {
  { cpu.sub32(cpu.r32(0x11d02ecu32), 0x0u32); }
  if !cpu.zf { pc = 0x1010177; continue; }
  pc = 0x1010172; continue;
 }
 0x1010172 => {
  pc = 0x10101fd; continue;
 }
 0x1010177 => {
  cpu.push(0x101017c); f_01010db9(cpu); if cpu.returned_to != 0x101017c { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x1010183); f_01010d97(cpu); if cpu.returned_to != 0x1010183 { return; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d02ecu32)); }
  if !cpu.zf { pc = 0x10101cd; continue; }
  pc = 0x1010193; continue;
 }
 0x1010193 => {
  { cpu.sub32(cpu.r32(0x11d02f0u32), 0x57u32); }
  if !cpu.zf { pc = 0x10101b2; continue; }
  pc = 0x101019c; continue;
 }
 0x101019c => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = 0x3fu32; cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x10101b0); f_0108f030(cpu); if cpu.returned_to != 0x10101b0 { return; }
  pc = 0x10101c3; continue;
 }
 0x10101b2 => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x10101c3); f_0108f030(cpu); if cpu.returned_to != 0x10101c3 { return; }
  pc = 0x10101c3; continue;
 }
 0x10101c3 => {
  { let v: u32 = cpu.r32(0x11d02ecu32); cpu.eax = v; }
  cpu.push(0x10101cd); f_0101042a(cpu); if cpu.returned_to != 0x10101cd { return; }
  pc = 0x10101cd; continue;
 }
 0x10101cd => {
  { cpu.sub32(cpu.r32(0x11d02e8u32), 0x0u32); }
  if cpu.zf { pc = 0x10101e1; continue; }
  pc = 0x10101d6; continue;
 }
 0x10101d6 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d02ecu32)); }
  if (cpu.sf != cpu.of) { pc = 0x10101e3; continue; }
  pc = 0x10101e1; continue;
 }
 0x10101e1 => {
  pc = 0x10101fd; continue;
 }
 0x10101e3 => {
  { let v: u32 = cpu.r32(0x11d02ccu32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0x11d02ecu32); cpu.idiv32(v); }
  cpu.push(0x10101fd); f_01012c1c(cpu); if cpu.returned_to != 0x10101fd { return; }
  pc = 0x10101fd; continue;
 }
 0x10101fd => {
  cpu.push(0x1010202); f_0105ebed(cpu); if cpu.returned_to != 0x1010202 { return; }
  pc = 0x1010202; continue;
 }
 0x1010201 => {
  { let v: u8 = cpu.r8(0x6581dc45u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x6581dc45u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.st(7) / cpu.st(0); cpu.set_st(7, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x751bdc7du32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x751bdc7du32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x1010212; continue;
 }
 0x1010202 => {
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)) & 0xffu32; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x1bu32); }
  if !cpu.zf { pc = 0x101025d; continue; }
  pc = 0x1010212; continue;
 }
 0x1010212 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d02d4u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x11d02e8u32, v); }
  { let v: u32 = 0x7u32; cpu.w32(0x11d02d8u32, v); }
  { let v: u32 = 0x7u32; cpu.w32(0x11d02ecu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x11d02d0u32, v); }
  { let v: u32 = 0x30u32; cpu.w32(0x11d02f0u32, v); }
  { let v: u32 = 0x30u32; cpu.w32(0x11d02f8u32, v); }
  pc = 0x101025d; continue;
 }
 0x101025d => {
  { cpu.and32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x1010270; continue; }
  pc = 0x1010266; continue;
 }
 0x1010266 => {
  cpu.push(0x101026b); f_0100fe81(cpu); if cpu.returned_to != 0x101026b { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x1010272; continue; }
  pc = 0x1010270; continue;
 }
 0x1010270 => {
  pc = 0x101027e; continue;
 }
 0x1010272 => {
  { let v: u32 = 0x9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10103dc; continue;
 }
 0x101027e => {
  { cpu.and32(cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if cpu.zf { pc = 0x1010290; continue; }
  pc = 0x1010287; continue;
 }
 0x1010287 => {
  { cpu.and32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x4u32); }
  if !cpu.zf { pc = 0x10102a2; continue; }
  pc = 0x1010290; continue;
 }
 0x1010290 => {
  cpu.push(0x1010295); f_0100fe81(cpu); if cpu.returned_to != 0x1010295 { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1010290; continue; }
  pc = 0x101029a; continue;
 }
 0x101029a => {
  { let v: u32 = cpu.r32(0x11d02c8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10102a2; continue;
 }
 0x10102a2 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10102ae); f_010110d4(cpu); if cpu.returned_to != 0x10102ae { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10102bc; continue; }
  pc = 0x10102b2; continue;
 }
 0x10102b2 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x101003e; continue; }
  pc = 0x10102bc; continue;
 }
 0x10102bc => {
  { cpu.sub32(cpu.r32(0x11d02ecu32), 0x0u32); }
  if !cpu.zf { pc = 0x1010304; continue; }
  pc = 0x10102c5; continue;
 }
 0x10102c5 => {
  { cpu.sub32(cpu.r32(0x11d02f0u32), 0x57u32); }
  if !cpu.zf { pc = 0x10102e4; continue; }
  pc = 0x10102ce; continue;
 }
 0x10102ce => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = 0x3fu32; cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x10102e2); f_0108f030(cpu); if cpu.returned_to != 0x10102e2 { return; }
  pc = 0x10102f5; continue;
 }
 0x10102e4 => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x10102f5); f_0108f030(cpu); if cpu.returned_to != 0x10102f5 { return; }
  pc = 0x10102f5; continue;
 }
 0x10102f5 => {
  { let v: u32 = cpu.r32(0x11d02ecu32); cpu.eax = v; }
  cpu.push(0x10102ff); f_0101042a(cpu); if cpu.returned_to != 0x10102ff { return; }
  pc = 0x10103c1; continue;
 }
 0x1010300 => {
  { let v: u32 = 0x83000000u32; cpu.ebp = v; }
  pc = 0x1010305; continue;
 }
 0x1010303 => {
  { let v: u8 = cpu.r8(0x1d02d03du32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x1d02d03du32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x10103c1; continue; }
  pc = 0x1010311; continue;
 }
 0x1010304 => {
  { cpu.sub32(cpu.r32(0x11d02d0u32), 0x0u32); }
  if cpu.zf { pc = 0x10103c1; continue; }
  pc = 0x1010311; continue;
 }
 0x1010305 => {
  { cpu.sub32(cpu.eax, 0x11d02d0u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(cpu.r8(0x83000000u32.wrapping_add(cpu.eax)), ((cpu.edx >> 8) as u8)); }
  { cpu.sub32(cpu.eax, 0x11d02f0u32); }
  { let v = cpu.edi; cpu.push(v); }
  if !cpu.zf { pc = 0x1010330; continue; }
  pc = 0x101031a; continue;
 }
 0x1010311 => {
  { cpu.sub32(cpu.r32(0x11d02f0u32), 0x57u32); }
  if !cpu.zf { pc = 0x1010330; continue; }
  pc = 0x101031a; continue;
 }
 0x101031a => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = 0x3fu32; cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x101032e); f_0108f030(cpu); if cpu.returned_to != 0x101032e { return; }
  pc = 0x1010341; continue;
 }
 0x1010330 => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x1010341); f_0108f030(cpu); if cpu.returned_to != 0x1010341 { return; }
  pc = 0x1010341; continue;
 }
 0x1010341 => {
  { let v: u32 = cpu.r32(0x11d02ecu32); cpu.eax = v; }
  cpu.push(0x101034b); f_0101042a(cpu); if cpu.returned_to != 0x101034b { return; }
  { let v: u32 = cpu.r32(0x11d02ecu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010354; continue;
 }
 0x1010354 => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1010367; continue; }
  pc = 0x101035a; continue;
 }
 0x101035a => {
  pc = 0x10103c1; continue;
 }
 0x101035f => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1010354; continue;
 }
 0x1010367 => {
  cpu.push(0x101036c); f_010104a4(cpu); if cpu.returned_to != 0x101036c { return; }
  { cpu.sub32(cpu.r32(0x11d02e8u32), 0x0u32); }
  if cpu.zf { pc = 0x101038f; continue; }
  pc = 0x1010375; continue;
 }
 0x1010375 => {
  { let v: u32 = cpu.r32(0x11d02ccu32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0x11d02ecu32); cpu.idiv32(v); }
  cpu.push(0x101038f); f_01012c1c(cpu); if cpu.returned_to != 0x101038f { return; }
  pc = 0x101038f; continue;
 }
 0x101038f => {
  { cpu.and32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x10103a2; continue; }
  pc = 0x1010398; continue;
 }
 0x1010398 => {
  cpu.push(0x101039d); f_0100fe81(cpu); if cpu.returned_to != 0x101039d { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x10103a4; continue; }
  pc = 0x10103a2; continue;
 }
 0x10103a2 => {
  pc = 0x10103ad; continue;
 }
 0x10103a4 => {
  { let v: u32 = 0x9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10103dc; continue;
 }
 0x10103ad => {
  cpu.push(0x10103b2); f_0100fe81(cpu); if cpu.returned_to != 0x10103b2 { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10103ad; continue; }
  pc = 0x10103b7; continue;
 }
 0x10103b7 => {
  { let v: u32 = cpu.r32(0x11d02c8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101035f; continue;
 }
 0x10103c1 => {
  cpu.push(0x10103c6); f_01012e61(cpu); if cpu.returned_to != 0x10103c6 { return; }
  cpu.push(0x10103cb); f_01072682(cpu); if cpu.returned_to != 0x10103cb { return; }
  cpu.push(0x10103d0); f_010103e7(cpu); if cpu.returned_to != 0x10103d0 { return; }
  cpu.push(0x10103d5); f_01010c9b(cpu); if cpu.returned_to != 0x10103d5 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10103dc; continue;
 }
 0x10103dc => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010106(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010106); }
 let mut pc: u32 = 0x1010106;
 loop { match pc {
 0x1010106 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(5517);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010103e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10103e7); }
 let mut pc: u32 = 0x10103e7;
 loop { match pc {
 0x10103e7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10103fc; continue;
 }
 0x10103fc => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x14u32); }
  if (cpu.sf != cpu.of) { pc = 0x101040c; continue; }
  pc = 0x1010402; continue;
 }
 0x1010402 => {
  pc = 0x1010421; continue;
 }
 0x1010404 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10103fc; continue;
 }
 0x101040c => {
  { let v: u32 = cpu.r32(0x10e3f13u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010414; continue;
 }
 0x1010414 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e3f13u32)); }
  if cpu.zf { cpu.poll(); pc = 0x1010414; continue; }
  pc = 0x101041f; continue;
 }
 0x101041f => {
  cpu.poll(); pc = 0x1010404; continue;
 }
 0x1010421 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_0101042a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101042a); } r_0101042a(cpu, 0x101042a); }
pub fn f_01010488(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010488); } r_0101042a(cpu, 0x1010488); }
pub fn f_0101049c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101049c); } r_0101042a(cpu, 0x101049c); }
fn r_0101042a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x101042a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1010461; continue; }
  pc = 0x1010441; continue;
 }
 0x1010441 => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.edx = v; }
  { let v: u32 = 0x11d0668u32; cpu.eax = v; }
  cpu.push(0x1010455); f_0108ef05(cpu); if cpu.returned_to != 0x1010455 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0f90u32, v); }
  pc = 0x101049b; continue;
 }
 0x1010461 => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = 0x11d0668u32; cpu.edx = v; }
  { let v: u32 = 0x11d0300u32; cpu.eax = v; }
  cpu.push(0x1010475); f_0108ef05(cpu); if cpu.returned_to != 0x1010475 { return; }
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.edx = v; }
  { let v: u32 = 0x11d0968u32; cpu.eax = v; }
  cpu.push(0x1010489); f_0108ef05(cpu); if cpu.returned_to != 0x1010489 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0fa0u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0f90u32, v); }
  pc = 0x101049b; continue;
 }
 0x1010488 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x11d0fa0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0f90u32, v); }
  pc = 0x101049b; continue;
 }
 0x101049b => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101049c => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_010104a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10104a4); } r_010104a4(cpu, 0x10104a4); }
pub fn f_01010504(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010504); } r_010104a4(cpu, 0x1010504); }
fn r_010104a4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10104a4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11d0f90u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x101056e; continue; }
  pc = 0x10104bf; continue;
 }
 0x10104bf => {
  { cpu.sub32(cpu.r32(0x11d0f90u32), 0x0u32); }
  if !cpu.zf { pc = 0x10104e1; continue; }
  pc = 0x10104c8; continue;
 }
 0x10104c8 => {
  { let v: u32 = 0x11d0668u32; cpu.eax = v; }
  cpu.push(0x10104d2); f_010110f8(cpu); if cpu.returned_to != 0x10104d2 { return; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x11d0f90u32, v); }
  pc = 0x101056e; continue;
 }
 0x10104e1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10104e8; continue;
 }
 0x10104e8 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x300u32); }
  if (cpu.sf != cpu.of) { pc = 0x10104fb; continue; }
  pc = 0x10104f1; continue;
 }
 0x10104f1 => {
  pc = 0x1010547; continue;
 }
 0x10104f3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10104e8; continue;
 }
 0x10104fb => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d0968u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x11d0300u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x11d0fa0u32)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0x11d0f90u32); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x11d0300u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11d0668u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x10104f3; continue;
 }
 0x1010504 => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0xf0558b01u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d0300u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x11d0fa0u32)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0x11d0f90u32); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x11d0300u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11d0668u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x10104f3; continue;
 }
 0x1010547 => {
  { let v: u32 = 0x11d0668u32; cpu.eax = v; }
  cpu.push(0x1010551); f_010110f8(cpu); if cpu.returned_to != 0x1010551 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11d0fa0u32)); cpu.w32(0x11d0fa0u32, v); }
  { let v: u32 = cpu.r32(0x11d0fa0u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0f90u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x101056e; continue; }
  pc = 0x1010564; continue;
 }
 0x1010564 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x11d0f90u32, v); }
  pc = 0x101056e; continue;
 }
 0x101056e => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010577(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010577); } r_01010577(cpu, 0x1010577); }
pub fn f_01010600(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010600); } r_01010577(cpu, 0x1010600); }
pub fn f_01010606(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010606); } r_01010577(cpu, 0x1010606); }
fn r_01010577(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1010577 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11d0f98u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0f94u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x110de4cu32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10105f1; continue; }
  pc = 0x10105b6; continue;
 }
 0x10105b6 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x10105ea; continue; }
  pc = 0x10105c5; continue;
 }
 0x10105c5 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0fb8u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0fa8u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x110de4cu32) | 0x4u8; cpu.w8(0x110de4cu32, v); }
  pc = 0x10105f1; continue;
 }
 0x10105ea => {
  { let v: u8 = cpu.r8(0x110de4fu32) | 0x80u8; cpu.w8(0x110de4fu32, v); }
  pc = 0x10105f1; continue;
 }
 0x10105f1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0xcu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x101061a; continue; }
  pc = 0x10105fa; continue;
 }
 0x10105fa => {
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x101060c); f_0108ef05(cpu); if cpu.returned_to != 0x101060c { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(0x300u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x110de4cu32) | 0x2u8; cpu.w8(0x110de4cu32, v); }
  pc = 0x101061a; continue;
 }
 0x1010600 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.unimplemented(0x1010601, "Hlt");
  { let v: u32 = 0x11d0c68u32; cpu.eax = v; }
  cpu.push(0x101060c); f_0108ef05(cpu); if cpu.returned_to != 0x101060c { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(0x300u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x110de4cu32) | 0x2u8; cpu.w8(0x110de4cu32, v); }
  pc = 0x101061a; continue;
 }
 0x1010606 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.push(0x101060e); cpu.call(0x46820615); if cpu.returned_to != 0x101060e { return; }
  cpu.unimplemented(0x101060e, "Hlt");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x110de4cu32) | 0x2u8; cpu.w8(0x110de4cu32, v); }
  pc = 0x101061a; continue;
 }
 0x101061a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10106fe; continue; }
  pc = 0x1010627; continue;
 }
 0x1010627 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.cf { pc = 0x101064b; continue; }
  pc = 0x1010636; continue;
 }
 0x1010636 => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if (cpu.cf || cpu.zf) { pc = 0x101068b; continue; }
  pc = 0x101063c; continue;
 }
 0x101063c => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x10106c0; continue; }
  pc = 0x1010646; continue;
 }
 0x1010646 => {
  pc = 0x10106f7; continue;
 }
 0x101064b => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x10106f7; continue; }
  pc = 0x1010655; continue;
 }
 0x1010655 => {
  { cpu.sub32(cpu.r32(0x110de44u32), 0x0u32); }
  if cpu.zf { pc = 0x101067c; continue; }
  pc = 0x101065e; continue;
 }
 0x101065e => {
  { let v: u32 = cpu.r32(0x110de38u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x110de3cu32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x110de44u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x101067c); f_01011bba(cpu); if cpu.returned_to != 0x101067c { return; }
  pc = 0x101067c; continue;
 }
 0x101067c => {
  { let v: u32 = cpu.r32(0x110de38u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x110de3cu32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101068b; continue;
 }
 0x101068b => {
  { cpu.sub32(cpu.r32(0x110de44u32), 0x0u32); }
  if cpu.zf { pc = 0x10106ae; continue; }
  pc = 0x1010694; continue;
 }
 0x1010694 => {
  { let v: u32 = cpu.r32(0x110de3cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x110de38u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110de44u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10106ae); f_01011bd8(cpu); if cpu.returned_to != 0x10106ae { return; }
  pc = 0x10106ae; continue;
 }
 0x10106ae => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x110de4cu32), 0x1u8); cpu.w8(0x110de4cu32, v); }
  pc = 0x10106fe; continue;
 }
 0x10106c0 => {
  { cpu.sub32(cpu.r32(0x110de44u32), 0x0u32); }
  if cpu.zf { pc = 0x10106e8; continue; }
  pc = 0x10106c9; continue;
 }
 0x10106c9 => {
  { let v: u32 = cpu.r32(0x110de38u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x110de3cu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x110de44u32); cpu.eax = v; }
  cpu.push(0x10106e8); f_01011b93(cpu); if cpu.returned_to != 0x10106e8 { return; }
  pc = 0x10106e8; continue;
 }
 0x10106e8 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x110de4cu32), 0x1u8); cpu.w8(0x110de4cu32, v); }
  pc = 0x10106fe; continue;
 }
 0x10106f7 => {
  { let v: u8 = cpu.or8(cpu.r8(0x110de4fu32), 0x80u8); cpu.w8(0x110de4fu32, v); }
  pc = 0x10106fe; continue;
 }
 0x10106fe => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010707(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010707); }
 let mut pc: u32 = 0x1010707;
 loop { match pc {
 0x1010707 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x110de40u32); cpu.eax = v; }
  cpu.push(0x101072a); f_0108e170(cpu); if cpu.returned_to != 0x101072a { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1010738; continue; }
  pc = 0x101072f; continue;
 }
 0x101072f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010748; continue;
 }
 0x1010738 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x11d0f9cu32), cpu.eax); cpu.w32(0x11d0f9cu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010748; continue;
 }
 0x1010748 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010753(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010753); }
 let mut pc: u32 = 0x1010753;
 loop { match pc {
 0x1010753 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0f68u32, v); }
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0f8cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0f80u32, v); }
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
pub fn f_01010786(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010786); }
 let mut pc: u32 = 0x1010786;
 loop { match pc {
 0x1010786 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0f6cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10107aa; continue; }
  pc = 0x10107a2; continue;
 }
 0x10107a2 => {
  { let v: u32 = cpu.r32(0x11d0f6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10107aa; continue;
 }
 0x10107aa => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x10107bc; continue; }
  pc = 0x10107b0; continue;
 }
 0x10107b0 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010868; continue;
 }
 0x10107bc => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11d0f80u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0f78u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10107d9; continue; }
  pc = 0x10107cd; continue;
 }
 0x10107cd => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010868; continue;
 }
 0x10107d9 => {
  { let v: u32 = cpu.r32(0x11d0f8cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11d0f78u32)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x1010805; continue; }
  pc = 0x10107f2; continue;
 }
 0x10107f2 => {
  { let v: u32 = 0x7u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010868; continue;
 }
 0x1010805 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0f8cu32); cpu.eax = v; }
  cpu.push(0x1010812); f_01010707(cpu); if cpu.returned_to != 0x1010812 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1010829; continue; }
  pc = 0x1010816; continue;
 }
 0x1010816 => {
  { let v: u32 = 0x5u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010868; continue;
 }
 0x1010829 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0f8cu32).wrapping_add(cpu.eax); cpu.w32(0x11d0f8cu32, v); }
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11d0f78u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0f8cu32)); }
  if !cpu.zf { pc = 0x101084f; continue; }
  pc = 0x1010845; continue;
 }
 0x1010845 => {
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0f8cu32, v); }
  pc = 0x101084f; continue;
 }
 0x101084f => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0f80u32).wrapping_add(cpu.eax); cpu.w32(0x11d0f80u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x11d0f6cu32), cpu.eax); cpu.w32(0x11d0f6cu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010868; continue;
 }
 0x1010868 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010874(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010874); } r_01010874(cpu, 0x1010874); }
pub fn f_010108dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10108dd); } r_01010874(cpu, 0x10108dd); }
pub fn f_01010900(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010900); } r_01010874(cpu, 0x1010900); }
fn r_01010874(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1010874 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0f80u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10108a6; continue; }
  pc = 0x1010890; continue;
 }
 0x1010890 => {
  { let v: u32 = 0xau32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010948; continue;
 }
 0x10108a6 => {
  { let v: u32 = cpu.r32(0x11d0f68u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11d0f78u32)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10108db; continue; }
  pc = 0x10108bf; continue;
 }
 0x10108bf => {
  { let v: u32 = cpu.r32(0x11d0f68u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0f68u32).wrapping_add(cpu.eax); cpu.w32(0x11d0f68u32, v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x11d0f80u32), cpu.eax); cpu.w32(0x11d0f80u32, v); }
  pc = 0x1010942; continue;
 }
 0x10108db => {
  { let v: u32 = cpu.r32(0x11d0f74u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11d0f78u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11d0f68u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010900; continue;
 }
 0x10108dd => {
  cpu.unimplemented(0x10108dd, "Reservednop");
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11d0f78u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11d0f68u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010900; continue;
 }
 0x1010900 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d0f68u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0f74u32); cpu.eax = v; }
  cpu.push(0x1010913); f_0108ef05(cpu); if cpu.returned_to != 0x1010913 { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0f74u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.push(0x1010929); f_0108ef05(cpu); if cpu.returned_to != 0x1010929 { return; }
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11d0f68u32, v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x11d0f80u32), cpu.eax); cpu.w32(0x11d0f80u32, v); }
  pc = 0x1010942; continue;
 }
 0x1010942 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010948; continue;
 }
 0x1010948 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010954(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010954); } r_01010954(cpu, 0x1010954); }
pub fn f_01010a00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010a00); } r_01010954(cpu, 0x1010a00); }
pub fn f_01010a0a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010a0a); } r_01010954(cpu, 0x1010a0a); }
pub fn f_01010c01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010c01); } r_01010954(cpu, 0x1010c01); }
pub fn f_01010c0c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010c0c); } r_01010954(cpu, 0x1010c0c); }
pub fn f_01010c68(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010c68); } r_01010954(cpu, 0x1010c68); }
fn r_01010954(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1010954 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0f9cu32, v); }
  { let v = 0x200u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1010981); f_0108defe(cpu); if cpu.returned_to != 0x1010981 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110de40u32, v); }
  { cpu.sub32(cpu.r32(0x110de40u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10109a8; continue; }
  pc = 0x1010992; continue;
 }
 0x1010992 => {
  { let v: u32 = 0x2u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c91; continue;
 }
 0x10109a8 => {
  { let v: u32 = 0x68u32; cpu.edx = v; }
  { let v: u32 = 0x11d0600u32; cpu.eax = v; }
  cpu.push(0x10109b7); f_01010707(cpu); if cpu.returned_to != 0x10109b7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10109c4; continue; }
  pc = 0x10109bb; continue;
 }
 0x10109bb => {
  { cpu.sub8(cpu.r8(0x11d0600u32), 0x4au8); }
  if cpu.zf { pc = 0x10109c6; continue; }
  pc = 0x10109c4; continue;
 }
 0x10109c4 => {
  pc = 0x10109cf; continue;
 }
 0x10109c6 => {
  { cpu.sub8(cpu.r8(0x11d0601u32), 0x56u8); }
  if cpu.zf { pc = 0x10109ea; continue; }
  pc = 0x10109cf; continue;
 }
 0x10109cf => {
  cpu.push(0x10109d4); f_01010c9b(cpu); if cpu.returned_to != 0x10109d4 { return; }
  { let v: u32 = 0x3u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c91; continue;
 }
 0x10109ea => {
  { let v: u32 = cpu.r32(0x11d0652u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0f7cu32, v); }
  { let v: u32 = cpu.r32(0x11d0652u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  pc = 0x1010a05; continue;
 }
 0x1010a00 => {
  { cpu.cf = false; }
  { let v: u8 = cpu.adc8((cpu.ecx as u8), (cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1010a09; continue; }
  pc = 0x1010a05; continue;
 }
 0x1010a05 => {
  cpu.push(0x1010a0a); f_01011d6d(cpu); if cpu.returned_to != 0x1010a0a { return; }
  pc = 0x1010a0a; continue;
 }
 0x1010a09 => {
  { let v: u8 = cpu.r8(0x11d0f94u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x11d0f94u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x11d0f94u32), 0x0u32); }
  if !cpu.zf { pc = 0x1010a33; continue; }
  pc = 0x1010a18; continue;
 }
 0x1010a0a => {
  { let v: u32 = cpu.eax; cpu.w32(0x11d0f94u32, v); }
  { cpu.sub32(cpu.r32(0x11d0f94u32), 0x0u32); }
  if !cpu.zf { pc = 0x1010a33; continue; }
  pc = 0x1010a18; continue;
 }
 0x1010a18 => {
  cpu.push(0x1010a1d); f_01010c9b(cpu); if cpu.returned_to != 0x1010a1d { return; }
  { let v: u32 = 0x7u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c91; continue;
 }
 0x1010a33 => {
  { let v: u32 = cpu.r32(0x11d0652u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0f94u32); cpu.eax = v; }
  cpu.push(0x1010a49); f_01010707(cpu); if cpu.returned_to != 0x1010a49 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1010a68; continue; }
  pc = 0x1010a4d; continue;
 }
 0x1010a4d => {
  cpu.push(0x1010a52); f_01010c9b(cpu); if cpu.returned_to != 0x1010a52 { return; }
  { let v: u32 = 0x5u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c91; continue;
 }
 0x1010a68 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0f70u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0f6cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0f78u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010a92; continue;
 }
 0x1010a92 => {
  { let v: u32 = cpu.r32(0x11d0652u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1010aac; continue; }
  pc = 0x1010a9f; continue;
 }
 0x1010a9f => {
  pc = 0x1010b14; continue;
 }
 0x1010aa4 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1010a92; continue;
 }
 0x1010aac => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0f94u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0f6cu32).wrapping_add(cpu.eax); cpu.w32(0x11d0f6cu32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0f94u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)).wrapping_sub(cpu.eax); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x11d0f78u32), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1010aed; continue; }
  pc = 0x1010add; continue;
 }
 0x1010add => {
  { let v: u32 = cpu.r32(0x11d0f78u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0f70u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1010aef; continue; }
  pc = 0x1010aed; continue;
 }
 0x1010aed => {
  pc = 0x1010af7; continue;
 }
 0x1010aef => {
  { let v: u32 = cpu.r32(0x11d0f70u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010af7; continue;
 }
 0x1010af7 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1010b12; continue; }
  pc = 0x1010afd; continue;
 }
 0x1010afd => {
  { let v: u32 = cpu.r32(0x11d0f70u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11d0f70u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0f78u32).wrapping_add(cpu.eax); cpu.w32(0x11d0f78u32, v); }
  cpu.poll(); pc = 0x1010af7; continue;
 }
 0x1010b12 => {
  cpu.poll(); pc = 0x1010aa4; continue;
 }
 0x1010b14 => {
  { let v: u32 = cpu.r32(0x11d0f70u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0f78u32).wrapping_add(cpu.eax); cpu.w32(0x11d0f78u32, v); }
  { let v: u32 = cpu.r32(0x11d0f78u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0f7cu32).wrapping_add(cpu.eax); cpu.w32(0x11d0f7cu32, v); }
  { let v: u32 = cpu.r32(0x11d0f78u32); cpu.eax = v; }
  cpu.push(0x1010b36); f_01011d6d(cpu); if cpu.returned_to != 0x1010b36 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0f88u32, v); }
  { cpu.sub32(cpu.r32(0x11d0f88u32), 0x0u32); }
  if !cpu.zf { pc = 0x1010b5f; continue; }
  pc = 0x1010b44; continue;
 }
 0x1010b44 => {
  cpu.push(0x1010b49); f_01010c9b(cpu); if cpu.returned_to != 0x1010b49 { return; }
  { let v: u32 = 0x7u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c91; continue;
 }
 0x1010b5f => {
  cpu.push(0x1010b64); f_01010753(cpu); if cpu.returned_to != 0x1010b64 { return; }
  { let v: u16 = cpu.r16(0x11d0f7eu32).wrapping_add(0x4u16); cpu.w16(0x11d0f7eu32, v); }
  { let v: u32 = 0x40000u32; cpu.eax = v; }
  cpu.push(0x1010b76); f_01011d6d(cpu); if cpu.returned_to != 0x1010b76 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0fb4u32, v); }
  { cpu.sub32(cpu.r32(0x11d0fb4u32), 0x0u32); }
  if !cpu.zf { pc = 0x1010b9f; continue; }
  pc = 0x1010b84; continue;
 }
 0x1010b84 => {
  cpu.push(0x1010b89); f_01010c9b(cpu); if cpu.returned_to != 0x1010b89 { return; }
  { let v: u32 = 0x7u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c91; continue;
 }
 0x1010b9f => {
  { let v: u32 = cpu.r32(0x11d0fb4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x40000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0fa4u32, v); }
  { let v: u32 = cpu.r32(0x11d0fb4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0facu32, v); }
  { let v: u32 = cpu.r32(0x11d0fb4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0fb0u32, v); }
  { let v: u32 = cpu.r32(0x11d0658u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0f7cu32).wrapping_add(cpu.eax); cpu.w32(0x11d0f7cu32, v); }
  { let v: u32 = cpu.r32(0x11d0658u32); cpu.eax = v; }
  cpu.push(0x1010bd7); f_01011d6d(cpu); if cpu.returned_to != 0x1010bd7 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0f74u32, v); }
  { cpu.sub32(cpu.r32(0x11d0f74u32), 0x0u32); }
  if !cpu.zf { pc = 0x1010c00; continue; }
  pc = 0x1010be5; continue;
 }
 0x1010be5 => {
  cpu.push(0x1010bea); f_01010c9b(cpu); if cpu.returned_to != 0x1010bea { return; }
  { let v: u32 = 0x7u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c91; continue;
 }
 0x1010c00 => {
  { let v: u32 = cpu.r32(0x11d0f78u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0f6cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1010c1b; continue; }
  pc = 0x1010c13; continue;
 }
 0x1010c01 => {
  if cpu.sf { pc = 0x1010c12; continue; }
  pc = 0x1010c03; continue;
 }
 0x1010c03 => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0xf8458901u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0f6cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1010c1b; continue; }
  pc = 0x1010c13; continue;
 }
 0x1010c0c => {
  { let v: u32 = cpu.add32(cpu.eax, 0x11d0f6cu32); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1010c1b; continue; }
  pc = 0x1010c13; continue;
 }
 0x1010c12 => {
  { let v: u8 = cpu.r8(0x11d0f6cu32.wrapping_add(cpu.ecx)) | ((cpu.eax >> 8) as u8); cpu.w8(0x11d0f6cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c1b; continue;
 }
 0x1010c13 => {
  { let v: u32 = cpu.r32(0x11d0f6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c1b; continue;
 }
 0x1010c1b => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1010c23); f_01010786(cpu); if cpu.returned_to != 0x1010c23 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1010c35; continue; }
  pc = 0x1010c27; continue;
 }
 0x1010c27 => {
  cpu.push(0x1010c2c); f_01010c9b(cpu); if cpu.returned_to != 0x1010c2c { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c91; continue;
 }
 0x1010c35 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11d0f98u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0f84u32, v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110de44u32, v); }
  { cpu.sub32(cpu.r32(0x110de40u32), 0x0u32); }
  if cpu.zf { pc = 0x1010c63; continue; }
  pc = 0x1010c5a; continue;
 }
 0x1010c5a => {
  { cpu.sub32(cpu.r32(0x110de44u32), 0x0u32); }
  if !cpu.zf { pc = 0x1010c65; continue; }
  pc = 0x1010c63; continue;
 }
 0x1010c63 => {
  pc = 0x1010c8a; continue;
 }
 0x1010c65 => {
  cpu.push(0x1010c6a); f_01011b65(cpu); if cpu.returned_to != 0x1010c6a { return; }
  { let v: u32 = cpu.r32(0x11d064eu32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d0650u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x110de44u32); cpu.eax = v; }
  cpu.push(0x1010c8a); f_01011b93(cpu); if cpu.returned_to != 0x1010c8a { return; }
  pc = 0x1010c8a; continue;
 }
 0x1010c68 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x11d064eu32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d0650u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x110de44u32); cpu.eax = v; }
  cpu.push(0x1010c8a); f_01011b93(cpu); if cpu.returned_to != 0x1010c8a { return; }
  pc = 0x1010c8a; continue;
 }
 0x1010c8a => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010c91; continue;
 }
 0x1010c91 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01010c9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010c9b); }
 let mut pc: u32 = 0x1010c9b;
 loop { match pc {
 0x1010c9b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11d0fb4u32), 0x0u32); }
  if cpu.zf { pc = 0x1010cbc; continue; }
  pc = 0x1010cb2; continue;
 }
 0x1010cb2 => {
  { let v: u32 = cpu.r32(0x11d0fb4u32); cpu.eax = v; }
  cpu.push(0x1010cbc); f_01011f13(cpu); if cpu.returned_to != 0x1010cbc { return; }
  pc = 0x1010cbc; continue;
 }
 0x1010cbc => {
  { cpu.sub32(cpu.r32(0x11d0f74u32), 0x0u32); }
  if cpu.zf { pc = 0x1010ccf; continue; }
  pc = 0x1010cc5; continue;
 }
 0x1010cc5 => {
  { let v: u32 = cpu.r32(0x11d0f74u32); cpu.eax = v; }
  cpu.push(0x1010ccf); f_01011f13(cpu); if cpu.returned_to != 0x1010ccf { return; }
  pc = 0x1010ccf; continue;
 }
 0x1010ccf => {
  { cpu.sub32(cpu.r32(0x11d0f88u32), 0x0u32); }
  if cpu.zf { pc = 0x1010ce2; continue; }
  pc = 0x1010cd8; continue;
 }
 0x1010cd8 => {
  { let v: u32 = cpu.r32(0x11d0f88u32); cpu.eax = v; }
  cpu.push(0x1010ce2); f_01011f13(cpu); if cpu.returned_to != 0x1010ce2 { return; }
  pc = 0x1010ce2; continue;
 }
 0x1010ce2 => {
  { cpu.sub32(cpu.r32(0x11d0f94u32), 0x0u32); }
  if cpu.zf { pc = 0x1010cf5; continue; }
  pc = 0x1010ceb; continue;
 }
 0x1010ceb => {
  { let v: u32 = cpu.r32(0x11d0f94u32); cpu.eax = v; }
  cpu.push(0x1010cf5); f_01011f13(cpu); if cpu.returned_to != 0x1010cf5 { return; }
  pc = 0x1010cf5; continue;
 }
 0x1010cf5 => {
  { cpu.sub32(cpu.r32(0x110de40u32), 0x0u32); }
  if cpu.zf { pc = 0x1010d08; continue; }
  pc = 0x1010cfe; continue;
 }
 0x1010cfe => {
  { let v: u32 = cpu.r32(0x110de40u32); cpu.eax = v; }
  cpu.push(0x1010d08); f_0108e26b(cpu); if cpu.returned_to != 0x1010d08 { return; }
  pc = 0x1010d08; continue;
 }
 0x1010d08 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11d0fb4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0f74u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0f88u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0f94u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x110de40u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010d4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010d4d); }
 let mut pc: u32 = 0x1010d4d;
 loop { match pc {
 0x1010d4d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11d064eu32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010d97(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010d97); }
 let mut pc: u32 = 0x1010d97;
 loop { match pc {
 0x1010d97 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11d0f98u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010db9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010db9); }
 let mut pc: u32 = 0x1010db9;
 loop { match pc {
 0x1010db9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11d0652u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010dde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010dde); }
 let mut pc: u32 = 0x1010dde;
 loop { match pc {
 0x1010dde => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11d0654u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010e03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010e03); }
 let mut pc: u32 = 0x1010e03;
 loop { match pc {
 0x1010e03 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11d0658u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010e25(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010e25); } r_01010e25(cpu, 0x1010e25); }
pub fn f_01010e3d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010e3d); } r_01010e25(cpu, 0x1010e3d); }
pub fn f_01010e3e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010e3e); } r_01010e25(cpu, 0x1010e3e); }
pub fn f_01010e3f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010e3f); } r_01010e25(cpu, 0x1010e3f); }
fn r_01010e25(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1010e25 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11d065cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1010e3e; continue;
 }
 0x1010e3d => {
  { cpu.df = false; }
  pc = 0x1010e3e; continue;
 }
 0x1010e3e => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1010e3f => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01010e47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010e47); }
 let mut pc: u32 = 0x1010e47;
 loop { match pc {
 0x1010e47 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x11d0660u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010e70); }
 let mut pc: u32 = 0x1010e70;
 loop { match pc {
 0x1010e70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d0660u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1010e9a; continue; }
  pc = 0x1010e93; continue;
 }
 0x1010e93 => {
  { let v: u32 = 0x7fu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1010e9a; continue;
 }
 0x1010e9a => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010ee9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010ee9); } r_01010ee9(cpu, 0x1010ee9); }
pub fn f_01010ef2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010ef2); } r_01010ee9(cpu, 0x1010ef2); }
pub fn f_01010ef3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010ef3); } r_01010ee9(cpu, 0x1010ef3); }
fn r_01010ee9(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1010ee9 => {
  if cpu.sf { pc = 0x1010efa; continue; }
  pc = 0x1010eeb; continue;
 }
 0x1010eeb => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0xfc458901u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1010ef3; continue;
 }
 0x1010ef2 => {
  { cpu.df = false; }
  pc = 0x1010ef3; continue;
 }
 0x1010ef3 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x1010efa; continue;
 }
 0x1010efa => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01010efc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1010efc); }
 let mut pc: u32 = 0x1010efc;
 loop { match pc {
 0x1010efc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x11d0600u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01010f1d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010f1d); } r_01010f1d(cpu, 0x1010f1d); }
pub fn f_01010f7d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010f7d); } r_01010f1d(cpu, 0x1010f7d); }
pub fn f_01010f7e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010f7e); } r_01010f1d(cpu, 0x1010f7e); }
pub fn f_01010f7f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010f7f); } r_01010f1d(cpu, 0x1010f7f); }
fn r_01010f1d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1010f1d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x11d0667u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d0667u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x7u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d0667u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d0667u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  pc = 0x1010f7f; continue;
 }
 0x1010f7d => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { pc = 0x1010f80; continue; }
  pc = 0x1010f7f; continue;
 }
 0x1010f7e => {
  { let v: u32 = cpu.add32(cpu.r32(0x8ad23110u32.wrapping_add(cpu.ecx)), cpu.ecx); cpu.w32(0x8ad23110u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x11d0602u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x11d0603u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x1010f7f => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x11d0602u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x11d0603u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x1010f80 => {
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.ror8(cpu.r8(0x1d060215u32.wrapping_add(cpu.edx)), (cpu.ecx & 0xff)); cpu.w8(0x1d060215u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10891045u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ecx); cpu.w32(0x10891045u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x11d0603u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01010fa3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010fa3); } r_01010fa3(cpu, 0x1010fa3); }
pub fn f_01010fcd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010fcd); } r_01010fa3(cpu, 0x1010fcd); }
pub fn f_01010fd0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010fd0); } r_01010fa3(cpu, 0x1010fd0); }
pub fn f_01010fd3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010fd3); } r_01010fa3(cpu, 0x1010fd3); }
pub fn f_01010fd4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1010fd4); } r_01010fa3(cpu, 0x1010fd4); }
pub fn f_01011065(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011065); } r_01010fa3(cpu, 0x1011065); }
fn r_01010fa3(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1010fa3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x110de40u32), 0x0u32); }
  if !cpu.zf { pc = 0x1010fd0; continue; }
  pc = 0x1010fba; continue;
 }
 0x1010fba => {
  { let v: u32 = 0x4u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011064; continue;
 }
 0x1010fcd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1d0f843du32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x1d0f843du32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x1010fe5; continue; }
  pc = 0x1010fd9; continue;
 }
 0x1010fd0 => {
  { cpu.sub32(cpu.r32(0x11d0f84u32), 0x0u32); }
  if cpu.zf { pc = 0x1010fe5; continue; }
  pc = 0x1010fd9; continue;
 }
 0x1010fd3 => {
  cpu.unimplemented(0x1010fd3, "Reservednop");
  { let v: u8 = cpu.r8(0xffffffc7u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xffffffc7u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1011064; continue;
 }
 0x1010fd4 => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0xc740001u32); cpu.eax = v; }
  pc = 0x1010fd9; continue;
 }
 0x1010fd9 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011064; continue;
 }
 0x1010fe5 => {
  { let v: u32 = cpu.r32(0x11d0f70u32); cpu.eax = v; }
  cpu.push(0x1010fef); f_01010786(cpu); if cpu.returned_to != 0x1010fef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1011001; continue; }
  pc = 0x1010ff3; continue;
 }
 0x1010ff3 => {
  cpu.push(0x1010ff8); f_01010c9b(cpu); if cpu.returned_to != 0x1010ff8 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011064; continue;
 }
 0x1011001 => {
  { let v: u32 = cpu.r32(0x11d0f98u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d0f94u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x101101e); f_01010874(cpu); if cpu.returned_to != 0x101101e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1011035; continue; }
  pc = 0x1011027; continue;
 }
 0x1011027 => {
  cpu.push(0x101102c); f_01010c9b(cpu); if cpu.returned_to != 0x101102c { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011064; continue;
 }
 0x1011035 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x101103d); f_01010577(cpu); if cpu.returned_to != 0x101103d { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11d0f98u32)); cpu.w32(0x11d0f98u32, v); }
  { let v: u32 = cpu.r32(0x11d0652u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d0f98u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x101105d; continue; }
  pc = 0x1011053; continue;
 }
 0x1011053 => {
  { let v: u32 = 0x1u32; cpu.w32(0x11d0f84u32, v); }
  pc = 0x101105d; continue;
 }
 0x101105d => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011064; continue;
 }
 0x1011064 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011065 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01011070(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011070); } r_01011070(cpu, 0x1011070); }
pub fn f_01011099(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011099); } r_01011070(cpu, 0x1011099); }
fn r_01011070(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1011070 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x110de48u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x110de48u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x1011099; continue;
 }
 0x1011099 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_010110a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10110a2); } r_010110a2(cpu, 0x10110a2); }
pub fn f_010110a6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10110a6); } r_010110a2(cpu, 0x10110a6); }
pub fn f_010110b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10110b3); } r_010110a2(cpu, 0x10110b3); }
pub fn f_010110b4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10110b4); } r_010110a2(cpu, 0x10110b4); }
pub fn f_010110b5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10110b5); } r_010110a2(cpu, 0x10110b5); }
fn r_010110a2(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10110a2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x10110a6; continue;
 }
 0x10110a6 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x110de4cu32); cpu.eax = v; }
  pc = 0x10110b5; continue;
 }
 0x10110b3 => {
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10110b5; continue;
 }
 0x10110b4 => {
  { let v: u32 = cpu.r32(0x5c7f845u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.ecx); cpu.w32(0x5c7f845u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let a = cpu.st(0); let b = (cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as f64); cpu.fcom(a, b); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x4589f845u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0x4589f845u32.wrapping_add(cpu.ebx), v); }
  { cpu.df = false; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10110b5 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x110de4cu32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_010110d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10110d4); } r_010110d4(cpu, 0x10110d4); }
pub fn f_010110d6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10110d6); } r_010110d4(cpu, 0x10110d6); }
pub fn f_010110ed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10110ed); } r_010110d4(cpu, 0x10110ed); }
pub fn f_010110f0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10110f0); } r_010110d4(cpu, 0x10110f0); }
fn r_010110d4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10110d4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x10110d6; continue;
 }
 0x10110d6 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11d0f84u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x10110ed; continue;
 }
 0x10110ed => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x10110f0; continue;
 }
 0x10110f0 => {
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
pub fn f_010110f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10110f8); }
 let mut pc: u32 = 0x10110f8;
 loop { match pc {
 0x10110f8 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x10110ff; continue;
 }
 0x10110ff => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10110ff; continue; }
  pc = 0x1011104; continue;
 }
 0x1011104 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x1011104; continue; }
  pc = 0x1011109; continue;
 }
 0x1011109 => {
  { let v: u16 = 0x3c8u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  pc = 0x1011117; continue;
 }
 0x1011117 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1011117; continue; }
  pc = 0x1011128; continue;
 }
 0x1011128 => {
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3c0u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x31u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011139(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011139); }
 let mut pc: u32 = 0x1011139;
 loop { match pc {
 0x1011139 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x101116b; continue; }
  pc = 0x1011145; continue;
 }
 0x1011145 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x101116b; continue; }
  pc = 0x101114b; continue;
 }
 0x101114b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x101116b; continue; }
  pc = 0x1011151; continue;
 }
 0x1011151 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x101116b; continue; }
  pc = 0x1011157; continue;
 }
 0x1011157 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x101116b; continue; }
  pc = 0x101115d; continue;
 }
 0x101115d => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x101116b; continue; }
  pc = 0x1011163; continue;
 }
 0x1011163 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.cf { pc = 0x101116b; continue; }
  pc = 0x1011169; continue;
 }
 0x1011169 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x101116b; continue;
 }
 0x101116b => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101116e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101116e); }
 let mut pc: u32 = 0x101116e;
 loop { match pc {
 0x101116e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1011174); f_01011139(cpu); if cpu.returned_to != 0x1011174 { return; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebx = v; }
  cpu.push(0x101117c); f_01011139(cpu); if cpu.returned_to != 0x101117c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011182(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011182); }
 let mut pc: u32 = 0x1011182;
 loop { match pc {
 0x1011182 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1011188); f_01011139(cpu); if cpu.returned_to != 0x1011188 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x101118f); f_01011139(cpu); if cpu.returned_to != 0x101118f { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  cpu.push(0x1011199); f_01011139(cpu); if cpu.returned_to != 0x1011199 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x10111a0); f_01011139(cpu); if cpu.returned_to != 0x10111a0 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
