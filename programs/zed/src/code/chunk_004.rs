#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_01023c4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1023c4c); }
 let mut pc: u32 = 0x1023c4c;
 loop { match pc {
 0x1023c4c => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.cmps(1, 0); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x2u32.wrapping_add(cpu.ebx), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u32 = cpu.r32(0x43u32.wrapping_add(cpu.esi)).wrapping_add(cpu.esi); cpu.w32(0x43u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[2] as u32); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x2u32.wrapping_add(cpu.edx)), 0x1u8); }
  { cpu.port_out(0x42u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1023c84, "Salc");
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x42u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cmps(1, 0); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x2u32.wrapping_add(cpu.edx)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u32 = cpu.r32(0x42u32.wrapping_add(cpu.esi)).wrapping_add(cpu.esi); cpu.w32(0x42u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[2] as u32); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x2u32.wrapping_add(cpu.ecx)), 0x1u8); }
  { cpu.port_out(0x41u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1023cc4, "Salc");
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x2u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x41u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cmps(1, 0); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x2u32.wrapping_add(cpu.ecx)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x2u32.wrapping_add(cpu.ecx), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u32 = cpu.r32(0x41u32.wrapping_add(cpu.esi)).wrapping_add(cpu.esi); cpu.w32(0x41u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[2] as u32); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x2u32.wrapping_add(cpu.eax)), 0x1u8); }
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1023d04, "Salc");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x40u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cmps(1, 0); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x2u32.wrapping_add(cpu.eax)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x2u32.wrapping_add(cpu.eax), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esi)).wrapping_add(cpu.esi); cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[2] as u32); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.idiv8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x3fu16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1023d44, "Salc");
  cpu.unimplemented(0x1023d45, "Aas");
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x1023d48); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01023da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1023da4); }
 let mut pc: u32 = 0x1023da4;
 loop { match pc {
 0x1023da4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x2u32.wrapping_add(cpu.esi)), cpu.eax); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sbb32(cpu.r32(0x2u32.wrapping_add(cpu.esi)), cpu.eax); cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = true; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0xda03400e); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01024681(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024681); } r_01024681(cpu, 0x1024681); }
pub fn f_010248d3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10248d3); } r_01024681(cpu, 0x10248d3); }
pub fn f_010248df(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10248df); } r_01024681(cpu, 0x10248df); }
pub fn f_010248eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10248eb); } r_01024681(cpu, 0x10248eb); }
pub fn f_010248f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10248f7); } r_01024681(cpu, 0x10248f7); }
pub fn f_01024903(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024903); } r_01024681(cpu, 0x1024903); }
pub fn f_0102490f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102490f); } r_01024681(cpu, 0x102490f); }
pub fn f_0102491b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102491b); } r_01024681(cpu, 0x102491b); }
pub fn f_01024927(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024927); } r_01024681(cpu, 0x1024927); }
pub fn f_01024933(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024933); } r_01024681(cpu, 0x1024933); }
pub fn f_0102493f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102493f); } r_01024681(cpu, 0x102493f); }
pub fn f_0102494b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102494b); } r_01024681(cpu, 0x102494b); }
pub fn f_01024957(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024957); } r_01024681(cpu, 0x1024957); }
pub fn f_01024963(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024963); } r_01024681(cpu, 0x1024963); }
pub fn f_0102496f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102496f); } r_01024681(cpu, 0x102496f); }
pub fn f_0102497b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102497b); } r_01024681(cpu, 0x102497b); }
pub fn f_01024987(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024987); } r_01024681(cpu, 0x1024987); }
pub fn f_01024993(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024993); } r_01024681(cpu, 0x1024993); }
pub fn f_0102499f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102499f); } r_01024681(cpu, 0x102499f); }
pub fn f_010249ab(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10249ab); } r_01024681(cpu, 0x10249ab); }
pub fn f_010249b7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10249b7); } r_01024681(cpu, 0x10249b7); }
pub fn f_010249c3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10249c3); } r_01024681(cpu, 0x10249c3); }
pub fn f_010249cf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10249cf); } r_01024681(cpu, 0x10249cf); }
pub fn f_010249db(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10249db); } r_01024681(cpu, 0x10249db); }
pub fn f_010249e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10249e7); } r_01024681(cpu, 0x10249e7); }
pub fn f_010249f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10249f3); } r_01024681(cpu, 0x10249f3); }
pub fn f_010249ff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10249ff); } r_01024681(cpu, 0x10249ff); }
pub fn f_01024a0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a0b); } r_01024681(cpu, 0x1024a0b); }
pub fn f_01024a17(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a17); } r_01024681(cpu, 0x1024a17); }
pub fn f_01024a23(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a23); } r_01024681(cpu, 0x1024a23); }
pub fn f_01024a2f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a2f); } r_01024681(cpu, 0x1024a2f); }
pub fn f_01024a3b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a3b); } r_01024681(cpu, 0x1024a3b); }
pub fn f_01024a47(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a47); } r_01024681(cpu, 0x1024a47); }
pub fn f_01024a53(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a53); } r_01024681(cpu, 0x1024a53); }
pub fn f_01024a5f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a5f); } r_01024681(cpu, 0x1024a5f); }
pub fn f_01024a6b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a6b); } r_01024681(cpu, 0x1024a6b); }
pub fn f_01024a77(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a77); } r_01024681(cpu, 0x1024a77); }
pub fn f_01024a83(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a83); } r_01024681(cpu, 0x1024a83); }
pub fn f_01024a8f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a8f); } r_01024681(cpu, 0x1024a8f); }
pub fn f_01024a9b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024a9b); } r_01024681(cpu, 0x1024a9b); }
pub fn f_01024aa7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024aa7); } r_01024681(cpu, 0x1024aa7); }
pub fn f_01024ab3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024ab3); } r_01024681(cpu, 0x1024ab3); }
pub fn f_01024abf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024abf); } r_01024681(cpu, 0x1024abf); }
pub fn f_01024acb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024acb); } r_01024681(cpu, 0x1024acb); }
pub fn f_01024ad7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024ad7); } r_01024681(cpu, 0x1024ad7); }
pub fn f_01024ae3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024ae3); } r_01024681(cpu, 0x1024ae3); }
pub fn f_01024aef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024aef); } r_01024681(cpu, 0x1024aef); }
pub fn f_01024afb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024afb); } r_01024681(cpu, 0x1024afb); }
pub fn f_01024b07(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b07); } r_01024681(cpu, 0x1024b07); }
pub fn f_01024b13(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b13); } r_01024681(cpu, 0x1024b13); }
pub fn f_01024b1f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b1f); } r_01024681(cpu, 0x1024b1f); }
pub fn f_01024b2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b2b); } r_01024681(cpu, 0x1024b2b); }
pub fn f_01024b37(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b37); } r_01024681(cpu, 0x1024b37); }
pub fn f_01024b43(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b43); } r_01024681(cpu, 0x1024b43); }
pub fn f_01024b4f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b4f); } r_01024681(cpu, 0x1024b4f); }
pub fn f_01024b5b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b5b); } r_01024681(cpu, 0x1024b5b); }
pub fn f_01024b67(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b67); } r_01024681(cpu, 0x1024b67); }
pub fn f_01024b73(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b73); } r_01024681(cpu, 0x1024b73); }
pub fn f_01024b7f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b7f); } r_01024681(cpu, 0x1024b7f); }
pub fn f_01024b8b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b8b); } r_01024681(cpu, 0x1024b8b); }
pub fn f_01024b97(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024b97); } r_01024681(cpu, 0x1024b97); }
pub fn f_01024ba3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024ba3); } r_01024681(cpu, 0x1024ba3); }
pub fn f_01024baf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024baf); } r_01024681(cpu, 0x1024baf); }
pub fn f_01024bbb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024bbb); } r_01024681(cpu, 0x1024bbb); }
pub fn f_01024bc7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024bc7); } r_01024681(cpu, 0x1024bc7); }
pub fn f_01024bd3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024bd3); } r_01024681(cpu, 0x1024bd3); }
pub fn f_01024bdf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024bdf); } r_01024681(cpu, 0x1024bdf); }
pub fn f_01024beb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024beb); } r_01024681(cpu, 0x1024beb); }
pub fn f_01024bf7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024bf7); } r_01024681(cpu, 0x1024bf7); }
pub fn f_01024c03(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c03); } r_01024681(cpu, 0x1024c03); }
pub fn f_01024c0f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c0f); } r_01024681(cpu, 0x1024c0f); }
pub fn f_01024c1b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c1b); } r_01024681(cpu, 0x1024c1b); }
pub fn f_01024c27(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c27); } r_01024681(cpu, 0x1024c27); }
pub fn f_01024c33(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c33); } r_01024681(cpu, 0x1024c33); }
pub fn f_01024c3f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c3f); } r_01024681(cpu, 0x1024c3f); }
pub fn f_01024c4b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c4b); } r_01024681(cpu, 0x1024c4b); }
pub fn f_01024c57(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c57); } r_01024681(cpu, 0x1024c57); }
pub fn f_01024c63(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c63); } r_01024681(cpu, 0x1024c63); }
pub fn f_01024c6f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c6f); } r_01024681(cpu, 0x1024c6f); }
pub fn f_01024c7b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c7b); } r_01024681(cpu, 0x1024c7b); }
pub fn f_01024c87(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c87); } r_01024681(cpu, 0x1024c87); }
pub fn f_01024c93(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c93); } r_01024681(cpu, 0x1024c93); }
pub fn f_01024c9f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024c9f); } r_01024681(cpu, 0x1024c9f); }
pub fn f_01024cab(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024cab); } r_01024681(cpu, 0x1024cab); }
pub fn f_01024cb7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024cb7); } r_01024681(cpu, 0x1024cb7); }
pub fn f_01024cc3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024cc3); } r_01024681(cpu, 0x1024cc3); }
pub fn f_01024ccf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024ccf); } r_01024681(cpu, 0x1024ccf); }
pub fn f_01024cde(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024cde); } r_01024681(cpu, 0x1024cde); }
pub fn f_01024cea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024cea); } r_01024681(cpu, 0x1024cea); }
pub fn f_01024cf6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024cf6); } r_01024681(cpu, 0x1024cf6); }
pub fn f_01024d02(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d02); } r_01024681(cpu, 0x1024d02); }
pub fn f_01024d0e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d0e); } r_01024681(cpu, 0x1024d0e); }
pub fn f_01024d1a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d1a); } r_01024681(cpu, 0x1024d1a); }
pub fn f_01024d26(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d26); } r_01024681(cpu, 0x1024d26); }
pub fn f_01024d32(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d32); } r_01024681(cpu, 0x1024d32); }
pub fn f_01024d3e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d3e); } r_01024681(cpu, 0x1024d3e); }
pub fn f_01024d4a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d4a); } r_01024681(cpu, 0x1024d4a); }
pub fn f_01024d56(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d56); } r_01024681(cpu, 0x1024d56); }
pub fn f_01024d62(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d62); } r_01024681(cpu, 0x1024d62); }
pub fn f_01024d6e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d6e); } r_01024681(cpu, 0x1024d6e); }
pub fn f_01024d7a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d7a); } r_01024681(cpu, 0x1024d7a); }
pub fn f_01024d86(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d86); } r_01024681(cpu, 0x1024d86); }
pub fn f_01024d92(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d92); } r_01024681(cpu, 0x1024d92); }
pub fn f_01024d9e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024d9e); } r_01024681(cpu, 0x1024d9e); }
pub fn f_01024daa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024daa); } r_01024681(cpu, 0x1024daa); }
pub fn f_01024db6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024db6); } r_01024681(cpu, 0x1024db6); }
pub fn f_01024dc2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024dc2); } r_01024681(cpu, 0x1024dc2); }
pub fn f_01024dce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024dce); } r_01024681(cpu, 0x1024dce); }
pub fn f_01024dda(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024dda); } r_01024681(cpu, 0x1024dda); }
pub fn f_01024de6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024de6); } r_01024681(cpu, 0x1024de6); }
pub fn f_01024df2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024df2); } r_01024681(cpu, 0x1024df2); }
pub fn f_01024dfe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024dfe); } r_01024681(cpu, 0x1024dfe); }
pub fn f_01024e0a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e0a); } r_01024681(cpu, 0x1024e0a); }
pub fn f_01024e16(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e16); } r_01024681(cpu, 0x1024e16); }
pub fn f_01024e22(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e22); } r_01024681(cpu, 0x1024e22); }
pub fn f_01024e2e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e2e); } r_01024681(cpu, 0x1024e2e); }
pub fn f_01024e3a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e3a); } r_01024681(cpu, 0x1024e3a); }
pub fn f_01024e46(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e46); } r_01024681(cpu, 0x1024e46); }
pub fn f_01024e52(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e52); } r_01024681(cpu, 0x1024e52); }
pub fn f_01024e5e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e5e); } r_01024681(cpu, 0x1024e5e); }
pub fn f_01024e6a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e6a); } r_01024681(cpu, 0x1024e6a); }
pub fn f_01024e76(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e76); } r_01024681(cpu, 0x1024e76); }
pub fn f_01024e82(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e82); } r_01024681(cpu, 0x1024e82); }
pub fn f_01024e8e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e8e); } r_01024681(cpu, 0x1024e8e); }
pub fn f_01024e9a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024e9a); } r_01024681(cpu, 0x1024e9a); }
pub fn f_01024ea6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024ea6); } r_01024681(cpu, 0x1024ea6); }
pub fn f_01024eb2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024eb2); } r_01024681(cpu, 0x1024eb2); }
pub fn f_01024ebe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024ebe); } r_01024681(cpu, 0x1024ebe); }
pub fn f_01024eca(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024eca); } r_01024681(cpu, 0x1024eca); }
pub fn f_01024ed6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1024ed6); } r_01024681(cpu, 0x1024ed6); }
fn r_01024681(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1024681 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1024687); f_01026a9c(cpu); if cpu.returned_to != 0x1024687 { return; }
  { let v: u32 = cpu.r32(0x1119f2cu32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x111a95cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a964u32)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a974u32); cpu.ecx = v; }
  { let v: u32 = 0x111a350u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x111a970u32); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10246c1; continue; }
  pc = 0x10246b5; continue;
 }
 0x10246b5 => {
  { let v: u32 = cpu.r32(0x10246c9u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  pc = 0x10248d1; continue;
 }
 0x10246c1 => {
  { let v: u32 = cpu.r32(0x1024821u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  pc = 0x10248d1; continue;
 }
 0x10248d1 => {
  cpu.poll(); pc = cpu.edx; continue;
 }
 0x10248d3 => {
  { let v: u8 = cpu.r8(0x55u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x55u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x55u32.wrapping_add(cpu.edi), v); }
  pc = 0x10248df; continue;
 }
 0x10248df => {
  { let v: u8 = cpu.r8(0x54u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x54u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x54u32.wrapping_add(cpu.edi), v); }
  pc = 0x10248eb; continue;
 }
 0x10248eb => {
  { let v: u8 = cpu.r8(0x53u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x53u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x53u32.wrapping_add(cpu.edi), v); }
  pc = 0x10248f7; continue;
 }
 0x10248f7 => {
  { let v: u8 = cpu.r8(0x52u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024903; continue;
 }
 0x1024903 => {
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x51u32.wrapping_add(cpu.edi), v); }
  pc = 0x102490f; continue;
 }
 0x102490f => {
  { let v: u8 = cpu.r8(0x50u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x50u32.wrapping_add(cpu.edi), v); }
  pc = 0x102491b; continue;
 }
 0x102491b => {
  { let v: u8 = cpu.r8(0x4fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4fu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4fu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024927; continue;
 }
 0x1024927 => {
  { let v: u8 = cpu.r8(0x4eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4eu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024933; continue;
 }
 0x1024933 => {
  { let v: u8 = cpu.r8(0x4du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4du32.wrapping_add(cpu.edi), v); }
  pc = 0x102493f; continue;
 }
 0x102493f => {
  { let v: u8 = cpu.r8(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4cu32.wrapping_add(cpu.edi), v); }
  pc = 0x102494b; continue;
 }
 0x102494b => {
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4bu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024957; continue;
 }
 0x1024957 => {
  { let v: u8 = cpu.r8(0x4au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4au32.wrapping_add(cpu.edi), v); }
  pc = 0x1024963; continue;
 }
 0x1024963 => {
  { let v: u8 = cpu.r8(0x49u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x49u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x49u32.wrapping_add(cpu.edi), v); }
  pc = 0x102496f; continue;
 }
 0x102496f => {
  { let v: u8 = cpu.r8(0x48u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x48u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x48u32.wrapping_add(cpu.edi), v); }
  pc = 0x102497b; continue;
 }
 0x102497b => {
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024987; continue;
 }
 0x1024987 => {
  { let v: u8 = cpu.r8(0x46u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x46u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x46u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024993; continue;
 }
 0x1024993 => {
  { let v: u8 = cpu.r8(0x45u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x45u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x45u32.wrapping_add(cpu.edi), v); }
  pc = 0x102499f; continue;
 }
 0x102499f => {
  { let v: u8 = cpu.r8(0x44u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x44u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.edi), v); }
  pc = 0x10249ab; continue;
 }
 0x10249ab => {
  { let v: u8 = cpu.r8(0x43u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x43u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x43u32.wrapping_add(cpu.edi), v); }
  pc = 0x10249b7; continue;
 }
 0x10249b7 => {
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  pc = 0x10249c3; continue;
 }
 0x10249c3 => {
  { let v: u8 = cpu.r8(0x41u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x41u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x41u32.wrapping_add(cpu.edi), v); }
  pc = 0x10249cf; continue;
 }
 0x10249cf => {
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x40u32.wrapping_add(cpu.edi), v); }
  pc = 0x10249db; continue;
 }
 0x10249db => {
  { let v: u8 = cpu.r8(0x3fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x3fu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3fu32.wrapping_add(cpu.edi), v); }
  pc = 0x10249e7; continue;
 }
 0x10249e7 => {
  { let v: u8 = cpu.r8(0x3eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x3eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3eu32.wrapping_add(cpu.edi), v); }
  pc = 0x10249f3; continue;
 }
 0x10249f3 => {
  { let v: u8 = cpu.r8(0x3du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x3du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3du32.wrapping_add(cpu.edi), v); }
  pc = 0x10249ff; continue;
 }
 0x10249ff => {
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a0b; continue;
 }
 0x1024a0b => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a17; continue;
 }
 0x1024a17 => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3au32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a23; continue;
 }
 0x1024a23 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a2f; continue;
 }
 0x1024a2f => {
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x38u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a3b; continue;
 }
 0x1024a3b => {
  { let v: u8 = cpu.r8(0x37u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x37u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x37u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a47; continue;
 }
 0x1024a47 => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x36u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a53; continue;
 }
 0x1024a53 => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a5f; continue;
 }
 0x1024a5f => {
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x34u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a6b; continue;
 }
 0x1024a6b => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x33u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a77; continue;
 }
 0x1024a77 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a83; continue;
 }
 0x1024a83 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x31u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a8f; continue;
 }
 0x1024a8f => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x30u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024a9b; continue;
 }
 0x1024a9b => {
  { let v: u8 = cpu.r8(0x2fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x2fu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2fu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024aa7; continue;
 }
 0x1024aa7 => {
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2eu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024ab3; continue;
 }
 0x1024ab3 => {
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2du32.wrapping_add(cpu.edi), v); }
  pc = 0x1024abf; continue;
 }
 0x1024abf => {
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2cu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024acb; continue;
 }
 0x1024acb => {
  { let v: u8 = cpu.r8(0x2bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x2bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2bu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024ad7; continue;
 }
 0x1024ad7 => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2au32.wrapping_add(cpu.edi), v); }
  pc = 0x1024ae3; continue;
 }
 0x1024ae3 => {
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x29u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024aef; continue;
 }
 0x1024aef => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x28u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024afb; continue;
 }
 0x1024afb => {
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b07; continue;
 }
 0x1024b07 => {
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b13; continue;
 }
 0x1024b13 => {
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x25u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b1f; continue;
 }
 0x1024b1f => {
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x24u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b2b; continue;
 }
 0x1024b2b => {
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b37; continue;
 }
 0x1024b37 => {
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b43; continue;
 }
 0x1024b43 => {
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b4f; continue;
 }
 0x1024b4f => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b5b; continue;
 }
 0x1024b5b => {
  { let v: u8 = cpu.r8(0x1fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1fu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b67; continue;
 }
 0x1024b67 => {
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1eu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b73; continue;
 }
 0x1024b73 => {
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1du32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b7f; continue;
 }
 0x1024b7f => {
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b8b; continue;
 }
 0x1024b8b => {
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024b97; continue;
 }
 0x1024b97 => {
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  pc = 0x1024ba3; continue;
 }
 0x1024ba3 => {
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024baf; continue;
 }
 0x1024baf => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024bbb; continue;
 }
 0x1024bbb => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024bc7; continue;
 }
 0x1024bc7 => {
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024bd3; continue;
 }
 0x1024bd3 => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024bdf; continue;
 }
 0x1024bdf => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024beb; continue;
 }
 0x1024beb => {
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024bf7; continue;
 }
 0x1024bf7 => {
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c03; continue;
 }
 0x1024c03 => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c0f; continue;
 }
 0x1024c0f => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c1b; continue;
 }
 0x1024c1b => {
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c27; continue;
 }
 0x1024c27 => {
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c33; continue;
 }
 0x1024c33 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c3f; continue;
 }
 0x1024c3f => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c4b; continue;
 }
 0x1024c4b => {
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c57; continue;
 }
 0x1024c57 => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c63; continue;
 }
 0x1024c63 => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c6f; continue;
 }
 0x1024c6f => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c7b; continue;
 }
 0x1024c7b => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c87; continue;
 }
 0x1024c87 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c93; continue;
 }
 0x1024c93 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024c9f; continue;
 }
 0x1024c9f => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024cab; continue;
 }
 0x1024cab => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024cb7; continue;
 }
 0x1024cb7 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024cc3; continue;
 }
 0x1024cc3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024ccf; continue;
 }
 0x1024ccf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024ee0; continue;
 }
 0x1024cde => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2au32.wrapping_add(cpu.edi), v); }
  pc = 0x1024cea; continue;
 }
 0x1024cea => {
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x29u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024cf6; continue;
 }
 0x1024cf6 => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x28u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d02; continue;
 }
 0x1024d02 => {
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d0e; continue;
 }
 0x1024d0e => {
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d1a; continue;
 }
 0x1024d1a => {
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x25u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d26; continue;
 }
 0x1024d26 => {
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x24u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d32; continue;
 }
 0x1024d32 => {
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d3e; continue;
 }
 0x1024d3e => {
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d4a; continue;
 }
 0x1024d4a => {
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d56; continue;
 }
 0x1024d56 => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d62; continue;
 }
 0x1024d62 => {
  { let v: u8 = cpu.r8(0x1fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1fu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d6e; continue;
 }
 0x1024d6e => {
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1eu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d7a; continue;
 }
 0x1024d7a => {
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1du32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d86; continue;
 }
 0x1024d86 => {
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d92; continue;
 }
 0x1024d92 => {
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024d9e; continue;
 }
 0x1024d9e => {
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  pc = 0x1024daa; continue;
 }
 0x1024daa => {
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024db6; continue;
 }
 0x1024db6 => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024dc2; continue;
 }
 0x1024dc2 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024dce; continue;
 }
 0x1024dce => {
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024dda; continue;
 }
 0x1024dda => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024de6; continue;
 }
 0x1024de6 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024df2; continue;
 }
 0x1024df2 => {
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024dfe; continue;
 }
 0x1024dfe => {
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e0a; continue;
 }
 0x1024e0a => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e16; continue;
 }
 0x1024e16 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e22; continue;
 }
 0x1024e22 => {
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e2e; continue;
 }
 0x1024e2e => {
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e3a; continue;
 }
 0x1024e3a => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e46; continue;
 }
 0x1024e46 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e52; continue;
 }
 0x1024e52 => {
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e5e; continue;
 }
 0x1024e5e => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e6a; continue;
 }
 0x1024e6a => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e76; continue;
 }
 0x1024e76 => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e82; continue;
 }
 0x1024e82 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e8e; continue;
 }
 0x1024e8e => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024e9a; continue;
 }
 0x1024e9a => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024ea6; continue;
 }
 0x1024ea6 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024eb2; continue;
 }
 0x1024eb2 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024ebe; continue;
 }
 0x1024ebe => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024eca; continue;
 }
 0x1024eca => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024ed6; continue;
 }
 0x1024ed6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024ee0; continue;
 }
 0x1024ee0 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x111a970u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x111a970u32)); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10248d1; continue; }
  pc = 0x1024ef9; continue;
 }
 0x1024ef9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010246c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10246c9); }
 let mut pc: u32 = 0x10246c9;
 loop { match pc {
 0x10246c9 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01024821(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1024821); }
 let mut pc: u32 = 0x1024821;
 loop { match pc {
 0x1024821 => {
  { let v: u32 = cpu.ror32(cpu.r32(0x2u32.wrapping_add(cpu.eax)), (cpu.ecx & 0xff)); cpu.w32(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(590); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01024efb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1024efb); }
 let mut pc: u32 = 0x1024efb;
 loop { match pc {
 0x1024efb => {
  { cpu.sub8(cpu.r8(0x111e9f5u32), 0x1u8); }
  if !cpu.zf { pc = 0x1024f21; continue; }
  pc = 0x1024f04; continue;
 }
 0x1024f04 => {
  { let v: u32 = cpu.r32(0x1119f28u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a970u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x111a974u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  pc = 0x1024f21; continue;
 }
 0x1024f21 => {
  { let v: u32 = cpu.r32(0x111a95cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a964u32)); cpu.edi = v; }
  { let v: u32 = 0x11751c8u32; cpu.esi = v; }
  { let v: u32 = cpu.r8(0x10ef36bu32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x1024f8e; continue; }
  pc = 0x1024f3d; continue;
 }
 0x1024f3d => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x111a954u32)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x3fu8); }
  if cpu.zf { pc = 0x1024f6a; continue; }
  pc = 0x1024f68; continue;
 }
 0x1024f68 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1024f6a; continue;
 }
 0x1024f6a => {
  { cpu.sub8(cpu.r8(0x111e9f5u32), 0x1u8); }
  if !cpu.zf { pc = 0x1024f88; continue; }
  pc = 0x1024f73; continue;
 }
 0x1024f73 => {
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x111a970u32)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x1119f28u32)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1024f88; continue;
 }
 0x1024f88 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x6u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1024f3d; continue; }
  pc = 0x1024f8e; continue;
 }
 0x1024f8e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01024f8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1024f8f); }
 let mut pc: u32 = 0x1024f8f;
 loop { match pc {
 0x1024f8f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffe0u32); cpu.esp = v; }
  { cpu.pushad(); }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x1024fa2; continue; }
  pc = 0x1024f9a; continue;
 }
 0x1024f9a => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.zf { pc = 0x1025038; continue; }
  pc = 0x1024fa2; continue;
 }
 0x1024fa2 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1024fa9; continue; }
  pc = 0x1024fa6; continue;
 }
 0x1024fa6 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x1024fa9; continue;
 }
 0x1024fa9 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x111a954u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.sf { pc = 0x1024fcb; continue; }
  pc = 0x1024fc6; continue;
 }
 0x1024fc6 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1024fcb; continue;
 }
 0x1024fcb => {
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a95cu32)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a964u32)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1025013; continue; }
  pc = 0x1024ff2; continue;
 }
 0x1024ff2 => {
  { let v: u32 = cpu.r32(0x10e41d0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x1025001; continue;
 }
 0x1025001 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if !cpu.sf { pc = 0x102500e; continue; }
  pc = 0x1025008; continue;
 }
 0x1025008 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  pc = 0x102500e; continue;
 }
 0x102500e => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1025001; continue; }
  pc = 0x1025011; continue;
 }
 0x1025011 => {
  pc = 0x1025032; continue;
 }
 0x1025013 => {
  { let v: u32 = cpu.r32(0x10e41d0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x1025022; continue;
 }
 0x1025022 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if !cpu.sf { pc = 0x102502f; continue; }
  pc = 0x102502b; continue;
 }
 0x102502b => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x102502f; continue;
 }
 0x102502f => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1025022; continue; }
  pc = 0x1025032; continue;
 }
 0x1025032 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  pc = 0x1025038; continue;
 }
 0x1025038 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a954u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a95cu32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x111a964u32)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e41d0u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025059(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025059); }
 let mut pc: u32 = 0x1025059;
 loop { match pc {
 0x1025059 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.sf { pc = 0x102508d; continue; }
  pc = 0x102505f; continue;
 }
 0x102505f => {
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.sf { pc = 0x102508d; continue; }
  pc = 0x1025064; continue;
 }
 0x1025064 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x111a970u32)); }
  if !cpu.cf { pc = 0x102508d; continue; }
  pc = 0x102506c; continue;
 }
 0x102506c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x111a974u32)); }
  if !cpu.cf { pc = 0x102508d; continue; }
  pc = 0x1025074; continue;
 }
 0x1025074 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a954u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a95cu32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x111a964u32)); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102508d; continue;
 }
 0x102508d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102508f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102508f); }
 let mut pc: u32 = 0x102508f;
 loop { match pc {
 0x102508f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.sf { pc = 0x10250cc; continue; }
  pc = 0x1025098; continue;
 }
 0x1025098 => {
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.sf { pc = 0x10250cc; continue; }
  pc = 0x102509d; continue;
 }
 0x102509d => {
  { cpu.sub32(cpu.eax, cpu.r32(0x111a970u32)); }
  if !cpu.cf { pc = 0x10250cc; continue; }
  pc = 0x10250a5; continue;
 }
 0x10250a5 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x111a974u32)); }
  if !cpu.cf { pc = 0x10250cc; continue; }
  pc = 0x10250ad; continue;
 }
 0x10250ad => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a954u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a95cu32)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a964u32)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x10250cc; continue;
 }
 0x10250cc => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010250d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10250d1); }
 let mut pc: u32 = 0x10250d1;
 loop { match pc {
 0x10250d1 => {
  { let v: u32 = 0x10e41ecu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.sf { pc = 0x102514a; continue; }
  pc = 0x10250dc; continue;
 }
 0x10250dc => {
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  pc = 0x10250f0; continue;
 }
 0x10250f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1025115; continue; }
  pc = 0x10250f6; continue;
 }
 0x10250f6 => {
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  cpu.push(0x102510f); f_01024f8f(cpu); if cpu.returned_to != 0x102510f { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.poll(); pc = 0x10250f0; continue;
 }
 0x1025115 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x111a978u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x102512b); f_01025059(cpu); if cpu.returned_to != 0x102512b { return; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x1025132); f_01025059(cpu); if cpu.returned_to != 0x1025132 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  cpu.push(0x102513a); f_01025059(cpu); if cpu.returned_to != 0x102513a { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  cpu.push(0x1025142); f_01025059(cpu); if cpu.returned_to != 0x1025142 { return; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x2u32); cpu.eax = v; }
  cpu.push(0x102514a); f_01025059(cpu); if cpu.returned_to != 0x102514a { return; }
  pc = 0x102514a; continue;
 }
 0x102514a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102514b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102514b); }
 let mut pc: u32 = 0x102514b;
 loop { match pc {
 0x102514b => {
  cpu.push(0x1025150); f_01026a8c(cpu); if cpu.returned_to != 0x1025150 { return; }
  { cpu.sub32(cpu.r32(0x111ea0au32), 0x0u32); }
  if cpu.zf { pc = 0x102519d; continue; }
  pc = 0x1025159; continue;
 }
 0x1025159 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x111ea0au32)); cpu.w32(0x111ea0au32, v); }
  { let v: u32 = cpu.r32(0x111a954u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111ea02u32, v); }
  { let v: u32 = cpu.r32(0x111a950u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111ea06u32, v); }
  { let v: u32 = cpu.r32(0x111a968u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x111a96cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x111a970u32)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x111a974u32)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x111a95cu32); cpu.edi = v; }
  cpu.push(0x102519d); f_01025a80(cpu); if cpu.returned_to != 0x102519d { return; }
  pc = 0x102519d; continue;
 }
 0x102519d => {
  cpu.push(0x10251a2); f_0102549a(cpu); if cpu.returned_to != 0x10251a2 { return; }
  { let v: u32 = cpu.r8(0x111e9f4u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax ^ 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x111e9f6u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x10251be; continue; }
  pc = 0x10251b6; continue;
 }
 0x10251b6 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10251cc; continue; }
  pc = 0x10251ba; continue;
 }
 0x10251ba => {
  if cpu.cf { pc = 0x10251d3; continue; }
  pc = 0x10251bc; continue;
 }
 0x10251bc => {
  pc = 0x10251c5; continue;
 }
 0x10251be => {
  cpu.push(0x10251c3); f_01023669(cpu); if cpu.returned_to != 0x10251c3 { return; }
  pc = 0x10251d8; continue;
 }
 0x10251c5 => {
  cpu.push(0x10251ca); f_01024681(cpu); if cpu.returned_to != 0x10251ca { return; }
  pc = 0x10251d8; continue;
 }
 0x10251cc => {
  cpu.push(0x10251d1); f_01023bcd(cpu); if cpu.returned_to != 0x10251d1 { return; }
  pc = 0x10251d8; continue;
 }
 0x10251d3 => {
  cpu.push(0x10251d8); f_01023c05(cpu); if cpu.returned_to != 0x10251d8 { return; }
  pc = 0x10251d8; continue;
 }
 0x10251d8 => {
  cpu.push(0x10251dd); f_01024efb(cpu); if cpu.returned_to != 0x10251dd { return; }
  cpu.push(0x10251e2); f_010250d1(cpu); if cpu.returned_to != 0x10251e2 { return; }
  { let v: u32 = cpu.r16(0x10e3f60u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4104u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x10e3f64u32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e4108u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10e41ccu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x111a95cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a964u32)); cpu.edi = v; }
  cpu.push(0x1025235); f_01025a80(cpu); if cpu.returned_to != 0x1025235 { return; }
  { cpu.sub8(cpu.r8(0x111e9f5u32), 0x1u8); }
  if !cpu.zf { pc = 0x1025243; continue; }
  pc = 0x102523e; continue;
 }
 0x102523e => {
  cpu.push(0x1025243); f_01025513(cpu); if cpu.returned_to != 0x1025243 { return; }
  pc = 0x1025243; continue;
 }
 0x1025243 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025244(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025244); }
 let mut pc: u32 = 0x1025244;
 loop { match pc {
 0x1025244 => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if !cpu.zf { pc = 0x1025299; continue; }
  pc = 0x102524d; continue;
 }
 0x102524d => {
  { let v: u32 = 0x34u32; cpu.w32(0x1119eb6u32, v); }
  { let v: u32 = 0x2bu32; cpu.w32(0x1119ebau32, v); }
  { let v: u32 = 0x34u32; cpu.w32(0x1119eaeu32, v); }
  { let v: u32 = 0x2bu32; cpu.w32(0x1119eb2u32, v); }
  { let v: u32 = 0x90u32; cpu.w32(0x1119ebeu32, v); }
  { let v: u32 = 0x3u32; cpu.w32(0x1119ec2u32, v); }
  { let v: u32 = cpu.r16(0x10e690cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.ebx = v; }
  pc = 0x10252fb; continue;
 }
 0x1025299 => {
  { let v: u32 = cpu.r16(0x10e690cu32) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x56u32); }
  if cpu.cf { pc = 0x10252aa; continue; }
  pc = 0x10252a5; continue;
 }
 0x10252a5 => {
  { let v: u32 = 0x56u32; cpu.eax = v; }
  pc = 0x10252aa; continue;
 }
 0x10252aa => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119eb2u32, v); }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x68u32); }
  if cpu.cf { pc = 0x10252c0; continue; }
  pc = 0x10252bb; continue;
 }
 0x10252bb => {
  { let v: u32 = 0x68u32; cpu.eax = v; }
  pc = 0x10252c0; continue;
 }
 0x10252c0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119eaeu32, v); }
  { let v: u32 = 0x68u32; cpu.w32(0x1119eb6u32, v); }
  { let v: u32 = 0x56u32; cpu.w32(0x1119ebau32, v); }
  { let v: u32 = 0x120u32; cpu.w32(0x1119ebeu32, v); }
  { let v: u32 = 0x6u32; cpu.w32(0x1119ec2u32, v); }
  { let v: u32 = cpu.r16(0x10e690cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.ebx = v; }
  pc = 0x10252fb; continue;
 }
 0x10252fb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x1119eaeu32); cpu.imul32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.r32(0x1119eb2u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x102537d; continue; }
  pc = 0x1025317; continue;
 }
 0x1025317 => {
  { cpu.pushad(); }
  { let v: u32 = 0x110f904u32; cpu.eax = v; }
  { let v: u32 = 0x1119eebu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1119f20u32)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x102534c; continue; }
  pc = 0x102533a; continue;
 }
 0x102533a => {
  { let v: u32 = 0x1119f03u32; cpu.ebx = v; }
  { let v: u32 = 0x110f95fu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x69c0u32); cpu.ecx = v; }
  pc = 0x1025352; continue;
 }
 0x102534c => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1a70u32); cpu.ecx = v; }
  pc = 0x1025352; continue;
 }
 0x1025352 => {
  { let v: u32 = 0x3u32; cpu.edx = v; }
  cpu.push(0x102535c); f_0105ce4d(cpu); if cpu.returned_to != 0x102535c { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.sf { f_0105f11c(cpu); return; }
  pc = 0x1025365; continue;
 }
 0x1025365 => {
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x1119eaeu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x111a970u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x111a974u32, v); }
  pc = 0x10253e2; continue;
 }
 0x102537d => {
  { cpu.pushad(); }
  { let v: u32 = 0x110f904u32; cpu.eax = v; }
  { let v: u32 = 0x1119ee0u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1119f20u32)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10253b2; continue; }
  pc = 0x10253a0; continue;
 }
 0x10253a0 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x69c0u32); cpu.ecx = v; }
  { let v: u32 = 0x110f95fu32; cpu.eax = v; }
  { let v: u32 = 0x1119ef7u32; cpu.ebx = v; }
  pc = 0x10253b8; continue;
 }
 0x10253b2 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1a70u32); cpu.ecx = v; }
  pc = 0x10253b8; continue;
 }
 0x10253b8 => {
  { let v: u32 = 0x3u32; cpu.edx = v; }
  cpu.push(0x10253c2); f_0105ce4d(cpu); if cpu.returned_to != 0x10253c2 { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.sf { f_0105f11c(cpu); return; }
  pc = 0x10253cb; continue;
 }
 0x10253cb => {
  { cpu.popad(); }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.r32(0x1119eb2u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x111a974u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x111a970u32, v); }
  pc = 0x10253e2; continue;
 }
 0x10253e2 => {
  { let v: u32 = cpu.r32(0x1119eb6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x111a974u32)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11b51aau32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1119ebeu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111a96cu32, v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x111a954u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1119ebau32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x111a970u32)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11b51a6u32, v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1119ec2u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x111a968u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111a964u32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x111a982u32; cpu.edi = v; }
  { let v: u32 = 0x801u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x10e690cu32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  pc = 0x1025443; continue;
 }
 0x1025443 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.r32(0x111a970u32); cpu.imul32(v); }
  { let v = cpu.esi; cpu.idiv32(v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x111a970u32)); }
  if cpu.cf { pc = 0x102545a; continue; }
  pc = 0x1025455; continue;
 }
 0x1025455 => {
  { let v: u32 = cpu.r32(0x111a970u32); cpu.eax = v; }
  pc = 0x102545a; continue;
 }
 0x102545a => {
  { cpu.stos(4, false); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1025443; continue; }
  pc = 0x102545e; continue;
 }
 0x102545e => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x111c986u32; cpu.edi = v; }
  { let v: u32 = 0x801u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  pc = 0x1025474; continue;
 }
 0x1025474 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.r32(0x111a974u32); cpu.imul32(v); }
  { let v = cpu.esi; cpu.idiv32(v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x111a974u32)); }
  if cpu.cf { pc = 0x102548b; continue; }
  pc = 0x1025486; continue;
 }
 0x1025486 => {
  { let v: u32 = cpu.r32(0x111a974u32); cpu.eax = v; }
  pc = 0x102548b; continue;
 }
 0x102548b => {
  { cpu.stos(4, false); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1025474; continue; }
  pc = 0x102548f; continue;
 }
 0x102548f => {
  { let v: u32 = 0x2u32; cpu.w32(0x111ea0au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102549a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102549a); } r_0102549a(cpu, 0x102549a); }
pub fn f_010254e2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10254e2); } r_0102549a(cpu, 0x10254e2); }
fn r_0102549a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102549a => {
  cpu.push(0x102549f); f_01025706(cpu); if cpu.returned_to != 0x102549f { return; }
  { cpu.sub8(cpu.r8(0x111a021u32), 0x0u8); }
  if cpu.zf { pc = 0x1025512; continue; }
  pc = 0x10254ac; continue;
 }
 0x10254ac => {
  { let v: u32 = cpu.r32(0x1119f40u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1119f2cu32); cpu.ecx = v; }
  { let v: u32 = 0x111a144u32; cpu.esi = v; }
  { let v: u32 = 0x10f74bdu32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x111a020u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x83u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x3u8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x111a020u32, v); }
  if !cpu.zf { pc = 0x1025512; continue; }
  pc = 0x10254d4; continue;
 }
 0x10254d4 => {
  { let v: u8 = cpu.xor8(cpu.r8(0x111a020u32), 0x80u8); cpu.w8(0x111a020u32, v); }
  if cpu.zf { pc = 0x10254f6; continue; }
  pc = 0x10254dd; continue;
 }
 0x10254dd => {
  { let v: u32 = cpu.edi.wrapping_add(0x5u32); cpu.edi = v; }
  pc = 0x10254f6; continue;
 }
 0x10254e2 => {
  { let v: u8 = cpu.r8(0x111a022u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10254e8; continue;
 }
 0x10254e8 => {
  { cpu.lods(4, false); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1025512; continue; }
  pc = 0x10254ee; continue;
 }
 0x10254ee => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  cpu.poll(); pc = 0x10254e8; continue;
 }
 0x10254f6 => {
  { cpu.lods(4, false); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1025512; continue; }
  pc = 0x10254fc; continue;
 }
 0x10254fc => {
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  cpu.poll(); pc = 0x10254f6; continue;
 }
 0x1025512 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025513(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025513); }
 let mut pc: u32 = 0x1025513;
 loop { match pc {
 0x1025513 => {
  { let v: u32 = cpu.r16(0x10e3cdbu32) as u32; cpu.eax = v; }
  { cpu.and8(cpu.r8(0x10ef352u32), 0xffu8); }
  if cpu.zf { pc = 0x1025530; continue; }
  pc = 0x1025523; continue;
 }
 0x1025523 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x21eu32); cpu.eax = v; }
  if cpu.sf { pc = 0x1025677; continue; }
  pc = 0x102552e; continue;
 }
 0x102552e => {
  pc = 0x102553b; continue;
 }
 0x1025530 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x10fu32); cpu.eax = v; }
  if cpu.sf { pc = 0x1025677; continue; }
  pc = 0x102553b; continue;
 }
 0x102553b => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x111a968u32)); cpu.eax = v; }
  if cpu.sf { pc = 0x1025677; continue; }
  pc = 0x1025547; continue;
 }
 0x1025547 => {
  { let v: u32 = cpu.r16(0x10e3cddu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x111a96cu32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x1025677; continue; }
  pc = 0x102555a; continue;
 }
 0x102555a => {
  { cpu.sub32(cpu.r32(0x111a970u32), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1025677; continue; }
  pc = 0x1025566; continue;
 }
 0x1025566 => {
  { cpu.sub32(cpu.r32(0x111a974u32), cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1025677; continue; }
  pc = 0x1025572; continue;
 }
 0x1025572 => {
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.esi, cpu.r32(0x111a970u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1119f24u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x10255e7; continue; }
  pc = 0x102558f; continue;
 }
 0x102558f => {
  { let v: u32 = cpu.r32(0x1119f28u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1025677; continue; }
  pc = 0x10255a1; continue;
 }
 0x10255a1 => {
  { let v: u32 = cpu.r8(0x10fd44bu32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1119ff9u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x10255bd; continue; }
  pc = 0x10255b6; continue;
 }
 0x10255b6 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub8((cpu.edx as u8), 0xfeu8); }
  if !cpu.zf { pc = 0x10255bd; continue; }
  pc = 0x10255bc; continue;
 }
 0x10255bc => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10255bd; continue;
 }
 0x10255bd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1119f20u32)); cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x10ef352u32), 0xffu8); }
  if cpu.zf { pc = 0x10255e3; continue; }
  pc = 0x10255de; continue;
 }
 0x10255de => {
  { let v: u32 = 0x8u32; cpu.ebp = v; }
  pc = 0x10255e3; continue;
 }
 0x10255e3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1025647; continue;
 }
 0x10255e7 => {
  { let v: u32 = 0x8u32; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x10ef352u32), 0xffu8); }
  if cpu.zf { pc = 0x10255fa; continue; }
  pc = 0x10255f5; continue;
 }
 0x10255f5 => {
  { let v: u32 = 0x10u32; cpu.ebp = v; }
  pc = 0x10255fa; continue;
 }
 0x10255fa => {
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10fd44bu32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1119f45u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1119f20u32)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10fd44bu32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1119f45u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x1119f20u32)); cpu.edx = v; }
  pc = 0x1025647; continue;
 }
 0x1025647 => {
  { cpu.and8(cpu.r8(0x10ef352u32), 0xffu8); }
  if cpu.zf { pc = 0x1025657; continue; }
  pc = 0x1025650; continue;
 }
 0x1025650 => {
  { let v: u32 = cpu.sub32(cpu.ebx, 0xau32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x1025662; continue; }
  pc = 0x1025655; continue;
 }
 0x1025655 => {
  pc = 0x102565c; continue;
 }
 0x1025657 => {
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x1025662; continue; }
  pc = 0x102565c; continue;
 }
 0x102565c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x1025662; continue;
 }
 0x1025662 => {
  { let v: u32 = cpu.r32(0x111a970u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  if !cpu.sf { pc = 0x1025672; continue; }
  pc = 0x102566e; continue;
 }
 0x102566e => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x1025672; continue;
 }
 0x1025672 => {
  cpu.push(0x1025677); f_01025b51(cpu); if cpu.returned_to != 0x1025677 { return; }
  pc = 0x1025677; continue;
 }
 0x1025677 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025678(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025678); }
 let mut pc: u32 = 0x1025678;
 loop { match pc {
 0x1025678 => {
  { let v: u32 = 0x111a144u32; cpu.edi = v; }
  { let v: u32 = 0x82u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = 0x0u8; cpu.w8(0x111a021u32, v); }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u32 = 0x10f7363u32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x111a022u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010256a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10256a1); }
 let mut pc: u32 = 0x10256a1;
 loop { match pc {
 0x10256a1 => {
  { let v: u32 = cpu.r32(0x1119f24u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a970u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x111a974u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x111a024u32; cpu.edi = v; }
  { let v: u32 = 0x116c0d8u32; cpu.esi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10256d1; continue;
 }
 0x10256d1 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x10256fa; continue; }
  pc = 0x10256d7; continue;
 }
 0x10256d7 => {
  { let v: u32 = cpu.esi; cpu.w32(0x90u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x111a970u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x111a982u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x10256fa; continue;
 }
 0x10256fa => {
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10256d1; continue; }
  pc = 0x10256ff; continue;
 }
 0x10256ff => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025706(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025706); }
 let mut pc: u32 = 0x1025706;
 loop { match pc {
 0x1025706 => {
  { let v: u32 = 0x111a024u32; cpu.esi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1025710; continue;
 }
 0x1025710 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x102578a; continue; }
  pc = 0x1025717; continue;
 }
 0x1025717 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1119f2cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1119f40u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1119f24u32); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x102578b; continue; }
  pc = 0x1025740; continue;
 }
 0x1025740 => {
  { let v: u8 = cpu.r8(0x10f74ccu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x8u8); }
  if cpu.zf { pc = 0x1025762; continue; }
  pc = 0x102574d; continue;
 }
 0x102574d => {
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a970u32)); cpu.edi = v; }
  pc = 0x1025762; continue;
 }
 0x1025762 => {
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a970u32)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x1025784; continue;
 }
 0x1025784 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1025710; continue; }
  pc = 0x102578a; continue;
 }
 0x102578a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102578b => {
  { let v: u8 = cpu.r8(0x10f74d1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a970u32)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a970u32)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a970u32)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  cpu.poll(); pc = 0x1025784; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102581c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102581c); } r_0102581c(cpu, 0x102581c); }
pub fn f_0102591b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102591b); } r_0102581c(cpu, 0x102591b); }
fn r_0102581c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102581c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1119f40u32); cpu.edi = v; }
  { let v: u32 = 0x8bcu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1025836; continue; }
  pc = 0x1025831; continue;
 }
 0x1025831 => {
  { let v: u32 = 0x22f0u32; cpu.ecx = v; }
  pc = 0x1025836; continue;
 }
 0x1025836 => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.r32(0x1119f44u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a970u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x111a974u32)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.r16(0x10e690cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111f024u32, v); }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111f028u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x111f034u32, v); }
  pc = 0x1025879; continue;
 }
 0x1025879 => {
  { let v: u32 = 0x10ea910u32; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x111f034u32), 0x0u32); }
  if !cpu.zf { pc = 0x10258ad; continue; }
  pc = 0x1025887; continue;
 }
 0x1025887 => {
  { let v: u32 = 0x80u32; cpu.w32(0x111f02cu32, v); }
  { let v: u32 = 0x10u32; cpu.w32(0x111f030u32, v); }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111f038u32, v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10258e7; continue;
 }
 0x10258ad => {
  { let v: u32 = 0xffffff80u32; cpu.w32(0x111f02cu32, v); }
  { let v: u32 = 0xfffffff0u32; cpu.w32(0x111f030u32, v); }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x80u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111f038u32, v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x10258e7; continue;
 }
 0x10258e7 => {
  { let v: u32 = 0x10f7363u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1119f44u32); cpu.ebx = v; }
  pc = 0x10258f2; continue;
 }
 0x10258f2 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x10258f5; continue;
 }
 0x10258f5 => {
  { let v: u32 = cpu.r32(0x1119f2cu32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.r32(0x111a970u32)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a982u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if !cpu.zf { pc = 0x1025954; continue; }
  pc = 0x1025919; continue;
 }
 0x1025919 => {
  pc = 0x1025954; continue;
 }
 0x102591b => {
  { cpu.sub8(cpu.r8(0x111a021u32), 0x0u8); }
  if !cpu.zf { pc = 0x1025954; continue; }
  pc = 0x1025924; continue;
 }
 0x1025924 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x111a144u32; cpu.esi = v; }
  pc = 0x102592c; continue;
 }
 0x102592c => {
  { cpu.lods(4, false); }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x102594f; continue; }
  pc = 0x1025931; continue;
 }
 0x1025931 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x102592c; continue; }
  pc = 0x1025936; continue;
 }
 0x1025936 => {
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x1025943); f_0101b70a(cpu); if cpu.returned_to != 0x1025943 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x10c6a48u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.esi), v); }
  pc = 0x102594f; continue;
 }
 0x102594f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10259a2; continue;
 }
 0x1025954 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x1eu8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x102595a; continue;
 }
 0x102595a => {
  { cpu.sub8((cpu.eax as u8), 0x3eu8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x102595e; continue;
 }
 0x102595e => {
  { cpu.sub8((cpu.eax as u8), 0x3fu8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x1025962; continue;
 }
 0x1025962 => {
  { cpu.sub8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x1025966; continue;
 }
 0x1025966 => {
  { cpu.sub8((cpu.eax as u8), 0x41u8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x102596a; continue;
 }
 0x102596a => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x102596e; continue;
 }
 0x102596e => {
  { cpu.sub8((cpu.eax as u8), 0x14u8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x1025972; continue;
 }
 0x1025972 => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x1025976; continue;
 }
 0x1025976 => {
  { cpu.sub8((cpu.eax as u8), 0x3cu8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x102597a; continue;
 }
 0x102597a => {
  { cpu.sub8((cpu.eax as u8), 0x3du8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x102597e; continue;
 }
 0x102597e => {
  { cpu.sub8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x1025982; continue;
 }
 0x1025982 => {
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x1025986; continue;
 }
 0x1025986 => {
  { cpu.sub8((cpu.eax as u8), 0x2cu8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x102598a; continue;
 }
 0x102598a => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x102598e; continue;
 }
 0x102598e => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x33u8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x1025994; continue;
 }
 0x1025994 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x1025998; continue;
 }
 0x1025998 => {
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x102599c; continue;
 }
 0x102599c => {
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if cpu.zf { pc = 0x10259a2; continue; }
  pc = 0x10259a0; continue;
 }
 0x10259a0 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10259a2; continue;
 }
 0x10259a2 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x3u8); }
  if !cpu.zf { pc = 0x10259a9; continue; }
  pc = 0x10259a7; continue;
 }
 0x10259a7 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10259a9; continue;
 }
 0x10259a9 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1))), ((cpu.eax >> 8) as u8)); }
  if (cpu.sf == cpu.of) { pc = 0x10259c0; continue; }
  pc = 0x10259ae; continue;
 }
 0x10259ae => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10f7363u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x10259c0; continue;
 }
 0x10259c0 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x111f024u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10258f5; continue; }
  pc = 0x10259cf; continue;
 }
 0x10259cf => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x111f02cu32)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x111f030u32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x111f038u32)); cpu.w32(0x111f038u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x10258f2; continue; }
  pc = 0x10259e8; continue;
 }
 0x10259e8 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x111f034u32)); cpu.w32(0x111f034u32, v); }
  { cpu.sub32(cpu.r32(0x111f034u32), 0x2u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1025879; continue; }
  pc = 0x10259fb; continue;
 }
 0x10259fb => {
  cpu.push(0x1025a00); f_01026f06(cpu); if cpu.returned_to != 0x1025a00 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025a02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025a02); }
 let mut pc: u32 = 0x1025a02;
 loop { match pc {
 0x1025a02 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x1025a09; continue; }
  pc = 0x1025a07; continue;
 }
 0x1025a07 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1025a09; continue;
 }
 0x1025a09 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x111ea02u32)); }
  if cpu.cf { pc = 0x1025a18; continue; }
  pc = 0x1025a11; continue;
 }
 0x1025a11 => {
  { let v: u32 = cpu.r32(0x111ea02u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1025a18; continue;
 }
 0x1025a18 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.sf { pc = 0x1025a1e; continue; }
  pc = 0x1025a1c; continue;
 }
 0x1025a1c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x1025a1e; continue;
 }
 0x1025a1e => {
  { cpu.sub32(cpu.edx, cpu.r32(0x111ea06u32)); }
  if cpu.cf { pc = 0x1025a2d; continue; }
  pc = 0x1025a26; continue;
 }
 0x1025a26 => {
  { let v: u32 = cpu.r32(0x111ea06u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x1025a2d; continue;
 }
 0x1025a2d => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1025a6a; continue; }
  pc = 0x1025a32; continue;
 }
 0x1025a32 => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1025a6a; continue; }
  pc = 0x1025a37; continue;
 }
 0x1025a37 => {
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.r32(0x111ea02u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x1025a51; continue;
 }
 0x1025a51 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x111ea02u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1025a51; continue; }
  pc = 0x1025a6a; continue;
 }
 0x1025a6a => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025a6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025a6c); }
 let mut pc: u32 = 0x1025a6c;
 loop { match pc {
 0x1025a6c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x1025a74); f_01025a02(cpu); if cpu.returned_to != 0x1025a74 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025a76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025a76); }
 let mut pc: u32 = 0x1025a76;
 loop { match pc {
 0x1025a76 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1025a7e); f_01025a02(cpu); if cpu.returned_to != 0x1025a7e { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025a80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025a80); }
 let mut pc: u32 = 0x1025a80;
 loop { match pc {
 0x1025a80 => {
  { cpu.pushad(); }
  cpu.push(0x1025a86); f_01025a6c(cpu); if cpu.returned_to != 0x1025a86 { return; }
  cpu.push(0x1025a8b); f_01025a76(cpu); if cpu.returned_to != 0x1025a8b { return; }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  cpu.push(0x1025a91); f_01025a76(cpu); if cpu.returned_to != 0x1025a91 { return; }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x1025a99); f_01025a6c(cpu); if cpu.returned_to != 0x1025a99 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025a9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025a9b); }
 let mut pc: u32 = 0x1025a9b;
 loop { match pc {
 0x1025a9b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff8u32); cpu.esp = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x111ea02u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x111ea06u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1025ac0; continue; }
  pc = 0x1025abb; continue;
 }
 0x1025abb => {
  cpu.push(0x1025ac0); f_01025a6c(cpu); if cpu.returned_to != 0x1025ac0 { return; }
  pc = 0x1025ac0; continue;
 }
 0x1025ac0 => {
  { cpu.sub32(cpu.eax, 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1025aca; continue; }
  pc = 0x1025ac5; continue;
 }
 0x1025ac5 => {
  cpu.push(0x1025aca); f_01025a76(cpu); if cpu.returned_to != 0x1025aca { return; }
  pc = 0x1025aca; continue;
 }
 0x1025aca => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x1025ad5; continue; }
  pc = 0x1025ad0; continue;
 }
 0x1025ad0 => {
  cpu.push(0x1025ad5); f_01025a76(cpu); if cpu.returned_to != 0x1025ad5 { return; }
  pc = 0x1025ad5; continue;
 }
 0x1025ad5 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x1025ae2; continue; }
  pc = 0x1025add; continue;
 }
 0x1025add => {
  cpu.push(0x1025ae2); f_01025a6c(cpu); if cpu.returned_to != 0x1025ae2 { return; }
  pc = 0x1025ae2; continue;
 }
 0x1025ae2 => {
  { cpu.popad(); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025ae5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025ae5); }
 let mut pc: u32 = 0x1025ae5;
 loop { match pc {
 0x1025ae5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  cpu.push(0x1025b07); f_01025a02(cpu); if cpu.returned_to != 0x1025b07 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025b09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025b09); }
 let mut pc: u32 = 0x1025b09;
 loop { match pc {
 0x1025b09 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  cpu.push(0x1025b2b); f_01025a9b(cpu); if cpu.returned_to != 0x1025b2b { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025b2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025b2d); }
 let mut pc: u32 = 0x1025b2d;
 loop { match pc {
 0x1025b2d => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x111a982u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  cpu.push(0x1025b4f); f_01025a80(cpu); if cpu.returned_to != 0x1025b4f { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01025b51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1025b51); }
 let mut pc: u32 = 0x1025b51;
 loop { match pc {
 0x1025b51 => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if !cpu.zf { pc = 0x1025e2b; continue; }
  pc = 0x1025b5e; continue;
 }
 0x1025b5e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x111a95cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a964u32)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.r32(0x111a954u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x1025d3e; continue; }
  pc = 0x1025b7f; continue;
 }
 0x1025b7f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025b87; continue; }
  pc = 0x1025b85; continue;
 }
 0x1025b85 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025b87; continue;
 }
 0x1025b87 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025b90; continue; }
  pc = 0x1025b8d; continue;
 }
 0x1025b8d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025b90; continue;
 }
 0x1025b90 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025b9a; continue; }
  pc = 0x1025b97; continue;
 }
 0x1025b97 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025b9a; continue;
 }
 0x1025b9a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ba4; continue; }
  pc = 0x1025ba1; continue;
 }
 0x1025ba1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ba4; continue;
 }
 0x1025ba4 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025bae; continue; }
  pc = 0x1025bab; continue;
 }
 0x1025bab => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025bae; continue;
 }
 0x1025bae => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025bb8; continue; }
  pc = 0x1025bb5; continue;
 }
 0x1025bb5 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025bb8; continue;
 }
 0x1025bb8 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025bc2; continue; }
  pc = 0x1025bbf; continue;
 }
 0x1025bbf => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025bc2; continue;
 }
 0x1025bc2 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025bcc; continue; }
  pc = 0x1025bc9; continue;
 }
 0x1025bc9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025bcc; continue;
 }
 0x1025bcc => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025be0; continue; }
  pc = 0x1025bde; continue;
 }
 0x1025bde => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025be0; continue;
 }
 0x1025be0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025be9; continue; }
  pc = 0x1025be6; continue;
 }
 0x1025be6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025be9; continue;
 }
 0x1025be9 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025bf3; continue; }
  pc = 0x1025bf0; continue;
 }
 0x1025bf0 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025bf3; continue;
 }
 0x1025bf3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025bfd; continue; }
  pc = 0x1025bfa; continue;
 }
 0x1025bfa => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025bfd; continue;
 }
 0x1025bfd => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c07; continue; }
  pc = 0x1025c04; continue;
 }
 0x1025c04 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c07; continue;
 }
 0x1025c07 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c11; continue; }
  pc = 0x1025c0e; continue;
 }
 0x1025c0e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c11; continue;
 }
 0x1025c11 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c1b; continue; }
  pc = 0x1025c18; continue;
 }
 0x1025c18 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c1b; continue;
 }
 0x1025c1b => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c25; continue; }
  pc = 0x1025c22; continue;
 }
 0x1025c22 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c25; continue;
 }
 0x1025c25 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c39; continue; }
  pc = 0x1025c37; continue;
 }
 0x1025c37 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c39; continue;
 }
 0x1025c39 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c42; continue; }
  pc = 0x1025c3f; continue;
 }
 0x1025c3f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c42; continue;
 }
 0x1025c42 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c4c; continue; }
  pc = 0x1025c49; continue;
 }
 0x1025c49 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c4c; continue;
 }
 0x1025c4c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c56; continue; }
  pc = 0x1025c53; continue;
 }
 0x1025c53 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c56; continue;
 }
 0x1025c56 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c60; continue; }
  pc = 0x1025c5d; continue;
 }
 0x1025c5d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c60; continue;
 }
 0x1025c60 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c6a; continue; }
  pc = 0x1025c67; continue;
 }
 0x1025c67 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c6a; continue;
 }
 0x1025c6a => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c74; continue; }
  pc = 0x1025c71; continue;
 }
 0x1025c71 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c74; continue;
 }
 0x1025c74 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c7e; continue; }
  pc = 0x1025c7b; continue;
 }
 0x1025c7b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c7e; continue;
 }
 0x1025c7e => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c92; continue; }
  pc = 0x1025c90; continue;
 }
 0x1025c90 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c92; continue;
 }
 0x1025c92 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025c9b; continue; }
  pc = 0x1025c98; continue;
 }
 0x1025c98 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025c9b; continue;
 }
 0x1025c9b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ca5; continue; }
  pc = 0x1025ca2; continue;
 }
 0x1025ca2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ca5; continue;
 }
 0x1025ca5 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025caf; continue; }
  pc = 0x1025cac; continue;
 }
 0x1025cac => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025caf; continue;
 }
 0x1025caf => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025cb9; continue; }
  pc = 0x1025cb6; continue;
 }
 0x1025cb6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025cb9; continue;
 }
 0x1025cb9 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025cc3; continue; }
  pc = 0x1025cc0; continue;
 }
 0x1025cc0 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025cc3; continue;
 }
 0x1025cc3 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ccd; continue; }
  pc = 0x1025cca; continue;
 }
 0x1025cca => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ccd; continue;
 }
 0x1025ccd => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025cd7; continue; }
  pc = 0x1025cd4; continue;
 }
 0x1025cd4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025cd7; continue;
 }
 0x1025cd7 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ceb; continue; }
  pc = 0x1025ce9; continue;
 }
 0x1025ce9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ceb; continue;
 }
 0x1025ceb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025cf4; continue; }
  pc = 0x1025cf1; continue;
 }
 0x1025cf1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025cf4; continue;
 }
 0x1025cf4 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025cfe; continue; }
  pc = 0x1025cfb; continue;
 }
 0x1025cfb => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025cfe; continue;
 }
 0x1025cfe => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d08; continue; }
  pc = 0x1025d05; continue;
 }
 0x1025d05 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d08; continue;
 }
 0x1025d08 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d12; continue; }
  pc = 0x1025d0f; continue;
 }
 0x1025d0f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d12; continue;
 }
 0x1025d12 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d1c; continue; }
  pc = 0x1025d19; continue;
 }
 0x1025d19 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d1c; continue;
 }
 0x1025d1c => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d26; continue; }
  pc = 0x1025d23; continue;
 }
 0x1025d23 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d26; continue;
 }
 0x1025d26 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d30; continue; }
  pc = 0x1025d2d; continue;
 }
 0x1025d2d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d30; continue;
 }
 0x1025d30 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1025d3e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d46; continue; }
  pc = 0x1025d44; continue;
 }
 0x1025d44 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d46; continue;
 }
 0x1025d46 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d50; continue; }
  pc = 0x1025d4d; continue;
 }
 0x1025d4d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d50; continue;
 }
 0x1025d50 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d5a; continue; }
  pc = 0x1025d57; continue;
 }
 0x1025d57 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d5a; continue;
 }
 0x1025d5a => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d64; continue; }
  pc = 0x1025d61; continue;
 }
 0x1025d61 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d64; continue;
 }
 0x1025d64 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d75; continue; }
  pc = 0x1025d73; continue;
 }
 0x1025d73 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d75; continue;
 }
 0x1025d75 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d7f; continue; }
  pc = 0x1025d7c; continue;
 }
 0x1025d7c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d7f; continue;
 }
 0x1025d7f => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d89; continue; }
  pc = 0x1025d86; continue;
 }
 0x1025d86 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d89; continue;
 }
 0x1025d89 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025d93; continue; }
  pc = 0x1025d90; continue;
 }
 0x1025d90 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025d93; continue;
 }
 0x1025d93 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025da4; continue; }
  pc = 0x1025da2; continue;
 }
 0x1025da2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025da4; continue;
 }
 0x1025da4 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025dae; continue; }
  pc = 0x1025dab; continue;
 }
 0x1025dab => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025dae; continue;
 }
 0x1025dae => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025db8; continue; }
  pc = 0x1025db5; continue;
 }
 0x1025db5 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025db8; continue;
 }
 0x1025db8 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025dc2; continue; }
  pc = 0x1025dbf; continue;
 }
 0x1025dbf => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025dc2; continue;
 }
 0x1025dc2 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025dd3; continue; }
  pc = 0x1025dd1; continue;
 }
 0x1025dd1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025dd3; continue;
 }
 0x1025dd3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ddd; continue; }
  pc = 0x1025dda; continue;
 }
 0x1025dda => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ddd; continue;
 }
 0x1025ddd => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025de7; continue; }
  pc = 0x1025de4; continue;
 }
 0x1025de4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025de7; continue;
 }
 0x1025de7 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025df1; continue; }
  pc = 0x1025dee; continue;
 }
 0x1025dee => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025df1; continue;
 }
 0x1025df1 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e02; continue; }
  pc = 0x1025e00; continue;
 }
 0x1025e00 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e02; continue;
 }
 0x1025e02 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e0c; continue; }
  pc = 0x1025e09; continue;
 }
 0x1025e09 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e0c; continue;
 }
 0x1025e0c => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e16; continue; }
  pc = 0x1025e13; continue;
 }
 0x1025e13 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e16; continue;
 }
 0x1025e16 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e20; continue; }
  pc = 0x1025e1d; continue;
 }
 0x1025e1d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e20; continue;
 }
 0x1025e20 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1025e2b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x111a95cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a964u32)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.r32(0x111a954u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x10264e8; continue; }
  pc = 0x1025e4c; continue;
 }
 0x1025e4c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e54; continue; }
  pc = 0x1025e52; continue;
 }
 0x1025e52 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e54; continue;
 }
 0x1025e54 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e5d; continue; }
  pc = 0x1025e5a; continue;
 }
 0x1025e5a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e5d; continue;
 }
 0x1025e5d => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e67; continue; }
  pc = 0x1025e64; continue;
 }
 0x1025e64 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e67; continue;
 }
 0x1025e67 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e71; continue; }
  pc = 0x1025e6e; continue;
 }
 0x1025e6e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e71; continue;
 }
 0x1025e71 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e7b; continue; }
  pc = 0x1025e78; continue;
 }
 0x1025e78 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e7b; continue;
 }
 0x1025e7b => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e85; continue; }
  pc = 0x1025e82; continue;
 }
 0x1025e82 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e85; continue;
 }
 0x1025e85 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e8f; continue; }
  pc = 0x1025e8c; continue;
 }
 0x1025e8c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e8f; continue;
 }
 0x1025e8f => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025e99; continue; }
  pc = 0x1025e96; continue;
 }
 0x1025e96 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025e99; continue;
 }
 0x1025e99 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ea3; continue; }
  pc = 0x1025ea0; continue;
 }
 0x1025ea0 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ea3; continue;
 }
 0x1025ea3 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ead; continue; }
  pc = 0x1025eaa; continue;
 }
 0x1025eaa => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ead; continue;
 }
 0x1025ead => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025eb7; continue; }
  pc = 0x1025eb4; continue;
 }
 0x1025eb4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025eb7; continue;
 }
 0x1025eb7 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ec1; continue; }
  pc = 0x1025ebe; continue;
 }
 0x1025ebe => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ec1; continue;
 }
 0x1025ec1 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ecb; continue; }
  pc = 0x1025ec8; continue;
 }
 0x1025ec8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ecb; continue;
 }
 0x1025ecb => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ed5; continue; }
  pc = 0x1025ed2; continue;
 }
 0x1025ed2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ed5; continue;
 }
 0x1025ed5 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025edf; continue; }
  pc = 0x1025edc; continue;
 }
 0x1025edc => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025edf; continue;
 }
 0x1025edf => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ee9; continue; }
  pc = 0x1025ee6; continue;
 }
 0x1025ee6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ee9; continue;
 }
 0x1025ee9 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025efd; continue; }
  pc = 0x1025efb; continue;
 }
 0x1025efb => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025efd; continue;
 }
 0x1025efd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f06; continue; }
  pc = 0x1025f03; continue;
 }
 0x1025f03 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f06; continue;
 }
 0x1025f06 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f10; continue; }
  pc = 0x1025f0d; continue;
 }
 0x1025f0d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f10; continue;
 }
 0x1025f10 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f1a; continue; }
  pc = 0x1025f17; continue;
 }
 0x1025f17 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f1a; continue;
 }
 0x1025f1a => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f24; continue; }
  pc = 0x1025f21; continue;
 }
 0x1025f21 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f24; continue;
 }
 0x1025f24 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f2e; continue; }
  pc = 0x1025f2b; continue;
 }
 0x1025f2b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f2e; continue;
 }
 0x1025f2e => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f38; continue; }
  pc = 0x1025f35; continue;
 }
 0x1025f35 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f38; continue;
 }
 0x1025f38 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f42; continue; }
  pc = 0x1025f3f; continue;
 }
 0x1025f3f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f42; continue;
 }
 0x1025f42 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f4c; continue; }
  pc = 0x1025f49; continue;
 }
 0x1025f49 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f4c; continue;
 }
 0x1025f4c => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f56; continue; }
  pc = 0x1025f53; continue;
 }
 0x1025f53 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f56; continue;
 }
 0x1025f56 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f60; continue; }
  pc = 0x1025f5d; continue;
 }
 0x1025f5d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f60; continue;
 }
 0x1025f60 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f6a; continue; }
  pc = 0x1025f67; continue;
 }
 0x1025f67 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f6a; continue;
 }
 0x1025f6a => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f74; continue; }
  pc = 0x1025f71; continue;
 }
 0x1025f71 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f74; continue;
 }
 0x1025f74 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f7e; continue; }
  pc = 0x1025f7b; continue;
 }
 0x1025f7b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f7e; continue;
 }
 0x1025f7e => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f88; continue; }
  pc = 0x1025f85; continue;
 }
 0x1025f85 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f88; continue;
 }
 0x1025f88 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025f92; continue; }
  pc = 0x1025f8f; continue;
 }
 0x1025f8f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025f92; continue;
 }
 0x1025f92 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025fa6; continue; }
  pc = 0x1025fa4; continue;
 }
 0x1025fa4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025fa6; continue;
 }
 0x1025fa6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025faf; continue; }
  pc = 0x1025fac; continue;
 }
 0x1025fac => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025faf; continue;
 }
 0x1025faf => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025fb9; continue; }
  pc = 0x1025fb6; continue;
 }
 0x1025fb6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025fb9; continue;
 }
 0x1025fb9 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025fc3; continue; }
  pc = 0x1025fc0; continue;
 }
 0x1025fc0 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025fc3; continue;
 }
 0x1025fc3 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025fcd; continue; }
  pc = 0x1025fca; continue;
 }
 0x1025fca => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025fcd; continue;
 }
 0x1025fcd => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025fd7; continue; }
  pc = 0x1025fd4; continue;
 }
 0x1025fd4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x1025fd7; continue;
 }
 0x1025fd7 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025fe1; continue; }
  pc = 0x1025fde; continue;
 }
 0x1025fde => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025fe1; continue;
 }
 0x1025fe1 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025feb; continue; }
  pc = 0x1025fe8; continue;
 }
 0x1025fe8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025feb; continue;
 }
 0x1025feb => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025ff5; continue; }
  pc = 0x1025ff2; continue;
 }
 0x1025ff2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1025ff5; continue;
 }
 0x1025ff5 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1025fff; continue; }
  pc = 0x1025ffc; continue;
 }
 0x1025ffc => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x1025fff; continue;
 }
 0x1025fff => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026009; continue; }
  pc = 0x1026006; continue;
 }
 0x1026006 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026009; continue;
 }
 0x1026009 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026013; continue; }
  pc = 0x1026010; continue;
 }
 0x1026010 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026013; continue;
 }
 0x1026013 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102601d; continue; }
  pc = 0x102601a; continue;
 }
 0x102601a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x102601d; continue;
 }
 0x102601d => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026027; continue; }
  pc = 0x1026024; continue;
 }
 0x1026024 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026027; continue;
 }
 0x1026027 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026031; continue; }
  pc = 0x102602e; continue;
 }
 0x102602e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026031; continue;
 }
 0x1026031 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102603b; continue; }
  pc = 0x1026038; continue;
 }
 0x1026038 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x102603b; continue;
 }
 0x102603b => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102604f; continue; }
  pc = 0x102604d; continue;
 }
 0x102604d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102604f; continue;
 }
 0x102604f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026058; continue; }
  pc = 0x1026055; continue;
 }
 0x1026055 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026058; continue;
 }
 0x1026058 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026062; continue; }
  pc = 0x102605f; continue;
 }
 0x102605f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026062; continue;
 }
 0x1026062 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102606c; continue; }
  pc = 0x1026069; continue;
 }
 0x1026069 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x102606c; continue;
 }
 0x102606c => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026076; continue; }
  pc = 0x1026073; continue;
 }
 0x1026073 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026076; continue;
 }
 0x1026076 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026080; continue; }
  pc = 0x102607d; continue;
 }
 0x102607d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x1026080; continue;
 }
 0x1026080 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102608a; continue; }
  pc = 0x1026087; continue;
 }
 0x1026087 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x102608a; continue;
 }
 0x102608a => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026094; continue; }
  pc = 0x1026091; continue;
 }
 0x1026091 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026094; continue;
 }
 0x1026094 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102609e; continue; }
  pc = 0x102609b; continue;
 }
 0x102609b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x102609e; continue;
 }
 0x102609e => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10260a8; continue; }
  pc = 0x10260a5; continue;
 }
 0x10260a5 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x10260a8; continue;
 }
 0x10260a8 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10260b2; continue; }
  pc = 0x10260af; continue;
 }
 0x10260af => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x10260b2; continue;
 }
 0x10260b2 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10260bc; continue; }
  pc = 0x10260b9; continue;
 }
 0x10260b9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x10260bc; continue;
 }
 0x10260bc => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10260c6; continue; }
  pc = 0x10260c3; continue;
 }
 0x10260c3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x10260c6; continue;
 }
 0x10260c6 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10260d0; continue; }
  pc = 0x10260cd; continue;
 }
 0x10260cd => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x10260d0; continue;
 }
 0x10260d0 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10260da; continue; }
  pc = 0x10260d7; continue;
 }
 0x10260d7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x10260da; continue;
 }
 0x10260da => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10260e4; continue; }
  pc = 0x10260e1; continue;
 }
 0x10260e1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x10260e4; continue;
 }
 0x10260e4 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10260f8; continue; }
  pc = 0x10260f6; continue;
 }
 0x10260f6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10260f8; continue;
 }
 0x10260f8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026101; continue; }
  pc = 0x10260fe; continue;
 }
 0x10260fe => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026101; continue;
 }
 0x1026101 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102610b; continue; }
  pc = 0x1026108; continue;
 }
 0x1026108 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x102610b; continue;
 }
 0x102610b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026115; continue; }
  pc = 0x1026112; continue;
 }
 0x1026112 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026115; continue;
 }
 0x1026115 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102611f; continue; }
  pc = 0x102611c; continue;
 }
 0x102611c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x102611f; continue;
 }
 0x102611f => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026129; continue; }
  pc = 0x1026126; continue;
 }
 0x1026126 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x1026129; continue;
 }
 0x1026129 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026133; continue; }
  pc = 0x1026130; continue;
 }
 0x1026130 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026133; continue;
 }
 0x1026133 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102613d; continue; }
  pc = 0x102613a; continue;
 }
 0x102613a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x102613d; continue;
 }
 0x102613d => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026147; continue; }
  pc = 0x1026144; continue;
 }
 0x1026144 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026147; continue;
 }
 0x1026147 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026151; continue; }
  pc = 0x102614e; continue;
 }
 0x102614e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026151; continue;
 }
 0x1026151 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102615b; continue; }
  pc = 0x1026158; continue;
 }
 0x1026158 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x102615b; continue;
 }
 0x102615b => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026165; continue; }
  pc = 0x1026162; continue;
 }
 0x1026162 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026165; continue;
 }
 0x1026165 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102616f; continue; }
  pc = 0x102616c; continue;
 }
 0x102616c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x102616f; continue;
 }
 0x102616f => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026179; continue; }
  pc = 0x1026176; continue;
 }
 0x1026176 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026179; continue;
 }
 0x1026179 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026183; continue; }
  pc = 0x1026180; continue;
 }
 0x1026180 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026183; continue;
 }
 0x1026183 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102618d; continue; }
  pc = 0x102618a; continue;
 }
 0x102618a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x102618d; continue;
 }
 0x102618d => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10261a1; continue; }
  pc = 0x102619f; continue;
 }
 0x102619f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10261a1; continue;
 }
 0x10261a1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10261aa; continue; }
  pc = 0x10261a7; continue;
 }
 0x10261a7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x10261aa; continue;
 }
 0x10261aa => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10261b4; continue; }
  pc = 0x10261b1; continue;
 }
 0x10261b1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x10261b4; continue;
 }
 0x10261b4 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10261be; continue; }
  pc = 0x10261bb; continue;
 }
 0x10261bb => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x10261be; continue;
 }
 0x10261be => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10261c8; continue; }
  pc = 0x10261c5; continue;
 }
 0x10261c5 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x10261c8; continue;
 }
 0x10261c8 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10261d2; continue; }
  pc = 0x10261cf; continue;
 }
 0x10261cf => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x10261d2; continue;
 }
 0x10261d2 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10261dc; continue; }
  pc = 0x10261d9; continue;
 }
 0x10261d9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x10261dc; continue;
 }
 0x10261dc => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10261e6; continue; }
  pc = 0x10261e3; continue;
 }
 0x10261e3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x10261e6; continue;
 }
 0x10261e6 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10261f0; continue; }
  pc = 0x10261ed; continue;
 }
 0x10261ed => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x10261f0; continue;
 }
 0x10261f0 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10261fa; continue; }
  pc = 0x10261f7; continue;
 }
 0x10261f7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x10261fa; continue;
 }
 0x10261fa => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026204; continue; }
  pc = 0x1026201; continue;
 }
 0x1026201 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026204; continue;
 }
 0x1026204 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102620e; continue; }
  pc = 0x102620b; continue;
 }
 0x102620b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x102620e; continue;
 }
 0x102620e => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026218; continue; }
  pc = 0x1026215; continue;
 }
 0x1026215 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026218; continue;
 }
 0x1026218 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026222; continue; }
  pc = 0x102621f; continue;
 }
 0x102621f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026222; continue;
 }
 0x1026222 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102622c; continue; }
  pc = 0x1026229; continue;
 }
 0x1026229 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x102622c; continue;
 }
 0x102622c => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026236; continue; }
  pc = 0x1026233; continue;
 }
 0x1026233 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026236; continue;
 }
 0x1026236 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102624a; continue; }
  pc = 0x1026248; continue;
 }
 0x1026248 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102624a; continue;
 }
 0x102624a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026253; continue; }
  pc = 0x1026250; continue;
 }
 0x1026250 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026253; continue;
 }
 0x1026253 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102625d; continue; }
  pc = 0x102625a; continue;
 }
 0x102625a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x102625d; continue;
 }
 0x102625d => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026267; continue; }
  pc = 0x1026264; continue;
 }
 0x1026264 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026267; continue;
 }
 0x1026267 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026271; continue; }
  pc = 0x102626e; continue;
 }
 0x102626e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026271; continue;
 }
 0x1026271 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102627b; continue; }
  pc = 0x1026278; continue;
 }
 0x1026278 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x102627b; continue;
 }
 0x102627b => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026285; continue; }
  pc = 0x1026282; continue;
 }
 0x1026282 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026285; continue;
 }
 0x1026285 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102628f; continue; }
  pc = 0x102628c; continue;
 }
 0x102628c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x102628f; continue;
 }
 0x102628f => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026299; continue; }
  pc = 0x1026296; continue;
 }
 0x1026296 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026299; continue;
 }
 0x1026299 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10262a3; continue; }
  pc = 0x10262a0; continue;
 }
 0x10262a0 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x10262a3; continue;
 }
 0x10262a3 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10262ad; continue; }
  pc = 0x10262aa; continue;
 }
 0x10262aa => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x10262ad; continue;
 }
 0x10262ad => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10262b7; continue; }
  pc = 0x10262b4; continue;
 }
 0x10262b4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x10262b7; continue;
 }
 0x10262b7 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10262c1; continue; }
  pc = 0x10262be; continue;
 }
 0x10262be => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x10262c1; continue;
 }
 0x10262c1 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10262cb; continue; }
  pc = 0x10262c8; continue;
 }
 0x10262c8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x10262cb; continue;
 }
 0x10262cb => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10262d5; continue; }
  pc = 0x10262d2; continue;
 }
 0x10262d2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x10262d5; continue;
 }
 0x10262d5 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10262df; continue; }
  pc = 0x10262dc; continue;
 }
 0x10262dc => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x10262df; continue;
 }
 0x10262df => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10262f3; continue; }
  pc = 0x10262f1; continue;
 }
 0x10262f1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10262f3; continue;
 }
 0x10262f3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10262fc; continue; }
  pc = 0x10262f9; continue;
 }
 0x10262f9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x10262fc; continue;
 }
 0x10262fc => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026306; continue; }
  pc = 0x1026303; continue;
 }
 0x1026303 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026306; continue;
 }
 0x1026306 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026310; continue; }
  pc = 0x102630d; continue;
 }
 0x102630d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026310; continue;
 }
 0x1026310 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102631a; continue; }
  pc = 0x1026317; continue;
 }
 0x1026317 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x102631a; continue;
 }
 0x102631a => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026324; continue; }
  pc = 0x1026321; continue;
 }
 0x1026321 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x1026324; continue;
 }
 0x1026324 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102632e; continue; }
  pc = 0x102632b; continue;
 }
 0x102632b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x102632e; continue;
 }
 0x102632e => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026338; continue; }
  pc = 0x1026335; continue;
 }
 0x1026335 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026338; continue;
 }
 0x1026338 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026342; continue; }
  pc = 0x102633f; continue;
 }
 0x102633f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026342; continue;
 }
 0x1026342 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102634c; continue; }
  pc = 0x1026349; continue;
 }
 0x1026349 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x102634c; continue;
 }
 0x102634c => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026356; continue; }
  pc = 0x1026353; continue;
 }
 0x1026353 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026356; continue;
 }
 0x1026356 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026360; continue; }
  pc = 0x102635d; continue;
 }
 0x102635d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026360; continue;
 }
 0x1026360 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102636a; continue; }
  pc = 0x1026367; continue;
 }
 0x1026367 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x102636a; continue;
 }
 0x102636a => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026374; continue; }
  pc = 0x1026371; continue;
 }
 0x1026371 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026374; continue;
 }
 0x1026374 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102637e; continue; }
  pc = 0x102637b; continue;
 }
 0x102637b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x102637e; continue;
 }
 0x102637e => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026388; continue; }
  pc = 0x1026385; continue;
 }
 0x1026385 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026388; continue;
 }
 0x1026388 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102639c; continue; }
  pc = 0x102639a; continue;
 }
 0x102639a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102639c; continue;
 }
 0x102639c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10263a5; continue; }
  pc = 0x10263a2; continue;
 }
 0x10263a2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x10263a5; continue;
 }
 0x10263a5 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10263af; continue; }
  pc = 0x10263ac; continue;
 }
 0x10263ac => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x10263af; continue;
 }
 0x10263af => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10263b9; continue; }
  pc = 0x10263b6; continue;
 }
 0x10263b6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x10263b9; continue;
 }
 0x10263b9 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10263c3; continue; }
  pc = 0x10263c0; continue;
 }
 0x10263c0 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x10263c3; continue;
 }
 0x10263c3 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10263cd; continue; }
  pc = 0x10263ca; continue;
 }
 0x10263ca => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x10263cd; continue;
 }
 0x10263cd => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10263d7; continue; }
  pc = 0x10263d4; continue;
 }
 0x10263d4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x10263d7; continue;
 }
 0x10263d7 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10263e1; continue; }
  pc = 0x10263de; continue;
 }
 0x10263de => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x10263e1; continue;
 }
 0x10263e1 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10263eb; continue; }
  pc = 0x10263e8; continue;
 }
 0x10263e8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x10263eb; continue;
 }
 0x10263eb => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10263f5; continue; }
  pc = 0x10263f2; continue;
 }
 0x10263f2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x10263f5; continue;
 }
 0x10263f5 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10263ff; continue; }
  pc = 0x10263fc; continue;
 }
 0x10263fc => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x10263ff; continue;
 }
 0x10263ff => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026409; continue; }
  pc = 0x1026406; continue;
 }
 0x1026406 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026409; continue;
 }
 0x1026409 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026413; continue; }
  pc = 0x1026410; continue;
 }
 0x1026410 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026413; continue;
 }
 0x1026413 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102641d; continue; }
  pc = 0x102641a; continue;
 }
 0x102641a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x102641d; continue;
 }
 0x102641d => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026427; continue; }
  pc = 0x1026424; continue;
 }
 0x1026424 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026427; continue;
 }
 0x1026427 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026431; continue; }
  pc = 0x102642e; continue;
 }
 0x102642e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x1026431; continue;
 }
 0x1026431 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026445; continue; }
  pc = 0x1026443; continue;
 }
 0x1026443 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026445; continue;
 }
 0x1026445 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102644e; continue; }
  pc = 0x102644b; continue;
 }
 0x102644b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x102644e; continue;
 }
 0x102644e => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026458; continue; }
  pc = 0x1026455; continue;
 }
 0x1026455 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026458; continue;
 }
 0x1026458 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026462; continue; }
  pc = 0x102645f; continue;
 }
 0x102645f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026462; continue;
 }
 0x1026462 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102646c; continue; }
  pc = 0x1026469; continue;
 }
 0x1026469 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x102646c; continue;
 }
 0x102646c => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026476; continue; }
  pc = 0x1026473; continue;
 }
 0x1026473 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x1026476; continue;
 }
 0x1026476 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026480; continue; }
  pc = 0x102647d; continue;
 }
 0x102647d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026480; continue;
 }
 0x1026480 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102648a; continue; }
  pc = 0x1026487; continue;
 }
 0x1026487 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  pc = 0x102648a; continue;
 }
 0x102648a => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026494; continue; }
  pc = 0x1026491; continue;
 }
 0x1026491 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026494; continue;
 }
 0x1026494 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102649e; continue; }
  pc = 0x102649b; continue;
 }
 0x102649b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x102649e; continue;
 }
 0x102649e => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10264a8; continue; }
  pc = 0x10264a5; continue;
 }
 0x10264a5 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x10264a8; continue;
 }
 0x10264a8 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10264b2; continue; }
  pc = 0x10264af; continue;
 }
 0x10264af => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x10264b2; continue;
 }
 0x10264b2 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10264bc; continue; }
  pc = 0x10264b9; continue;
 }
 0x10264b9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x10264bc; continue;
 }
 0x10264bc => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10264c6; continue; }
  pc = 0x10264c3; continue;
 }
 0x10264c3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x10264c6; continue;
 }
 0x10264c6 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10264d0; continue; }
  pc = 0x10264cd; continue;
 }
 0x10264cd => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x10264d0; continue;
 }
 0x10264d0 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10264da; continue; }
  pc = 0x10264d7; continue;
 }
 0x10264d7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x10264da; continue;
 }
 0x10264da => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10264e8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10264f0; continue; }
  pc = 0x10264ee; continue;
 }
 0x10264ee => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10264f0; continue;
 }
 0x10264f0 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10264fa; continue; }
  pc = 0x10264f7; continue;
 }
 0x10264f7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x10264fa; continue;
 }
 0x10264fa => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026504; continue; }
  pc = 0x1026501; continue;
 }
 0x1026501 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026504; continue;
 }
 0x1026504 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102650e; continue; }
  pc = 0x102650b; continue;
 }
 0x102650b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x102650e; continue;
 }
 0x102650e => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026518; continue; }
  pc = 0x1026515; continue;
 }
 0x1026515 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026518; continue;
 }
 0x1026518 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026522; continue; }
  pc = 0x102651f; continue;
 }
 0x102651f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026522; continue;
 }
 0x1026522 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102652c; continue; }
  pc = 0x1026529; continue;
 }
 0x1026529 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x102652c; continue;
 }
 0x102652c => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026536; continue; }
  pc = 0x1026533; continue;
 }
 0x1026533 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026536; continue;
 }
 0x1026536 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026547; continue; }
  pc = 0x1026545; continue;
 }
 0x1026545 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026547; continue;
 }
 0x1026547 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026551; continue; }
  pc = 0x102654e; continue;
 }
 0x102654e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026551; continue;
 }
 0x1026551 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102655b; continue; }
  pc = 0x1026558; continue;
 }
 0x1026558 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x102655b; continue;
 }
 0x102655b => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026565; continue; }
  pc = 0x1026562; continue;
 }
 0x1026562 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026565; continue;
 }
 0x1026565 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102656f; continue; }
  pc = 0x102656c; continue;
 }
 0x102656c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x102656f; continue;
 }
 0x102656f => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026579; continue; }
  pc = 0x1026576; continue;
 }
 0x1026576 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026579; continue;
 }
 0x1026579 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026583; continue; }
  pc = 0x1026580; continue;
 }
 0x1026580 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026583; continue;
 }
 0x1026583 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102658d; continue; }
  pc = 0x102658a; continue;
 }
 0x102658a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x102658d; continue;
 }
 0x102658d => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102659e; continue; }
  pc = 0x102659c; continue;
 }
 0x102659c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102659e; continue;
 }
 0x102659e => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10265a8; continue; }
  pc = 0x10265a5; continue;
 }
 0x10265a5 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x10265a8; continue;
 }
 0x10265a8 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10265b2; continue; }
  pc = 0x10265af; continue;
 }
 0x10265af => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x10265b2; continue;
 }
 0x10265b2 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10265bc; continue; }
  pc = 0x10265b9; continue;
 }
 0x10265b9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x10265bc; continue;
 }
 0x10265bc => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10265c6; continue; }
  pc = 0x10265c3; continue;
 }
 0x10265c3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x10265c6; continue;
 }
 0x10265c6 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10265d0; continue; }
  pc = 0x10265cd; continue;
 }
 0x10265cd => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x10265d0; continue;
 }
 0x10265d0 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10265da; continue; }
  pc = 0x10265d7; continue;
 }
 0x10265d7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x10265da; continue;
 }
 0x10265da => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10265e4; continue; }
  pc = 0x10265e1; continue;
 }
 0x10265e1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x10265e4; continue;
 }
 0x10265e4 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10265f5; continue; }
  pc = 0x10265f3; continue;
 }
 0x10265f3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10265f5; continue;
 }
 0x10265f5 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10265ff; continue; }
  pc = 0x10265fc; continue;
 }
 0x10265fc => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x10265ff; continue;
 }
 0x10265ff => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026609; continue; }
  pc = 0x1026606; continue;
 }
 0x1026606 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026609; continue;
 }
 0x1026609 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026613; continue; }
  pc = 0x1026610; continue;
 }
 0x1026610 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026613; continue;
 }
 0x1026613 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102661d; continue; }
  pc = 0x102661a; continue;
 }
 0x102661a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x102661d; continue;
 }
 0x102661d => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026627; continue; }
  pc = 0x1026624; continue;
 }
 0x1026624 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026627; continue;
 }
 0x1026627 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026631; continue; }
  pc = 0x102662e; continue;
 }
 0x102662e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026631; continue;
 }
 0x1026631 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102663b; continue; }
  pc = 0x1026638; continue;
 }
 0x1026638 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x102663b; continue;
 }
 0x102663b => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102664c; continue; }
  pc = 0x102664a; continue;
 }
 0x102664a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102664c; continue;
 }
 0x102664c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026656; continue; }
  pc = 0x1026653; continue;
 }
 0x1026653 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026656; continue;
 }
 0x1026656 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026660; continue; }
  pc = 0x102665d; continue;
 }
 0x102665d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026660; continue;
 }
 0x1026660 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102666a; continue; }
  pc = 0x1026667; continue;
 }
 0x1026667 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x102666a; continue;
 }
 0x102666a => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026674; continue; }
  pc = 0x1026671; continue;
 }
 0x1026671 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026674; continue;
 }
 0x1026674 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102667e; continue; }
  pc = 0x102667b; continue;
 }
 0x102667b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x102667e; continue;
 }
 0x102667e => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026688; continue; }
  pc = 0x1026685; continue;
 }
 0x1026685 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026688; continue;
 }
 0x1026688 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026692; continue; }
  pc = 0x102668f; continue;
 }
 0x102668f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026692; continue;
 }
 0x1026692 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10266a3; continue; }
  pc = 0x10266a1; continue;
 }
 0x10266a1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10266a3; continue;
 }
 0x10266a3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10266ad; continue; }
  pc = 0x10266aa; continue;
 }
 0x10266aa => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x10266ad; continue;
 }
 0x10266ad => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10266b7; continue; }
  pc = 0x10266b4; continue;
 }
 0x10266b4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x10266b7; continue;
 }
 0x10266b7 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10266c1; continue; }
  pc = 0x10266be; continue;
 }
 0x10266be => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x10266c1; continue;
 }
 0x10266c1 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10266cb; continue; }
  pc = 0x10266c8; continue;
 }
 0x10266c8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x10266cb; continue;
 }
 0x10266cb => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10266d5; continue; }
  pc = 0x10266d2; continue;
 }
 0x10266d2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x10266d5; continue;
 }
 0x10266d5 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10266df; continue; }
  pc = 0x10266dc; continue;
 }
 0x10266dc => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x10266df; continue;
 }
 0x10266df => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10266e9; continue; }
  pc = 0x10266e6; continue;
 }
 0x10266e6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x10266e9; continue;
 }
 0x10266e9 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10266fa; continue; }
  pc = 0x10266f8; continue;
 }
 0x10266f8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10266fa; continue;
 }
 0x10266fa => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026704; continue; }
  pc = 0x1026701; continue;
 }
 0x1026701 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026704; continue;
 }
 0x1026704 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102670e; continue; }
  pc = 0x102670b; continue;
 }
 0x102670b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x102670e; continue;
 }
 0x102670e => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026718; continue; }
  pc = 0x1026715; continue;
 }
 0x1026715 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026718; continue;
 }
 0x1026718 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026722; continue; }
  pc = 0x102671f; continue;
 }
 0x102671f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026722; continue;
 }
 0x1026722 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102672c; continue; }
  pc = 0x1026729; continue;
 }
 0x1026729 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x102672c; continue;
 }
 0x102672c => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026736; continue; }
  pc = 0x1026733; continue;
 }
 0x1026733 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026736; continue;
 }
 0x1026736 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026740; continue; }
  pc = 0x102673d; continue;
 }
 0x102673d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026740; continue;
 }
 0x1026740 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026751; continue; }
  pc = 0x102674f; continue;
 }
 0x102674f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026751; continue;
 }
 0x1026751 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102675b; continue; }
  pc = 0x1026758; continue;
 }
 0x1026758 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x102675b; continue;
 }
 0x102675b => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026765; continue; }
  pc = 0x1026762; continue;
 }
 0x1026762 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026765; continue;
 }
 0x1026765 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102676f; continue; }
  pc = 0x102676c; continue;
 }
 0x102676c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x102676f; continue;
 }
 0x102676f => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026779; continue; }
  pc = 0x1026776; continue;
 }
 0x1026776 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026779; continue;
 }
 0x1026779 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026783; continue; }
  pc = 0x1026780; continue;
 }
 0x1026780 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026783; continue;
 }
 0x1026783 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102678d; continue; }
  pc = 0x102678a; continue;
 }
 0x102678a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x102678d; continue;
 }
 0x102678d => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026797; continue; }
  pc = 0x1026794; continue;
 }
 0x1026794 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026797; continue;
 }
 0x1026797 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10267a8; continue; }
  pc = 0x10267a6; continue;
 }
 0x10267a6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10267a8; continue;
 }
 0x10267a8 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10267b2; continue; }
  pc = 0x10267af; continue;
 }
 0x10267af => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x10267b2; continue;
 }
 0x10267b2 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10267bc; continue; }
  pc = 0x10267b9; continue;
 }
 0x10267b9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x10267bc; continue;
 }
 0x10267bc => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10267c6; continue; }
  pc = 0x10267c3; continue;
 }
 0x10267c3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x10267c6; continue;
 }
 0x10267c6 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10267d0; continue; }
  pc = 0x10267cd; continue;
 }
 0x10267cd => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x10267d0; continue;
 }
 0x10267d0 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10267da; continue; }
  pc = 0x10267d7; continue;
 }
 0x10267d7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x10267da; continue;
 }
 0x10267da => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10267e4; continue; }
  pc = 0x10267e1; continue;
 }
 0x10267e1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x10267e4; continue;
 }
 0x10267e4 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10267ee; continue; }
  pc = 0x10267eb; continue;
 }
 0x10267eb => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x10267ee; continue;
 }
 0x10267ee => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10267ff; continue; }
  pc = 0x10267fd; continue;
 }
 0x10267fd => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10267ff; continue;
 }
 0x10267ff => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026809; continue; }
  pc = 0x1026806; continue;
 }
 0x1026806 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026809; continue;
 }
 0x1026809 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026813; continue; }
  pc = 0x1026810; continue;
 }
 0x1026810 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026813; continue;
 }
 0x1026813 => {
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102681d; continue; }
  pc = 0x102681a; continue;
 }
 0x102681a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  pc = 0x102681d; continue;
 }
 0x102681d => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026827; continue; }
  pc = 0x1026824; continue;
 }
 0x1026824 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026827; continue;
 }
 0x1026827 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026831; continue; }
  pc = 0x102682e; continue;
 }
 0x102682e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026831; continue;
 }
 0x1026831 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x102683b; continue; }
  pc = 0x1026838; continue;
 }
 0x1026838 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x102683b; continue;
 }
 0x102683b => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1026845; continue; }
  pc = 0x1026842; continue;
 }
 0x1026842 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1026845; continue;
 }
 0x1026845 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a954u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026850); }
 let mut pc: u32 = 0x1026850;
 loop { match pc {
 0x1026850 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u32 = 0x111a350u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x111a978u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u8 = cpu.r8(0x111a979u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u8 = cpu.r8(0x111a97au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u8 = cpu.r8(0x111a97bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  pc = 0x10268a6; continue;
 }
 0x10268a6 => {
  { cpu.stos(1, false); }
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10268a6; continue; }
  pc = 0x10268ac; continue;
 }
 0x10268ac => {
  { let v: u32 = cpu.r32(0x111a978u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111a97cu32, v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010268b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10268b8); }
 let mut pc: u32 = 0x10268b8;
 loop { match pc {
 0x10268b8 => {
  cpu.push(0x10268bd); f_010268ed(cpu); if cpu.returned_to != 0x10268bd { return; }
  cpu.push(0x10268c2); f_01026ae0(cpu); if cpu.returned_to != 0x10268c2 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x111a960u32, v); }
  f_010268cc(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010268cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10268cc); }
 let mut pc: u32 = 0x10268cc;
 loop { match pc {
 0x10268cc => {
  { cpu.sub32(cpu.r32(0x111a960u32), 0x0u32); }
  if !cpu.zf { pc = 0x10268da; continue; }
  pc = 0x10268d5; continue;
 }
 0x10268d5 => {
  cpu.push(0x10268da); f_01026ae0(cpu); if cpu.returned_to != 0x10268da { return; }
  pc = 0x10268da; continue;
 }
 0x10268da => {
  { let v: u32 = cpu.r32(0x111a960u32) & 0x1u32; cpu.w32(0x111a960u32, v); }
  { let v: u32 = cpu.or32(cpu.r32(0x111a960u32), 0x22u32); cpu.w32(0x111a960u32, v); }
  cpu.push(0x10268ed); f_010268ed(cpu); if cpu.returned_to != 0x10268ed { return; }
  f_010268ed(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010268ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10268ed); }
 let mut pc: u32 = 0x10268ed;
 loop { match pc {
 0x10268ed => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1119f30u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a970u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x111a974u32)); cpu.ecx = v; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.r32(0x111a970u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111ea02u32, v); }
  { let v: u32 = cpu.r32(0x111a974u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111ea06u32, v); }
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x1026923; continue;
 }
 0x1026923 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1026964; continue; }
  pc = 0x1026929; continue;
 }
 0x1026929 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.esi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x7u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if cpu.cf { pc = 0x1026957; continue; }
  pc = 0x1026952; continue;
 }
 0x1026952 => {
  { let v: u32 = 0x4u32; cpu.esi = v; }
  pc = 0x1026957; continue;
 }
 0x1026957 => {
  { let v: u32 = cpu.r32(0x1119f30u32); cpu.edi = v; }
  cpu.push(0x1026962); f_01025ae5(cpu); if cpu.returned_to != 0x1026962 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1026964; continue;
 }
 0x1026964 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1026923; continue; }
  pc = 0x1026969; continue;
 }
 0x1026969 => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x1026973; continue;
 }
 0x1026973 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x10269a8; continue; }
  pc = 0x1026979; continue;
 }
 0x1026979 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1119f30u32); cpu.edi = v; }
  cpu.push(0x10269a6); f_01025b09(cpu); if cpu.returned_to != 0x10269a6 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x10269a8; continue;
 }
 0x10269a8 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1026973; continue; }
  pc = 0x10269ad; continue;
 }
 0x10269ad => {
  { let v: u32 = 0x2u32; cpu.w32(0x111ea0au32, v); }
  cpu.push(0x10269bc); f_010269be(cpu); if cpu.returned_to != 0x10269bc { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010269be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10269be); }
 let mut pc: u32 = 0x10269be;
 loop { match pc {
 0x10269be => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1119f34u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a970u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x111a974u32)); cpu.ecx = v; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.r32(0x111a970u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111ea02u32, v); }
  { let v: u32 = cpu.r32(0x111a974u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111ea06u32, v); }
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x10269f4; continue;
 }
 0x10269f4 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1026a30; continue; }
  pc = 0x10269fa; continue;
 }
 0x10269fa => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.esi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x7u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if !cpu.zf { pc = 0x1026a2e; continue; }
  pc = 0x1026a23; continue;
 }
 0x1026a23 => {
  { let v: u32 = cpu.r32(0x1119f34u32); cpu.edi = v; }
  cpu.push(0x1026a2e); f_01025b2d(cpu); if cpu.returned_to != 0x1026a2e { return; }
  pc = 0x1026a2e; continue;
 }
 0x1026a2e => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1026a30; continue;
 }
 0x1026a30 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10269f4; continue; }
  pc = 0x1026a35; continue;
 }
 0x1026a35 => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x1026a3f; continue;
 }
 0x1026a3f => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1026a7b; continue; }
  pc = 0x1026a45; continue;
 }
 0x1026a45 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.esi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x7u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if cpu.zf { pc = 0x1026a79; continue; }
  pc = 0x1026a6e; continue;
 }
 0x1026a6e => {
  { let v: u32 = cpu.r32(0x1119f34u32); cpu.edi = v; }
  cpu.push(0x1026a79); f_01025b2d(cpu); if cpu.returned_to != 0x1026a79 { return; }
  pc = 0x1026a79; continue;
 }
 0x1026a79 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1026a7b; continue;
 }
 0x1026a7b => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1026a3f; continue; }
  pc = 0x1026a80; continue;
 }
 0x1026a80 => {
  { let v: u32 = 0x2u32; cpu.w32(0x111ea0au32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026a8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026a8c); }
 let mut pc: u32 = 0x1026a8c;
 loop { match pc {
 0x1026a8c => {
  { cpu.sub32(cpu.r32(0x111a960u32), 0x0u32); }
  if cpu.zf { pc = 0x1026a9b; continue; }
  pc = 0x1026a95; continue;
 }
 0x1026a95 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x111a960u32)); cpu.w32(0x111a960u32, v); }
  pc = 0x1026a9b; continue;
 }
 0x1026a9b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026a9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026a9c); }
 let mut pc: u32 = 0x1026a9c;
 loop { match pc {
 0x1026a9c => {
  { let v: u32 = cpu.r32(0x1119f34u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x111a960u32), 0x0u32); }
  if cpu.zf { pc = 0x1026abd; continue; }
  pc = 0x1026aab; continue;
 }
 0x1026aab => {
  { cpu.and32(cpu.r32(0x111a960u32), 0x1u32); }
  if cpu.zf { pc = 0x1026abd; continue; }
  pc = 0x1026ab7; continue;
 }
 0x1026ab7 => {
  { let v: u32 = cpu.r32(0x1119f3cu32); cpu.esi = v; }
  pc = 0x1026abd; continue;
 }
 0x1026abd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026abe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026abe); }
 let mut pc: u32 = 0x1026abe;
 loop { match pc {
 0x1026abe => {
  { let v: u32 = cpu.r32(0x1119f30u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x111a960u32), 0x0u32); }
  if cpu.zf { pc = 0x1026adf; continue; }
  pc = 0x1026acd; continue;
 }
 0x1026acd => {
  { cpu.and32(cpu.r32(0x111a960u32), 0x1u32); }
  if cpu.zf { pc = 0x1026adf; continue; }
  pc = 0x1026ad9; continue;
 }
 0x1026ad9 => {
  { let v: u32 = cpu.r32(0x1119f38u32); cpu.esi = v; }
  pc = 0x1026adf; continue;
 }
 0x1026adf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026ae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026ae0); }
 let mut pc: u32 = 0x1026ae0;
 loop { match pc {
 0x1026ae0 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1119f30u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1119f38u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a970u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x111a974u32)); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1119f34u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1119f3cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x111a970u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x111a974u32)); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026b19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026b19); }
 let mut pc: u32 = 0x1026b19;
 loop { match pc {
 0x1026b19 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10fcc02u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fcbfau32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x10fcbfeu32)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10fcbfeu32, v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  if !cpu.sf { pc = 0x1026b39; continue; }
  pc = 0x1026b37; continue;
 }
 0x1026b37 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1026b39; continue;
 }
 0x1026b39 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10fcc02u32, v); }
  cpu.push(0x1026b43); f_01026dee(cpu); if cpu.returned_to != 0x1026b43 { return; }
  cpu.push(0x1026b48); f_0102332f(cpu); if cpu.returned_to != 0x1026b48 { return; }
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111a958u32, v); }
  { let v: u32 = cpu.r32(0x1119f10u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x111a95cu32, v); }
  { let v: u32 = 0x30u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1026b6f; continue; }
  pc = 0x1026b6a; continue;
 }
 0x1026b6a => {
  { let v: u32 = 0x60u32; cpu.eax = v; }
  pc = 0x1026b6f; continue;
 }
 0x1026b6f => {
  { let v: u32 = cpu.eax; cpu.w32(0x111a954u32, v); }
  { let v: u32 = 0xc8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1026b87; continue; }
  pc = 0x1026b82; continue;
 }
 0x1026b82 => {
  { let v: u32 = 0x190u32; cpu.eax = v; }
  pc = 0x1026b87; continue;
 }
 0x1026b87 => {
  { let v: u32 = cpu.eax; cpu.w32(0x111a950u32, v); }
  cpu.push(0x1026b91); f_0102514b(cpu); if cpu.returned_to != 0x1026b91 { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e41b0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e41b4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e41b8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10e419cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e41a4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e41a0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e41a4u32); cpu.edx = v; }
  cpu.push(0x1026bc4); f_0101fe7c(cpu); if cpu.returned_to != 0x1026bc4 { return; }
  cpu.push(0x1026bc9); f_0101e85a(cpu); if cpu.returned_to != 0x1026bc9 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3d6bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1026bfa; continue; }
  pc = 0x1026bd4; continue;
 }
 0x1026bd4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3d6du32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x10e3d6fu32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x10e3d71u32) as u32; cpu.edx = v; }
  cpu.push(0x1026bf3); f_0101f9a1(cpu); if cpu.returned_to != 0x1026bf3 { return; }
  { let v: u8 = 0x3u8; cpu.w8(0x111e9e9u32, v); }
  pc = 0x1026bfa; continue;
 }
 0x1026bfa => {
  cpu.push(0x1026bff); f_01022b56(cpu); if cpu.returned_to != 0x1026bff { return; }
  cpu.push(0x1026c04); f_0101cbf7(cpu); if cpu.returned_to != 0x1026c04 { return; }
  cpu.push(0x1026c09); f_0101cd6f(cpu); if cpu.returned_to != 0x1026c09 { return; }
  cpu.push(0x1026c0e); f_0103cf02(cpu); if cpu.returned_to != 0x1026c0e { return; }
  { cpu.and8(cpu.r8(0x10ef352u32), 0xffu8); }
  if !cpu.zf { pc = 0x1026c1e; continue; }
  pc = 0x1026c17; continue;
 }
 0x1026c17 => {
  cpu.push(0x1026c1c); f_0101dc69(cpu); if cpu.returned_to != 0x1026c1c { return; }
  pc = 0x1026c23; continue;
 }
 0x1026c1e => {
  cpu.push(0x1026c23); f_0101e5df(cpu); if cpu.returned_to != 0x1026c23 { return; }
  pc = 0x1026c23; continue;
 }
 0x1026c23 => {
  cpu.push(0x1026c28); f_0101cc10(cpu); if cpu.returned_to != 0x1026c28 { return; }
  { cpu.sub32(cpu.r32(0x110eab4u32), 0x0u32); }
  if cpu.zf { pc = 0x1026c52; continue; }
  pc = 0x1026c31; continue;
 }
 0x1026c31 => {
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x0u32); }
  if cpu.zf { pc = 0x1026c52; continue; }
  pc = 0x1026c3a; continue;
 }
 0x1026c3a => {
  { let v: u8 = cpu.r8(0x10ef352u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef352u32, v); }
  cpu.push(0x1026c4c); f_0101cd6f(cpu); if cpu.returned_to != 0x1026c4c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef352u32, v); }
  pc = 0x1026c52; continue;
 }
 0x1026c52 => {
  cpu.push(0x1026c57); f_0101cbf7(cpu); if cpu.returned_to != 0x1026c57 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026c61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026c61); }
 let mut pc: u32 = 0x1026c61;
 loop { match pc {
 0x1026c61 => {
  { cpu.pushad(); }
  cpu.push(0x1026c67); f_010338d0(cpu); if cpu.returned_to != 0x1026c67 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1119f20u32)); cpu.esi = v; }
  cpu.push(0x1026c89); f_01026d45(cpu); if cpu.returned_to != 0x1026c89 { return; }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1119f20u32)); cpu.esi = v; }
  cpu.push(0x1026cab); f_01026d45(cpu); if cpu.returned_to != 0x1026cab { return; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1119f20u32)); cpu.esi = v; }
  cpu.push(0x1026ccd); f_01026d45(cpu); if cpu.returned_to != 0x1026ccd { return; }
  { let v: u32 = 0x2du32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1119f20u32)); cpu.esi = v; }
  cpu.push(0x1026cef); f_01026d45(cpu); if cpu.returned_to != 0x1026cef { return; }
  { let v: u32 = 0x51u32; cpu.eax = v; }
  pc = 0x1026cf4; continue;
 }
 0x1026cf4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1119f20u32)); cpu.esi = v; }
  cpu.push(0x1026d12); f_01026d45(cpu); if cpu.returned_to != 0x1026d12 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x57u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1026cf4; continue; }
  pc = 0x1026d19; continue;
 }
 0x1026d19 => {
  { let v: u32 = 0x64u32; cpu.eax = v; }
  pc = 0x1026d1e; continue;
 }
 0x1026d1e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1119f20u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1119f20u32)); cpu.esi = v; }
  cpu.push(0x1026d3c); f_01026d45(cpu); if cpu.returned_to != 0x1026d3c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x70u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1026d1e; continue; }
  pc = 0x1026d43; continue;
 }
 0x1026d43 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026d45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026d45); }
 let mut pc: u32 = 0x1026d45;
 loop { match pc {
 0x1026d45 => {
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  pc = 0x1026d4a; continue;
 }
 0x1026d4a => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.esi), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1026d4a; continue; }
  pc = 0x1026d51; continue;
 }
 0x1026d51 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026d52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026d52); }
 let mut pc: u32 = 0x1026d52;
 loop { match pc {
 0x1026d52 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1026d61; continue;
 }
 0x1026d61 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.edx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1026d61; continue; }
  pc = 0x1026d7f; continue;
 }
 0x1026d7f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026d81(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026d81); }
 let mut pc: u32 = 0x1026d81;
 loop { match pc {
 0x1026d81 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1121308u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x4u16); }
  if cpu.zf { pc = 0x1026da5; continue; }
  pc = 0x1026d91; continue;
 }
 0x1026d91 => {
  { let v: u32 = 0x1121314u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if cpu.zf { pc = 0x1026da5; continue; }
  pc = 0x1026da0; continue;
 }
 0x1026da0 => {
  { let v: u32 = 0x112131du32; cpu.ebp = v; }
  pc = 0x1026da5; continue;
 }
 0x1026da5 => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x10ee91du32)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1026db6; continue;
 }
 0x1026db6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x1026dd1; continue; }
  pc = 0x1026dbe; continue;
 }
 0x1026dbe => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))) | 0x80u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1026dd1; continue; }
  pc = 0x1026dcb; continue;
 }
 0x1026dcb => {
  { let v: u8 = 0x8u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x1026dd6; continue;
 }
 0x1026dd1 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x1026db6; continue; }
  pc = 0x1026dd6; continue;
 }
 0x1026dd6 => {
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1131374u32); cpu.esi = v; }
  { let v: u16 = 0x40u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1026dec); f_01033375(cpu); if cpu.returned_to != 0x1026dec { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026dee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026dee); }
 let mut pc: u32 = 0x1026dee;
 loop { match pc {
 0x1026dee => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1026f05; continue; }
  pc = 0x1026dfb; continue;
 }
 0x1026dfb => {
  { cpu.sub8(cpu.r8(0x110587bu32), 0x0u8); }
  if cpu.zf { pc = 0x1026f05; continue; }
  pc = 0x1026e08; continue;
 }
 0x1026e08 => {
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v = cpu.r32(0x1119f14u32); cpu.push(v); }
  { let v: u32 = cpu.r32(0x1119f18u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1119f14u32, v); }
  { cpu.and8(cpu.r8(0x10ef352u32), 0xffu8); }
  if !cpu.zf { pc = 0x1026e2d; continue; }
  pc = 0x1026e26; continue;
 }
 0x1026e26 => {
  { let v: u32 = 0x11212f0u32; cpu.edi = v; }
  pc = 0x1026e32; continue;
 }
 0x1026e2d => {
  { let v: u32 = 0x11212fcu32; cpu.edi = v; }
  pc = 0x1026e32; continue;
 }
 0x1026e32 => {
  { let v: u32 = 0x1121308u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x4u16); }
  if cpu.zf { pc = 0x1026e55; continue; }
  pc = 0x1026e41; continue;
 }
 0x1026e41 => {
  { let v: u32 = 0x1121314u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if cpu.zf { pc = 0x1026e55; continue; }
  pc = 0x1026e50; continue;
 }
 0x1026e50 => {
  { let v: u32 = 0x112131du32; cpu.ebp = v; }
  pc = 0x1026e55; continue;
 }
 0x1026e55 => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x10ee91du32)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.sf { pc = 0x1026e72; continue; }
  pc = 0x1026e6d; continue;
 }
 0x1026e6d => {
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  pc = 0x1026e72; continue;
 }
 0x1026e72 => {
  { let v: u32 = cpu.eax.wrapping_add(0x74u32); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  cpu.push(0x1026e80); f_010231ea(cpu); if cpu.returned_to != 0x1026e80 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1026e55; continue; }
  pc = 0x1026e8a; continue;
 }
 0x1026e8a => {
  { let v: u32 = cpu.r8(0x110587cu32) as u32; cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1026eae; continue; }
  pc = 0x1026e95; continue;
 }
 0x1026e95 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110587cu32, v); }
  if !cpu.zf { pc = 0x1026eac; continue; }
  pc = 0x1026ea0; continue;
 }
 0x1026ea0 => {
  cpu.push(0x1026ea5); f_0100c4d3(cpu); if cpu.returned_to != 0x1026ea5 { return; }
  { let v: u8 = 0x7u8; cpu.w8(0x111e9e6u32, v); }
  pc = 0x1026eac; continue;
 }
 0x1026eac => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1026eae; continue;
 }
 0x1026eae => {
  { let v: u32 = 0x7eu32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x530u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1026ec4; continue; }
  pc = 0x1026ebf; continue;
 }
 0x1026ebf => {
  { let v: u32 = 0x12e0u32; cpu.edi = v; }
  pc = 0x1026ec4; continue;
 }
 0x1026ec4 => {
  cpu.push(0x1026ec9); f_0102328e(cpu); if cpu.returned_to != 0x1026ec9 { return; }
  { let v: u32 = cpu.r8(0x111e9f8u32) as u32; cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1026edf; continue; }
  pc = 0x1026ed4; continue;
 }
 0x1026ed4 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x111e9f8u32, v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1026edf; continue;
 }
 0x1026edf => {
  { let v: u32 = 0x27u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x519u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1026ef5; continue; }
  pc = 0x1026ef0; continue;
 }
 0x1026ef0 => {
  { let v: u32 = 0x12b2u32; cpu.edi = v; }
  pc = 0x1026ef5; continue;
 }
 0x1026ef5 => {
  cpu.push(0x1026efa); f_010231ea(cpu); if cpu.returned_to != 0x1026efa { return; }
  cpu.push(0x1026eff); f_0100c355(cpu); if cpu.returned_to != 0x1026eff { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x1119f14u32, v); }
  pc = 0x1026f05; continue;
 }
 0x1026f05 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026f06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026f06); }
 let mut pc: u32 = 0x1026f06;
 loop { match pc {
 0x1026f06 => {
  { cpu.pushad(); }
  cpu.push(0x1026f0c); f_01025678(cpu); if cpu.returned_to != 0x1026f0c { return; }
  { let v: u32 = 0x116fbd4u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x1026f16; continue;
 }
 0x1026f16 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1026f66; continue; }
  pc = 0x1026f25; continue;
 }
 0x1026f25 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x111c986u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.r32(0x111a970u32)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x111a982u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.edi = v; }
  { let v: u32 = 0x111a144u32; cpu.esi = v; }
  pc = 0x1026f43; continue;
 }
 0x1026f43 => {
  { cpu.lods(4, false); }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x1026f66; continue; }
  pc = 0x1026f48; continue;
 }
 0x1026f48 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x1026f43; continue; }
  pc = 0x1026f4d; continue;
 }
 0x1026f4d => {
  { let v: u32 = cpu.edi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x1026f5a); f_0101b70a(cpu); if cpu.returned_to != 0x1026f5a { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x10c6a48u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.esi), v); }
  pc = 0x1026f66; continue;
 }
 0x1026f66 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x6u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1026f16; continue; }
  pc = 0x1026f6d; continue;
 }
 0x1026f6d => {
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x111a021u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026f76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026f76); }
 let mut pc: u32 = 0x1026f76;
 loop { match pc {
 0x1026f76 => {
  { let v: u32 = 0x111f03cu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10fd44bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x111f051u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x111f057u32; cpu.eax = v; }
  { let v: u32 = 0x111f060u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  cpu.push(0x1026f9e); f_0105ce4d(cpu); if cpu.returned_to != 0x1026f9e { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.sf { f_0105f11c(cpu); return; }
  pc = 0x1026fa7; continue;
 }
 0x1026fa7 => {
  { let v: u32 = 0x111f046u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10fd44bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x111f051u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x111f057u32; cpu.eax = v; }
  { let v: u32 = 0x11c634cu32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  cpu.push(0x1026fcf); f_0105ce4d(cpu); if cpu.returned_to != 0x1026fcf { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.sf { f_0105f11c(cpu); return; }
  pc = 0x1026fd8; continue;
 }
 0x1026fd8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01026fd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1026fd9); }
 let mut pc: u32 = 0x1026fd9;
 loop { match pc {
 0x1026fd9 => {
  { let v: u32 = 0x1120258u32; cpu.edi = v; }
  { let v: u32 = 0x1120cb4u32; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ecx = v; }
  cpu.push(0x1026fed); f_01027054(cpu); if cpu.returned_to != 0x1026fed { return; }
  { let v: u32 = 0x1120292u32; cpu.edi = v; }
  { let v: u32 = 0x1120aa0u32; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ecx = v; }
  cpu.push(0x1027001); f_01027054(cpu); if cpu.returned_to != 0x1027001 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01027002(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1027002); }
 let mut pc: u32 = 0x1027002;
 loop { match pc {
 0x1027002 => {
  { let v: u32 = 0x1120258u32; cpu.edi = v; }
  { let v: u32 = 0x11210dcu32; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ecx = v; }
  cpu.push(0x1027016); f_01027054(cpu); if cpu.returned_to != 0x1027016 { return; }
  { let v: u32 = 0x1120292u32; cpu.edi = v; }
  { let v: u32 = 0x1120aa0u32; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ecx = v; }
  cpu.push(0x102702a); f_01027054(cpu); if cpu.returned_to != 0x102702a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102702b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102702b); }
 let mut pc: u32 = 0x102702b;
 loop { match pc {
 0x102702b => {
  { let v: u32 = 0x1120258u32; cpu.edi = v; }
  { let v: u32 = 0x1120cb4u32; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ecx = v; }
  cpu.push(0x102703f); f_01027054(cpu); if cpu.returned_to != 0x102703f { return; }
  { let v: u32 = 0x1120292u32; cpu.edi = v; }
  { let v: u32 = 0x1120ec8u32; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ecx = v; }
  cpu.push(0x1027053); f_01027054(cpu); if cpu.returned_to != 0x1027053 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01027054(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1027054); }
 let mut pc: u32 = 0x1027054;
 loop { match pc {
 0x1027054 => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4au32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1027054; continue; }
  pc = 0x1027064; continue;
 }
 0x1027064 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01027065(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1027065); }
 let mut pc: u32 = 0x1027065;
 loop { match pc {
 0x1027065 => {
  cpu.push(0x102706a); f_010722ed(cpu); if cpu.returned_to != 0x102706a { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x111f060u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e3d85u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  cpu.push(0x1027084); f_0102709b(cpu); if cpu.returned_to != 0x1027084 { return; }
  { let v: u32 = 0x111f060u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4114u32); cpu.edi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  cpu.push(0x1027099); f_0102709b(cpu); if cpu.returned_to != 0x1027099 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102709b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102709b); }
 let mut pc: u32 = 0x102709b;
 loop { match pc {
 0x102709b => {
  { let v: u32 = cpu.edi.wrapping_add(0x1678u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = 0x3c4u32; cpu.edx = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10270ad; continue;
 }
 0x10270ad => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x44u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x48u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x50u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x54u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x58u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x64u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x68u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x6cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x41u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x45u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x49u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x55u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x59u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x61u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x65u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x69u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x6du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x46u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x52u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x56u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x66u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x6au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x6eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x37u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x43u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x53u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x57u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x63u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x6bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x70u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x52u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10270ad; continue; }
  pc = 0x10272f1; continue;
 }
 0x10272f1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010277cf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10277cf); } r_010277cf(cpu, 0x10277cf); }
pub fn f_010277d3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10277d3); } r_010277cf(cpu, 0x10277d3); }
pub fn f_010277d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10277d7); } r_010277cf(cpu, 0x10277d7); }
pub fn f_010277db(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10277db); } r_010277cf(cpu, 0x10277db); }
pub fn f_010277df(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10277df); } r_010277cf(cpu, 0x10277df); }
pub fn f_010277e3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10277e3); } r_010277cf(cpu, 0x10277e3); }
pub fn f_010277e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10277e7); } r_010277cf(cpu, 0x10277e7); }
pub fn f_01027805(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027805); } r_010277cf(cpu, 0x1027805); }
fn r_010277cf(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10277cf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10277d3; continue;
 }
 0x10277d3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10277d7; continue;
 }
 0x10277d7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10277db; continue;
 }
 0x10277db => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10277df; continue;
 }
 0x10277df => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10277e3; continue;
 }
 0x10277e3 => {
  { let v: u32 = cpu.eax.wrapping_add(0x6000000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.unimplemented(0x10277f3, "Wrmsr");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.esi ^ cpu.r32(0x39383736u32.wrapping_add(cpu.esi.wrapping_mul(1))); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x51u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v: u16 = 0x300u16; cpu.w16(0x11d6490u32, v); }
  { let v: u8 = 0x10u8; cpu.w8(0x11d6494u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11d6495u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11d6498u32, v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ebx = v; }
  { let v: u32 = 0x11d64feu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11d64acu32, v); }
  { let v: u32 = 0x11d64feu32; cpu.w32(0x11d64a4u32, v); }
  { let v: u32 = 0x11d64acu32; cpu.ecx = v; }
  { let v: u32 = 0x11d6490u32; cpu.ebx = v; }
  { let v: u32 = 0x11d6490u32; cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x102786c); f_0108eb96(cpu); if cpu.returned_to != 0x102786c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10277e7 => {
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.unimplemented(0x10277f3, "Wrmsr");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.esi ^ cpu.r32(0x39383736u32.wrapping_add(cpu.esi.wrapping_mul(1))); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x51u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v: u16 = 0x300u16; cpu.w16(0x11d6490u32, v); }
  { let v: u8 = 0x10u8; cpu.w8(0x11d6494u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11d6495u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11d6498u32, v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ebx = v; }
  { let v: u32 = 0x11d64feu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11d64acu32, v); }
  { let v: u32 = 0x11d64feu32; cpu.w32(0x11d64a4u32, v); }
  { let v: u32 = 0x11d64acu32; cpu.ecx = v; }
  { let v: u32 = 0x11d6490u32; cpu.ebx = v; }
  { let v: u32 = 0x11d6490u32; cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x102786c); f_0108eb96(cpu); if cpu.returned_to != 0x102786c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1027805 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v: u16 = 0x300u16; cpu.w16(0x11d6490u32, v); }
  { let v: u8 = 0x10u8; cpu.w8(0x11d6494u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11d6495u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11d6498u32, v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ebx = v; }
  { let v: u32 = 0x11d64feu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11d64acu32, v); }
  { let v: u32 = 0x11d64feu32; cpu.w32(0x11d64a4u32, v); }
  { let v: u32 = 0x11d64acu32; cpu.ecx = v; }
  { let v: u32 = 0x11d6490u32; cpu.ebx = v; }
  { let v: u32 = 0x11d6490u32; cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x102786c); f_0108eb96(cpu); if cpu.returned_to != 0x102786c { return; }
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
pub fn f_01027873(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1027873); }
 let mut pc: u32 = 0x1027873;
 loop { match pc {
 0x1027873 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6490u32, v); }
  { let v: u32 = 0x11d6490u32; cpu.ebx = v; }
  { let v: u32 = 0x11d6490u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  cpu.push(0x10278a1); f_0108ef70(cpu); if cpu.returned_to != 0x10278a1 { return; }
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
pub fn f_010278aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10278aa); }
 let mut pc: u32 = 0x10278aa;
 loop { match pc {
 0x10278aa => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x3c0u16); }
  if !cpu.zf { pc = 0x10278d2; continue; }
  pc = 0x10278c5; continue;
 }
 0x10278c5 => {
  { let v: u32 = 0x3dau32; cpu.eax = v; }
  cpu.push(0x10278cf); f_010951ba(cpu); if cpu.returned_to != 0x10278cf { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10278d2; continue;
 }
 0x10278d2 => {
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x10278df); f_010951c2(cpu); if cpu.returned_to != 0x10278df { return; }
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x10278e9); f_010951ba(cpu); if cpu.returned_to != 0x10278e9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_010278f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10278f7); }
 let mut pc: u32 = 0x10278f7;
 loop { match pc {
 0x10278f7 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1027919); f_010951c2(cpu); if cpu.returned_to != 0x1027919 { return; }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x1027927); f_010951c2(cpu); if cpu.returned_to != 0x1027927 { return; }
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
pub fn f_0102792e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102792e); }
 let mut pc: u32 = 0x102792e;
 loop { match pc {
 0x102792e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1027952); f_010278aa(cpu); if cpu.returned_to != 0x1027952 { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1027975); f_010278f7(cpu); if cpu.returned_to != 0x1027975 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102797b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102797b); }
 let mut pc: u32 = 0x102797b;
 loop { match pc {
 0x102797b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x11d648cu32)); cpu.edx = v; }
  { let v: u32 = 0x11d60f4u32; cpu.eax = v; }
  cpu.push(0x10279a6); f_0108ef05(cpu); if cpu.returned_to != 0x10279a6 { return; }
  { let v: u32 = 0x11d60f4u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_01027a5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1027a5b); }
 let mut pc: u32 = 0x1027a5b;
 loop { match pc {
 0x1027a5b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1027a7d); f_010278aa(cpu); if cpu.returned_to != 0x1027a7d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = !cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1027a98); f_010278f7(cpu); if cpu.returned_to != 0x1027a98 { return; }
  { let v: u32 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1027aa5); f_010278aa(cpu); if cpu.returned_to != 0x1027aa5 { return; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1027ac3); f_010278f7(cpu); if cpu.returned_to != 0x1027ac3 { return; }
  { let v: u32 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1027ad0); f_010278aa(cpu); if cpu.returned_to != 0x1027ad0 { return; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1027ae9); f_010278f7(cpu); if cpu.returned_to != 0x1027ae9 { return; }
  { cpu.sub16(cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x1027af9; continue; }
  pc = 0x1027af0; continue;
 }
 0x1027af0 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1027afb; continue; }
  pc = 0x1027af9; continue;
 }
 0x1027af9 => {
  pc = 0x1027b01; continue;
 }
 0x1027afb => {
  { let v: u8 = 0x1u8; cpu.w8(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1027b05; continue;
 }
 0x1027b01 => {
  { let v: u8 = 0x0u8; cpu.w8(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1027b05; continue;
 }
 0x1027b05 => {
  { let v: u8 = cpu.r8(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01027b15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1027b15); }
 let mut pc: u32 = 0x1027b15;
 loop { match pc {
 0x1027b15 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1027b3a); f_01027a5b(cpu); if cpu.returned_to != 0x1027b3a { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01027b48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1027b48); }
 let mut pc: u32 = 0x1027b48;
 loop { match pc {
 0x1027b48 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v: u32 = 0x3c8u32; cpu.eax = v; }
  cpu.push(0x1027b60); f_010951ba(cpu); if cpu.returned_to != 0x1027b60 { return; }
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
pub fn f_01027b67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1027b67); }
 let mut pc: u32 = 0x1027b67;
 loop { match pc {
 0x1027b67 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x1027b7a); f_01027b48(cpu); if cpu.returned_to != 0x1027b7a { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x1027b84); f_010951ba(cpu); if cpu.returned_to != 0x1027b84 { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x1027b8e); f_010951ba(cpu); if cpu.returned_to != 0x1027b8e { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x1027b98); f_010951ba(cpu); if cpu.returned_to != 0x1027b98 { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x1027ba2); f_010951ba(cpu); if cpu.returned_to != 0x1027ba2 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d663eu32, v); }
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
pub fn f_01027bb1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027bb1); } r_01027bb1(cpu, 0x1027bb1); }
pub fn f_01027be4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027be4); } r_01027bb1(cpu, 0x1027be4); }
pub fn f_01027c84(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027c84); } r_01027bb1(cpu, 0x1027c84); }
pub fn f_01027ca6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027ca6); } r_01027bb1(cpu, 0x1027ca6); }
pub fn f_01027cef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027cef); } r_01027bb1(cpu, 0x1027cef); }
pub fn f_01027d0a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027d0a); } r_01027bb1(cpu, 0x1027d0a); }
pub fn f_01027d1d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027d1d); } r_01027bb1(cpu, 0x1027d1d); }
pub fn f_01027d41(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027d41); } r_01027bb1(cpu, 0x1027d41); }
pub fn f_01027d64(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027d64); } r_01027bb1(cpu, 0x1027d64); }
pub fn f_01027d83(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027d83); } r_01027bb1(cpu, 0x1027d83); }
pub fn f_01027da7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027da7); } r_01027bb1(cpu, 0x1027da7); }
pub fn f_01027dcb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027dcb); } r_01027bb1(cpu, 0x1027dcb); }
pub fn f_01027e19(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027e19); } r_01027bb1(cpu, 0x1027e19); }
pub fn f_01027e39(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027e39); } r_01027bb1(cpu, 0x1027e39); }
pub fn f_01027e54(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027e54); } r_01027bb1(cpu, 0x1027e54); }
pub fn f_01027e7e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027e7e); } r_01027bb1(cpu, 0x1027e7e); }
pub fn f_01027e99(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027e99); } r_01027bb1(cpu, 0x1027e99); }
pub fn f_01027eb6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027eb6); } r_01027bb1(cpu, 0x1027eb6); }
pub fn f_01027ed7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027ed7); } r_01027bb1(cpu, 0x1027ed7); }
pub fn f_01027f4b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027f4b); } r_01027bb1(cpu, 0x1027f4b); }
pub fn f_01027f50(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027f50); } r_01027bb1(cpu, 0x1027f50); }
pub fn f_01027f91(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027f91); } r_01027bb1(cpu, 0x1027f91); }
pub fn f_01027faa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027faa); } r_01027bb1(cpu, 0x1027faa); }
pub fn f_01027fc0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1027fc0); } r_01027bb1(cpu, 0x1027fc0); }
pub fn f_01028027(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1028027); } r_01027bb1(cpu, 0x1028027); }
pub fn f_01028041(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1028041); } r_01027bb1(cpu, 0x1028041); }
pub fn f_01028084(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1028084); } r_01027bb1(cpu, 0x1028084); }
fn r_01027bb1(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1027bb1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x11d6652u32)); }
  if !cpu.zf { pc = 0x1027bdc; continue; }
  pc = 0x1027bcf; continue;
 }
 0x1027bcf => {
  { let v: u32 = cpu.r32(0x11d6650u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1027bde; continue; }
  pc = 0x1027bdc; continue;
 }
 0x1027bdc => {
  pc = 0x1027c58; continue;
 }
 0x1027bde => {
  { let v = cpu.pop(); cpu.set_flags(v); }
  pc = 0x1028085; continue;
 }
 0x1027be4 => {
  if (cpu.sf == cpu.of) { pc = 0x1027be9; continue; }
  pc = 0x1027be7; continue;
 }
 0x1027be7 => {
  { let v: u32 = cpu.add32(cpu.r32(0xa701027du32.wrapping_add(cpu.ebx)), cpu.eax); cpu.w32(0xa701027du32.wrapping_add(cpu.ebx), v); }
  pc = 0x1027bed; continue;
 }
 0x1027be9 => {
  if (cpu.sf == cpu.of) { pc = 0x1027bed; continue; }
  pc = 0x1027beb; continue;
 }
 0x1027beb => {
  { let v: u32 = cpu.add32(cpu.r32(0x9901027du32.wrapping_add(cpu.edi)), cpu.esp); cpu.w32(0x9901027du32.wrapping_add(cpu.edi), v); }
  pc = 0x1027bf1; continue;
 }
 0x1027bed => {
  if (cpu.sf == cpu.of) { pc = 0x1027bf1; continue; }
  pc = 0x1027bef; continue;
 }
 0x1027bef => {
  { let v: u32 = cpu.add32(cpu.r32(0xc001027eu32.wrapping_add(cpu.ecx)), cpu.ebx); cpu.w32(0xc001027eu32.wrapping_add(cpu.ecx), v); }
  pc = 0x1027bf5; continue;
 }
 0x1027bf1 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1027bf5; continue; }
  pc = 0x1027bf3; continue;
 }
 0x1027bf3 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1027bf5; continue;
 }
 0x1027bf5 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1027bf9; continue; }
  pc = 0x1027bf7; continue;
 }
 0x1027bf7 => {
  { let v: u32 = cpu.add32(cpu.r32(0xaa01027fu32.wrapping_add(cpu.ecx)), cpu.edx); cpu.w32(0xaa01027fu32.wrapping_add(cpu.ecx), v); }
  pc = 0x1027bfd; continue;
 }
 0x1027bf9 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1027bfd; continue; }
  pc = 0x1027bfb; continue;
 }
 0x1027bfb => {
  { let v: u32 = cpu.add32(cpu.r32(0x4b01027fu32.wrapping_add(cpu.edx)), cpu.ebp); cpu.w32(0x4b01027fu32.wrapping_add(cpu.edx), v); }
  pc = 0x1027c01; continue;
 }
 0x1027bfd => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1027c01; continue; }
  pc = 0x1027bff; continue;
 }
 0x1027bff => {
  { let v: u32 = cpu.r32(0x7fu32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ecx); cpu.w32(0x7fu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x1027c05; continue;
 }
 0x1027c01 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1027c05; continue; }
  pc = 0x1027c03; continue;
 }
 0x1027c03 => {
  { let v: u32 = cpu.r32(0x7fu32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x7fu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x1027c09; continue;
 }
 0x1027c05 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1027c09; continue; }
  pc = 0x1027c07; continue;
 }
 0x1027c07 => {
  { let v: u32 = cpu.r32(0x7fu32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0x7fu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x1027c09 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1027c0d; continue; }
  pc = 0x1027c0b; continue;
 }
 0x1027c0b => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ecx); cpu.ebx = v; }
  pc = 0x1027c0d; continue;
 }
 0x1027c0d => {
  if (cpu.sf == cpu.of) { pc = 0x1027c11; continue; }
  pc = 0x1027c0f; continue;
 }
 0x1027c0f => {
  { let v: u32 = cpu.add32(cpu.r32(0x4101027du32), cpu.ebx); cpu.w32(0x4101027du32, v); }
  pc = 0x1027c15; continue;
 }
 0x1027c11 => {
  if (cpu.sf == cpu.of) { pc = 0x1027c15; continue; }
  pc = 0x1027c13; continue;
 }
 0x1027c13 => {
  { let v: u32 = cpu.r32(0x7du32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x7du32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sbb32(cpu.r32(0x2u32.wrapping_add(cpu.esi)), cpu.edi); cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  pc = 0x1027c1b; continue;
 }
 0x1027c15 => {
  if (cpu.sf == cpu.of) { pc = 0x1027c19; continue; }
  pc = 0x1027c17; continue;
 }
 0x1027c17 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1027c19; continue;
 }
 0x1027c19 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1027c1d; continue; }
  pc = 0x1027c1b; continue;
 }
 0x1027c1b => {
  { let v: u32 = cpu.r32(0x7eu32.wrapping_add(cpu.esi)).wrapping_add(cpu.edi); cpu.w32(0x7eu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x7ca60102u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  pc = 0x1027c29; continue;
 }
 0x1027c1d => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1027c21; continue; }
  pc = 0x1027c1f; continue;
 }
 0x1027c1f => {
  { let v: u32 = cpu.r32(0x7ca60102u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.eax); cpu.w32(0x7ca60102u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  pc = 0x1027c29; continue;
 }
 0x1027c21 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.cmps(1, 0); }
  if (cpu.sf != cpu.of) { pc = 0x1027c29; continue; }
  pc = 0x1027c27; continue;
 }
 0x1027c27 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x1027c29; continue;
 }
 0x1027c29 => {
  if (cpu.sf != cpu.of) { pc = 0x1027c2d; continue; }
  pc = 0x1027c2b; continue;
 }
 0x1027c2b => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.edx); cpu.w32(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  pc = 0x1027c2f; continue;
 }
 0x1027c2d => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1027c31; continue; }
  pc = 0x1027c2f; continue;
 }
 0x1027c2f => {
  { let v: u32 = cpu.r32(0x7f500102u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4))).wrapping_add(cpu.eax); cpu.w32(0x7f500102u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  pc = 0x1027c39; continue;
 }
 0x1027c31 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.push(v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1027c39; continue; }
  pc = 0x1027c37; continue;
 }
 0x1027c37 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.edx); cpu.edi = v; }
  pc = 0x1027c39; continue;
 }
 0x1027c39 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1027c3d; continue; }
  pc = 0x1027c3b; continue;
 }
 0x1027c3b => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x1027c3d; continue;
 }
 0x1027c3d => {
  if (cpu.sf == cpu.of) { pc = 0x1027c41; continue; }
  pc = 0x1027c3f; continue;
 }
 0x1027c3f => {
  { let v: u32 = cpu.r32(0x7e390102u32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add(cpu.eax); cpu.w32(0x7e390102u32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1027c49; continue;
 }
 0x1027c41 => {
  if (cpu.sf != cpu.of) { pc = 0x1027c45; continue; }
  pc = 0x1027c43; continue;
 }
 0x1027c43 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.edi); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1027c45; continue;
 }
 0x1027c45 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1027c49; continue; }
  pc = 0x1027c47; continue;
 }
 0x1027c47 => {
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0xffffff80u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7eu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7eu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1027c54, "Daa");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x1027c58; continue;
 }
 0x1027c49 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x7eu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7eu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x1027c54, "Daa");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x1027c58; continue;
 }
 0x1027c58 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6652u32, v); }
  { let v: u8 = cpu.r8(0x11d6896u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x1cu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1028084; continue; }
  pc = 0x1027c75; continue;
 }
 0x1027c75 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x1027be4u32.wrapping_add(cpu.eax)); continue;
 }
 0x1027c84 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0xf0u32; cpu.ebx = v; }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1027ca1); f_0102792e(cpu); if cpu.returned_to != 0x1027ca1 { return; }
  pc = 0x1028084; continue;
 }
 0x1027ca6 => {
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0xdu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1027cbe); f_010278f7(cpu); if cpu.returned_to != 0x1027cbe { return; }
  { let v: u32 = 0x3ccu32; cpu.eax = v; }
  cpu.push(0x1027cc8); f_010951ba(cpu); if cpu.returned_to != 0x1027cc8 { return; }
  { let v: u32 = cpu.eax & 0xdfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x1u16); }
  if cpu.zf { pc = 0x1027cdc; continue; }
  pc = 0x1027cd8; continue;
 }
 0x1027cd8 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0x20u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1027cdc; continue;
 }
 0x1027cdc => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3c2u32; cpu.eax = v; }
  cpu.push(0x1027cea); f_010951c2(cpu); if cpu.returned_to != 0x1027cea { return; }
  pc = 0x1028084; continue;
 }
 0x1027cef => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x11u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0xdu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1027d05); f_010278f7(cpu); if cpu.returned_to != 0x1027d05 { return; }
  pc = 0x1028084; continue;
 }
 0x1027d0a => {
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3d7u32; cpu.eax = v; }
  cpu.push(0x1027d18); f_010951c2(cpu); if cpu.returned_to != 0x1027d18 { return; }
  pc = 0x1028084; continue;
 }
 0x1027d1d => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x1eu32; cpu.ebx = v; }
  { let v: u32 = 0xb2u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d6640u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1027d3c); f_0102792e(cpu); if cpu.returned_to != 0x1027d3c { return; }
  pc = 0x1028084; continue;
 }
 0x1027d41 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x22u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0xeeu32; cpu.ebx = v; }
  { let v: u32 = 0xb2u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d6640u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1027d5f); f_0102792e(cpu); if cpu.returned_to != 0x1027d5f { return; }
  pc = 0x1028084; continue;
 }
 0x1027d64 => {
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11d664cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1027d7e); f_010278f7(cpu); if cpu.returned_to != 0x1027d7e { return; }
  pc = 0x1028084; continue;
 }
 0x1027d83 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11d664cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1027da2); f_010278f7(cpu); if cpu.returned_to != 0x1027da2 { return; }
  pc = 0x1028084; continue;
 }
 0x1027da7 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11d664cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1027dc6); f_010278f7(cpu); if cpu.returned_to != 0x1027dc6 { return; }
  pc = 0x1028084; continue;
 }
 0x1027dcb => {
  { let v: u32 = 0x3ccu32; cpu.eax = v; }
  cpu.push(0x1027dd5); f_010951ba(cpu); if cpu.returned_to != 0x1027dd5 { return; }
  { let v: u32 = cpu.eax & 0xdfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x2u16); }
  if cpu.zf { pc = 0x1027de9; continue; }
  pc = 0x1027de5; continue;
 }
 0x1027de5 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0x20u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1027de9; continue;
 }
 0x1027de9 => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3c2u32; cpu.eax = v; }
  cpu.push(0x1027df7); f_010951c2(cpu); if cpu.returned_to != 0x1027df7 { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1027e14); f_0102792e(cpu); if cpu.returned_to != 0x1027e14 { return; }
  pc = 0x1028084; continue;
 }
 0x1027e19 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x9u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0x6u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1027e34); f_010278f7(cpu); if cpu.returned_to != 0x1027e34 { return; }
  pc = 0x1028084; continue;
 }
 0x1027e39 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x11u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0xc5u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1027e4f); f_010278f7(cpu); if cpu.returned_to != 0x1027e4f { return; }
  pc = 0x1028084; continue;
 }
 0x1027e54 => {
  { cpu.sub8(cpu.r8(0x11d6895u32), 0xau8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1027e61; continue; }
  pc = 0x1027e5d; continue;
 }
 0x1027e5d => {
  { let v: u32 = cpu.shl32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 2u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1027e61; continue;
 }
 0x1027e61 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0x18u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1027e79); f_010278f7(cpu); if cpu.returned_to != 0x1027e79 { return; }
  pc = 0x1028084; continue;
 }
 0x1027e7e => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x11u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0x11u32; cpu.edx = v; }
  { let v: u32 = 0x3deu32; cpu.eax = v; }
  cpu.push(0x1027e94); f_010278f7(cpu); if cpu.returned_to != 0x1027e94 { return; }
  pc = 0x1028084; continue;
 }
 0x1027e99 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1027eb1); f_010278f7(cpu); if cpu.returned_to != 0x1027eb1 { return; }
  pc = 0x1028084; continue;
 }
 0x1027eb6 => {
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3d6u32; cpu.eax = v; }
  cpu.push(0x1027ec4); f_010951c2(cpu); if cpu.returned_to != 0x1027ec4 { return; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3d7u32; cpu.eax = v; }
  cpu.push(0x1027ed2); f_010951c2(cpu); if cpu.returned_to != 0x1027ed2 { return; }
  pc = 0x1028084; continue;
 }
 0x1027ed7 => {
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  { let v: u32 = 0x38u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1027eee); f_010278f7(cpu); if cpu.returned_to != 0x1027eee { return; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = 0x9u32; cpu.ebx = v; }
  { let v: u32 = 0x31u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1027f0a); f_0102792e(cpu); if cpu.returned_to != 0x1027f0a { return; }
  { cpu.sub8(cpu.r8(0x11d6895u32), 0xau8); }
  if !cpu.zf { pc = 0x1027f17; continue; }
  pc = 0x1027f13; continue;
 }
 0x1027f13 => {
  { let v: u32 = cpu.shl32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 2u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1027f17; continue;
 }
 0x1027f17 => {
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0xfu32; cpu.ebx = v; }
  { let v: u32 = 0x35u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1027f32); f_0102792e(cpu); if cpu.returned_to != 0x1027f32 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x38u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1027f46); f_010278f7(cpu); if cpu.returned_to != 0x1027f46 { return; }
  pc = 0x1028084; continue;
 }
 0x1027f4b => {
  pc = 0x1028084; continue;
 }
 0x1027f50 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1027f61); f_010278f7(cpu); if cpu.returned_to != 0x1027f61 { return; }
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1027f70); f_010278aa(cpu); if cpu.returned_to != 0x1027f70 { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0xfu32; cpu.ebx = v; }
  { let v: u32 = 0xeu32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1027f8c); f_0102792e(cpu); if cpu.returned_to != 0x1027f8c { return; }
  pc = 0x1028084; continue;
 }
 0x1027f91 => {
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = 0x40u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x3cdu32; cpu.eax = v; }
  cpu.push(0x1027fa5); f_010951c2(cpu); if cpu.returned_to != 0x1027fa5 { return; }
  pc = 0x1028084; continue;
 }
 0x1027faa => {
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x11u32); cpu.edx = v; }
  { let v: u32 = 0x3cdu32; cpu.eax = v; }
  cpu.push(0x1027fbb); f_010951c2(cpu); if cpu.returned_to != 0x1027fbb { return; }
  pc = 0x1028084; continue;
 }
 0x1027fc0 => {
  { let v: u32 = 0x3ccu32; cpu.eax = v; }
  cpu.push(0x1027fca); f_010951ba(cpu); if cpu.returned_to != 0x1027fca { return; }
  { let v: u32 = cpu.eax & 0xdfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x2u16); }
  if cpu.zf { pc = 0x1027fde; continue; }
  pc = 0x1027fda; continue;
 }
 0x1027fda => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0x20u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1027fde; continue;
 }
 0x1027fde => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3c2u32; cpu.eax = v; }
  cpu.push(0x1027fec); f_010951c2(cpu); if cpu.returned_to != 0x1027fec { return; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0xf9u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1028004); f_0102792e(cpu); if cpu.returned_to != 0x1028004 { return; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0xf6u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1028025); f_0102792e(cpu); if cpu.returned_to != 0x1028025 { return; }
  pc = 0x1028084; continue;
 }
 0x1028027 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x102803f); f_010278f7(cpu); if cpu.returned_to != 0x102803f { return; }
  pc = 0x1028084; continue;
 }
 0x1028041 => {
  { let v: u16 = 0x0u16; cpu.w16(0x11d6494u32, v); }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 6u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d6656u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d649cu32, v); }
  { let v: u32 = 0x4f05u32; cpu.eax = v; }
  cpu.push(0x1028071); f_01027873(cpu); if cpu.returned_to != 0x1028071 { return; }
  { let v: u16 = 0x1u16; cpu.w16(0x11d6494u32, v); }
  { let v: u32 = 0x4f05u32; cpu.eax = v; }
  cpu.push(0x1028084); f_01027873(cpu); if cpu.returned_to != 0x1028084 { return; }
  pc = 0x1028084; continue;
 }
 0x1028084 => {
  { let v = cpu.pop(); cpu.set_flags(v); }
  pc = 0x1028085; continue;
 }
 0x1028085 => {
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
pub fn f_0102808e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102808e); } r_0102808e(cpu, 0x102808e); }
pub fn f_010280ff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10280ff); } r_0102808e(cpu, 0x10280ff); }
fn r_0102808e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102808e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1121340u32), 0x0u32); }
  if !cpu.zf { pc = 0x102813a; continue; }
  pc = 0x10280a9; continue;
 }
 0x10280a9 => {
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11d64acu32; cpu.eax = v; }
  cpu.push(0x10280ba); f_0108f030(cpu); if cpu.returned_to != 0x10280ba { return; }
  { let v: u16 = 0x100u16; cpu.w16(0x11d6490u32, v); }
  { let v: u16 = 0x100u16; cpu.w16(0x11d6494u32, v); }
  { let v: u32 = 0x11d64acu32; cpu.ecx = v; }
  { let v: u32 = 0x11d6490u32; cpu.ebx = v; }
  { let v: u32 = 0x11d6490u32; cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x10280e5); f_0108eb96(cpu); if cpu.returned_to != 0x10280e5 { return; }
  { let v: u16 = cpu.r16(0x11d6490u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6640u32, v); }
  { let v: u16 = cpu.r16(0x11d649cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6644u32, v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10280ff; continue;
 }
 0x10280ff => {
  { let v: u16 = cpu.r16(0x11d6644u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d64fcu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11d64f8u32, v); }
  { let v: u32 = cpu.r32(0x11d663eu32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0xfffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x102812d); f_01013dec(cpu); if cpu.returned_to != 0x102812d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1121340u32, v); }
  pc = 0x102813a; continue;
 }
 0x102813a => {
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
pub fn f_01028143(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1028143); }
 let mut pc: u32 = 0x1028143;
 loop { match pc {
 0x1028143 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1121340u32), 0x0u32); }
  if cpu.zf { pc = 0x10281c2; continue; }
  pc = 0x102815a; continue;
 }
 0x102815a => {
  { let v: u32 = cpu.r32(0x11d663eu32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = 0xfffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1028176); f_01013e03(cpu); if cpu.returned_to != 0x1028176 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0xcu32; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x11d64acu32; cpu.eax = v; }
  cpu.push(0x102818a); f_0108f030(cpu); if cpu.returned_to != 0x102818a { return; }
  { let v: u16 = 0x101u16; cpu.w16(0x11d6490u32, v); }
  { let v: u16 = cpu.r16(0x11d6644u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d649cu32, v); }
  { let v: u32 = 0x11d64acu32; cpu.ecx = v; }
  { let v: u32 = 0x11d6490u32; cpu.ebx = v; }
  { let v: u32 = 0x11d6490u32; cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x10281b8); f_0108eb96(cpu); if cpu.returned_to != 0x10281b8 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1121340u32, v); }
  pc = 0x10281c2; continue;
 }
 0x10281c2 => {
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
pub fn f_010281cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10281cb); }
 let mut pc: u32 = 0x10281cb;
 loop { match pc {
 0x10281cb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  cpu.push(0x10281de); f_0102808e(cpu); if cpu.returned_to != 0x10281de { return; }
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11d64feu32; cpu.eax = v; }
  cpu.push(0x10281ef); f_0108f030(cpu); if cpu.returned_to != 0x10281ef { return; }
  { let v: u32 = 0x4f00u32; cpu.w32(0x11d651au32, v); }
  { let v: u16 = cpu.r16(0x11d6640u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6520u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d64feu32, v); }
  cpu.push(0x1028214); f_01027805(cpu); if cpu.returned_to != 0x1028214 { return; }
  { let v = 0x100u32; cpu.push(v); }
  { let v: u16 = cpu.r16(0x11d64fcu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11d64f8u32); cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.esi = v; }
  { let v: u32 = 0x11d6530u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x102823e); f_010951cc(cpu); if cpu.returned_to != 0x102823e { return; }
  { let v: u32 = cpu.r32(0x11d6532u32); cpu.eax = v; }
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
pub fn f_01028255(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1028255); }
 let mut pc: u32 = 0x1028255;
 loop { match pc {
 0x1028255 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11d64feu32; cpu.eax = v; }
  cpu.push(0x102827b); f_0108f030(cpu); if cpu.returned_to != 0x102827b { return; }
  { let v: u32 = 0x4f01u32; cpu.w32(0x11d651au32, v); }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d6516u32, v); }
  { let v: u16 = cpu.r16(0x11d6640u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6520u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d64feu32, v); }
  cpu.push(0x10282a9); f_01027805(cpu); if cpu.returned_to != 0x10282a9 { return; }
  { let v = 0x100u32; cpu.push(v); }
  { let v: u16 = cpu.r16(0x11d64fcu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11d64f8u32); cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.esi = v; }
  { let v: u32 = 0x11d6773u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10282d3); f_010951cc(cpu); if cpu.returned_to != 0x10282d3 { return; }
  { cpu.and8(cpu.r8(0x11d6773u32), 0x1u8); }
  if cpu.zf { pc = 0x102837a; continue; }
  pc = 0x10282e0; continue;
 }
 0x10282e0 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x11d6777u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6658u32, v); }
  { let v: u32 = cpu.r32(0x11d6781u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d64f4u32, v); }
  { let v: u16 = cpu.r16(0x11d6785u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6650u32, v); }
  { let v: u16 = cpu.r16(0x11d6787u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d664au32, v); }
  { let v: u8 = cpu.r8(0x11d678cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.cf { pc = 0x1028339; continue; }
  pc = 0x1028323; continue;
 }
 0x1028323 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if (cpu.cf || cpu.zf) { pc = 0x1028361; continue; }
  pc = 0x1028329; continue;
 }
 0x1028329 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if (cpu.cf || cpu.zf) { pc = 0x102836a; continue; }
  pc = 0x102832f; continue;
 }
 0x102832f => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x1028373; continue; }
  pc = 0x1028335; continue;
 }
 0x1028335 => {
  pc = 0x102837a; continue;
 }
 0x1028339 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.cf { pc = 0x102834d; continue; }
  pc = 0x102833f; continue;
 }
 0x102833f => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if (cpu.cf || cpu.zf) { pc = 0x102834f; continue; }
  pc = 0x1028345; continue;
 }
 0x1028345 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x1028358; continue; }
  pc = 0x102834b; continue;
 }
 0x102834b => {
  pc = 0x102837a; continue;
 }
 0x102834d => {
  pc = 0x102837a; continue;
 }
 0x102834f => {
  { let v: u8 = 0xau8; cpu.w8(0x11d6895u32, v); }
  pc = 0x102837a; continue;
 }
 0x1028358 => {
  { let v: u8 = 0xcu8; cpu.w8(0x11d6895u32, v); }
  pc = 0x102837a; continue;
 }
 0x1028361 => {
  { let v: u8 = 0xdu8; cpu.w8(0x11d6895u32, v); }
  pc = 0x102837a; continue;
 }
 0x102836a => {
  { let v: u8 = 0xeu8; cpu.w8(0x11d6895u32, v); }
  pc = 0x102837a; continue;
 }
 0x1028373 => {
  { let v: u8 = 0xfu8; cpu.w8(0x11d6895u32, v); }
  pc = 0x102837a; continue;
 }
 0x102837a => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_0102838c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102838c); }
 let mut pc: u32 = 0x102838c;
 loop { match pc {
 0x102838c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x10283a7); f_01027873(cpu); if cpu.returned_to != 0x10283a7 { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x10283b2); f_01027873(cpu); if cpu.returned_to != 0x10283b2 { return; }
  { let v: u32 = 0x6u32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x10283c1); f_010278aa(cpu); if cpu.returned_to != 0x10283c1 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_010283d9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10283d9); } r_010283d9(cpu, 0x10283d9); }
pub fn f_0102840f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102840f); } r_010283d9(cpu, 0x102840f); }
fn r_010283d9(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10283d9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6494u32, v); }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x10283fe); f_01027873(cpu); if cpu.returned_to != 0x10283fe { return; }
  { cpu.sub16(cpu.r16(0x11d6490u32), 0x10u16); }
  if !cpu.zf { pc = 0x102840e; continue; }
  pc = 0x1028408; continue;
 }
 0x1028408 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028412; continue;
 }
 0x102840e => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028412; continue;
 }
 0x102840f => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x4588f045u32.wrapping_add(cpu.edx)), (cpu.ecx as u8)); cpu.w8(0x4588f045u32.wrapping_add(cpu.edx), v); }
  { cpu.df = false; }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1028412 => {
  { let v: u8 = cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01028423(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1028423); }
 let mut pc: u32 = 0x1028423;
 loop { match pc {
 0x1028423 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4f03u32; cpu.eax = v; }
  cpu.push(0x102843f); f_01027873(cpu); if cpu.returned_to != 0x102843f { return; }
  { let v: u8 = cpu.r8(0x11d6490u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x11d6494u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01028461(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1028461); } r_01028461(cpu, 0x1028461); }
pub fn f_010288e9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10288e9); } r_01028461(cpu, 0x10288e9); }
pub fn f_010288fa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10288fa); } r_01028461(cpu, 0x10288fa); }
pub fn f_0102894f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102894f); } r_01028461(cpu, 0x102894f); }
pub fn f_010289fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10289fb); } r_01028461(cpu, 0x10289fb); }
pub fn f_01028a17(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1028a17); } r_01028461(cpu, 0x1028a17); }
fn r_01028461(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1028461 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6654u32, v); }
  { let v: u8 = cpu.r8(0x11d6896u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0xcu8); }
  if cpu.cf { pc = 0x10284fa; continue; }
  pc = 0x102848d; continue;
 }
 0x102848d => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0xcu8); }
  if (cpu.cf || cpu.zf) { pc = 0x10286ec; continue; }
  pc = 0x1028497; continue;
 }
 0x1028497 => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.cf { pc = 0x10284cb; continue; }
  pc = 0x102849d; continue;
 }
 0x102849d => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x19u8); }
  if (cpu.cf || cpu.zf) { pc = 0x1028869; continue; }
  pc = 0x10284a7; continue;
 }
 0x10284a7 => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.cf { pc = 0x10284c6; continue; }
  pc = 0x10284ad; continue;
 }
 0x10284ad => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if (cpu.cf || cpu.zf) { pc = 0x1028831; continue; }
  pc = 0x10284b7; continue;
 }
 0x10284b7 => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.zf { pc = 0x1028aad; continue; }
  pc = 0x10284c1; continue;
 }
 0x10284c1 => {
  pc = 0x1028ad8; continue;
 }
 0x10284c6 => {
  pc = 0x102880c; continue;
 }
 0x10284cb => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x14u8); }
  if cpu.cf { pc = 0x10284ea; continue; }
  pc = 0x10284d1; continue;
 }
 0x10284d1 => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x14u8); }
  if (cpu.cf || cpu.zf) { pc = 0x1028773; continue; }
  pc = 0x10284db; continue;
 }
 0x10284db => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x1028a61; continue; }
  pc = 0x10284e5; continue;
 }
 0x10284e5 => {
  pc = 0x1028ad8; continue;
 }
 0x10284ea => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0xeu8); }
  if (cpu.cf || cpu.zf) { pc = 0x1028544; continue; }
  pc = 0x10284f0; continue;
 }
 0x10284f0 => {
  pc = 0x1028ad8; continue;
 }
 0x10284fa => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.cf { pc = 0x102852e; continue; }
  pc = 0x1028500; continue;
 }
 0x1028500 => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x5u8); }
  if (cpu.cf || cpu.zf) { pc = 0x1028703; continue; }
  pc = 0x102850a; continue;
 }
 0x102850a => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.cf { pc = 0x1028529; continue; }
  pc = 0x1028510; continue;
 }
 0x1028510 => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x7u8); }
  if (cpu.cf || cpu.zf) { pc = 0x102888b; continue; }
  pc = 0x102851a; continue;
 }
 0x102851a => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x10288c8; continue; }
  pc = 0x1028524; continue;
 }
 0x1028524 => {
  pc = 0x1028ad8; continue;
 }
 0x1028529 => {
  pc = 0x10287f5; continue;
 }
 0x102852e => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.cf { pc = 0x102853f; continue; }
  pc = 0x1028534; continue;
 }
 0x1028534 => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x2u8); }
  if (cpu.cf || cpu.zf) { pc = 0x102859a; continue; }
  pc = 0x102853a; continue;
 }
 0x102853a => {
  pc = 0x1028637; continue;
 }
 0x102853f => {
  pc = 0x1028ad8; continue;
 }
 0x1028544 => {
  { let v: u32 = 0x5506u32; cpu.w32(0x11d650eu32, v); }
  { let v: u32 = 0xffffu32; cpu.w32(0x11d6506u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d6502u32, v); }
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1200u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d651au32, v); }
  cpu.push(0x1028575); f_01027805(cpu); if cpu.returned_to != 0x1028575 { return; }
  { let v: u32 = cpu.r32(0x11d6506u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if !cpu.zf { pc = 0x102858c; continue; }
  pc = 0x1028586; continue;
 }
 0x1028586 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028595; continue;
 }
 0x102858c => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1028595); f_01027873(cpu); if cpu.returned_to != 0x1028595 { return; }
  pc = 0x1028595; continue;
 }
 0x1028595 => {
  pc = 0x1028ae4; continue;
 }
 0x102859a => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x10285a3); f_0102838c(cpu); if cpu.returned_to != 0x10285a3 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x46e8u32; cpu.eax = v; }
  cpu.push(0x10285b0); f_010951ba(cpu); if cpu.returned_to != 0x10285b0 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x46e8u32; cpu.eax = v; }
  cpu.push(0x10285c5); f_010951c2(cpu); if cpu.returned_to != 0x10285c5 { return; }
  { let v: u32 = 0x103u32; cpu.eax = v; }
  cpu.push(0x10285cf); f_010951ba(cpu); if cpu.returned_to != 0x10285cf { return; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x103u32; cpu.eax = v; }
  cpu.push(0x10285dd); f_010951c2(cpu); if cpu.returned_to != 0x10285dd { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xefu32; cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x46e8u32; cpu.eax = v; }
  cpu.push(0x10285f2); f_010951c2(cpu); if cpu.returned_to != 0x10285f2 { return; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11d664cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1028612); f_0102792e(cpu); if cpu.returned_to != 0x1028612 { return; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11d664cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1028632); f_0102792e(cpu); if cpu.returned_to != 0x1028632 { return; }
  pc = 0x1028ae4; continue;
 }
 0x1028637 => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1028640); f_0102838c(cpu); if cpu.returned_to != 0x1028640 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x46e8u32; cpu.eax = v; }
  cpu.push(0x102864d); f_010951ba(cpu); if cpu.returned_to != 0x102864d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x46e8u32; cpu.eax = v; }
  cpu.push(0x1028662); f_010951c2(cpu); if cpu.returned_to != 0x1028662 { return; }
  { let v: u32 = 0x103u32; cpu.eax = v; }
  cpu.push(0x102866c); f_010951ba(cpu); if cpu.returned_to != 0x102866c { return; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x103u32; cpu.eax = v; }
  cpu.push(0x102867a); f_010951c2(cpu); if cpu.returned_to != 0x102867a { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xefu32; cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x46e8u32; cpu.eax = v; }
  cpu.push(0x102868f); f_010951c2(cpu); if cpu.returned_to != 0x102868f { return; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11d664cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x10286af); f_0102792e(cpu); if cpu.returned_to != 0x10286af { return; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11d664cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x10286cf); f_0102792e(cpu); if cpu.returned_to != 0x10286cf { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0xcu32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11d664cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x10286e7); f_010278f7(cpu); if cpu.returned_to != 0x10286e7 { return; }
  pc = 0x1028ae4; continue;
 }
 0x10286ec => {
  { let v: u8 = cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11d6494u32, v); }
  { let v: u32 = 0x70u32; cpu.eax = v; }
  cpu.push(0x10286fe); f_01027873(cpu); if cpu.returned_to != 0x10286fe { return; }
  pc = 0x1028ae4; continue;
 }
 0x1028703 => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x102870c); f_0102838c(cpu); if cpu.returned_to != 0x102870c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x17u32; cpu.ebx = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1028728); f_0102792e(cpu); if cpu.returned_to != 0x1028728 { return; }
  { let v: u32 = 0x85u32; cpu.ebx = v; }
  { let v: u32 = 0x29u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102873f); f_010278f7(cpu); if cpu.returned_to != 0x102873f { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1028755); f_0102792e(cpu); if cpu.returned_to != 0x1028755 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x62u32; cpu.ebx = v; }
  { let v: u32 = 0x2fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102876e); f_0102792e(cpu); if cpu.returned_to != 0x102876e { return; }
  pc = 0x1028ae4; continue;
 }
 0x1028773 => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x102877c); f_0102838c(cpu); if cpu.returned_to != 0x102877c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1028793); f_010278f7(cpu); if cpu.returned_to != 0x1028793 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x18u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x10287a4); f_010278f7(cpu); if cpu.returned_to != 0x10287a4 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x10287b5); f_010278f7(cpu); if cpu.returned_to != 0x10287b5 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x1au32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x10287c6); f_010278f7(cpu); if cpu.returned_to != 0x10287c6 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x1bu32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x10287d7); f_010278f7(cpu); if cpu.returned_to != 0x10287d7 { return; }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u32 = 0x1cu32; cpu.ebx = v; }
  { let v: u32 = 0x1eu32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x10287f0); f_0102792e(cpu); if cpu.returned_to != 0x10287f0 { return; }
  pc = 0x1028ae4; continue;
 }
 0x10287f5 => {
  { let v: u8 = cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11d6494u32, v); }
  { let v: u32 = 0x6f05u32; cpu.eax = v; }
  cpu.push(0x1028807); f_01027873(cpu); if cpu.returned_to != 0x1028807 { return; }
  pc = 0x1028ae4; continue;
 }
 0x102880c => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1028815); f_0102838c(cpu); if cpu.returned_to != 0x1028815 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x87u32; cpu.ebx = v; }
  { let v: u32 = 0xa7u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x102882c); f_010278f7(cpu); if cpu.returned_to != 0x102882c { return; }
  pc = 0x1028ae4; continue;
 }
 0x1028831 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6494u32, v); }
  { let v: u32 = 0x4f02u32; cpu.eax = v; }
  cpu.push(0x1028844); f_01027873(cpu); if cpu.returned_to != 0x1028844 { return; }
  { cpu.sub16(cpu.r16(0x11d6490u32), 0x4fu16); }
  if cpu.zf { pc = 0x1028854; continue; }
  pc = 0x102884e; continue;
 }
 0x102884e => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028864; continue;
 }
 0x1028854 => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x102885d); f_01028255(cpu); if cpu.returned_to != 0x102885d { return; }
  { let v: u8 = 0x1bu8; cpu.w8(0x11d6896u32, v); }
  pc = 0x1028864; continue;
 }
 0x1028864 => {
  pc = 0x1028ae4; continue;
 }
 0x1028869 => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1028872); f_01027873(cpu); if cpu.returned_to != 0x1028872 { return; }
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  { let v: u32 = 0x6u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1028886); f_010278f7(cpu); if cpu.returned_to != 0x1028886 { return; }
  pc = 0x1028ae4; continue;
 }
 0x102888b => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1028894); f_01027873(cpu); if cpu.returned_to != 0x1028894 { return; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x10288ad); f_0102792e(cpu); if cpu.returned_to != 0x10288ad { return; }
  pc = 0x1028ae4; continue;
 }
 0x10288c8 => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x4u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1028a5c; continue; }
  pc = 0x10288dc; continue;
 }
 0x10288dc => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x10288b4u32.wrapping_add(cpu.eax)); continue;
 }
 0x10288e9 => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x10288f2); f_0102838c(cpu); if cpu.returned_to != 0x10288f2 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028a5c; continue;
 }
 0x10288fa => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x10e0u32; cpu.eax = v; }
  cpu.push(0x102890c); f_010283d9(cpu); if cpu.returned_to != 0x102890c { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1028946; continue; }
  pc = 0x1028910; continue;
 }
 0x1028910 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028917; continue;
 }
 0x1028917 => {
  { cpu.sub16(cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xfu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1028928; continue; }
  pc = 0x102891e; continue;
 }
 0x102891e => {
  pc = 0x1028944; continue;
 }
 0x1028920 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1028917; continue;
 }
 0x1028928 => {
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1028931); f_01027bb1(cpu); if cpu.returned_to != 0x1028931 { return; }
  { let v: u32 = 0x10000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xa0000u32; cpu.eax = v; }
  cpu.push(0x1028942); f_0108f030(cpu); if cpu.returned_to != 0x1028942 { return; }
  cpu.poll(); pc = 0x1028920; continue;
 }
 0x1028944 => {
  pc = 0x102894a; continue;
 }
 0x1028946 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102894a; continue;
 }
 0x102894a => {
  pc = 0x1028a5c; continue;
 }
 0x102894f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xffu32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x10f0u32; cpu.eax = v; }
  cpu.push(0x1028967); f_010283d9(cpu); if cpu.returned_to != 0x1028967 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10289f5; continue; }
  pc = 0x102896f; continue;
 }
 0x102896f => {
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x3bfu32; cpu.eax = v; }
  cpu.push(0x102897e); f_010951c2(cpu); if cpu.returned_to != 0x102897e { return; }
  { let v: u32 = 0xa0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  cpu.push(0x1028993); f_010951c2(cpu); if cpu.returned_to != 0x1028993 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x13u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10289a7); f_010278f7(cpu); if cpu.returned_to != 0x10289a7 { return; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x3fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10289c3); f_0102792e(cpu); if cpu.returned_to != 0x10289c3 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x13u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10289d7); f_010278f7(cpu); if cpu.returned_to != 0x10289d7 { return; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x3fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10289f3); f_0102792e(cpu); if cpu.returned_to != 0x10289f3 { return; }
  pc = 0x10289f9; continue;
 }
 0x10289f5 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10289f9; continue;
 }
 0x10289f9 => {
  pc = 0x1028a5c; continue;
 }
 0x10289fb => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x10f0u32; cpu.eax = v; }
  cpu.push(0x1028a0d); f_010283d9(cpu); if cpu.returned_to != 0x1028a0d { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1028a15; continue; }
  pc = 0x1028a11; continue;
 }
 0x1028a11 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028a15; continue;
 }
 0x1028a15 => {
  pc = 0x1028a5c; continue;
 }
 0x1028a17 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x10f0u32; cpu.eax = v; }
  cpu.push(0x1028a29); f_010283d9(cpu); if cpu.returned_to != 0x1028a29 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1028a58; continue; }
  pc = 0x1028a2d; continue;
 }
 0x1028a2d => {
  cpu.push(0x1028a32); f_01027b67(cpu); if cpu.returned_to != 0x1028a32 { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x1028a3c); f_010951ba(cpu); if cpu.returned_to != 0x1028a3c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x1028a51); f_010951c2(cpu); if cpu.returned_to != 0x1028a51 { return; }
  cpu.push(0x1028a56); f_01027b48(cpu); if cpu.returned_to != 0x1028a56 { return; }
  pc = 0x1028a5c; continue;
 }
 0x1028a58 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028a5c; continue;
 }
 0x1028a5c => {
  pc = 0x1028ae4; continue;
 }
 0x1028a61 => {
  { cpu.sub16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x100u16); }
  if (cpu.sf == cpu.of) { pc = 0x1028a77; continue; }
  pc = 0x1028a69; continue;
 }
 0x1028a69 => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1028a72); f_0102838c(cpu); if cpu.returned_to != 0x1028a72 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028aab; continue;
 }
 0x1028a77 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6494u32, v); }
  { let v: u32 = 0x4f02u32; cpu.eax = v; }
  cpu.push(0x1028a8a); f_01027873(cpu); if cpu.returned_to != 0x1028a8a { return; }
  { cpu.sub16(cpu.r16(0x11d6490u32), 0x4fu16); }
  if !cpu.zf { pc = 0x1028aab; continue; }
  pc = 0x1028a94; continue;
 }
 0x1028a94 => {
  { cpu.sub16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x200u16); }
  if (cpu.sf == cpu.of) { pc = 0x1028aa7; continue; }
  pc = 0x1028a9c; continue;
 }
 0x1028a9c => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1028aa5); f_01028255(cpu); if cpu.returned_to != 0x1028aa5 { return; }
  pc = 0x1028aab; continue;
 }
 0x1028aa7 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028aab; continue;
 }
 0x1028aab => {
  pc = 0x1028ae4; continue;
 }
 0x1028aad => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1028ab6); f_0102838c(cpu); if cpu.returned_to != 0x1028ab6 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11d6895u32), 0xeu8); }
  if !cpu.zf { pc = 0x1028ad6; continue; }
  pc = 0x1028ac2; continue;
 }
 0x1028ac2 => {
  cpu.push(0x1028ac7); f_01027b67(cpu); if cpu.returned_to != 0x1028ac7 { return; }
  { let v: u32 = 0xc0u32; cpu.edx = v; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x1028ad6); f_010951c2(cpu); if cpu.returned_to != 0x1028ad6 { return; }
  pc = 0x1028ad6; continue;
 }
 0x1028ad6 => {
  pc = 0x1028ae4; continue;
 }
 0x1028ad8 => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x1028ae1); f_0102838c(cpu); if cpu.returned_to != 0x1028ae1 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028ae4; continue;
 }
 0x1028ae4 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x11d6652u32, v); }
  { let v: u8 = cpu.r8(0x11d6895u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.cf { pc = 0x1028b09; continue; }
  pc = 0x1028afb; continue;
 }
 0x1028afb => {
  { cpu.sub8(cpu.r8(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if (cpu.cf || cpu.zf) { pc = 0x1028b0f; continue; }
  pc = 0x1028b01; continue;
 }
 0x1028b01 => {
  { cpu.sub8(cpu.r8(0xffffffe4u32.wrapping_add(cpu.ebp)), 0xau8); }
  if cpu.zf { pc = 0x1028b1a; continue; }
  pc = 0x1028b07; continue;
 }
 0x1028b07 => {
  pc = 0x1028b25; continue;
 }
 0x1028b09 => {
  { cpu.sub8(cpu.r8(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1028b25; continue; }
  pc = 0x1028b0f; continue;
 }
 0x1028b0f => {
  { let v: u16 = 0x2u16; cpu.w16(0x11d6656u32, v); }
  pc = 0x1028b2e; continue;
 }
 0x1028b1a => {
  { let v: u16 = 0x4u16; cpu.w16(0x11d6656u32, v); }
  pc = 0x1028b2e; continue;
 }
 0x1028b25 => {
  { let v: u16 = 0x1u16; cpu.w16(0x11d6656u32, v); }
  pc = 0x1028b2e; continue;
 }
 0x1028b2e => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028b35; continue;
 }
 0x1028b35 => {
  { let v: u32 = cpu.r32(0x11d6658u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 6u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1028b5d; continue; }
  pc = 0x1028b53; continue;
 }
 0x1028b53 => {
  pc = 0x1028b7a; continue;
 }
 0x1028b55 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1028b35; continue;
 }
 0x1028b5d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1028b67); f_01027bb1(cpu); if cpu.returned_to != 0x1028b67 { return; }
  { let v: u32 = 0x10000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xa0000u32; cpu.eax = v; }
  cpu.push(0x1028b78); f_0108f030(cpu); if cpu.returned_to != 0x1028b78 { return; }
  cpu.poll(); pc = 0x1028b55; continue;
 }
 0x1028b7a => {
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1028b93); f_0102792e(cpu); if cpu.returned_to != 0x1028b93 { return; }
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_010288b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10288b4); }
 let mut pc: u32 = 0x10288b4;
 loop { match pc {
 0x10288b4 => {
  cpu.call(0xfb038b41); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01028ba5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1028ba5); }
 let mut pc: u32 = 0x1028ba5;
 loop { match pc {
 0x1028ba5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x74u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x11d6895u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028bc1; continue;
 }
 0x1028bc1 => {
  { cpu.sub16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x1u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1028bd1; continue; }
  pc = 0x1028bc8; continue;
 }
 0x1028bc8 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028bd8; continue;
 }
 0x1028bd1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028bd8; continue;
 }
 0x1028bd8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)), cpu.eax); }
  if cpu.zf { pc = 0x1028d3e; continue; }
  pc = 0x1028be6; continue;
 }
 0x1028be6 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1028bf0); f_01027bb1(cpu); if cpu.returned_to != 0x1028bf0 { return; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = 0xa0000u32; cpu.edx = v; }
  { let v: u32 = 0xffffff90u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x1028c02); f_0108ef05(cpu); if cpu.returned_to != 0x1028c02 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028c09; continue;
 }
 0x1028c09 => {
  { cpu.sub16(cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x64u16); }
  if (cpu.sf != cpu.of) { pc = 0x1028c1a; continue; }
  pc = 0x1028c10; continue;
 }
 0x1028c10 => {
  pc = 0x1028c31; continue;
 }
 0x1028c12 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1028c09; continue;
 }
 0x1028c1a => {
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffff90u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) ^ 0xaau8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xa0000u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x1028c12; continue;
 }
 0x1028c31 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1028c3b); f_01027bb1(cpu); if cpu.returned_to != 0x1028c3b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028c46; continue;
 }
 0x1028c46 => {
  { cpu.sub16(cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x64u16); }
  if (cpu.sf != cpu.of) { pc = 0x1028c57; continue; }
  pc = 0x1028c4d; continue;
 }
 0x1028c4d => {
  pc = 0x1028c7e; continue;
 }
 0x1028c4f => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1028c46; continue;
 }
 0x1028c57 => {
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffff90u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ 0xaau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xa0000u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x1028c7c; continue; }
  pc = 0x1028c78; continue;
 }
 0x1028c78 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028c7c; continue;
 }
 0x1028c7c => {
  cpu.poll(); pc = 0x1028c4f; continue;
 }
 0x1028c7e => {
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = 0xffffff90u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xa0000u32; cpu.eax = v; }
  cpu.push(0x1028c90); f_0108ef05(cpu); if cpu.returned_to != 0x1028c90 { return; }
  { cpu.sub8(cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1028d35; continue; }
  pc = 0x1028c9a; continue;
 }
 0x1028c9a => {
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1028ca7); f_01027bb1(cpu); if cpu.returned_to != 0x1028ca7 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028cae; continue;
 }
 0x1028cae => {
  { cpu.sub16(cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x64u16); }
  if (cpu.sf != cpu.of) { pc = 0x1028cbf; continue; }
  pc = 0x1028cb5; continue;
 }
 0x1028cb5 => {
  pc = 0x1028cd6; continue;
 }
 0x1028cb7 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1028cae; continue;
 }
 0x1028cbf => {
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffff90u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) ^ 0x55u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xa0000u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x1028cb7; continue;
 }
 0x1028cd6 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1028ce0); f_01027bb1(cpu); if cpu.returned_to != 0x1028ce0 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028ceb; continue;
 }
 0x1028ceb => {
  { cpu.sub16(cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x64u16); }
  if (cpu.sf != cpu.of) { pc = 0x1028cfc; continue; }
  pc = 0x1028cf2; continue;
 }
 0x1028cf2 => {
  pc = 0x1028d23; continue;
 }
 0x1028cf4 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1028ceb; continue;
 }
 0x1028cfc => {
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffff90u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ 0x55u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xa0000u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x1028d21; continue; }
  pc = 0x1028d1d; continue;
 }
 0x1028d1d => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1028d21; continue;
 }
 0x1028d21 => {
  cpu.poll(); pc = 0x1028cf4; continue;
 }
 0x1028d23 => {
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = 0xffffff90u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xa0000u32; cpu.eax = v; }
  cpu.push(0x1028d35); f_0108ef05(cpu); if cpu.returned_to != 0x1028d35 { return; }
  pc = 0x1028d35; continue;
 }
 0x1028d35 => {
  { let v: u16 = cpu.sar16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)), 1u32); cpu.w16(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1028bc1; continue;
 }
 0x1028d3e => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d665au32, v); }
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
pub fn f_01028d53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1028d53); }
 let mut pc: u32 = 0x1028d53;
 loop { match pc {
 0x1028d53 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11d6896u32), 0x1bu8); }
  if cpu.zf { pc = 0x1028dc3; continue; }
  pc = 0x1028d6d; continue;
 }
 0x1028d6d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11d6658u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0xdu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1028da7); f_010278f7(cpu); if cpu.returned_to != 0x1028da7 { return; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0xcu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1028dc3); f_010278f7(cpu); if cpu.returned_to != 0x1028dc3 { return; }
  pc = 0x1028dc3; continue;
 }
 0x1028dc3 => {
  { let v: u8 = cpu.r8(0x11d6896u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xdu8); }
  if cpu.cf { pc = 0x1028e4b; continue; }
  pc = 0x1028dd5; continue;
 }
 0x1028dd5 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xdu8); }
  if (cpu.cf || cpu.zf) { pc = 0x1029000; continue; }
  pc = 0x1028ddf; continue;
 }
 0x1028ddf => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.cf { pc = 0x1028e1d; continue; }
  pc = 0x1028de5; continue;
 }
 0x1028de5 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if (cpu.cf || cpu.zf) { pc = 0x10290ac; continue; }
  pc = 0x1028def; continue;
 }
 0x1028def => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if cpu.cf { pc = 0x1028e0e; continue; }
  pc = 0x1028df5; continue;
 }
 0x1028df5 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1du8); }
  if (cpu.cf || cpu.zf) { pc = 0x1029121; continue; }
  pc = 0x1028dff; continue;
 }
 0x1028dff => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x10290f9; continue; }
  pc = 0x1028e09; continue;
 }
 0x1028e09 => {
  pc = 0x1029139; continue;
 }
 0x1028e0e => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.zf { pc = 0x1029067; continue; }
  pc = 0x1028e18; continue;
 }
 0x1028e18 => {
  pc = 0x1029139; continue;
 }
 0x1028e1d => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x13u8); }
  if cpu.cf { pc = 0x1028e3c; continue; }
  pc = 0x1028e23; continue;
 }
 0x1028e23 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x13u8); }
  if (cpu.cf || cpu.zf) { pc = 0x102904a; continue; }
  pc = 0x1028e2d; continue;
 }
 0x1028e2d => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x14u8); }
  if cpu.zf { pc = 0x1028fe0; continue; }
  pc = 0x1028e37; continue;
 }
 0x1028e37 => {
  pc = 0x1029139; continue;
 }
 0x1028e3c => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xeu8); }
  if cpu.zf { pc = 0x1029025; continue; }
  pc = 0x1028e46; continue;
 }
 0x1028e46 => {
  pc = 0x1029139; continue;
 }
 0x1028e4b => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.cf { pc = 0x1028e81; continue; }
  pc = 0x1028e51; continue;
 }
 0x1028e51 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if (cpu.cf || cpu.zf) { pc = 0x1028ea5; continue; }
  pc = 0x1028e57; continue;
 }
 0x1028e57 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xau8); }
  if cpu.cf { pc = 0x1028e76; continue; }
  pc = 0x1028e5d; continue;
 }
 0x1028e5d => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xau8); }
  if (cpu.cf || cpu.zf) { pc = 0x1028ee9; continue; }
  pc = 0x1028e67; continue;
 }
 0x1028e67 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xbu8); }
  if cpu.zf { pc = 0x1028f0e; continue; }
  pc = 0x1028e71; continue;
 }
 0x1028e71 => {
  pc = 0x1029139; continue;
 }
 0x1028e76 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x1028ec9; continue; }
  pc = 0x1028e7c; continue;
 }
 0x1028e7c => {
  pc = 0x1029139; continue;
 }
 0x1028e81 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.cf { pc = 0x1028e96; continue; }
  pc = 0x1028e87; continue;
 }
 0x1028e87 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if (cpu.cf || cpu.zf) { pc = 0x1028f8d; continue; }
  pc = 0x1028e91; continue;
 }
 0x1028e91 => {
  pc = 0x1028f66; continue;
 }
 0x1028e96 => {
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.cf { pc = 0x1028faf; continue; }
  pc = 0x1028ea0; continue;
 }
 0x1028ea0 => {
  pc = 0x1029139; continue;
 }
 0x1028ea5 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = 0x23u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1028ec4); f_0102792e(cpu); if cpu.returned_to != 0x1028ec4 { return; }
  pc = 0x1029139; continue;
 }
 0x1028ec9 => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0x33u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1028ee4); f_0102792e(cpu); if cpu.returned_to != 0x1028ee4 { return; }
  pc = 0x1029139; continue;
 }
 0x1028ee9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x1eu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1028f09); f_0102792e(cpu); if cpu.returned_to != 0x1028f09 { return; }
  pc = 0x1029139; continue;
 }
 0x1028f0e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x80u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0xa0u32; cpu.ebx = v; }
  { let v: u32 = 0x1eu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1028f33); f_0102792e(cpu); if cpu.returned_to != 0x1028f33 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1028f44); f_010278f7(cpu); if cpu.returned_to != 0x1028f44 { return; }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0xeu32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1028f61); f_0102792e(cpu); if cpu.returned_to != 0x1028f61 { return; }
  pc = 0x1029139; continue;
 }
 0x1028f66 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x30u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0xf6u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1028f88); f_0102792e(cpu); if cpu.returned_to != 0x1028f88 { return; }
  pc = 0x1029139; continue;
 }
 0x1028f8d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0xdu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1028faa); f_0102792e(cpu); if cpu.returned_to != 0x1028faa { return; }
  pc = 0x1029139; continue;
 }
 0x1028faf => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0xcu32; cpu.edx = v; }
  { let v: u32 = 0x3d6u32; cpu.eax = v; }
  cpu.push(0x1028fc2); f_010278f7(cpu); if cpu.returned_to != 0x1028fc2 { return; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u32 = 0x3d6u32; cpu.eax = v; }
  cpu.push(0x1028fdb); f_0102792e(cpu); if cpu.returned_to != 0x1028fdb { return; }
  pc = 0x1029139; continue;
 }
 0x1028fe0 => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0xfu32; cpu.ebx = v; }
  { let v: u32 = 0x31u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1028ffb); f_0102792e(cpu); if cpu.returned_to != 0x1028ffb { return; }
  pc = 0x1029139; continue;
 }
 0x1029000 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0xb0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d6640u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1029020); f_0102792e(cpu); if cpu.returned_to != 0x1029020 { return; }
  pc = 0x1029139; continue;
 }
 0x1029025 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0xc0u32; cpu.ebx = v; }
  { let v: u32 = 0xb0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d6640u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1029045); f_0102792e(cpu); if cpu.returned_to != 0x1029045 { return; }
  pc = 0x1029139; continue;
 }
 0x102904a => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0x1cu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1029062); f_0102792e(cpu); if cpu.returned_to != 0x1029062 { return; }
  pc = 0x1029139; continue;
 }
 0x1029067 => {
  { let v: u16 = 0x0u16; cpu.w16(0x11d6494u32, v); }
  { let v: u32 = 0x280u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11d6498u32, v); }
  { let v: u32 = 0x280u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d649cu32, v); }
  { let v: u32 = 0x4f07u32; cpu.eax = v; }
  cpu.push(0x10290a7); f_01027873(cpu); if cpu.returned_to != 0x10290a7 { return; }
  pc = 0x1029139; continue;
 }
 0x10290ac => {
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  { let v: u32 = 0x38u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10290c3); f_010278f7(cpu); if cpu.returned_to != 0x10290c3 { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0x31u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10290e3); f_0102792e(cpu); if cpu.returned_to != 0x10290e3 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x38u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10290f7); f_010278f7(cpu); if cpu.returned_to != 0x10290f7 { return; }
  pc = 0x1029139; continue;
 }
 0x10290f9 => {
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x1u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1029104; continue; }
  pc = 0x1029100; continue;
 }
 0x1029100 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0x2u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1029104; continue;
 }
 0x1029104 => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v: u32 = 0x1bu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102911f); f_0102792e(cpu); if cpu.returned_to != 0x102911f { return; }
  pc = 0x1029139; continue;
 }
 0x1029121 => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u32 = 0x21u32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1029139); f_0102792e(cpu); if cpu.returned_to != 0x1029139 { return; }
  pc = 0x1029139; continue;
 }
 0x1029139 => {
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
pub fn f_01029142(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1029142); } r_01029142(cpu, 0x1029142); }
pub fn f_0102917a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102917a); } r_01029142(cpu, 0x102917a); }
pub fn f_010291b4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10291b4); } r_01029142(cpu, 0x10291b4); }
pub fn f_01029200(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1029200); } r_01029142(cpu, 0x1029200); }
pub fn f_01029214(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1029214); } r_01029142(cpu, 0x1029214); }
pub fn f_01029228(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1029228); } r_01029142(cpu, 0x1029228); }
pub fn f_01029239(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1029239); } r_01029142(cpu, 0x1029239); }
pub fn f_0102924a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102924a); } r_01029142(cpu, 0x102924a); }
pub fn f_0102925b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102925b); } r_01029142(cpu, 0x102925b); }
pub fn f_0102926c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102926c); } r_01029142(cpu, 0x102926c); }
pub fn f_0102927d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102927d); } r_01029142(cpu, 0x102927d); }
pub fn f_0102928e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102928e); } r_01029142(cpu, 0x102928e); }
fn r_01029142(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1029142 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136f0cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x102916f); f_0108dece(cpu); if cpu.returned_to != 0x102916f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1029168 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.eax)).wrapping_add(cpu.edx); cpu.w32(0xffffffe8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.add8(cpu.r8(0xec8910c4u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0xec8910c4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102917a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11d63f6u32), 0x0u8); }
  if cpu.zf { pc = 0x1029313; continue; }
  pc = 0x1029199; continue;
 }
 0x1029199 => {
  { let v: u32 = 0x5f00u32; cpu.eax = v; }
  cpu.push(0x10291a3); f_01027873(cpu); if cpu.returned_to != 0x10291a3 { return; }
  { cpu.sub8(cpu.r8(0x11d6490u32), 0x5fu8); }
  if cpu.zf { pc = 0x10291d8; continue; }
  pc = 0x10291ac; continue;
 }
 0x10291ac => {
  pc = 0x1029313; continue;
 }
 0x10291b4 => {
  { let v: u8 = cpu.r8(0x92140102u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x92140102u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x92390102u32.wrapping_add(cpu.edx)).wrapping_sub((cpu.edx as u8)); cpu.w8(0x92390102u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r16(0x924a0102u32.wrapping_add(cpu.edx)); cpu.set_segment(2, v as u16); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10291d0, "Insb");
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1029168; continue; }
  pc = 0x10291d6; continue;
 }
 0x10291d6 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10291d8; continue;
 }
 0x10291d8 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d6494u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x8u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102928e; continue; }
  pc = 0x10291f3; continue;
 }
 0x10291f3 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x10291b4u32.wrapping_add(cpu.eax)); continue;
 }
 0x1029200 => {
  { let v: u32 = 0x1136f20u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102920f); f_0108e30e(cpu); if cpu.returned_to != 0x102920f { return; }
  pc = 0x10292a5; continue;
 }
 0x1029214 => {
  { let v: u32 = 0x1136f34u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029223); f_0108e30e(cpu); if cpu.returned_to != 0x1029223 { return; }
  pc = 0x10292a5; continue;
 }
 0x1029228 => {
  { let v: u32 = 0x1136f48u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029237); f_0108e30e(cpu); if cpu.returned_to != 0x1029237 { return; }
  pc = 0x10292a5; continue;
 }
 0x1029239 => {
  { let v: u32 = 0x1136f5cu32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029248); f_0108e30e(cpu); if cpu.returned_to != 0x1029248 { return; }
  pc = 0x10292a5; continue;
 }
 0x102924a => {
  { let v: u32 = 0x1136f70u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029259); f_0108e30e(cpu); if cpu.returned_to != 0x1029259 { return; }
  pc = 0x10292a5; continue;
 }
 0x102925b => {
  { let v: u32 = 0x1136f84u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102926a); f_0108e30e(cpu); if cpu.returned_to != 0x102926a { return; }
  pc = 0x10292a5; continue;
 }
 0x102926c => {
  { let v: u32 = 0x1136f98u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102927b); f_0108e30e(cpu); if cpu.returned_to != 0x102927b { return; }
  pc = 0x10292a5; continue;
 }
 0x102927d => {
  { let v: u32 = 0x1136facu32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102928c); f_0108e30e(cpu); if cpu.returned_to != 0x102928c { return; }
  pc = 0x10292a5; continue;
 }
 0x102928e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d6494u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x1136fc0u32; cpu.eax = v; }
  cpu.push(0x10292a5); f_01029142(cpu); if cpu.returned_to != 0x10292a5 { return; }
  pc = 0x10292a5; continue;
 }
 0x10292a5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d6494u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.cf { pc = 0x10292c6; continue; }
  pc = 0x10292b8; continue;
 }
 0x10292b8 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10292c8; continue; }
  pc = 0x10292be; continue;
 }
 0x10292be => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x3u32); }
  if cpu.zf { pc = 0x10292d1; continue; }
  pc = 0x10292c4; continue;
 }
 0x10292c4 => {
  pc = 0x10292da; continue;
 }
 0x10292c6 => {
  pc = 0x10292da; continue;
 }
 0x10292c8 => {
  { let v: u8 = 0x3u8; cpu.w8(0x11d6896u32, v); }
  pc = 0x10292e1; continue;
 }
 0x10292d1 => {
  { let v: u8 = 0x4u8; cpu.w8(0x11d6896u32, v); }
  pc = 0x10292e1; continue;
 }
 0x10292da => {
  { let v: u8 = 0x2u8; cpu.w8(0x11d6896u32, v); }
  pc = 0x10292e1; continue;
 }
 0x10292e1 => {
  { let v: u8 = cpu.r8(0x11d6495u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.cf { pc = 0x10292fd; continue; }
  pc = 0x10292ef; continue;
 }
 0x10292ef => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if (cpu.cf || cpu.zf) { pc = 0x10292ff; continue; }
  pc = 0x10292f5; continue;
 }
 0x10292f5 => {
  { cpu.sub8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x102930a; continue; }
  pc = 0x10292fb; continue;
 }
 0x10292fb => {
  pc = 0x1029313; continue;
 }
 0x10292fd => {
  pc = 0x1029313; continue;
 }
 0x10292ff => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x1029313; continue;
 }
 0x102930a => {
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  pc = 0x1029313; continue;
 }
 0x1029313 => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01029325(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1029325); }
 let mut pc: u32 = 0x1029325;
 loop { match pc {
 0x1029325 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11d63f9u32), 0x0u8); }
  if cpu.zf { pc = 0x10295ed; continue; }
  pc = 0x1029344; continue;
 }
 0x1029344 => {
  { let v: u32 = 0xfu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1029353); f_010278aa(cpu); if cpu.returned_to != 0x1029353 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x17u32; cpu.ebx = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x102936c); f_0102792e(cpu); if cpu.returned_to != 0x102936c { return; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x1029380); f_01027a5b(cpu); if cpu.returned_to != 0x1029380 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10295da; continue; }
  pc = 0x1029388; continue;
 }
 0x1029388 => {
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x102939c); f_010278f7(cpu); if cpu.returned_to != 0x102939c { return; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x10293b0); f_01027a5b(cpu); if cpu.returned_to != 0x10293b0 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10295da; continue; }
  pc = 0x10293b8; continue;
 }
 0x10293b8 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x29u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10293ce); f_010278aa(cpu); if cpu.returned_to != 0x10293ce { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1136fcdu32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x10293e0); f_0108e30e(cpu); if cpu.returned_to != 0x10293e0 { return; }
  { let v: u32 = 0x85u32; cpu.ecx = v; }
  { let v: u32 = 0x8fu32; cpu.ebx = v; }
  { let v: u32 = 0x29u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10293fc); f_0102792e(cpu); if cpu.returned_to != 0x10293fc { return; }
  { let v: u32 = 0x2bu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102940e); f_01027b15(cpu); if cpu.returned_to != 0x102940e { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1029426; continue; }
  pc = 0x1029412; continue;
 }
 0x1029412 => {
  { let v: u32 = 0x1136fd7u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029421); f_0108ef8d(cpu); if cpu.returned_to != 0x1029421 { return; }
  pc = 0x102957d; continue;
 }
 0x1029426 => {
  { let v: u32 = 0x6u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1029435); f_010278aa(cpu); if cpu.returned_to != 0x1029435 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  { let v: u32 = 0x6u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x102944c); f_010278f7(cpu); if cpu.returned_to != 0x102944c { return; }
  { let v: u32 = 0xf0u32; cpu.ebx = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1029460); f_01027a5b(cpu); if cpu.returned_to != 0x1029460 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1029478; continue; }
  pc = 0x1029464; continue;
 }
 0x1029464 => {
  { let v: u32 = 0x1136fdeu32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029473); f_0108ef8d(cpu); if cpu.returned_to != 0x1029473 { return; }
  pc = 0x102956a; continue;
 }
 0x1029478 => {
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1029487); f_01027b15(cpu); if cpu.returned_to != 0x1029487 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10294e0; continue; }
  pc = 0x102948b; continue;
 }
 0x102948b => {
  { let v: u32 = 0x1136fe6u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102949a); f_0108ef8d(cpu); if cpu.returned_to != 0x102949a { return; }
  { let v: u32 = 0xa6u32; cpu.ebx = v; }
  { let v: u32 = 0x34u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10294b1); f_010278f7(cpu); if cpu.returned_to != 0x10294b1 { return; }
  { let v: u32 = 0x32u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10294c3); f_010278aa(cpu); if cpu.returned_to != 0x10294c3 { return; }
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x10294db; continue; }
  pc = 0x10294c7; continue;
 }
 0x10294c7 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x34u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10294db); f_010278f7(cpu); if cpu.returned_to != 0x10294db { return; }
  pc = 0x10294db; continue;
 }
 0x10294db => {
  pc = 0x102956a; continue;
 }
 0x10294e0 => {
  { let v: u32 = 0xfu32; cpu.ebx = v; }
  { let v: u32 = 0x14u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x10294f4); f_01027a5b(cpu); if cpu.returned_to != 0x10294f4 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1029532; continue; }
  pc = 0x10294f8; continue;
 }
 0x10294f8 => {
  { let v: u32 = 0x37u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102950a); f_010278aa(cpu); if cpu.returned_to != 0x102950a { return; }
  { cpu.sub16((cpu.eax as u16), 0x31u16); }
  if !cpu.zf { pc = 0x1029521; continue; }
  pc = 0x1029510; continue;
 }
 0x1029510 => {
  { let v: u32 = 0x1136feeu32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102951f); f_0108ef8d(cpu); if cpu.returned_to != 0x102951f { return; }
  pc = 0x1029530; continue;
 }
 0x1029521 => {
  { let v: u32 = 0x1136ff6u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029530); f_0108ef8d(cpu); if cpu.returned_to != 0x1029530 { return; }
  pc = 0x1029530; continue;
 }
 0x1029530 => {
  pc = 0x102956a; continue;
 }
 0x1029532 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x1029546); f_01027a5b(cpu); if cpu.returned_to != 0x1029546 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x102955b; continue; }
  pc = 0x102954a; continue;
 }
 0x102954a => {
  { let v: u32 = 0x1136ffeu32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029559); f_0108ef8d(cpu); if cpu.returned_to != 0x1029559 { return; }
  pc = 0x102956a; continue;
 }
 0x102955b => {
  { let v: u32 = 0x1137006u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102956a); f_0108ef8d(cpu); if cpu.returned_to != 0x102956a { return; }
  pc = 0x102956a; continue;
 }
 0x102956a => {
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0x6u32; cpu.edx = v; }
  { let v: u32 = 0x3c4u32; cpu.eax = v; }
  cpu.push(0x102957d); f_010278f7(cpu); if cpu.returned_to != 0x102957d { return; }
  pc = 0x102957d; continue;
 }
 0x102957d => {
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x102958c); f_010278aa(cpu); if cpu.returned_to != 0x102958c { return; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if cpu.cf { pc = 0x10295a7; continue; }
  pc = 0x1029599; continue;
 }
 0x1029599 => {
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10295a9; continue; }
  pc = 0x102959f; continue;
 }
 0x102959f => {
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x3u32); }
  if cpu.zf { pc = 0x10295b4; continue; }
  pc = 0x10295a5; continue;
 }
 0x10295a5 => {
  pc = 0x10295bd; continue;
 }
 0x10295a7 => {
  pc = 0x10295bd; continue;
 }
 0x10295a9 => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x10295bd; continue;
 }
 0x10295b4 => {
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  pc = 0x10295bd; continue;
 }
 0x10295bd => {
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0x29u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10295d3); f_010278f7(cpu); if cpu.returned_to != 0x10295d3 { return; }
  { let v: u8 = 0x5u8; cpu.w8(0x11d6896u32, v); }
  pc = 0x10295da; continue;
 }
 0x10295da => {
  { let v: u32 = cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  { let v: u32 = 0x3ceu32; cpu.eax = v; }
  cpu.push(0x10295ed); f_010278f7(cpu); if cpu.returned_to != 0x10295ed { return; }
  pc = 0x10295ed; continue;
 }
 0x10295ed => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01029c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1029c60); }
 let mut pc: u32 = 0x1029c60;
 loop { match pc {
 0x1029c60 => {
  { let v: u32 = cpu.r32(0xac01029cu32); cpu.eax = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x9cu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(672);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01029c6f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1029c6f); } r_01029c6f(cpu, 0x1029c6f); }
pub fn f_01029ca1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1029ca1); } r_01029c6f(cpu, 0x1029ca1); }
pub fn f_01029cac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1029cac); } r_01029c6f(cpu, 0x1029cac); }
pub fn f_01029cb7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1029cb7); } r_01029c6f(cpu, 0x1029cb7); }
pub fn f_01029cc2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1029cc2); } r_01029c6f(cpu, 0x1029cc2); }
fn r_01029c6f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1029c6f => {
  { let v: u32 = cpu.r32(0x1fu32.wrapping_add(cpu.edx)).wrapping_add(cpu.edi); cpu.w32(0x1fu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x1029c82); f_010278aa(cpu); if cpu.returned_to != 0x1029c82 { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x3u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1029ccb; continue; }
  pc = 0x1029c91; continue;
 }
 0x1029c91 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x1029c60u32.wrapping_add(cpu.eax)); continue;
 }
 0x1029ca1 => {
  { let v: u16 = 0x100u16; cpu.w16(0x11d665au32, v); }
  pc = 0x1029ccb; continue;
 }
 0x1029cac => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x1029ccb; continue;
 }
 0x1029cb7 => {
  { let v: u16 = 0x300u16; cpu.w16(0x11d665au32, v); }
  pc = 0x1029ccb; continue;
 }
 0x1029cc2 => {
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  pc = 0x1029ccb; continue;
 }
 0x1029ccb => {
  pc = 0x1029cec; continue;
 }
 0x1029cec => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01029cfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1029cfe); }
 let mut pc: u32 = 0x1029cfe;
 loop { match pc {
 0x1029cfe => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11d6404u32), 0x0u8); }
  if cpu.zf { pc = 0x1029de1; continue; }
  pc = 0x1029d1d; continue;
 }
 0x1029d1d => {
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  { let v: u32 = 0xdu32; cpu.edx = v; }
  { let v: u32 = 0x3deu32; cpu.eax = v; }
  cpu.push(0x1029d31); f_01027a5b(cpu); if cpu.returned_to != 0x1029d31 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1029de1; continue; }
  pc = 0x1029d39; continue;
 }
 0x1029d39 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1137158u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029d4c); f_0108e30e(cpu); if cpu.returned_to != 0x1029d4c { return; }
  { let v: u32 = 0x11u32; cpu.edx = v; }
  { let v: u32 = 0x3deu32; cpu.eax = v; }
  cpu.push(0x1029d5b); f_01027b15(cpu); if cpu.returned_to != 0x1029d5b { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1029d94; continue; }
  pc = 0x1029d5f; continue;
 }
 0x1029d5f => {
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u32 = 0x3deu32; cpu.eax = v; }
  cpu.push(0x1029d6e); f_010278aa(cpu); if cpu.returned_to != 0x1029d6e { return; }
  { cpu.sub16((cpu.eax as u16), 0x5u16); }
  if !cpu.zf { pc = 0x1029d85; continue; }
  pc = 0x1029d74; continue;
 }
 0x1029d74 => {
  { let v: u32 = 0x1137161u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029d83); f_0108e30e(cpu); if cpu.returned_to != 0x1029d83 { return; }
  pc = 0x1029d94; continue;
 }
 0x1029d85 => {
  { let v: u32 = 0x1137169u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x1029d94); f_0108e30e(cpu); if cpu.returned_to != 0x1029d94 { return; }
  pc = 0x1029d94; continue;
 }
 0x1029d94 => {
  { let v: u32 = 0xdu32; cpu.edx = v; }
  { let v: u32 = 0x3deu32; cpu.eax = v; }
  cpu.push(0x1029da3); f_010278aa(cpu); if cpu.returned_to != 0x1029da3 { return; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if cpu.cf { pc = 0x1029dbe; continue; }
  pc = 0x1029db0; continue;
 }
 0x1029db0 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1029dc6; continue; }
  pc = 0x1029db6; continue;
 }
 0x1029db6 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x3u32); }
  if cpu.zf { pc = 0x1029dd1; continue; }
  pc = 0x1029dbc; continue;
 }
 0x1029dbc => {
  pc = 0x1029dda; continue;
 }
 0x1029dbe => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x1029dd1; continue; }
  pc = 0x1029dc4; continue;
 }
 0x1029dc4 => {
  pc = 0x1029dda; continue;
 }
 0x1029dc6 => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x1029dda; continue;
 }
 0x1029dd1 => {
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  pc = 0x1029dda; continue;
 }
 0x1029dda => {
  { let v: u8 = 0x10u8; cpu.w8(0x11d6896u32, v); }
  pc = 0x1029de1; continue;
 }
 0x1029de1 => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_0102a2a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a2a4); } r_0102a2a4(cpu, 0x102a2a4); }
pub fn f_0102a2d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a2d8); } r_0102a2a4(cpu, 0x102a2d8); }
pub fn f_0102a2e3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a2e3); } r_0102a2a4(cpu, 0x102a2e3); }
pub fn f_0102a2ee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a2ee); } r_0102a2a4(cpu, 0x102a2ee); }
pub fn f_0102a2f9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a2f9); } r_0102a2a4(cpu, 0x102a2f9); }
fn r_0102a2a4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102a2a4 => {
  { let v = cpu.st(0) - cpu.rf32(0xa2e30102u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xa2f90102u32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d6499u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102a302; continue; }
  pc = 0x102a2cb; continue;
 }
 0x102a2cb => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x102a2a4u32.wrapping_add(cpu.eax)); continue;
 }
 0x102a2d8 => {
  { let v: u16 = 0x100u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a302; continue;
 }
 0x102a2e3 => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a302; continue;
 }
 0x102a2ee => {
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a302; continue;
 }
 0x102a2f9 => {
  { let v: u16 = 0x800u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a302; continue;
 }
 0x102a302 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x113720au32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x102a31b); f_0108dece(cpu); if cpu.returned_to != 0x102a31b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u8 = 0xcu8; cpu.w8(0x11d6896u32, v); }
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_0102a337(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a337); } r_0102a337(cpu, 0x102a337); }
pub fn f_0102a384(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a384); } r_0102a337(cpu, 0x102a384); }
pub fn f_0102a436(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a436); } r_0102a337(cpu, 0x102a436); }
pub fn f_0102a44a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a44a); } r_0102a337(cpu, 0x102a44a); }
pub fn f_0102a45e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a45e); } r_0102a337(cpu, 0x102a45e); }
pub fn f_0102a46f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a46f); } r_0102a337(cpu, 0x102a46f); }
pub fn f_0102a480(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a480); } r_0102a337(cpu, 0x102a480); }
pub fn f_0102a491(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a491); } r_0102a337(cpu, 0x102a491); }
pub fn f_0102a4a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a4a0); } r_0102a337(cpu, 0x102a4a0); }
pub fn f_0102a4b1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a4b1); } r_0102a337(cpu, 0x102a4b1); }
pub fn f_0102a4c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a4c4); } r_0102a337(cpu, 0x102a4c4); }
pub fn f_0102a505(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a505); } r_0102a337(cpu, 0x102a505); }
pub fn f_0102a51d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a51d); } r_0102a337(cpu, 0x102a51d); }
pub fn f_0102a52e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a52e); } r_0102a337(cpu, 0x102a52e); }
pub fn f_0102a53f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a53f); } r_0102a337(cpu, 0x102a53f); }
pub fn f_0102a550(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a550); } r_0102a337(cpu, 0x102a550); }
fn r_0102a337(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102a337 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11d6401u32), 0x0u8); }
  if !cpu.zf { pc = 0x102a35f; continue; }
  pc = 0x102a352; continue;
 }
 0x102a352 => {
  { cpu.sub8(cpu.r8(0x11d6402u32), 0x0u8); }
  if cpu.zf { pc = 0x102a681; continue; }
  pc = 0x102a35f; continue;
 }
 0x102a35f => {
  { let v: u32 = 0x1137216u32; cpu.ebx = v; }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x102a373); f_0102797b(cpu); if cpu.returned_to != 0x102a373 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x102a37a); f_01094c50(cpu); if cpu.returned_to != 0x102a37a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x102a3a4; continue; }
  pc = 0x102a37e; continue;
 }
 0x102a37e => {
  pc = 0x102a681; continue;
 }
 0x102a384 => {
  { cpu.movs(1, false); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.movs(1, false); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.movs(1, false); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x102a390, "Outsd");
  { cpu.movs(1, false); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2a4b101u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))) & 0x1u8; cpu.w8(0x2a4b101u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.movs(1, false); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xc60102a4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.cf = false; }
  { let v: u32 = cpu.r32(0x11d648cu32.wrapping_add(cpu.ecx)).wrapping_add(cpu.esp); cpu.w32(0x11d648cu32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = cpu.r16(0x80u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x3133u16); }
  if cpu.cf { pc = 0x102a3d8; continue; }
  pc = 0x102a3bf; continue;
 }
 0x102a3a4 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11d648cu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x80u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x3133u16); }
  if cpu.cf { pc = 0x102a3d8; continue; }
  pc = 0x102a3bf; continue;
 }
 0x102a3bf => {
  { cpu.sub16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x3133u16); }
  if (cpu.cf || cpu.zf) { pc = 0x102a3dd; continue; }
  pc = 0x102a3c7; continue;
 }
 0x102a3c7 => {
  { cpu.sub16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x3233u16); }
  if cpu.zf { pc = 0x102a65c; continue; }
  pc = 0x102a3d3; continue;
 }
 0x102a3d3 => {
  pc = 0x102a681; continue;
 }
 0x102a3d8 => {
  pc = 0x102a681; continue;
 }
 0x102a3dd => {
  { let v: u32 = cpu.r32(0x11d648cu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6642u32, v); }
  { let v: u32 = 0x1137220u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a3fb); f_0108e30e(cpu); if cpu.returned_to != 0x102a3fb { return; }
  { let v: u32 = 0xbbu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d6640u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a40d); f_010278aa(cpu); if cpu.returned_to != 0x102a40d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)), 0x7u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102a4b1; continue; }
  pc = 0x102a426; continue;
 }
 0x102a426 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x102a384u32.wrapping_add(cpu.eax)); continue;
 }
 0x102a436 => {
  { let v: u32 = 0x113722fu32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102a445); f_0108e30e(cpu); if cpu.returned_to != 0x102a445 { return; }
  pc = 0x102a4d8; continue;
 }
 0x102a44a => {
  { let v: u32 = 0x1137233u32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102a459); f_0108e30e(cpu); if cpu.returned_to != 0x102a459 { return; }
  pc = 0x102a4d8; continue;
 }
 0x102a45e => {
  { let v: u32 = 0x1137245u32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102a46d); f_0108e30e(cpu); if cpu.returned_to != 0x102a46d { return; }
  pc = 0x102a4d8; continue;
 }
 0x102a46f => {
  { let v: u32 = 0x1137250u32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102a47e); f_0108e30e(cpu); if cpu.returned_to != 0x102a47e { return; }
  pc = 0x102a4d8; continue;
 }
 0x102a480 => {
  { let v: u32 = 0x113725du32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102a48f); f_0108e30e(cpu); if cpu.returned_to != 0x102a48f { return; }
  pc = 0x102a4d8; continue;
 }
 0x102a491 => {
  { let v: u32 = 0x1137261u32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102a4a0); f_0108e30e(cpu); if cpu.returned_to != 0x102a4a0 { return; }
  pc = 0x102a4a0; continue;
 }
 0x102a4a0 => {
  { let v: u32 = 0x1137262u32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102a4af); f_0108e30e(cpu); if cpu.returned_to != 0x102a4af { return; }
  pc = 0x102a4d8; continue;
 }
 0x102a4b1 => {
  { let v: u32 = 0x113726du32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102a4c0); f_0108e30e(cpu); if cpu.returned_to != 0x102a4c0 { return; }
  pc = 0x102a4d8; continue;
 }
 0x102a4c4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1d0102a5u32); cpu.eax = v; }
  { cpu.movs(4, false); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.movs(4, false); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x102a4d0, "Aas");
  { cpu.movs(4, false); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.movs(4, false); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x102a4d8; continue;
 }
 0x102a4d8 => {
  { let v: u8 = 0xeu8; cpu.w8(0x11d6896u32, v); }
  { let v: u32 = cpu.r32(0x11d648cu32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x43u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x31u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102a579; continue; }
  pc = 0x102a4f6; continue;
 }
 0x102a4f6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x102a4c4u32.wrapping_add(cpu.eax)); continue;
 }
 0x102a505 => {
  { let v: u32 = 0x1137277u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a514); f_0108ef8d(cpu); if cpu.returned_to != 0x102a514 { return; }
  { let v: u8 = 0xdu8; cpu.w8(0x11d6896u32, v); }
  pc = 0x102a579; continue;
 }
 0x102a51d => {
  { let v: u32 = 0x1137280u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a52c); f_0108ef8d(cpu); if cpu.returned_to != 0x102a52c { return; }
  pc = 0x102a579; continue;
 }
 0x102a52e => {
  { let v: u32 = 0x113728bu32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a53d); f_0108ef8d(cpu); if cpu.returned_to != 0x102a53d { return; }
  pc = 0x102a579; continue;
 }
 0x102a53f => {
  { let v: u32 = 0x1137296u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a54e); f_0108ef8d(cpu); if cpu.returned_to != 0x102a54e { return; }
  pc = 0x102a579; continue;
 }
 0x102a550 => {
  { let v: u32 = 0x11372a1u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a55f); f_0108ef8d(cpu); if cpu.returned_to != 0x102a55f { return; }
  { let v: u32 = cpu.r32(0x11d648cu32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x44u32.wrapping_add(cpu.eax)), 0x80u8); }
  if cpu.zf { pc = 0x102a579; continue; }
  pc = 0x102a56a; continue;
 }
 0x102a56a => {
  { let v: u32 = 0x11372acu32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a579); f_0108ef8d(cpu); if cpu.returned_to != 0x102a579 { return; }
  pc = 0x102a579; continue;
 }
 0x102a579 => {
  { let v: u32 = cpu.r32(0x11d648cu32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x43u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x33u8); }
  if cpu.cf { pc = 0x102a5a0; continue; }
  pc = 0x102a58a; continue;
 }
 0x102a58a => {
  { cpu.sub8(cpu.r8(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x33u8); }
  if (cpu.cf || cpu.zf) { pc = 0x102a5cf; continue; }
  pc = 0x102a590; continue;
 }
 0x102a590 => {
  { cpu.sub8(cpu.r8(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x35u8); }
  if (cpu.cf || cpu.zf) { pc = 0x102a5f3; continue; }
  pc = 0x102a596; continue;
 }
 0x102a596 => {
  pc = 0x102a65a; continue;
 }
 0x102a5a0 => {
  { cpu.sub8(cpu.r8(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x31u8); }
  if !cpu.cf { pc = 0x102a5ab; continue; }
  pc = 0x102a5a6; continue;
 }
 0x102a5a6 => {
  pc = 0x102a65a; continue;
 }
 0x102a5ab => {
  { let v: u32 = 0xbbu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d6640u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a5bd); f_010278aa(cpu); if cpu.returned_to != 0x102a5bd { return; }
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x102a5ca; continue; }
  pc = 0x102a5c1; continue;
 }
 0x102a5c1 => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a5ca; continue;
 }
 0x102a5ca => {
  pc = 0x102a65a; continue;
 }
 0x102a5cf => {
  { let v: u32 = 0xb0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d6640u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a5e1); f_010278aa(cpu); if cpu.returned_to != 0x102a5e1 { return; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x102a5ee; continue; }
  pc = 0x102a5e5; continue;
 }
 0x102a5e5 => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a5ee; continue;
 }
 0x102a5ee => {
  pc = 0x102a65a; continue;
 }
 0x102a5f3 => {
  { let v: u32 = 0xb0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d6640u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a605); f_010278aa(cpu); if cpu.returned_to != 0x102a605 { return; }
  { let v: u32 = cpu.eax & 0x18u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x8u16); }
  if cpu.cf { pc = 0x102a634; continue; }
  pc = 0x102a614; continue;
 }
 0x102a614 => {
  { cpu.sub16(cpu.r16(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x8u16); }
  if (cpu.cf || cpu.zf) { pc = 0x102a651; continue; }
  pc = 0x102a61b; continue;
 }
 0x102a61b => {
  { cpu.sub16(cpu.r16(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x10u16); }
  if cpu.cf { pc = 0x102a632; continue; }
  pc = 0x102a622; continue;
 }
 0x102a622 => {
  { cpu.sub16(cpu.r16(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x10u16); }
  if (cpu.cf || cpu.zf) { pc = 0x102a646; continue; }
  pc = 0x102a629; continue;
 }
 0x102a629 => {
  { cpu.sub16(cpu.r16(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x18u16); }
  if cpu.zf { pc = 0x102a651; continue; }
  pc = 0x102a630; continue;
 }
 0x102a630 => {
  pc = 0x102a65a; continue;
 }
 0x102a632 => {
  pc = 0x102a65a; continue;
 }
 0x102a634 => {
  { cpu.sub16(cpu.r16(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x102a65a; continue; }
  pc = 0x102a63b; continue;
 }
 0x102a63b => {
  { let v: u16 = 0x100u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a65a; continue;
 }
 0x102a646 => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a65a; continue;
 }
 0x102a651 => {
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a65a; continue;
 }
 0x102a65a => {
  pc = 0x102a681; continue;
 }
 0x102a65c => {
  { let v: u32 = 0x11372bcu32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a66b); f_0108e30e(cpu); if cpu.returned_to != 0x102a66b { return; }
  { let v: u32 = 0x113722fu32; cpu.edx = v; }
  { let v: u32 = 0x11d6890u32; cpu.eax = v; }
  cpu.push(0x102a67a); f_0108e30e(cpu); if cpu.returned_to != 0x102a67a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11d6896u32, v); }
  pc = 0x102a681; continue;
 }
 0x102a681 => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_0102a693(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102a693); }
 let mut pc: u32 = 0x102a693;
 loop { match pc {
 0x102a693 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11d640bu32), 0x0u8); }
  if cpu.zf { pc = 0x102a7b7; continue; }
  pc = 0x102a6b2; continue;
 }
 0x102a6b2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x38u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a6c6); f_010278f7(cpu); if cpu.returned_to != 0x102a6c6 { return; }
  { let v: u32 = 0xfu32; cpu.ebx = v; }
  { let v: u32 = 0x35u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a6dd); f_01027a5b(cpu); if cpu.returned_to != 0x102a6dd { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x102a7b7; continue; }
  pc = 0x102a6e5; continue;
 }
 0x102a6e5 => {
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  { let v: u32 = 0x38u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a6fc); f_010278f7(cpu); if cpu.returned_to != 0x102a6fc { return; }
  { let v: u32 = 0xfu32; cpu.ebx = v; }
  { let v: u32 = 0x35u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a713); f_01027a5b(cpu); if cpu.returned_to != 0x102a713 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x102a7b7; continue; }
  pc = 0x102a71b; continue;
 }
 0x102a71b => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x17u8; cpu.w8(0x11d6896u32, v); }
  { let v: u32 = 0x30u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a738); f_010278aa(cpu); if cpu.returned_to != 0x102a738 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x81u16); }
  if cpu.cf { pc = 0x102a75b; continue; }
  pc = 0x102a749; continue;
 }
 0x102a749 => {
  { cpu.sub16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x81u16); }
  if (cpu.cf || cpu.zf) { pc = 0x102a75d; continue; }
  pc = 0x102a751; continue;
 }
 0x102a751 => {
  { cpu.sub16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x82u16); }
  if cpu.zf { pc = 0x102a76e; continue; }
  pc = 0x102a759; continue;
 }
 0x102a759 => {
  pc = 0x102a77f; continue;
 }
 0x102a75b => {
  pc = 0x102a77f; continue;
 }
 0x102a75d => {
  { let v: u32 = 0x11372cbu32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a76c); f_0108e30e(cpu); if cpu.returned_to != 0x102a76c { return; }
  pc = 0x102a78d; continue;
 }
 0x102a76e => {
  { let v: u32 = 0x11372d5u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a77d); f_0108e30e(cpu); if cpu.returned_to != 0x102a77d { return; }
  pc = 0x102a78d; continue;
 }
 0x102a77f => {
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x11372e0u32; cpu.eax = v; }
  cpu.push(0x102a78d); f_01029142(cpu); if cpu.returned_to != 0x102a78d { return; }
  pc = 0x102a78d; continue;
 }
 0x102a78d => {
  { let v: u32 = 0x41u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a79f); f_010278aa(cpu); if cpu.returned_to != 0x102a79f { return; }
  { cpu.and8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x102a7ae; continue; }
  pc = 0x102a7a3; continue;
 }
 0x102a7a3 => {
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a7b7; continue;
 }
 0x102a7ae => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a7b7; continue;
 }
 0x102a7b7 => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_0102a838(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a838); } r_0102a838(cpu, 0x102a838); }
pub fn f_0102a893(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a893); } r_0102a838(cpu, 0x102a893); }
pub fn f_0102a89e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a89e); } r_0102a838(cpu, 0x102a89e); }
pub fn f_0102a8a9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a8a9); } r_0102a838(cpu, 0x102a8a9); }
pub fn f_0102a8b4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102a8b4); } r_0102a838(cpu, 0x102a8b4); }
fn r_0102a838(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102a838 => {
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  { let v: u32 = cpu.r32(0xa90102a8u32.wrapping_add(cpu.esi)).wrapping_add(cpu.ebx); cpu.w32(0xa90102a8u32.wrapping_add(cpu.esi), v); }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  { let v: u32 = cpu.r32(0x45c60102u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(4))).wrapping_add(cpu.esi); cpu.w32(0x45c60102u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { cpu.cf = false; }
  { let v: u32 = cpu.r32(0x11372e3u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edi); cpu.w32(0x11372e3u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a85b); f_0108e30e(cpu); if cpu.returned_to != 0x102a85b { return; }
  { let v: u8 = 0x18u8; cpu.w8(0x11d6896u32, v); }
  { let v: u32 = 0x1eu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102a874); f_010278aa(cpu); if cpu.returned_to != 0x102a874 { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x3u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102a8bd; continue; }
  pc = 0x102a883; continue;
 }
 0x102a883 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x102a838u32.wrapping_add(cpu.eax)); continue;
 }
 0x102a893 => {
  { let v: u16 = 0x100u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a8bd; continue;
 }
 0x102a89e => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a8bd; continue;
 }
 0x102a8a9 => {
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a8bd; continue;
 }
 0x102a8b4 => {
  { let v: u16 = 0x800u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102a8bd; continue;
 }
 0x102a8bd => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_0102a8cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102a8cf); }
 let mut pc: u32 = 0x102a8cf;
 loop { match pc {
 0x102a8cf => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11d640fu32), 0x0u8); }
  if cpu.zf { pc = 0x102a920; continue; }
  pc = 0x102a8ea; continue;
 }
 0x102a8ea => {
  { let v: u32 = 0x4f03u32; cpu.eax = v; }
  cpu.push(0x102a8f4); f_01027873(cpu); if cpu.returned_to != 0x102a8f4 { return; }
  { cpu.sub8(cpu.r8(0x11d6490u32), 0x4fu8); }
  if !cpu.zf { pc = 0x102a920; continue; }
  pc = 0x102a8fd; continue;
 }
 0x102a8fd => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11372f5u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a910); f_0108e30e(cpu); if cpu.returned_to != 0x102a910 { return; }
  { let v: u8 = 0x1bu8; cpu.w8(0x11d6896u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x1121344u32, v); }
  pc = 0x102a920; continue;
 }
 0x102a920 => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_0102a932(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102a932); }
 let mut pc: u32 = 0x102a932;
 loop { match pc {
 0x102a932 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11d6409u32), 0x0u8); }
  if cpu.zf { pc = 0x102a978; continue; }
  pc = 0x102a94d; continue;
 }
 0x102a94d => {
  { let v: u32 = 0x7cu32; cpu.ebx = v; }
  { let v: u32 = 0x7cu32; cpu.edx = v; }
  { let v: u32 = 0x3d4u32; cpu.eax = v; }
  cpu.push(0x102a961); f_01027a5b(cpu); if cpu.returned_to != 0x102a961 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x102a978; continue; }
  pc = 0x102a965; continue;
 }
 0x102a965 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11372fau32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102a978); f_0108e30e(cpu); if cpu.returned_to != 0x102a978 { return; }
  pc = 0x102a978; continue;
 }
 0x102a978 => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_0102ae98(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ae98); } r_0102ae98(cpu, 0x102ae98); }
pub fn f_0102aed9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102aed9); } r_0102ae98(cpu, 0x102aed9); }
pub fn f_0102aee4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102aee4); } r_0102ae98(cpu, 0x102aee4); }
pub fn f_0102aeef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102aeef); } r_0102ae98(cpu, 0x102aeef); }
pub fn f_0102af0c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102af0c); } r_0102ae98(cpu, 0x102af0c); }
fn r_0102ae98(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102ae98 => {
  { cpu.fpu_control = cpu.r16(0xaee40102u32.wrapping_add(cpu.esi)); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { cpu.scas(1, 0); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0xafu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1eu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d664au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102aeba); f_010278aa(cpu); if cpu.returned_to != 0x102aeba { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)), 0x3u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102af27; continue; }
  pc = 0x102aec9; continue;
 }
 0x102aec9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x102ae98u32.wrapping_add(cpu.eax)); continue;
 }
 0x102aed9 => {
  { let v: u16 = 0x100u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102af27; continue;
 }
 0x102aee4 => {
  { let v: u16 = 0x200u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102af27; continue;
 }
 0x102aeef => {
  { cpu.sub16(cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x102af01; continue; }
  pc = 0x102aef6; continue;
 }
 0x102aef6 => {
  { let v: u16 = 0x300u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102af0a; continue;
 }
 0x102af01 => {
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102af0a; continue;
 }
 0x102af0a => {
  pc = 0x102af27; continue;
 }
 0x102af0c => {
  { cpu.sub16(cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x102af1e; continue; }
  pc = 0x102af13; continue;
 }
 0x102af13 => {
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102af27; continue;
 }
 0x102af1e => {
  { let v: u16 = 0x800u16; cpu.w16(0x11d665au32, v); }
  pc = 0x102af27; continue;
 }
 0x102af27 => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_0102af39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102af39); }
 let mut pc: u32 = 0x102af39;
 loop { match pc {
 0x102af39 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x11d6896u32), 0x18u8); }
  if !cpu.zf { pc = 0x102af70; continue; }
  pc = 0x102af50; continue;
 }
 0x102af50 => {
  { let v: u16 = cpu.r16(0x10277dbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d664eu32, v); }
  { let v: u32 = 0x113737eu32; cpu.edx = v; }
  { let v: u32 = 0x11d64ccu32; cpu.eax = v; }
  cpu.push(0x102af6b); f_0108e30e(cpu); if cpu.returned_to != 0x102af6b { return; }
  pc = 0x102b1b8; continue;
 }
 0x102af70 => {
  { let v: u32 = 0x1137389u32; cpu.edx = v; }
  { let v: u32 = 0x11d64ccu32; cpu.eax = v; }
  cpu.push(0x102af7f); f_0108e30e(cpu); if cpu.returned_to != 0x102af7f { return; }
  { let v: u16 = cpu.r16(0x10277d3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d664eu32, v); }
  cpu.push(0x102af90); f_01027b48(cpu); if cpu.returned_to != 0x102af90 { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102af9a); f_010951ba(cpu); if cpu.returned_to != 0x102af9a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102af9d; continue;
 }
 0x102af9d => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102afad); f_010951ba(cpu); if cpu.returned_to != 0x102afad { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { cpu.poll(); pc = 0x102af9d; continue; }
  pc = 0x102afb9; continue;
 }
 0x102afb9 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x102afc4); f_01027b67(cpu); if cpu.returned_to != 0x102afc4 { return; }
  { cpu.sub16(cpu.r16(0x11d663eu32), 0x8eu16); }
  if !cpu.zf { pc = 0x102affa; continue; }
  pc = 0x102afcf; continue;
 }
 0x102afcf => {
  { let v: u32 = 0x8u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102afd6; continue;
 }
 0x102afd6 => {
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102afe0); f_010951ba(cpu); if cpu.returned_to != 0x102afe0 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x8eu16); }
  if cpu.zf { pc = 0x102aff8; continue; }
  pc = 0x102aff1; continue;
 }
 0x102aff1 => {
  { cpu.sub16(cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { cpu.poll(); pc = 0x102afd6; continue; }
  pc = 0x102aff8; continue;
 }
 0x102aff8 => {
  pc = 0x102b003; continue;
 }
 0x102affa => {
  { let v: u16 = cpu.r16(0x11d663eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102b003; continue;
 }
 0x102b003 => {
  { cpu.sub16(cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x8eu16); }
  if !cpu.zf { pc = 0x102b030; continue; }
  pc = 0x102b00b; continue;
 }
 0x102b00b => {
  { let v: u16 = cpu.r16(0x10277dfu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d664eu32, v); }
  { let v: u32 = 0x1137390u32; cpu.edx = v; }
  { let v: u32 = 0x11d64ccu32; cpu.eax = v; }
  cpu.push(0x102b026); f_0108e30e(cpu); if cpu.returned_to != 0x102b026 { return; }
  cpu.push(0x102b02b); f_01027b48(cpu); if cpu.returned_to != 0x102b02b { return; }
  pc = 0x102b1b8; continue;
 }
 0x102b030 => {
  cpu.push(0x102b035); f_01027b67(cpu); if cpu.returned_to != 0x102b035 { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b03f); f_010951ba(cpu); if cpu.returned_to != 0x102b03f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x102b047); f_01027b48(cpu); if cpu.returned_to != 0x102b047 { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b051); f_010951ba(cpu); if cpu.returned_to != 0x102b051 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b06a); f_010951c2(cpu); if cpu.returned_to != 0x102b06a { return; }
  cpu.push(0x102b06f); f_01027b67(cpu); if cpu.returned_to != 0x102b06f { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b079); f_010951ba(cpu); if cpu.returned_to != 0x102b079 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x102b1a5; continue; }
  pc = 0x102b089; continue;
 }
 0x102b089 => {
  cpu.push(0x102b08e); f_01027b67(cpu); if cpu.returned_to != 0x102b08e { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x60u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b0a4); f_010951c2(cpu); if cpu.returned_to != 0x102b0a4 { return; }
  cpu.push(0x102b0a9); f_01027b67(cpu); if cpu.returned_to != 0x102b0a9 { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b0b3); f_010951ba(cpu); if cpu.returned_to != 0x102b0b3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1137395u32; cpu.edx = v; }
  { let v: u32 = 0x11d64ccu32; cpu.eax = v; }
  cpu.push(0x102b0c5); f_0108e30e(cpu); if cpu.returned_to != 0x102b0c5 { return; }
  { let v: u16 = cpu.r16(0x10277d7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d664eu32, v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xe0u32; cpu.eax = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xe0u32; cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x102b192; continue; }
  pc = 0x102b0ed; continue;
 }
 0x102b0ed => {
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b0f7); f_010951ba(cpu); if cpu.returned_to != 0x102b0f7 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x102b0ff); f_01027b48(cpu); if cpu.returned_to != 0x102b0ff { return; }
  { let v: u32 = 0x11373a4u32; cpu.edx = v; }
  { let v: u32 = 0x11d64ccu32; cpu.eax = v; }
  cpu.push(0x102b10e); f_0108e30e(cpu); if cpu.returned_to != 0x102b10e { return; }
  { let v: u16 = cpu.r16(0x10277d7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d664eu32, v); }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b124); f_010951ba(cpu); if cpu.returned_to != 0x102b124 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x102b192; continue; }
  pc = 0x102b12e; continue;
 }
 0x102b12e => {
  { let v: u32 = 0x11373b3u32; cpu.edx = v; }
  { let v: u32 = 0x11d64ccu32; cpu.eax = v; }
  cpu.push(0x102b13d); f_0108e30e(cpu); if cpu.returned_to != 0x102b13d { return; }
  { let v: u16 = cpu.r16(0x10277e3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d664eu32, v); }
  cpu.push(0x102b14e); f_01027b67(cpu); if cpu.returned_to != 0x102b14e { return; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b15d); f_010951c2(cpu); if cpu.returned_to != 0x102b15d { return; }
  cpu.push(0x102b162); f_01027b67(cpu); if cpu.returned_to != 0x102b162 { return; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b16c); f_010951ba(cpu); if cpu.returned_to != 0x102b16c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)), 0xffu16); }
  if cpu.zf { pc = 0x102b192; continue; }
  pc = 0x102b177; continue;
 }
 0x102b177 => {
  { let v: u32 = 0x11373c0u32; cpu.edx = v; }
  { let v: u32 = 0x11d64ccu32; cpu.eax = v; }
  cpu.push(0x102b186); f_0108e30e(cpu); if cpu.returned_to != 0x102b186 { return; }
  { let v: u16 = cpu.r16(0x10277e7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d664eu32, v); }
  pc = 0x102b192; continue;
 }
 0x102b192 => {
  cpu.push(0x102b197); f_01027b67(cpu); if cpu.returned_to != 0x102b197 { return; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b1a5); f_010951c2(cpu); if cpu.returned_to != 0x102b1a5 { return; }
  pc = 0x102b1a5; continue;
 }
 0x102b1a5 => {
  cpu.push(0x102b1aa); f_01027b48(cpu); if cpu.returned_to != 0x102b1aa { return; }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = 0x3c6u32; cpu.eax = v; }
  cpu.push(0x102b1b8); f_010951c2(cpu); if cpu.returned_to != 0x102b1b8 { return; }
  pc = 0x102b1b8; continue;
 }
 0x102b1b8 => {
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
pub fn f_0102b1c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102b1c1); }
 let mut pc: u32 = 0x102b1c1;
 loop { match pc {
 0x102b1c1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xc0000u32; cpu.w32(0x11d648cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102b1e0; continue;
 }
 0x102b1e0 => {
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x6u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102b1f1; continue; }
  pc = 0x102b1e7; continue;
 }
 0x102b1e7 => {
  pc = 0x102b1fc; continue;
 }
 0x102b1e9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102b1e0; continue;
 }
 0x102b1f1 => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffff0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  cpu.poll(); pc = 0x102b1e9; continue;
 }
 0x102b1fc => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102b203; continue;
 }
 0x102b203 => {
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x6u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102b214; continue; }
  pc = 0x102b20a; continue;
 }
 0x102b20a => {
  pc = 0x102b254; continue;
 }
 0x102b20c => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102b203; continue;
 }
 0x102b214 => {
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11d6495u32, v); }
  { let v: u32 = 0x1130u32; cpu.eax = v; }
  cpu.push(0x102b226); f_01027873(cpu); if cpu.returned_to != 0x102b226 { return; }
  { cpu.sub16(cpu.r16(0x11d64acu32), 0xc000u16); }
  if cpu.cf { pc = 0x102b23c; continue; }
  pc = 0x102b231; continue;
 }
 0x102b231 => {
  { let v: u16 = cpu.r16(0x11d64acu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x102b23e; continue;
 }
 0x102b23c => {
  pc = 0x102b252; continue;
 }
 0x102b23e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11d64acu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0xc000u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)))); cpu.w8(0xfffffff0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x102b252; continue;
 }
 0x102b252 => {
  cpu.poll(); pc = 0x102b20c; continue;
 }
 0x102b254 => {
  { let v: u16 = 0x0u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102b25a; continue;
 }
 0x102b25a => {
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x6u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102b26c; continue; }
  pc = 0x102b261; continue;
 }
 0x102b261 => {
  pc = 0x102b27d; continue;
 }
 0x102b263 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.inc16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102b25a; continue;
 }
 0x102b26c => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 15u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xc0000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102b263; continue;
 }
 0x102b27d => {
  { let v: u16 = 0x6u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102b283; continue;
 }
 0x102b283 => {
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if (cpu.sf == cpu.of) { pc = 0x102b295; continue; }
  pc = 0x102b28a; continue;
 }
 0x102b28a => {
  pc = 0x102b2b3; continue;
 }
 0x102b28c => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.dec16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102b283; continue;
 }
 0x102b295 => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0xfffffff0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))), 0x0u8); }
  if (cpu.cf || cpu.zf) { pc = 0x102b2b1; continue; }
  pc = 0x102b2a0; continue;
 }
 0x102b2a0 => {
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 15u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xc0000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d648cu32, v); }
  pc = 0x102b2b1; continue;
 }
 0x102b2b1 => {
  cpu.poll(); pc = 0x102b28c; continue;
 }
 0x102b2b3 => {
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
pub fn f_0102b2bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102b2bc); } r_0102b2bc(cpu, 0x102b2bc); }
pub fn f_0102b3ec(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102b3ec); } r_0102b2bc(cpu, 0x102b3ec); }
pub fn f_0102b44a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102b44a); } r_0102b2bc(cpu, 0x102b44a); }
pub fn f_0102b45b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102b45b); } r_0102b2bc(cpu, 0x102b45b); }
pub fn f_0102b46c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102b46c); } r_0102b2bc(cpu, 0x102b46c); }
pub fn f_0102b47b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102b47b); } r_0102b2bc(cpu, 0x102b47b); }
fn r_0102b2bc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102b2bc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0x10277cfu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d664eu32, v); }
  { let v: u32 = 0x11373ccu32; cpu.edx = v; }
  { let v: u32 = 0x11d643cu32; cpu.eax = v; }
  cpu.push(0x102b2e5); f_0108e30e(cpu); if cpu.returned_to != 0x102b2e5 { return; }
  { let v: u32 = 0x1137261u32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102b2f4); f_0108e30e(cpu); if cpu.returned_to != 0x102b2f4 { return; }
  { let v: u8 = 0x1fu8; cpu.w8(0x11d6896u32, v); }
  { let v: u32 = 0x11373ccu32; cpu.edx = v; }
  { let v: u32 = 0x11d64e0u32; cpu.eax = v; }
  cpu.push(0x102b30a); f_0108e30e(cpu); if cpu.returned_to != 0x102b30a { return; }
  { let v: u32 = 0x1137261u32; cpu.edx = v; }
  { let v: u32 = 0x11d6414u32; cpu.eax = v; }
  cpu.push(0x102b319); f_0108e30e(cpu); if cpu.returned_to != 0x102b319 { return; }
  { let v: u32 = 0x11373cfu32; cpu.edx = v; }
  { let v: u32 = 0x11d6890u32; cpu.eax = v; }
  cpu.push(0x102b328); f_0108e30e(cpu); if cpu.returned_to != 0x102b328 { return; }
  { let v: u8 = 0x12u8; cpu.w8(0x11d6491u32, v); }
  { let v: u16 = 0x1010u16; cpu.w16(0x11d6494u32, v); }
  { let v: u32 = 0x11d6490u32; cpu.ebx = v; }
  { let v: u32 = 0x11d6490u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  cpu.push(0x102b34c); f_0108ef70(cpu); if cpu.returned_to != 0x102b34c { return; }
  { cpu.sub8(cpu.r8(0x11d6495u32), 0x1u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102b505; continue; }
  pc = 0x102b359; continue;
 }
 0x102b359 => {
  { let v: u32 = 0x113722fu32; cpu.edx = v; }
  { let v: u32 = 0x11d6890u32; cpu.eax = v; }
  cpu.push(0x102b368); f_0108e30e(cpu); if cpu.returned_to != 0x102b368 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11d6896u32, v); }
  { let v: u32 = 0x3ccu32; cpu.eax = v; }
  cpu.push(0x102b379); f_010951ba(cpu); if cpu.returned_to != 0x102b379 { return; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x102b388; continue; }
  pc = 0x102b37d; continue;
 }
 0x102b37d => {
  { let v: u16 = 0x3d4u16; cpu.w16(0x11d664cu32, v); }
  pc = 0x102b391; continue;
 }
 0x102b388 => {
  { let v: u16 = 0x3b4u16; cpu.w16(0x11d664cu32, v); }
  pc = 0x102b391; continue;
 }
 0x102b391 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d6494u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d665au32, v); }
  { let v: u32 = 0x1a00u32; cpu.eax = v; }
  cpu.push(0x102b3a8); f_01027873(cpu); if cpu.returned_to != 0x102b3a8 { return; }
  { cpu.sub8(cpu.r8(0x11d6490u32), 0x1au8); }
  if !cpu.zf { pc = 0x102b505; continue; }
  pc = 0x102b3b5; continue;
 }
 0x102b3aa => {
  {  }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x850f1a01u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1d64943du32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x1d64943du32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(2))).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(2)), v); }
  { let v: u32 = cpu.r32(0x1d64953du32.wrapping_add(cpu.eax)) | cpu.eax; cpu.w32(0x1d64953du32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102b3c9; continue; }
  pc = 0x102b3c7; continue;
 }
 0x102b3b5 => {
  { cpu.sub8(cpu.r8(0x11d6494u32), 0x4u8); }
  if !cpu.cf { pc = 0x102b3c7; continue; }
  pc = 0x102b3be; continue;
 }
 0x102b3be => {
  { cpu.sub8(cpu.r8(0x11d6495u32), 0x3u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102b3c9; continue; }
  pc = 0x102b3c7; continue;
 }
 0x102b3c7 => {
  pc = 0x102b41c; continue;
 }
 0x102b3c9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d6494u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11d6646u32, v); }
  { let v: u8 = cpu.r8(0x11d6495u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11d6494u32, v); }
  { let v: u8 = cpu.r8(0x11d6646u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11d6495u32, v); }
  pc = 0x102b41c; continue;
 }
 0x102b3ec => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.edx)).wrapping_add(cpu.ecx); cpu.w32(0xffffffb4u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.pf() { cpu.poll(); pc = 0x102b3aa; continue; }
  pc = 0x102b3f6; continue;
 }
 0x102b3f6 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ebx); cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ebx); cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x102b408, "Insb");
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.edi); cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ebx); cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x102b418, "Insb");
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x11373d4u32.wrapping_add(cpu.edx)).wrapping_add(cpu.edi); cpu.w32(0x11373d4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x11d6890u32; cpu.eax = v; }
  cpu.push(0x102b42b); f_0108e30e(cpu); if cpu.returned_to != 0x102b42b { return; }
  { let v: u8 = cpu.r8(0x11d6494u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xbu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102b47b; continue; }
  pc = 0x102b43b; continue;
 }
 0x102b41c => {
  { let v: u32 = 0x11373d4u32; cpu.edx = v; }
  { let v: u32 = 0x11d6890u32; cpu.eax = v; }
  cpu.push(0x102b42b); f_0108e30e(cpu); if cpu.returned_to != 0x102b42b { return; }
  { let v: u8 = cpu.r8(0x11d6494u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xbu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x102b47b; continue; }
  pc = 0x102b43b; continue;
 }
 0x102b43b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x102b3ecu32.wrapping_add(cpu.eax)); continue;
 }
 0x102b44a => {
  { let v: u32 = 0x11373d9u32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102b459); f_0108e30e(cpu); if cpu.returned_to != 0x102b459 { return; }
  pc = 0x102b47b; continue;
 }
 0x102b45b => {
  { let v: u32 = 0x1137245u32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102b46a); f_0108e30e(cpu); if cpu.returned_to != 0x102b46a { return; }
  pc = 0x102b47b; continue;
 }
 0x102b46c => {
  { let v: u32 = 0x1137250u32; cpu.edx = v; }
  { let v: u32 = 0x11d64b8u32; cpu.eax = v; }
  cpu.push(0x102b47b); f_0108e30e(cpu); if cpu.returned_to != 0x102b47b { return; }
  pc = 0x102b47b; continue;
 }
 0x102b47b => {
  { let v: u8 = cpu.r8(0x11d6495u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.cf { pc = 0x102b497; continue; }
  pc = 0x102b489; continue;
 }
 0x102b489 => {
  { cpu.sub8(cpu.r8(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if (cpu.cf || cpu.zf) { pc = 0x102b499; continue; }
  pc = 0x102b48f; continue;
 }
 0x102b48f => {
  { cpu.sub8(cpu.r8(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x102b4a8; continue; }
  pc = 0x102b495; continue;
 }
 0x102b495 => {
  pc = 0x102b4b7; continue;
 }
 0x102b497 => {
  pc = 0x102b4b7; continue;
 }
 0x102b499 => {
  { let v: u32 = 0x1137245u32; cpu.edx = v; }
  { let v: u32 = 0x11d64e0u32; cpu.eax = v; }
  cpu.push(0x102b4a8); f_0108e30e(cpu); if cpu.returned_to != 0x102b4a8 { return; }
  pc = 0x102b4a8; continue;
 }
 0x102b4a8 => {
  { let v: u32 = 0x113725du32; cpu.edx = v; }
  { let v: u32 = 0x11d64e0u32; cpu.eax = v; }
  cpu.push(0x102b4b7); f_0108e30e(cpu); if cpu.returned_to != 0x102b4b7 { return; }
  pc = 0x102b4b7; continue;
 }
 0x102b4b7 => {
  cpu.push(0x102b4bc); f_0102b1c1(cpu); if cpu.returned_to != 0x102b4bc { return; }
  { cpu.sub8(cpu.r8(0x11d6494u32), 0xau8); }
  if cpu.cf { pc = 0x102b4ce; continue; }
  pc = 0x102b4c5; continue;
 }
 0x102b4c5 => {
  { cpu.sub8(cpu.r8(0x11d6494u32), 0xcu8); }
  if (cpu.cf || cpu.zf) { pc = 0x102b505; continue; }
  pc = 0x102b4ce; continue;
 }
 0x102b4ce => {
  { let v: u32 = 0x11373e3u32; cpu.edx = v; }
  { let v: u32 = 0x11d6890u32; cpu.eax = v; }
  cpu.push(0x102b4dd); f_0108e30e(cpu); if cpu.returned_to != 0x102b4dd { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x11d6896u32, v); }
  { let v: u16 = 0x100u16; cpu.w16(0x11d665au32, v); }
  cpu.push(0x102b4f2); f_0102a8cf(cpu); if cpu.returned_to != 0x102b4f2 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x102b505; continue; }
  pc = 0x102b4f6; continue;
 }
 0x102b4f6 => {
  { let v: u32 = 0x11373e7u32; cpu.edx = v; }
  { let v: u32 = 0x11d643cu32; cpu.eax = v; }
  cpu.push(0x102b505); f_0108e30e(cpu); if cpu.returned_to != 0x102b505 { return; }
  pc = 0x102b505; continue;
 }
 0x102b505 => {
  { let v: u8 = cpu.r8(0x11d6896u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
