#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0109a796(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a796); }
 let mut pc: u32 = 0x109a796;
 loop { match pc {
 0x109a796 => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a799(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a799); }
 let mut pc: u32 = 0x109a799;
 loop { match pc {
 0x109a799 => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a79c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a79c); }
 let mut pc: u32 = 0x109a79c;
 loop { match pc {
 0x109a79c => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a79f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a79f); }
 let mut pc: u32 = 0x109a79f;
 loop { match pc {
 0x109a79f => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7a2); }
 let mut pc: u32 = 0x109a7a2;
 loop { match pc {
 0x109a7a2 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7a5); }
 let mut pc: u32 = 0x109a7a5;
 loop { match pc {
 0x109a7a5 => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7a8); }
 let mut pc: u32 = 0x109a7a8;
 loop { match pc {
 0x109a7a8 => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7ab); }
 let mut pc: u32 = 0x109a7ab;
 loop { match pc {
 0x109a7ab => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7ae); }
 let mut pc: u32 = 0x109a7ae;
 loop { match pc {
 0x109a7ae => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7b1); }
 let mut pc: u32 = 0x109a7b1;
 loop { match pc {
 0x109a7b1 => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7b4); }
 let mut pc: u32 = 0x109a7b4;
 loop { match pc {
 0x109a7b4 => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7b7); }
 let mut pc: u32 = 0x109a7b7;
 loop { match pc {
 0x109a7b7 => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7ba); }
 let mut pc: u32 = 0x109a7ba;
 loop { match pc {
 0x109a7ba => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7bd); }
 let mut pc: u32 = 0x109a7bd;
 loop { match pc {
 0x109a7bd => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7c0); }
 let mut pc: u32 = 0x109a7c0;
 loop { match pc {
 0x109a7c0 => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7c3); }
 let mut pc: u32 = 0x109a7c3;
 loop { match pc {
 0x109a7c3 => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7c6); }
 let mut pc: u32 = 0x109a7c6;
 loop { match pc {
 0x109a7c6 => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7c9); }
 let mut pc: u32 = 0x109a7c9;
 loop { match pc {
 0x109a7c9 => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7cc); }
 let mut pc: u32 = 0x109a7cc;
 loop { match pc {
 0x109a7cc => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7cf); }
 let mut pc: u32 = 0x109a7cf;
 loop { match pc {
 0x109a7cf => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7d2); }
 let mut pc: u32 = 0x109a7d2;
 loop { match pc {
 0x109a7d2 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7d5); }
 let mut pc: u32 = 0x109a7d5;
 loop { match pc {
 0x109a7d5 => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7d8); }
 let mut pc: u32 = 0x109a7d8;
 loop { match pc {
 0x109a7d8 => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7db); }
 let mut pc: u32 = 0x109a7db;
 loop { match pc {
 0x109a7db => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7de); }
 let mut pc: u32 = 0x109a7de;
 loop { match pc {
 0x109a7de => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7e1); }
 let mut pc: u32 = 0x109a7e1;
 loop { match pc {
 0x109a7e1 => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7e4); }
 let mut pc: u32 = 0x109a7e4;
 loop { match pc {
 0x109a7e4 => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7e7); }
 let mut pc: u32 = 0x109a7e7;
 loop { match pc {
 0x109a7e7 => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7ea); }
 let mut pc: u32 = 0x109a7ea;
 loop { match pc {
 0x109a7ea => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7ed); }
 let mut pc: u32 = 0x109a7ed;
 loop { match pc {
 0x109a7ed => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7f0); }
 let mut pc: u32 = 0x109a7f0;
 loop { match pc {
 0x109a7f0 => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7f3); }
 let mut pc: u32 = 0x109a7f3;
 loop { match pc {
 0x109a7f3 => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7f6); }
 let mut pc: u32 = 0x109a7f6;
 loop { match pc {
 0x109a7f6 => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7f9); }
 let mut pc: u32 = 0x109a7f9;
 loop { match pc {
 0x109a7f9 => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7fc); }
 let mut pc: u32 = 0x109a7fc;
 loop { match pc {
 0x109a7fc => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a7ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a7ff); }
 let mut pc: u32 = 0x109a7ff;
 loop { match pc {
 0x109a7ff => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a802(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a802); }
 let mut pc: u32 = 0x109a802;
 loop { match pc {
 0x109a802 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a805(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a805); }
 let mut pc: u32 = 0x109a805;
 loop { match pc {
 0x109a805 => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a808(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a808); }
 let mut pc: u32 = 0x109a808;
 loop { match pc {
 0x109a808 => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a80b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a80b); }
 let mut pc: u32 = 0x109a80b;
 loop { match pc {
 0x109a80b => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a80e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a80e); }
 let mut pc: u32 = 0x109a80e;
 loop { match pc {
 0x109a80e => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a811(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a811); }
 let mut pc: u32 = 0x109a811;
 loop { match pc {
 0x109a811 => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a814(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a814); }
 let mut pc: u32 = 0x109a814;
 loop { match pc {
 0x109a814 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a817(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a817); }
 let mut pc: u32 = 0x109a817;
 loop { match pc {
 0x109a817 => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a81a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a81a); }
 let mut pc: u32 = 0x109a81a;
 loop { match pc {
 0x109a81a => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a81d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a81d); }
 let mut pc: u32 = 0x109a81d;
 loop { match pc {
 0x109a81d => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a820); }
 let mut pc: u32 = 0x109a820;
 loop { match pc {
 0x109a820 => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a823(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a823); }
 let mut pc: u32 = 0x109a823;
 loop { match pc {
 0x109a823 => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a826(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a826); }
 let mut pc: u32 = 0x109a826;
 loop { match pc {
 0x109a826 => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a829(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a829); }
 let mut pc: u32 = 0x109a829;
 loop { match pc {
 0x109a829 => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a82c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a82c); }
 let mut pc: u32 = 0x109a82c;
 loop { match pc {
 0x109a82c => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a82f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a82f); }
 let mut pc: u32 = 0x109a82f;
 loop { match pc {
 0x109a82f => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a832(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a832); }
 let mut pc: u32 = 0x109a832;
 loop { match pc {
 0x109a832 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a835(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a835); }
 let mut pc: u32 = 0x109a835;
 loop { match pc {
 0x109a835 => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a838(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a838); }
 let mut pc: u32 = 0x109a838;
 loop { match pc {
 0x109a838 => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a83b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a83b); }
 let mut pc: u32 = 0x109a83b;
 loop { match pc {
 0x109a83b => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a83e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a83e); }
 let mut pc: u32 = 0x109a83e;
 loop { match pc {
 0x109a83e => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a841(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a841); }
 let mut pc: u32 = 0x109a841;
 loop { match pc {
 0x109a841 => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a844(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a844); }
 let mut pc: u32 = 0x109a844;
 loop { match pc {
 0x109a844 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a847(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a847); }
 let mut pc: u32 = 0x109a847;
 loop { match pc {
 0x109a847 => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a84a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a84a); }
 let mut pc: u32 = 0x109a84a;
 loop { match pc {
 0x109a84a => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a84d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a84d); }
 let mut pc: u32 = 0x109a84d;
 loop { match pc {
 0x109a84d => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a850); }
 let mut pc: u32 = 0x109a850;
 loop { match pc {
 0x109a850 => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a853(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a853); }
 let mut pc: u32 = 0x109a853;
 loop { match pc {
 0x109a853 => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a856(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a856); }
 let mut pc: u32 = 0x109a856;
 loop { match pc {
 0x109a856 => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a859(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a859); }
 let mut pc: u32 = 0x109a859;
 loop { match pc {
 0x109a859 => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a85c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a85c); }
 let mut pc: u32 = 0x109a85c;
 loop { match pc {
 0x109a85c => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a85f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a85f); }
 let mut pc: u32 = 0x109a85f;
 loop { match pc {
 0x109a85f => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a862(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a862); }
 let mut pc: u32 = 0x109a862;
 loop { match pc {
 0x109a862 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a865(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a865); }
 let mut pc: u32 = 0x109a865;
 loop { match pc {
 0x109a865 => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a868(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a868); }
 let mut pc: u32 = 0x109a868;
 loop { match pc {
 0x109a868 => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a86b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a86b); }
 let mut pc: u32 = 0x109a86b;
 loop { match pc {
 0x109a86b => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a86e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a86e); }
 let mut pc: u32 = 0x109a86e;
 loop { match pc {
 0x109a86e => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a871(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a871); }
 let mut pc: u32 = 0x109a871;
 loop { match pc {
 0x109a871 => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a874(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a874); }
 let mut pc: u32 = 0x109a874;
 loop { match pc {
 0x109a874 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a877(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a877); }
 let mut pc: u32 = 0x109a877;
 loop { match pc {
 0x109a877 => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a87a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a87a); }
 let mut pc: u32 = 0x109a87a;
 loop { match pc {
 0x109a87a => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a87d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a87d); }
 let mut pc: u32 = 0x109a87d;
 loop { match pc {
 0x109a87d => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a880); }
 let mut pc: u32 = 0x109a880;
 loop { match pc {
 0x109a880 => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a883(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a883); }
 let mut pc: u32 = 0x109a883;
 loop { match pc {
 0x109a883 => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a886(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a886); }
 let mut pc: u32 = 0x109a886;
 loop { match pc {
 0x109a886 => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a889(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a889); }
 let mut pc: u32 = 0x109a889;
 loop { match pc {
 0x109a889 => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a88c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a88c); }
 let mut pc: u32 = 0x109a88c;
 loop { match pc {
 0x109a88c => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a88f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a88f); }
 let mut pc: u32 = 0x109a88f;
 loop { match pc {
 0x109a88f => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a892(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a892); }
 let mut pc: u32 = 0x109a892;
 loop { match pc {
 0x109a892 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a895(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a895); }
 let mut pc: u32 = 0x109a895;
 loop { match pc {
 0x109a895 => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a898(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a898); }
 let mut pc: u32 = 0x109a898;
 loop { match pc {
 0x109a898 => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a89b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a89b); }
 let mut pc: u32 = 0x109a89b;
 loop { match pc {
 0x109a89b => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a89e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a89e); }
 let mut pc: u32 = 0x109a89e;
 loop { match pc {
 0x109a89e => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8a1); }
 let mut pc: u32 = 0x109a8a1;
 loop { match pc {
 0x109a8a1 => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8a4); }
 let mut pc: u32 = 0x109a8a4;
 loop { match pc {
 0x109a8a4 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8a7); }
 let mut pc: u32 = 0x109a8a7;
 loop { match pc {
 0x109a8a7 => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8aa); }
 let mut pc: u32 = 0x109a8aa;
 loop { match pc {
 0x109a8aa => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8ad); }
 let mut pc: u32 = 0x109a8ad;
 loop { match pc {
 0x109a8ad => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8b0); }
 let mut pc: u32 = 0x109a8b0;
 loop { match pc {
 0x109a8b0 => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8b3); }
 let mut pc: u32 = 0x109a8b3;
 loop { match pc {
 0x109a8b3 => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8b6); }
 let mut pc: u32 = 0x109a8b6;
 loop { match pc {
 0x109a8b6 => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8b9); }
 let mut pc: u32 = 0x109a8b9;
 loop { match pc {
 0x109a8b9 => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8bc); }
 let mut pc: u32 = 0x109a8bc;
 loop { match pc {
 0x109a8bc => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8bf); }
 let mut pc: u32 = 0x109a8bf;
 loop { match pc {
 0x109a8bf => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8c2); }
 let mut pc: u32 = 0x109a8c2;
 loop { match pc {
 0x109a8c2 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8c5); }
 let mut pc: u32 = 0x109a8c5;
 loop { match pc {
 0x109a8c5 => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8c8); }
 let mut pc: u32 = 0x109a8c8;
 loop { match pc {
 0x109a8c8 => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8cb); }
 let mut pc: u32 = 0x109a8cb;
 loop { match pc {
 0x109a8cb => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8ce); }
 let mut pc: u32 = 0x109a8ce;
 loop { match pc {
 0x109a8ce => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8d1); }
 let mut pc: u32 = 0x109a8d1;
 loop { match pc {
 0x109a8d1 => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8d4); }
 let mut pc: u32 = 0x109a8d4;
 loop { match pc {
 0x109a8d4 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8d7); }
 let mut pc: u32 = 0x109a8d7;
 loop { match pc {
 0x109a8d7 => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8da); }
 let mut pc: u32 = 0x109a8da;
 loop { match pc {
 0x109a8da => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8dd); }
 let mut pc: u32 = 0x109a8dd;
 loop { match pc {
 0x109a8dd => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8e0); }
 let mut pc: u32 = 0x109a8e0;
 loop { match pc {
 0x109a8e0 => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8e3); }
 let mut pc: u32 = 0x109a8e3;
 loop { match pc {
 0x109a8e3 => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8e6); }
 let mut pc: u32 = 0x109a8e6;
 loop { match pc {
 0x109a8e6 => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8e9); }
 let mut pc: u32 = 0x109a8e9;
 loop { match pc {
 0x109a8e9 => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8ec); }
 let mut pc: u32 = 0x109a8ec;
 loop { match pc {
 0x109a8ec => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8ef); }
 let mut pc: u32 = 0x109a8ef;
 loop { match pc {
 0x109a8ef => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8f2); }
 let mut pc: u32 = 0x109a8f2;
 loop { match pc {
 0x109a8f2 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8f5); }
 let mut pc: u32 = 0x109a8f5;
 loop { match pc {
 0x109a8f5 => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8f8); }
 let mut pc: u32 = 0x109a8f8;
 loop { match pc {
 0x109a8f8 => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8fb); }
 let mut pc: u32 = 0x109a8fb;
 loop { match pc {
 0x109a8fb => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a8fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a8fe); }
 let mut pc: u32 = 0x109a8fe;
 loop { match pc {
 0x109a8fe => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a901(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a901); }
 let mut pc: u32 = 0x109a901;
 loop { match pc {
 0x109a901 => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a904(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a904); }
 let mut pc: u32 = 0x109a904;
 loop { match pc {
 0x109a904 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a907(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a907); }
 let mut pc: u32 = 0x109a907;
 loop { match pc {
 0x109a907 => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a90a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a90a); }
 let mut pc: u32 = 0x109a90a;
 loop { match pc {
 0x109a90a => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a90d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a90d); }
 let mut pc: u32 = 0x109a90d;
 loop { match pc {
 0x109a90d => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a910); }
 let mut pc: u32 = 0x109a910;
 loop { match pc {
 0x109a910 => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a913(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a913); }
 let mut pc: u32 = 0x109a913;
 loop { match pc {
 0x109a913 => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a916(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a916); }
 let mut pc: u32 = 0x109a916;
 loop { match pc {
 0x109a916 => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a919(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a919); }
 let mut pc: u32 = 0x109a919;
 loop { match pc {
 0x109a919 => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a91c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a91c); }
 let mut pc: u32 = 0x109a91c;
 loop { match pc {
 0x109a91c => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a91f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a91f); }
 let mut pc: u32 = 0x109a91f;
 loop { match pc {
 0x109a91f => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a922(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a922); }
 let mut pc: u32 = 0x109a922;
 loop { match pc {
 0x109a922 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a925(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a925); }
 let mut pc: u32 = 0x109a925;
 loop { match pc {
 0x109a925 => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a928(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a928); }
 let mut pc: u32 = 0x109a928;
 loop { match pc {
 0x109a928 => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a92b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a92b); }
 let mut pc: u32 = 0x109a92b;
 loop { match pc {
 0x109a92b => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a92e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a92e); }
 let mut pc: u32 = 0x109a92e;
 loop { match pc {
 0x109a92e => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a931(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a931); }
 let mut pc: u32 = 0x109a931;
 loop { match pc {
 0x109a931 => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a934(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a934); }
 let mut pc: u32 = 0x109a934;
 loop { match pc {
 0x109a934 => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a937(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a937); }
 let mut pc: u32 = 0x109a937;
 loop { match pc {
 0x109a937 => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a93a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a93a); }
 let mut pc: u32 = 0x109a93a;
 loop { match pc {
 0x109a93a => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a93d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a93d); }
 let mut pc: u32 = 0x109a93d;
 loop { match pc {
 0x109a93d => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a940); }
 let mut pc: u32 = 0x109a940;
 loop { match pc {
 0x109a940 => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a943(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a943); }
 let mut pc: u32 = 0x109a943;
 loop { match pc {
 0x109a943 => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a946(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a946); }
 let mut pc: u32 = 0x109a946;
 loop { match pc {
 0x109a946 => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a949(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a949); }
 let mut pc: u32 = 0x109a949;
 loop { match pc {
 0x109a949 => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a94c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a94c); }
 let mut pc: u32 = 0x109a94c;
 loop { match pc {
 0x109a94c => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a94f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a94f); }
 let mut pc: u32 = 0x109a94f;
 loop { match pc {
 0x109a94f => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a952(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a952); }
 let mut pc: u32 = 0x109a952;
 loop { match pc {
 0x109a952 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a955(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a955); }
 let mut pc: u32 = 0x109a955;
 loop { match pc {
 0x109a955 => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a958(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a958); }
 let mut pc: u32 = 0x109a958;
 loop { match pc {
 0x109a958 => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a95b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a95b); }
 let mut pc: u32 = 0x109a95b;
 loop { match pc {
 0x109a95b => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a95e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a95e); }
 let mut pc: u32 = 0x109a95e;
 loop { match pc {
 0x109a95e => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a961(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a961); }
 let mut pc: u32 = 0x109a961;
 loop { match pc {
 0x109a961 => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a964(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a964); }
 let mut pc: u32 = 0x109a964;
 loop { match pc {
 0x109a964 => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a967(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a967); }
 let mut pc: u32 = 0x109a967;
 loop { match pc {
 0x109a967 => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a96a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a96a); }
 let mut pc: u32 = 0x109a96a;
 loop { match pc {
 0x109a96a => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a96d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a96d); }
 let mut pc: u32 = 0x109a96d;
 loop { match pc {
 0x109a96d => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a970); }
 let mut pc: u32 = 0x109a970;
 loop { match pc {
 0x109a970 => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a973(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a973); }
 let mut pc: u32 = 0x109a973;
 loop { match pc {
 0x109a973 => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a976(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a976); }
 let mut pc: u32 = 0x109a976;
 loop { match pc {
 0x109a976 => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a979(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a979); }
 let mut pc: u32 = 0x109a979;
 loop { match pc {
 0x109a979 => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a97c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a97c); }
 let mut pc: u32 = 0x109a97c;
 loop { match pc {
 0x109a97c => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a97f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a97f); }
 let mut pc: u32 = 0x109a97f;
 loop { match pc {
 0x109a97f => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a982(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a982); }
 let mut pc: u32 = 0x109a982;
 loop { match pc {
 0x109a982 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a985(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a985); }
 let mut pc: u32 = 0x109a985;
 loop { match pc {
 0x109a985 => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a988(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a988); }
 let mut pc: u32 = 0x109a988;
 loop { match pc {
 0x109a988 => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a98b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a98b); }
 let mut pc: u32 = 0x109a98b;
 loop { match pc {
 0x109a98b => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a98e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a98e); }
 let mut pc: u32 = 0x109a98e;
 loop { match pc {
 0x109a98e => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a991(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a991); }
 let mut pc: u32 = 0x109a991;
 loop { match pc {
 0x109a991 => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a994(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a994); }
 let mut pc: u32 = 0x109a994;
 loop { match pc {
 0x109a994 => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a997(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a997); }
 let mut pc: u32 = 0x109a997;
 loop { match pc {
 0x109a997 => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a99a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a99a); }
 let mut pc: u32 = 0x109a99a;
 loop { match pc {
 0x109a99a => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a99d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a99d); }
 let mut pc: u32 = 0x109a99d;
 loop { match pc {
 0x109a99d => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a9a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a9a0); }
 let mut pc: u32 = 0x109a9a0;
 loop { match pc {
 0x109a9a0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1133e68u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x109a9b5); f_0109a22e(cpu); if cpu.returned_to != 0x109a9b5 { return; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x109a9c1; continue; }
  pc = 0x109a9b9; continue;
 }
 0x109a9b9 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x109a9c3; continue;
 }
 0x109a9c1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109a9c3; continue;
 }
 0x109a9c3 => {
  { let v: u32 = cpu.r32(0x1133e68u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133e6cu32, v); }
  cpu.push(0x109a9d5); f_010a6cae(cpu); if cpu.returned_to != 0x109a9d5 { return; }
  { let v: u32 = cpu.r32(0x11ef5a0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11ef59cu32); cpu.eax = v; }
  cpu.push(0x109a9e5); f_0100001f(cpu); if cpu.returned_to != 0x109a9e5 { return; }
  cpu.push(0x109a9ea); f_0108dd4b(cpu); if cpu.returned_to != 0x109a9ea { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a9f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a9f0); }
 let mut pc: u32 = 0x109a9f0;
 loop { match pc {
 0x109a9f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x109a9f7; continue;
 }
 0x109a9f7 => {
  { let v: u32 = 0x1139344u32; cpu.esi = v; }
  { let v: u32 = 0x1139380u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x109aa07; continue;
 }
 0x109aa07 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x109aa1f; continue; }
  pc = 0x109aa0b; continue;
 }
 0x109aa0b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x109aa1a; continue; }
  pc = 0x109aa10; continue;
 }
 0x109aa10 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x109aa1a; continue; }
  pc = 0x109aa15; continue;
 }
 0x109aa15 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x109aa1a; continue;
 }
 0x109aa1a => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x109aa07; continue;
 }
 0x109aa1f => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x109aa35; continue; }
  pc = 0x109aa23; continue;
 }
 0x109aa23 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x109aa30; continue; }
  pc = 0x109aa2a; continue;
 }
 0x109aa2a => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.flat_ds(0x109aa2d); cpu.push(0x109aa2f); cpu.call(t); if cpu.returned_to != 0x109aa2f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x109aa30; continue;
 }
 0x109aa30 => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x109a9f7; continue;
 }
 0x109aa35 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109aa3a); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109aa3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109aa3b); }
 let mut pc: u32 = 0x109aa3b;
 loop { match pc {
 0x109aa3b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x109aa43; continue;
 }
 0x109aa43 => {
  { let v: u32 = 0x1139380u32; cpu.esi = v; }
  { let v: u32 = 0x1139392u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x109aa53; continue;
 }
 0x109aa53 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x109aa6b; continue; }
  pc = 0x109aa57; continue;
 }
 0x109aa57 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x109aa66; continue; }
  pc = 0x109aa5c; continue;
 }
 0x109aa5c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x109aa66; continue; }
  pc = 0x109aa61; continue;
 }
 0x109aa61 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x109aa66; continue;
 }
 0x109aa66 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x109aa53; continue;
 }
 0x109aa6b => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x109aa85; continue; }
  pc = 0x109aa6f; continue;
 }
 0x109aa6f => {
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x109aa80; continue; }
  pc = 0x109aa73; continue;
 }
 0x109aa73 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x109aa80; continue; }
  pc = 0x109aa7a; continue;
 }
 0x109aa7a => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.flat_ds(0x109aa7d); cpu.push(0x109aa7f); cpu.call(t); if cpu.returned_to != 0x109aa7f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x109aa80; continue;
 }
 0x109aa80 => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x109aa43; continue;
 }
 0x109aa85 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109aa89); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109aa8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109aa8f); }
 let mut pc: u32 = 0x109aa8f;
 loop { match pc {
 0x109aa8f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x109aa9c); f_010a6d17(cpu); if cpu.returned_to != 0x109aa9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109aaa9; continue; }
  pc = 0x109aaa0; continue;
 }
 0x109aaa0 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x109aaa9); f_0108ee41(cpu); if cpu.returned_to != 0x109aaa9 { return; }
  pc = 0x109aaa9; continue;
 }
 0x109aaa9 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109aaac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109aaac); }
 let mut pc: u32 = 0x109aaac;
 loop { match pc {
 0x109aaac => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x76u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x6cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.unimplemented(0x109aab4, "Fnsave");
  { let v = cpu.rf80(0x1cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x6cu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x109aac4); f_010a6da0(cpu); if cpu.returned_to != 0x109aac4 { return; }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = cpu.st(0); cpu.wf80(0x1cu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u16 = (cpu.eax as u16) & 0xffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.unimplemented(0x109aad8, "Frstor");
  { let v: u32 = cpu.add32(cpu.esp, 0x76u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109aae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109aae0); }
 let mut pc: u32 = 0x109aae0;
 loop { match pc {
 0x109aae0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = (cpu.segments[0] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = (cpu.segments[2] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = (cpu.segments[5] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ab08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ab08); }
 let mut pc: u32 = 0x109ab08;
 loop { match pc {
 0x109ab08 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133e90u32); cpu.ecx = v; }
  cpu.push(0x109ab14); f_010a6fa0(cpu); if cpu.returned_to != 0x109ab14 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ab16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ab16); }
 let mut pc: u32 = 0x109ab16;
 loop { match pc {
 0x109ab16 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x10a70f0u32; cpu.edx = v; }
  { let v: u32 = 0x10a7239u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1134db8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1134dbcu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ab62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ab62); }
 let mut pc: u32 = 0x109ab62;
 loop { match pc {
 0x109ab62 => {
  { let v: u8 = 0x3u8; cpu.w8(0x1133c14u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1133fc0u32.wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x109ab70, "Smsw");
  { let v: u16 = cpu.r16(0x1133fbeu32.wrapping_add(cpu.bases[3])) & 0x6u16; cpu.w16(0x1133fbeu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x109ab83, "Sidt");
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x1133e80u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x109abb4; continue; }
  pc = 0x109ab9a; continue;
 }
 0x109ab9a => {
  cpu.flat_ds(0x109ab9a); cpu.push(0x109ab9f); f_0109ac2d(cpu); if cpu.returned_to != 0x109ab9f { return; }
  { let v = cpu.r16(0x1133bfeu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  cpu.flat_ds(0x109abad); cpu.push(0x109abb2); f_0109ac97(cpu); if cpu.returned_to != 0x109abb2 { return; }
  pc = 0x109ac2c; continue;
 }
 0x109abb4 => {
  { cpu.sub8(cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x109abd8; continue; }
  pc = 0x109abbd; continue;
 }
 0x109abbd => {
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x10a7630u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x109ac2c; continue;
 }
 0x109abd8 => {
  { cpu.sub8(cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x109ac00; continue; }
  pc = 0x109abe1; continue;
 }
 0x109abe1 => {
  cpu.flat_ds(0x109abe1); cpu.push(0x109abe6); f_0109ac2d(cpu); if cpu.returned_to != 0x109abe6 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xe02u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1133bf8u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x109abfb); cpu.push(0x109abfe); cpu.call(t); if cpu.returned_to != 0x109abfe { return; }
  pc = 0x109ac2c; continue;
 }
 0x109ac00 => {
  { cpu.sub8(cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x109ac19; continue; }
  pc = 0x109ac09; continue;
 }
 0x109ac09 => {
  { let v: u16 = cpu.r16(0x1133bfcu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x109ac12); cpu.push(0x109ac17); f_010aa46e(cpu); if cpu.returned_to != 0x109ac17 { return; }
  pc = 0x109ac2c; continue;
 }
 0x109ac19 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x109ac1e); cpu.push(0x109ac23); f_010aa46e(cpu); if cpu.returned_to != 0x109ac23 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x109ac2c; continue; }
  pc = 0x109ac27; continue;
 }
 0x109ac27 => {
  cpu.flat_ds(0x109ac27); cpu.push(0x109ac2c); f_0109ac4a(cpu); if cpu.returned_to != 0x109ac2c { return; }
  pc = 0x109ac2c; continue;
 }
 0x109ac2c => {
  cpu.flat_ds(0x109ac2c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ac2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ac2d); }
 let mut pc: u32 = 0x109ac2d;
 loop { match pc {
 0x109ac2d => {
  { let v: u32 = 0x10a7630u32; cpu.ecx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 13u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), 0x8eu8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ac4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ac4a); }
 let mut pc: u32 = 0x109ac4a;
 loop { match pc {
 0x109ac4a => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133fb8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x1133fbcu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  if (cpu.sf != cpu.of) { pc = 0x109ac81; continue; }
  pc = 0x109ac71; continue;
 }
 0x109ac71 => {
  { let v: u32 = 0x10a75c0u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x109ac7a); cpu.push(0x109ac7f); f_0109ac97(cpu); if cpu.returned_to != 0x109ac7f { return; }
  pc = 0x109ac96; continue;
 }
 0x109ac81 => {
  { let v: u32 = 0x10a7630u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffffdu32; cpu.ecx = v; }
  {  }
  pc = 0x109ac96; continue;
 }
 0x109ac96 => {
  cpu.flat_ds(0x109ac96); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ac97(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ac97); }
 let mut pc: u32 = 0x109ac97;
 loop { match pc {
 0x109ac97 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x1133f78u32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.r32(0x1133f78u32) | 0x4u32; cpu.w32(0x1133f78u32, v); }
  { let v: u32 = cpu.and32(cpu.r32(0x1133f78u32), 0xfffffffdu32); cpu.w32(0x1133f78u32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ad31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ad31); }
 let mut pc: u32 = 0x109ad31;
 loop { match pc {
 0x109ad31 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133fb8u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x1133fbcu32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.sub8(cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x109ad58; continue; }
  pc = 0x109ad51; continue;
 }
 0x109ad51 => {
  cpu.flat_ds(0x109ad51); cpu.push(0x109ad56); f_0109ad6c(cpu); if cpu.returned_to != 0x109ad56 { return; }
  pc = 0x109ad6b; continue;
 }
 0x109ad58 => {
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff9u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1133fbeu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) | (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  {  }
  pc = 0x109ad6b; continue;
 }
 0x109ad6b => {
  cpu.flat_ds(0x109ad6b); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ad6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ad6c); }
 let mut pc: u32 = 0x109ad6c;
 loop { match pc {
 0x109ad6c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x1133f78u32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1133fbeu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1133f78u32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ad8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ad8d); }
 let mut pc: u32 = 0x109ad8d;
 loop { match pc {
 0x109ad8d => {
  {  }
  cpu.unimplemented(0x109ad8e, "Fnsave");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ad92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ad92); }
 let mut pc: u32 = 0x109ad92;
 loop { match pc {
 0x109ad92 => {
  cpu.unimplemented(0x109ad92, "Frstor");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ad96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ad96); }
 let mut pc: u32 = 0x109ad96;
 loop { match pc {
 0x109ad96 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x1133c15u32), 0x0u8); }
  if cpu.zf { pc = 0x109adb7; continue; }
  pc = 0x109ada1; continue;
 }
 0x109ada1 => {
  { let v: u32 = 0x109ad8du32; cpu.edx = v; }
  { let v: u32 = 0x109ad92u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1134e44u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1134e48u32, v); }
  pc = 0x109adb7; continue;
 }
 0x109adb7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1134ed0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x109adc4); f_010aa59c(cpu); if cpu.returned_to != 0x109adc4 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109adc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109adc7); }
 let mut pc: u32 = 0x109adc7;
 loop { match pc {
 0x109adc7 => {
  { cpu.sub8(cpu.r8(0x1133c15u32), 0x0u8); }
  if !cpu.zf { f_0109ad96(cpu); return; }
  pc = 0x109add0; continue;
 }
 0x109add0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109add1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109add1); }
 let mut pc: u32 = 0x109add1;
 loop { match pc {
 0x109add1 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x1133c14u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x109ae19; continue; }
  pc = 0x109addc; continue;
 }
 0x109addc => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1133c15u32, v); }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { let v = cpu.fpu_control; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x109ae04; continue; }
  pc = 0x109adfb; continue;
 }
 0x109adfb => {
  cpu.push(0x109ae00); f_0109ad96(cpu); if cpu.returned_to != 0x109ae00 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x109ae04; continue;
 }
 0x109ae04 => {
  { cpu.sub8(cpu.r8(0x1133e7cu32), 0x0u8); }
  if !cpu.zf { pc = 0x109ae19; continue; }
  pc = 0x109ae0d; continue;
 }
 0x109ae0d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1133c14u32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x1133c15u32, v); }
  pc = 0x109ae19; continue;
 }
 0x109ae19 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ae20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ae20); }
 let mut pc: u32 = 0x109ae20;
 loop { match pc {
 0x109ae20 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x109ae50; continue; }
  pc = 0x109ae24; continue;
 }
 0x109ae24 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  pc = 0x109ae26; continue;
 }
 0x109ae26 => {
  { cpu.and8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x109ae33; continue; }
  pc = 0x109ae2a; continue;
 }
 0x109ae2a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x109ae26; continue; }
  pc = 0x109ae33; continue;
 }
 0x109ae33 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  cpu.push(0x109ae3c); f_0109ae57(cpu); if cpu.returned_to != 0x109ae3c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x109ae50; continue; }
  pc = 0x109ae42; continue;
 }
 0x109ae42 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x109ae50; continue; }
  pc = 0x109ae47; continue;
 }
 0x109ae47 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x109ae50; continue; }
  pc = 0x109ae4d; continue;
 }
 0x109ae4d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x109ae50; continue;
 }
 0x109ae50 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ae57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ae57); }
 let mut pc: u32 = 0x109ae57;
 loop { match pc {
 0x109ae57 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x109aec2; continue; }
  pc = 0x109ae5b; continue;
 }
 0x109ae5b => {
  { cpu.and8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x109ae67; continue; }
  pc = 0x109ae5f; continue;
 }
 0x109ae5f => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x109ae5b; continue; }
  pc = 0x109ae67; continue;
 }
 0x109ae67 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x109aea7; continue; }
  pc = 0x109ae6d; continue;
 }
 0x109ae6d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x109ae99; continue; }
  pc = 0x109ae70; continue;
 }
 0x109ae70 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x109ae96; continue; }
  pc = 0x109ae7e; continue;
 }
 0x109ae7e => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x109ae70; continue; }
  pc = 0x109ae93; continue;
 }
 0x109ae93 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x109ae96; continue;
 }
 0x109ae96 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x109ae99; continue;
 }
 0x109ae99 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x109aea7; continue;
 }
 0x109aea7 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x109aec2; continue; }
  pc = 0x109aead; continue;
 }
 0x109aead => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x109aec2; continue; }
  pc = 0x109aeb5; continue;
 }
 0x109aeb5 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x109aec2; continue; }
  pc = 0x109aebd; continue;
 }
 0x109aebd => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x109aec2; continue;
 }
 0x109aec2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109aec3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109aec3); }
 let mut pc: u32 = 0x109aec3;
 loop { match pc {
 0x109aec3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x109af4f; continue; }
  pc = 0x109aed9; continue;
 }
 0x109aed9 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xefu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ecx), v); }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x109af87; continue; }
  pc = 0x109aeed; continue;
 }
 0x109aeed => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x109af87; continue; }
  pc = 0x109aef8; continue;
 }
 0x109aef8 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  pc = 0x109aefd; continue;
 }
 0x109aefd => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x109af87; continue; }
  pc = 0x109af05; continue;
 }
 0x109af05 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x109af87; continue; }
  pc = 0x109af0d; continue;
 }
 0x109af0d => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x109af19); f_010a4c30(cpu); if cpu.returned_to != 0x109af19 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x109af2d; continue; }
  pc = 0x109af20; continue;
 }
 0x109af20 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x109af49; continue;
 }
 0x109af2d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109af49; continue; }
  pc = 0x109af31; continue;
 }
 0x109af31 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x109af3b); f_01099923(cpu); if cpu.returned_to != 0x109af3b { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x20u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x109af49; continue;
 }
 0x109af49 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  cpu.poll(); pc = 0x109aefd; continue;
 }
 0x109af4f => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x109af87; continue; }
  pc = 0x109af55; continue;
 }
 0x109af55 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)) & 0xefu8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x20u8); }
  if !cpu.zf { pc = 0x109af87; continue; }
  pc = 0x109af5f; continue;
 }
 0x109af5f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109af77; continue; }
  pc = 0x109af66; continue;
 }
 0x109af66 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x109af77); f_01094bff(cpu); if cpu.returned_to != 0x109af77 { return; }
  pc = 0x109af77; continue;
 }
 0x109af77 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x109af87; continue; }
  pc = 0x109af7c; continue;
 }
 0x109af7c => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x20u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x109af87; continue;
 }
 0x109af87 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
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
pub fn f_0109af9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109af9c); }
 let mut pc: u32 = 0x109af9c;
 loop { match pc {
 0x109af9c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x109afaa); f_01094bff(cpu); if cpu.returned_to != 0x109afaa { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109afad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109afad); }
 let mut pc: u32 = 0x109afad;
 loop { match pc {
 0x109afad => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { f_0109aec3(cpu); return; }
  pc = 0x109afb5; continue;
 }
 0x109afb5 => {
  cpu.push(0x109afba); f_010aa5c3(cpu); if cpu.returned_to != 0x109afba { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109afbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109afbd); }
 let mut pc: u32 = 0x109afbd;
 loop { match pc {
 0x109afbd => {
  { let v: u32 = cpu.r32(0x1133e5cu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109afc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109afc3); }
 let mut pc: u32 = 0x109afc3;
 loop { match pc {
 0x109afc3 => {
  f_01098c63(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109afc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109afc8); }
 let mut pc: u32 = 0x109afc8;
 loop { match pc {
 0x109afc8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x109afd1); f_0109a1fd(cpu); if cpu.returned_to != 0x109afd1 { return; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x109affd; continue; }
  pc = 0x109afd7; continue;
 }
 0x109afd7 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x109afe8; continue; }
  pc = 0x109afdf; continue;
 }
 0x109afdf => {
  { let v: u32 = 0x86u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x109affd; continue;
 }
 0x109afe8 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x109aff6; continue; }
  pc = 0x109afed; continue;
 }
 0x109afed => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x109affd; continue;
 }
 0x109aff6 => {
  { let v: u32 = 0x1000u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x109affd; continue;
 }
 0x109affd => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x109b005); f_01094a9a(cpu); if cpu.returned_to != 0x109b005 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109b02c; continue; }
  pc = 0x109b00c; continue;
 }
 0x109b00c => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8) & 0xf8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x109b030; continue;
 }
 0x109b02c => {
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x8u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x109b030; continue;
 }
 0x109b030 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b03f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b03f); }
 let mut pc: u32 = 0x109b03f;
 loop { match pc {
 0x109b03f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x109b05c; continue; }
  pc = 0x109b052; continue;
 }
 0x109b052 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x109b05c); f_0109995d(cpu); if cpu.returned_to != 0x109b05c { return; }
  pc = 0x109b05c; continue;
 }
 0x109b05c => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b05e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b05e); }
 let mut pc: u32 = 0x109b05e;
 loop { match pc {
 0x109b05e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x109b084; continue; }
  pc = 0x109b06a; continue;
 }
 0x109b06a => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x109b074); f_01099923(cpu); if cpu.returned_to != 0x109b074 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x109b0a4; continue;
 }
 0x109b084 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x109b098; continue; }
  pc = 0x109b08f; continue;
 }
 0x109b08f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x109b096); f_0109b0e7(cpu); if cpu.returned_to != 0x109b096 { return; }
  pc = 0x109b0a4; continue;
 }
 0x109b098 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x109b0a4; continue;
 }
 0x109b0a4 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x40u8); }
  if !cpu.zf { pc = 0x109b0e2; continue; }
  pc = 0x109b0aa; continue;
 }
 0x109b0aa => {
  { cpu.sub32(cpu.eax, 0xdu32); }
  if !cpu.zf { pc = 0x109b0cf; continue; }
  pc = 0x109b0af; continue;
 }
 0x109b0af => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x109b0c3; continue; }
  pc = 0x109b0ba; continue;
 }
 0x109b0ba => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x109b0c1); f_0109b0e7(cpu); if cpu.returned_to != 0x109b0c1 { return; }
  pc = 0x109b0cf; continue;
 }
 0x109b0c3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x109b0cf; continue;
 }
 0x109b0cf => {
  { cpu.sub32(cpu.eax, 0x1au32); }
  if !cpu.zf { pc = 0x109b0e2; continue; }
  pc = 0x109b0d4; continue;
 }
 0x109b0d4 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x10u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x109b0e2; continue;
 }
 0x109b0e2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b0e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b0e7); }
 let mut pc: u32 = 0x109b0e7;
 loop { match pc {
 0x109b0e7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x109b0f1); f_0109b116(cpu); if cpu.returned_to != 0x109b0f1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109b0fe; continue; }
  pc = 0x109b0f5; continue;
 }
 0x109b0f5 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109b0fe => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b116(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b116); }
 let mut pc: u32 = 0x109b116;
 loop { match pc {
 0x109b116 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x109b127; continue; }
  pc = 0x109b122; continue;
 }
 0x109b122 => {
  cpu.push(0x109b127); f_0109afc8(cpu); if cpu.returned_to != 0x109b127 { return; }
  pc = 0x109b127; continue;
 }
 0x109b127 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x109b13e; continue; }
  pc = 0x109b12f; continue;
 }
 0x109b12f => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x6u8); }
  if cpu.zf { pc = 0x109b13e; continue; }
  pc = 0x109b134; continue;
 }
 0x109b134 => {
  { let v: u32 = 0x2000u32; cpu.eax = v; }
  cpu.push(0x109b13e); f_010aa5c8(cpu); if cpu.returned_to != 0x109b13e { return; }
  pc = 0x109b13e; continue;
 }
 0x109b13e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xfbu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x2400u32; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.eax, 0x2400u32); }
  if !cpu.zf { pc = 0x109b17e; continue; }
  pc = 0x109b15b; continue;
 }
 0x109b15b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x109b17e; continue; }
  pc = 0x109b162; continue;
 }
 0x109b162 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x109b16a); f_010aa5f7(cpu); if cpu.returned_to != 0x109b16a { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109b19b; continue; }
  pc = 0x109b171; continue;
 }
 0x109b171 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109b19b; continue;
 }
 0x109b17e => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x4u8); }
  if cpu.zf { pc = 0x109b18b; continue; }
  pc = 0x109b184; continue;
 }
 0x109b184 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x109b18e; continue;
 }
 0x109b18b => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  pc = 0x109b18e; continue;
 }
 0x109b18e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x109b198); f_0109b03f(cpu); if cpu.returned_to != 0x109b198 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109b19b; continue;
 }
 0x109b19b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109b1ba; continue; }
  pc = 0x109b1a2; continue;
 }
 0x109b1a2 => {
  if !cpu.zf { pc = 0x109b1aa; continue; }
  pc = 0x109b1a4; continue;
 }
 0x109b1a4 => {
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x10u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x109b1ba; continue;
 }
 0x109b1aa => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.or8(((cpu.ebx >> 8) as u8), 0x20u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x109b1ba; continue;
 }
 0x109b1ba => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b1c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b1c2); }
 let mut pc: u32 = 0x109b1c2;
 loop { match pc {
 0x109b1c2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1133fc4u32), 0x0u32); }
  if cpu.zf { pc = 0x109b1f6; continue; }
  pc = 0x109b1d4; continue;
 }
 0x109b1d4 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x109b1d9; continue;
 }
 0x109b1d9 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x109b1e2); f_010aa614(cpu); if cpu.returned_to != 0x109b1e2 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x109b1f2; continue; }
  pc = 0x109b1eb; continue;
 }
 0x109b1eb => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x109b1f2; continue; }
  pc = 0x109b1ef; continue;
 }
 0x109b1ef => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x109b1d9; continue;
 }
 0x109b1f2 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x109b1f8; continue;
 }
 0x109b1f6 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x109b1f8; continue;
 }
 0x109b1f8 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b200); }
 let mut pc: u32 = 0x109b200;
 loop { match pc {
 0x109b200 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1133fc4u32), 0x0u32); }
  if cpu.zf { pc = 0x109b229; continue; }
  pc = 0x109b212; continue;
 }
 0x109b212 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x109b225; continue; }
  pc = 0x109b218; continue;
 }
 0x109b218 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  cpu.push(0x109b21f); f_010aa614(cpu); if cpu.returned_to != 0x109b21f { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x109b212; continue; }
  pc = 0x109b225; continue;
 }
 0x109b225 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x109b22b; continue;
 }
 0x109b229 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x109b22b; continue;
 }
 0x109b22b => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b231(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b231); }
 let mut pc: u32 = 0x109b231;
 loop { match pc {
 0x109b231 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1133fc4u32), 0x0u32); }
  if cpu.zf { pc = 0x109b246; continue; }
  pc = 0x109b23d; continue;
 }
 0x109b23d => {
  cpu.push(0x109b242); f_0109b200(cpu); if cpu.returned_to != 0x109b242 { return; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109b246 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b39d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b39d); }
 let mut pc: u32 = 0x109b39d;
 loop { match pc {
 0x109b39d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x1133fc4u32), 0x0u32); }
  if cpu.zf { pc = 0x109b47c; continue; }
  pc = 0x109b3ba; continue;
 }
 0x109b3ba => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x109b441; continue; }
  pc = 0x109b3c3; continue;
 }
 0x109b3c3 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x109b3c7; continue;
 }
 0x109b3c7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x109b3df; continue; }
  pc = 0x109b3cd; continue;
 }
 0x109b3cd => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x109b3dd; continue; }
  pc = 0x109b3d1; continue;
 }
 0x109b3d1 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x109b3df; continue; }
  pc = 0x109b3d8; continue;
 }
 0x109b3d8 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x109b3c7; continue; }
  pc = 0x109b3dd; continue;
 }
 0x109b3dd => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x109b3df; continue;
 }
 0x109b3df => {
  { let v: u32 = cpu.r32(0x1133fc4u32); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1133fc4u32); cpu.esi = v; }
  pc = 0x109b3f0; continue;
 }
 0x109b3f0 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x109b43a; continue; }
  pc = 0x109b3f5; continue;
 }
 0x109b3f5 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x109b41a; continue; }
  pc = 0x109b3f9; continue;
 }
 0x109b3f9 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x109b412; continue; }
  pc = 0x109b3fd; continue;
 }
 0x109b3fd => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1133fc9u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x109b414; continue;
 }
 0x109b412 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x109b414; continue;
 }
 0x109b414 => {
  if cpu.zf { pc = 0x109b41a; continue; }
  pc = 0x109b416; continue;
 }
 0x109b416 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x109b43a; continue;
 }
 0x109b41a => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x109b433; continue; }
  pc = 0x109b41e; continue;
 }
 0x109b41e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1133fc9u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  pc = 0x109b435; continue;
 }
 0x109b433 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109b435; continue;
 }
 0x109b435 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.poll(); pc = 0x109b3f0; continue;
 }
 0x109b43a => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x109b441; continue; }
  pc = 0x109b43e; continue;
 }
 0x109b43e => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ebp = v; }
  pc = 0x109b441; continue;
 }
 0x109b441 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x109b485; continue;
 }
 0x109b47c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x109b485); f_0108e32d(cpu); if cpu.returned_to != 0x109b485 { return; }
  pc = 0x109b485; continue;
 }
 0x109b485 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
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
pub fn f_0109b48f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b48f); }
 let mut pc: u32 = 0x109b48f;
 loop { match pc {
 0x109b48f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109b49e); f_0109b200(cpu); if cpu.returned_to != 0x109b49e { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x109b4a9); f_0109b4ae(cpu); if cpu.returned_to != 0x109b4a9 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b4ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b4ae); }
 let mut pc: u32 = 0x109b4ae;
 loop { match pc {
 0x109b396 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109b4ae => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x1133fc4u32), 0x0u32); }
  if cpu.zf { pc = 0x109b558; continue; }
  pc = 0x109b4c1; continue;
 }
 0x109b4c1 => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x1133fc4u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x109b4ec; continue; }
  pc = 0x109b4dd; continue;
 }
 0x109b4dd => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))) as u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x1133fc9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  pc = 0x109b4ec; continue;
 }
 0x109b4ec => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x109b4f5; continue; }
  pc = 0x109b4f0; continue;
 }
 0x109b4f0 => {
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x109b4f5; continue; }
  pc = 0x109b4f4; continue;
 }
 0x109b4f4 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x109b4f5; continue;
 }
 0x109b4f5 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x109b51d; continue; }
  pc = 0x109b4fc; continue;
 }
 0x109b4fc => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.poll(); pc = 0x109b396; continue;
 }
 0x109b51d => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x109b522; continue;
 }
 0x109b522 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x109b53a; continue; }
  pc = 0x109b52a; continue;
 }
 0x109b52a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x109b522; continue; }
  pc = 0x109b53a; continue;
 }
 0x109b53a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if cpu.zf { cpu.poll(); pc = 0x109b396; continue; }
  pc = 0x109b543; continue;
 }
 0x109b543 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.push(0x109b553); f_0108f030(cpu); if cpu.returned_to != 0x109b553 { return; }
  cpu.poll(); pc = 0x109b396; continue;
 }
 0x109b558 => {
  cpu.push(0x109b55d); f_01098c3e(cpu); if cpu.returned_to != 0x109b55d { return; }
  cpu.poll(); pc = 0x109b396; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b562(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b562); }
 let mut pc: u32 = 0x109b562;
 loop { match pc {
 0x109b562 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133fc4u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x109b580; continue; }
  pc = 0x109b56d; continue;
 }
 0x109b56d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1133fc9u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  pc = 0x109b580; continue;
 }
 0x109b580 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x109b597; continue; }
  pc = 0x109b584; continue;
 }
 0x109b584 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109b597 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b5a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b5a0); }
 let mut pc: u32 = 0x109b5a0;
 loop { match pc {
 0x109b5a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x109b5b6; continue; }
  pc = 0x109b5aa; continue;
 }
 0x109b5aa => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109b5b6 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b5ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b5ba); }
 let mut pc: u32 = 0x109b5ba;
 loop { match pc {
 0x109b5ba => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x109b5da; continue; }
  pc = 0x109b5c0; continue;
 }
 0x109b5c0 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x11ee154u32, v); }
  { let v: u8 = cpu.xor8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11ee155u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x11ee156u32, v); }
  pc = 0x109b5e7; continue;
 }
 0x109b5da => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11ee154u32, v); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x11ee155u32, v); }
  pc = 0x109b5e7; continue;
 }
 0x109b5e7 => {
  { let v: u32 = 0x11ee154u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b5ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b5ee); }
 let mut pc: u32 = 0x109b5ee;
 loop { match pc {
 0x109b5ee => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.push(0x109b5f5); f_0109b61c(cpu); if cpu.returned_to != 0x109b5f5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109b5fe; continue; }
  pc = 0x109b5f9; continue;
 }
 0x109b5f9 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x109b619; continue; }
  pc = 0x109b5fe; continue;
 }
 0x109b5fe => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u16); }
  if !cpu.zf { pc = 0x109b60f; continue; }
  pc = 0x109b607; continue;
 }
 0x109b607 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u16); }
  if cpu.zf { pc = 0x109b619; continue; }
  pc = 0x109b60f; continue;
 }
 0x109b60f => {
  { let v: u32 = 0x1u32; cpu.w32(0x1133fc4u32, v); }
  pc = 0x109b619; continue;
 }
 0x109b619 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b61c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b61c); }
 let mut pc: u32 = 0x109b61c;
 loop { match pc {
 0x109b61c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x109b6c2; continue; }
  pc = 0x109b632; continue;
 }
 0x109b632 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109b6c2; continue; }
  pc = 0x109b63b; continue;
 }
 0x109b63b => {
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  { let v: u32 = 0x5cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x109b64b); f_0108f030(cpu); if cpu.returned_to != 0x109b64b { return; }
  { let v: u32 = 0x28u32; cpu.ebx = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x2511u32; cpu.ecx = v; }
  cpu.push(0x109b660); f_0108f030(cpu); if cpu.returned_to != 0x109b660 { return; }
  { let v: u32 = 0x6300u32; cpu.ebx = v; }
  { let v: u32 = 0x21u32; cpu.edx = v; }
  { let v: u32 = 0x5cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x66u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x5cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x50u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x109b694); f_010aa79f(cpu); if cpu.returned_to != 0x109b694 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if cpu.zf { pc = 0x109b71e; continue; }
  pc = 0x109b6a6; continue;
 }
 0x109b6a6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  pc = 0x109b6b7; continue;
 }
 0x109b6b7 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11ef624u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x109b722; continue;
 }
 0x109b6c2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x109b71e; continue; }
  pc = 0x109b6ce; continue;
 }
 0x109b6ce => {
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  cpu.push(0x109b6de); f_0108f030(cpu); if cpu.returned_to != 0x109b6de { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.xor8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x6300u32; cpu.edx = v; }
  { let v: u8 = 0x21u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u16 = 0x300u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x109b71e; continue; }
  pc = 0x109b70a; continue;
 }
 0x109b70a => {
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x109b6b7; continue;
 }
 0x109b71e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109b722; continue;
 }
 0x109b722 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b729(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b729); }
 let mut pc: u32 = 0x109b729;
 loop { match pc {
 0x109b729 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x109b72e); cpu.call(t); if cpu.returned_to != 0x109b72e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b730); }
 let mut pc: u32 = 0x109b730;
 loop { match pc {
 0x109b730 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(0x109b736); cpu.call(t); if cpu.returned_to != 0x109b736 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109b738(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b738); }
 let mut pc: u32 = 0x109b738;
 loop { match pc {
 0x109b738 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfdu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x109b757; continue;
 }
 0x109b757 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffu32); cpu.ebx = v; }
  if cpu.zf { pc = 0x109b95f; continue; }
  pc = 0x109b76e; continue;
 }
 0x109b76e => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109b794; continue; }
  pc = 0x109b786; continue;
 }
 0x109b786 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109b78d); f_0109ba74(cpu); if cpu.returned_to != 0x109b78d { return; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x109b91f; continue;
 }
 0x109b794 => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x109b7be; continue; }
  pc = 0x109b799; continue;
 }
 0x109b799 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109b7a0); f_0109b729(cpu); if cpu.returned_to != 0x109b7a0 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x109b91e; continue; }
  pc = 0x109b7a8; continue;
 }
 0x109b7a8 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x109b95f; continue; }
  pc = 0x109b7b2; continue;
 }
 0x109b7b2 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x109b7b9); f_0109b730(cpu); if cpu.returned_to != 0x109b7b9 { return; }
  pc = 0x109b95f; continue;
 }
 0x109b7be => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x109b7c7); f_0109b97f(cpu); if cpu.returned_to != 0x109b7c7 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x109b7d6; continue; }
  pc = 0x109b7d2; continue;
 }
 0x109b7d2 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x109b7d6; continue;
 }
 0x109b7d6 => {
  { cpu.sub32(cpu.ebx, 0x65u32); }
  if cpu.cf { pc = 0x109b830; continue; }
  pc = 0x109b7db; continue;
 }
 0x109b7db => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if (cpu.cf || cpu.zf) { pc = 0x109b8bc; continue; }
  pc = 0x109b7e4; continue;
 }
 0x109b7e4 => {
  { cpu.sub32(cpu.ebx, 0x70u32); }
  if cpu.cf { pc = 0x109b816; continue; }
  pc = 0x109b7e9; continue;
 }
 0x109b7e9 => {
  if (cpu.cf || cpu.zf) { pc = 0x109b8b0; continue; }
  pc = 0x109b7ef; continue;
 }
 0x109b7ef => {
  { cpu.sub32(cpu.ebx, 0x75u32); }
  if cpu.cf { pc = 0x109b808; continue; }
  pc = 0x109b7f4; continue;
 }
 0x109b7f4 => {
  if (cpu.cf || cpu.zf) { pc = 0x109b8a3; continue; }
  pc = 0x109b7fa; continue;
 }
 0x109b7fa => {
  { cpu.sub32(cpu.ebx, 0x78u32); }
  if cpu.zf { pc = 0x109b8b0; continue; }
  pc = 0x109b803; continue;
 }
 0x109b803 => {
  pc = 0x109b91f; continue;
 }
 0x109b808 => {
  { cpu.sub32(cpu.ebx, 0x73u32); }
  if cpu.zf { pc = 0x109b8c7; continue; }
  pc = 0x109b811; continue;
 }
 0x109b811 => {
  pc = 0x109b91f; continue;
 }
 0x109b816 => {
  { cpu.sub32(cpu.ebx, 0x6eu32); }
  if cpu.cf { pc = 0x109b826; continue; }
  pc = 0x109b81b; continue;
 }
 0x109b81b => {
  if (cpu.cf || cpu.zf) { pc = 0x109b902; continue; }
  pc = 0x109b821; continue;
 }
 0x109b821 => {
  pc = 0x109b897; continue;
 }
 0x109b826 => {
  { cpu.sub32(cpu.ebx, 0x69u32); }
  if cpu.zf { pc = 0x109b88a; continue; }
  pc = 0x109b82b; continue;
 }
 0x109b82b => {
  pc = 0x109b91f; continue;
 }
 0x109b830 => {
  { cpu.sub32(cpu.ebx, 0x58u32); }
  if cpu.cf { pc = 0x109b856; continue; }
  pc = 0x109b835; continue;
 }
 0x109b835 => {
  if (cpu.cf || cpu.zf) { pc = 0x109b8b0; continue; }
  pc = 0x109b83b; continue;
 }
 0x109b83b => {
  { cpu.sub32(cpu.ebx, 0x63u32); }
  if cpu.cf { pc = 0x109b848; continue; }
  pc = 0x109b840; continue;
 }
 0x109b840 => {
  if (cpu.cf || cpu.zf) { pc = 0x109b8df; continue; }
  pc = 0x109b846; continue;
 }
 0x109b846 => {
  pc = 0x109b875; continue;
 }
 0x109b848 => {
  { cpu.sub32(cpu.ebx, 0x5bu32); }
  if cpu.zf { pc = 0x109b8d2; continue; }
  pc = 0x109b851; continue;
 }
 0x109b851 => {
  pc = 0x109b91f; continue;
 }
 0x109b856 => {
  { cpu.sub32(cpu.ebx, 0x45u32); }
  if cpu.cf { pc = 0x109b867; continue; }
  pc = 0x109b85b; continue;
 }
 0x109b85b => {
  if (cpu.cf || cpu.zf) { pc = 0x109b8bc; continue; }
  pc = 0x109b85d; continue;
 }
 0x109b85d => {
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x109b8bc; continue; }
  pc = 0x109b862; continue;
 }
 0x109b862 => {
  pc = 0x109b91f; continue;
 }
 0x109b867 => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x109b90f; continue; }
  pc = 0x109b870; continue;
 }
 0x109b870 => {
  pc = 0x109b91f; continue;
 }
 0x109b875 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  pc = 0x109b87f; continue;
 }
 0x109b87f => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x109b883; continue;
 }
 0x109b883 => {
  cpu.push(0x109b888); f_0109c11d(cpu); if cpu.returned_to != 0x109b888 { return; }
  pc = 0x109b8e8; continue;
 }
 0x109b88a => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.poll(); pc = 0x109b883; continue;
 }
 0x109b897 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x109b87f; continue;
 }
 0x109b8a3 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x109b883; continue;
 }
 0x109b8b0 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x109b87f; continue;
 }
 0x109b8bc => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109b8c5); f_0109be06(cpu); if cpu.returned_to != 0x109b8c5 { return; }
  pc = 0x109b8e8; continue;
 }
 0x109b8c7 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109b8d0); f_0109bb48(cpu); if cpu.returned_to != 0x109b8d0 { return; }
  pc = 0x109b8e8; continue;
 }
 0x109b8d2 => {
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109b8dd); f_0109bd0c(cpu); if cpu.returned_to != 0x109b8dd { return; }
  pc = 0x109b8e8; continue;
 }
 0x109b8df => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109b8e8); f_0109bab3(cpu); if cpu.returned_to != 0x109b8e8 { return; }
  pc = 0x109b8e8; continue;
 }
 0x109b8e8 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109b95f; continue; }
  pc = 0x109b8f2; continue;
 }
 0x109b8f2 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x109b91f; continue; }
  pc = 0x109b8fc; continue;
 }
 0x109b8fc => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x109b91f; continue;
 }
 0x109b902 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109b90d); f_0109bc6b(cpu); if cpu.returned_to != 0x109b90d { return; }
  pc = 0x109b91f; continue;
 }
 0x109b90f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109b916); f_0109b729(cpu); if cpu.returned_to != 0x109b916 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x109b7a8; continue; }
  pc = 0x109b91e; continue;
 }
 0x109b91e => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x109b91f; continue;
 }
 0x109b91f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x109b757; continue; }
  pc = 0x109b929; continue;
 }
 0x109b929 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x25u32); }
  if !cpu.zf { pc = 0x109b95f; continue; }
  pc = 0x109b938; continue;
 }
 0x109b938 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x109b945); f_0109b97f(cpu); if cpu.returned_to != 0x109b945 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x109b95f; continue; }
  pc = 0x109b954; continue;
 }
 0x109b954 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109b95f); f_0109bc6b(cpu); if cpu.returned_to != 0x109b95f { return; }
  pc = 0x109b95f; continue;
 }
 0x109b95f => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x109b973; continue; }
  pc = 0x109b966; continue;
 }
 0x109b966 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x109b973; continue; }
  pc = 0x109b96c; continue;
 }
 0x109b96c => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x109b977; continue;
 }
 0x109b973 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x109b977; continue;
 }
 0x109b977 => {
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
pub fn f_0109b97f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109b97f); }
 let mut pc: u32 = 0x109b97f;
 loop { match pc {
 0x109b97f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x109b9b2; continue; }
  pc = 0x109b9a8; continue;
 }
 0x109b9a8 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x109b9b2; continue;
 }
 0x109b9b2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x109ba04; continue; }
  pc = 0x109b9d5; continue;
 }
 0x109b9d5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109b9d7; continue;
 }
 0x109b9d7 => {
  { let v: u32 = cpu.imul2_32(cpu.edx, 0xau32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x30u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x20u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x109b9d7; continue; }
  pc = 0x109ba01; continue;
 }
 0x109ba01 => {
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x109ba04; continue;
 }
 0x109ba04 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x4eu32); }
  if !cpu.zf { pc = 0x109ba18; continue; }
  pc = 0x109ba10; continue;
 }
 0x109ba10 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)) | 0x8u8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x109ba28; continue;
 }
 0x109ba18 => {
  { cpu.sub32(cpu.edx, 0x46u32); }
  if !cpu.zf { pc = 0x109ba28; continue; }
  pc = 0x109ba1d; continue;
 }
 0x109ba1d => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x109ba28; continue;
 }
 0x109ba28 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.cf { pc = 0x109ba42; continue; }
  pc = 0x109ba32; continue;
 }
 0x109ba32 => {
  if (cpu.cf || cpu.zf) { pc = 0x109ba49; continue; }
  pc = 0x109ba34; continue;
 }
 0x109ba34 => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x109ba6d; continue; }
  pc = 0x109ba39; continue;
 }
 0x109ba39 => {
  if (cpu.cf || cpu.zf) { pc = 0x109ba56; continue; }
  pc = 0x109ba3b; continue;
 }
 0x109ba3b => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x109ba56; continue; }
  pc = 0x109ba40; continue;
 }
 0x109ba40 => {
  pc = 0x109ba6d; continue;
 }
 0x109ba42 => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x109ba62; continue; }
  pc = 0x109ba47; continue;
 }
 0x109ba47 => {
  pc = 0x109ba6d; continue;
 }
 0x109ba49 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x10u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x109ba6d; continue;
 }
 0x109ba56 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x109ba6d; continue;
 }
 0x109ba62 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x109ba6d; continue;
 }
 0x109ba6d => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ba74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ba74); }
 let mut pc: u32 = 0x109ba74;
 loop { match pc {
 0x109ba74 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x109ba7b; continue;
 }
 0x109ba7b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x109ba82); f_0109b729(cpu); if cpu.returned_to != 0x109ba82 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x109baa0; continue; }
  pc = 0x109ba9d; continue;
 }
 0x109ba9d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x109ba7b; continue;
 }
 0x109baa0 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if !cpu.zf { pc = 0x109baad; continue; }
  pc = 0x109baa6; continue;
 }
 0x109baa6 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x109baad); f_0109b730(cpu); if cpu.returned_to != 0x109baad { return; }
  pc = 0x109baad; continue;
 }
 0x109baad => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109bab3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109bab3); } r_0109bab3(cpu, 0x109bab3); }
pub fn f_0109bb48(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109bb48); } r_0109bab3(cpu, 0x109bb48); }
pub fn f_0109be06(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109be06); } r_0109bab3(cpu, 0x109be06); }
fn r_0109bab3(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x109bab3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x109bafa; continue; }
  pc = 0x109bac3; continue;
 }
 0x109bac3 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x109bad7; continue; }
  pc = 0x109bacb; continue;
 }
 0x109bacb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x109bafa; continue;
 }
 0x109bad7 => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x109baec; continue; }
  pc = 0x109badc; continue;
 }
 0x109badc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x109bafa; continue;
 }
 0x109baec => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x109bafa; continue;
 }
 0x109bafa => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x109bb09; continue; }
  pc = 0x109bb04; continue;
 }
 0x109bb04 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x109bb09; continue;
 }
 0x109bb09 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109bb3d; continue; }
  pc = 0x109bb0d; continue;
 }
 0x109bb0d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x109bb14); f_0109b729(cpu); if cpu.returned_to != 0x109bb14 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x109bb3d; continue; }
  pc = 0x109bb1e; continue;
 }
 0x109bb1e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x109bb09; continue; }
  pc = 0x109bb24; continue;
 }
 0x109bb24 => {
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x109bb34; continue; }
  pc = 0x109bb28; continue;
 }
 0x109bb28 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  cpu.poll(); pc = 0x109bb09; continue;
 }
 0x109bb34 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x109bb09; continue;
 }
 0x109bb3d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109bb42; continue;
 }
 0x109bb42 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109bb48 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x109bb5f; continue; }
  pc = 0x109bb58; continue;
 }
 0x109bb58 => {
  { let v: u8 = 0x2u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x109bb68; continue;
 }
 0x109bb5f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x10u8); }
  { let v: u8 = 0x1u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x109bb68; continue;
 }
 0x109bb68 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x109bba1; continue; }
  pc = 0x109bb6f; continue;
 }
 0x109bb6f => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x109bb7f; continue; }
  pc = 0x109bb73; continue;
 }
 0x109bb73 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x109bba1; continue;
 }
 0x109bb7f => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x109bb93; continue; }
  pc = 0x109bb83; continue;
 }
 0x109bb83 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x109bba1; continue;
 }
 0x109bb93 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x109bba1; continue;
 }
 0x109bba1 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x109bba3; continue;
 }
 0x109bba3 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x109bbaa); f_0109b729(cpu); if cpu.returned_to != 0x109bbaa { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109bbc9; continue; }
  pc = 0x109bbc6; continue;
 }
 0x109bbc6 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x109bba3; continue;
 }
 0x109bbc9 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if cpu.zf { pc = 0x109bbd6; continue; }
  pc = 0x109bbcf; continue;
 }
 0x109bbcf => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x109bc40; continue;
 }
 0x109bbd6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109bc36; continue; }
  pc = 0x109bbe3; continue;
 }
 0x109bbe3 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x109bc0e; continue; }
  pc = 0x109bbec; continue;
 }
 0x109bbec => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x109bbff; continue; }
  pc = 0x109bbf7; continue;
 }
 0x109bbf7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x109bc06; continue;
 }
 0x109bbff => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x109bc06; continue;
 }
 0x109bc06 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x109bc0e; continue;
 }
 0x109bc0e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x109bc15); f_0109c334(cpu); if cpu.returned_to != 0x109bc15 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109bc40; continue; }
  pc = 0x109bc1d; continue;
 }
 0x109bc1d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x109bbe3; continue; }
  pc = 0x109bc36; continue;
 }
 0x109bc36 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x109bc40); f_0109b730(cpu); if cpu.returned_to != 0x109bc40 { return; }
  pc = 0x109bc40; continue;
 }
 0x109bc40 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x109bc61; continue; }
  pc = 0x109bc46; continue;
 }
 0x109bc46 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109bc61; continue; }
  pc = 0x109bc4a; continue;
 }
 0x109bc4a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x109bc5b; continue; }
  pc = 0x109bc55; continue;
 }
 0x109bc55 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x109bc61; continue;
 }
 0x109bc5b => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x109bc61; continue;
 }
 0x109bc61 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.poll(); pc = 0x109bb42; continue;
 }
 0x109be06 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  pc = 0x109be1d; continue;
 }
 0x109be1d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109be24); f_0109b729(cpu); if cpu.returned_to != 0x109be24 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.edx as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x109be44; continue; }
  pc = 0x109be3e; continue;
 }
 0x109be3e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x60u32.wrapping_add(cpu.esp))); cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x109be1d; continue;
 }
 0x109be44 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x109c057; continue; }
  pc = 0x109be4e; continue;
 }
 0x109be4e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x109c04e; continue; }
  pc = 0x109be5f; continue;
 }
 0x109be5f => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x109be69; continue; }
  pc = 0x109be64; continue;
 }
 0x109be64 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x109be87; continue; }
  pc = 0x109be69; continue;
 }
 0x109be69 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x109be76); f_0109c334(cpu); if cpu.returned_to != 0x109be76 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c057; continue; }
  pc = 0x109be87; continue;
 }
 0x109be87 => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x109bea8; continue; }
  pc = 0x109be9f; continue;
 }
 0x109be9f => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x109c04e; continue; }
  pc = 0x109bea8; continue;
 }
 0x109bea8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109bf10; continue; }
  pc = 0x109bec8; continue;
 }
 0x109bec8 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x109becd; continue;
 }
 0x109becd => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(((cpu.edx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x109bee7; continue; }
  pc = 0x109bed8; continue;
 }
 0x109bed8 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6au32.wrapping_add(cpu.esp)), 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6au32.wrapping_add(cpu.esp), v); }
  pc = 0x109bee7; continue;
 }
 0x109bee7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x109beef); f_0109c334(cpu); if cpu.returned_to != 0x109beef { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c057; continue; }
  pc = 0x109befa; continue;
 }
 0x109befa => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x109becd; continue; }
  pc = 0x109bf10; continue;
 }
 0x109bf10 => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x109bfc0; continue; }
  pc = 0x109bf19; continue;
 }
 0x109bf19 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x109bf22); f_0109c334(cpu); if cpu.returned_to != 0x109bf22 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c057; continue; }
  pc = 0x109bf2e; continue;
 }
 0x109bf2e => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x109bf4c; continue; }
  pc = 0x109bf32; continue;
 }
 0x109bf32 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109c04e; continue; }
  pc = 0x109bf4c; continue;
 }
 0x109bf4c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x109bf4d; continue;
 }
 0x109bf4d => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109bf77; continue; }
  pc = 0x109bf65; continue;
 }
 0x109bf65 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x109bf6f); f_0109c334(cpu); if cpu.returned_to != 0x109bf6f { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x109bf4d; continue; }
  pc = 0x109bf77; continue;
 }
 0x109bf77 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x109bfb7; continue; }
  pc = 0x109bf7d; continue;
 }
 0x109bf7d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  pc = 0x109bf85; continue;
 }
 0x109bf85 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x109bfae; continue; }
  pc = 0x109bf90; continue;
 }
 0x109bf90 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.w32(0x64u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x109bf85; continue;
 }
 0x109bfae => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x109bfb7; continue;
 }
 0x109bfb7 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c057; continue; }
  pc = 0x109bfc0; continue;
 }
 0x109bfc0 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x109c04e; continue; }
  pc = 0x109bfca; continue;
 }
 0x109bfca => {
  { cpu.sub32(cpu.ecx, 0x65u32); }
  if cpu.zf { pc = 0x109bfd8; continue; }
  pc = 0x109bfcf; continue;
 }
 0x109bfcf => {
  { cpu.sub32(cpu.ecx, 0x45u32); }
  if !cpu.zf { pc = 0x109c04e; continue; }
  pc = 0x109bfd8; continue;
 }
 0x109bfd8 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x109bfe2); f_0109c334(cpu); if cpu.returned_to != 0x109bfe2 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c057; continue; }
  pc = 0x109bfee; continue;
 }
 0x109bfee => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x109bff8; continue; }
  pc = 0x109bff3; continue;
 }
 0x109bff3 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x109c00a; continue; }
  pc = 0x109bff8; continue;
 }
 0x109bff8 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x109c002); f_0109c334(cpu); if cpu.returned_to != 0x109c002 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c057; continue; }
  pc = 0x109c00a; continue;
 }
 0x109c00a => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x109c026; continue; }
  pc = 0x109c022; continue;
 }
 0x109c022 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x109c04e; continue;
 }
 0x109c026 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x109c030); f_0109c334(cpu); if cpu.returned_to != 0x109c030 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c057; continue; }
  pc = 0x109c038; continue;
 }
 0x109c038 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x109c026; continue; }
  pc = 0x109c04e; continue;
 }
 0x109c04e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x109c057); f_0109b730(cpu); if cpu.returned_to != 0x109c057 { return; }
  pc = 0x109c057; continue;
 }
 0x109c057 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109c112; continue; }
  pc = 0x109c05f; continue;
 }
 0x109c05f => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x109c112; continue; }
  pc = 0x109c071; continue;
 }
 0x109c071 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x109c08a; continue; }
  pc = 0x109c07a; continue;
 }
 0x109c07a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x109c096; continue; }
  pc = 0x109c084; continue;
 }
 0x109c084 => {
  { let v: u32 = cpu.neg32(cpu.r32(0x68u32.wrapping_add(cpu.esp))); cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x109c096; continue;
 }
 0x109c08a => {
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  let t = cpu.r32(0x1134dbcu32); cpu.push(0x109c096); cpu.call(t); if cpu.returned_to != 0x109c096 { return; }
  pc = 0x109c096; continue;
 }
 0x109c096 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x109c0ae; continue; }
  pc = 0x109c09e; continue;
 }
 0x109c09e => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ebx = v; cpu.set_segment(0, selector); }
  pc = 0x109c0d9; continue;
 }
 0x109c0ae => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x109c0c7; continue; }
  pc = 0x109c0b3; continue;
 }
 0x109c0b3 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  pc = 0x109c0d9; continue;
 }
 0x109c0c7 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  pc = 0x109c0d9; continue;
 }
 0x109c0d9 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if cpu.zf { pc = 0x109c0e7; continue; }
  pc = 0x109c0e1; continue;
 }
 0x109c0e1 => {
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x109c10f; continue;
 }
 0x109c0e7 => {
  { cpu.and8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x109c0f1; continue; }
  pc = 0x109c0ec; continue;
 }
 0x109c0ec => {
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x109c102; continue; }
  pc = 0x109c0f1; continue;
 }
 0x109c0f1 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x109c112; continue;
 }
 0x109c102 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x109c10f); f_010aa8c0(cpu); if cpu.returned_to != 0x109c10f { return; }
  pc = 0x109c10f; continue;
 }
 0x109c10f => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x109c112; continue;
 }
 0x109c112 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.poll(); pc = 0x109bb42; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109bc6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109bc6b); }
 let mut pc: u32 = 0x109bc6b;
 loop { match pc {
 0x109bc6b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x109bcc4; continue; }
  pc = 0x109bc77; continue;
 }
 0x109bc77 => {
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x109bc88; continue; }
  pc = 0x109bc7c; continue;
 }
 0x109bc7c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x109bcad; continue;
 }
 0x109bc88 => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x109bc9d; continue; }
  pc = 0x109bc8d; continue;
 }
 0x109bc8d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x109bcad; continue;
 }
 0x109bc9d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x109bcad; continue;
 }
 0x109bcad => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x109bcbe; continue; }
  pc = 0x109bcb5; continue;
 }
 0x109bcb5 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109bcbe => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x109bcc4; continue;
 }
 0x109bcc4 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109bcc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109bcc9); }
 let mut pc: u32 = 0x109bcc9;
 loop { match pc {
 0x109bcc9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x109bcde); f_0108f030(cpu); if cpu.returned_to != 0x109bcde { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109bd06; continue; }
  pc = 0x109bce7; continue;
 }
 0x109bce7 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1139134u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109bd06; continue; }
  pc = 0x109bd00; continue;
 }
 0x109bd00 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x5du32); }
  if !cpu.zf { cpu.poll(); pc = 0x109bce7; continue; }
  pc = 0x109bd06; continue;
 }
 0x109bd06 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109bd0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109bd0c); }
 let mut pc: u32 = 0x109bd0c;
 loop { match pc {
 0x109bd0c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x5eu32); }
  { let v: u8 = (cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x109bd33; continue; }
  pc = 0x109bd30; continue;
 }
 0x109bd30 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x109bd33; continue;
 }
 0x109bd33 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x109bd3c); f_0109bcc9(cpu); if cpu.returned_to != 0x109bd3c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x109bd7a; continue; }
  pc = 0x109bd46; continue;
 }
 0x109bd46 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x109bd57; continue; }
  pc = 0x109bd4b; continue;
 }
 0x109bd4b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.esi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.esi = v; cpu.set_segment(0, selector); }
  pc = 0x109bd7a; continue;
 }
 0x109bd57 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x109bd6c; continue; }
  pc = 0x109bd5c; continue;
 }
 0x109bd5c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  pc = 0x109bd7a; continue;
 }
 0x109bd6c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.esi = v; }
  pc = 0x109bd7a; continue;
 }
 0x109bd7a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x109bd7f; continue;
 }
 0x109bd7f => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x109bded; continue; }
  pc = 0x109bd83; continue;
 }
 0x109bd83 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x109bd8a); f_0109b729(cpu); if cpu.returned_to != 0x109bd8a { return; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x109bded; continue; }
  pc = 0x109bd96; continue;
 }
 0x109bd96 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1139134u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebp); }
  { let v: u8 = (cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.zf { pc = 0x109bdd9; continue; }
  pc = 0x109bdd0; continue;
 }
 0x109bdd0 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x109bdd7); f_0109b730(cpu); if cpu.returned_to != 0x109bdd7 { return; }
  pc = 0x109bded; continue;
 }
 0x109bdd9 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x109bd7f; continue; }
  pc = 0x109bde3; continue;
 }
 0x109bde3 => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x109bd7f; continue;
 }
 0x109bded => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x109bdfb; continue; }
  pc = 0x109bdf3; continue;
 }
 0x109bdf3 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109bdfb; continue; }
  pc = 0x109bdf7; continue;
 }
 0x109bdf7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x109bdfb; continue;
 }
 0x109bdfb => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c11d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c11d); }
 let mut pc: u32 = 0x109c11d;
 loop { match pc {
 0x109c11d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x109c137; continue;
 }
 0x109c137 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109c13e); f_0109b729(cpu); if cpu.returned_to != 0x109c13e { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109c159; continue; }
  pc = 0x109c154; continue;
 }
 0x109c154 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x109c137; continue;
 }
 0x109c159 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x109c295; continue; }
  pc = 0x109c163; continue;
 }
 0x109c163 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109c28c; continue; }
  pc = 0x109c174; continue;
 }
 0x109c174 => {
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x109c1a8; continue; }
  pc = 0x109c185; continue;
 }
 0x109c185 => {
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if cpu.zf { pc = 0x109c18f; continue; }
  pc = 0x109c18a; continue;
 }
 0x109c18a => {
  { cpu.sub32(cpu.ebx, 0x2du32); }
  if !cpu.zf { pc = 0x109c1a8; continue; }
  pc = 0x109c18f; continue;
 }
 0x109c18f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x109c19d); f_0109c334(cpu); if cpu.returned_to != 0x109c19d { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c295; continue; }
  pc = 0x109c1a8; continue;
 }
 0x109c1a8 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x109c204; continue; }
  pc = 0x109c1ac; continue;
 }
 0x109c1ac => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x109c1fd; continue; }
  pc = 0x109c1b1; continue;
 }
 0x109c1b1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x109c1bd); f_0109c334(cpu); if cpu.returned_to != 0x109c1bd { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c295; continue; }
  pc = 0x109c1c8; continue;
 }
 0x109c1c8 => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x109c1d2; continue; }
  pc = 0x109c1cd; continue;
 }
 0x109c1cd => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x109c1f6; continue; }
  pc = 0x109c1d2; continue;
 }
 0x109c1d2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  cpu.push(0x109c1dc); f_0109c334(cpu); if cpu.returned_to != 0x109c1dc { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c295; continue; }
  pc = 0x109c1ef; continue;
 }
 0x109c1ef => {
  { let v: u32 = 0x10u32; cpu.edx = v; }
  pc = 0x109c248; continue;
 }
 0x109c1f6 => {
  { let v: u32 = 0x8u32; cpu.edx = v; }
  pc = 0x109c248; continue;
 }
 0x109c1fd => {
  { let v: u32 = 0xau32; cpu.edx = v; }
  pc = 0x109c248; continue;
 }
 0x109c204 => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if !cpu.zf { pc = 0x109c248; continue; }
  pc = 0x109c209; continue;
 }
 0x109c209 => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x109c248; continue; }
  pc = 0x109c20e; continue;
 }
 0x109c20e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x109c21a); f_0109c334(cpu); if cpu.returned_to != 0x109c21a { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c295; continue; }
  pc = 0x109c225; continue;
 }
 0x109c225 => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x109c22f; continue; }
  pc = 0x109c22a; continue;
 }
 0x109c22a => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x109c248; continue; }
  pc = 0x109c22f; continue;
 }
 0x109c22f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x109c239); f_0109c334(cpu); if cpu.returned_to != 0x109c239 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x109c241; continue;
 }
 0x109c241 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c295; continue; }
  pc = 0x109c248; continue;
 }
 0x109c248 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x109c24f); f_0109c30d(cpu); if cpu.returned_to != 0x109c24f { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x109c262; continue; }
  pc = 0x109c253; continue;
 }
 0x109c253 => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x109c260); f_0109c334(cpu); if cpu.returned_to != 0x109c260 { return; }
  cpu.poll(); pc = 0x109c241; continue;
 }
 0x109c262 => {
  { cpu.sub32(cpu.ebx, 0x3au32); }
  if !cpu.zf { pc = 0x109c28c; continue; }
  pc = 0x109c267; continue;
 }
 0x109c267 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x109c28c; continue; }
  pc = 0x109c26d; continue;
 }
 0x109c26d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x109c275); f_0109c334(cpu); if cpu.returned_to != 0x109c275 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c295; continue; }
  pc = 0x109c27c; continue;
 }
 0x109c27c => {
  cpu.push(0x109c281); f_0109c30d(cpu); if cpu.returned_to != 0x109c281 { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x109c28c; continue; }
  pc = 0x109c285; continue;
 }
 0x109c285 => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  cpu.poll(); pc = 0x109c26d; continue;
 }
 0x109c28c => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x109c295); f_0109b730(cpu); if cpu.returned_to != 0x109c295 { return; }
  pc = 0x109c295; continue;
 }
 0x109c295 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x2du32); }
  if !cpu.zf { pc = 0x109c29e; continue; }
  pc = 0x109c29c; continue;
 }
 0x109c29c => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x109c29e; continue;
 }
 0x109c29e => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109c303; continue; }
  pc = 0x109c2a2; continue;
 }
 0x109c2a2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x109c303; continue; }
  pc = 0x109c2af; continue;
 }
 0x109c2af => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x109c2c4; continue; }
  pc = 0x109c2b4; continue;
 }
 0x109c2b4 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.eax = v; cpu.set_segment(0, selector); }
  pc = 0x109c2ef; continue;
 }
 0x109c2c4 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x109c2dd; continue; }
  pc = 0x109c2c9; continue;
 }
 0x109c2c9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x109c2ef; continue;
 }
 0x109c2dd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x109c2ef; continue;
 }
 0x109c2ef => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x10u8); }
  if cpu.zf { pc = 0x109c2fd; continue; }
  pc = 0x109c2f7; continue;
 }
 0x109c2f7 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x109c303; continue;
 }
 0x109c2fd => {
  { cpu.and8((cpu.ebx as u8), 0x20u8); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x109c303; continue;
 }
 0x109c303 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c30d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c30d); }
 let mut pc: u32 = 0x109c30d;
 loop { match pc {
 0x109c30d => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x109c31b; continue; }
  pc = 0x109c312; continue;
 }
 0x109c312 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109c31b; continue; }
  pc = 0x109c317; continue;
 }
 0x109c317 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109c31b => {
  cpu.push(0x109c320); f_0109a0f2(cpu); if cpu.returned_to != 0x109c320 { return; }
  { cpu.sub32(cpu.eax, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x109c32e; continue; }
  pc = 0x109c325; continue;
 }
 0x109c325 => {
  { cpu.sub32(cpu.eax, 0x66u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109c32e; continue; }
  pc = 0x109c32a; continue;
 }
 0x109c32a => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x57u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109c32e => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c334(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c334); }
 let mut pc: u32 = 0x109c334;
 loop { match pc {
 0x109c334 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109c352; continue; }
  pc = 0x109c345; continue;
 }
 0x109c345 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x109c34c); f_0109b729(cpu); if cpu.returned_to != 0x109c34c { return; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edx)), 0x2u8); }
  if cpu.zf { pc = 0x109c357; continue; }
  pc = 0x109c352; continue;
 }
 0x109c352 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x109c357; continue;
 }
 0x109c357 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c35c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c35c); }
 let mut pc: u32 = 0x109c35c;
 loop { match pc {
 0x109c35c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x1134262u32)); cpu.w32(0x1134262u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c369(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c369); }
 let mut pc: u32 = 0x109c369;
 loop { match pc {
 0x109c369 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x1134262u32)); cpu.w32(0x1134262u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c376(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c376); }
 let mut pc: u32 = 0x109c376;
 loop { match pc {
 0x109c376 => {
  { cpu.sub32(cpu.r32(0x113425eu32), 0x0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x109c495; continue; }
  pc = 0x109c384; continue;
 }
 0x109c384 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  { let v = cpu.r16(0x1134266u32); cpu.set_segment(3, v as u16); }
  { let v = cpu.r16(0x1134266u32); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.inc32(cpu.r32(0x113425eu32.wrapping_add(cpu.bases[3]))); cpu.w32(0x113425eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x1134c74u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x1134c78u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x1134c74u32; cpu.esp = v; }
  { let v: u32 = cpu.r32(0x113425au32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x109c3c6; continue;
 }
 0x109c3c6 => {
  { cpu.sub32(cpu.r32(0x113410cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x2u32); }
  if !cpu.zf { pc = 0x109c3f1; continue; }
  pc = 0x109c3cf; continue;
 }
 0x109c3cf => {
  { let v: u32 = cpu.r32(0x113414cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x113418cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if cpu.cf { pc = 0x109c3eb; continue; }
  pc = 0x109c3df; continue;
 }
 0x109c3df => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x113418cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x11341ccu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.w32(0x11341ccu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x109c3eb; continue;
 }
 0x109c3eb => {
  { let v: u32 = cpu.eax; cpu.w32(0x113414cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x109c3f1; continue;
 }
 0x109c3f1 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if cpu.cf { cpu.poll(); pc = 0x109c3c6; continue; }
  pc = 0x109c3f9; continue;
 }
 0x109c3f9 => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { cpu.interrupts_off = false; }
  { cpu.sub32(cpu.r32(0x1134262u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109c436; continue; }
  pc = 0x109c407; continue;
 }
 0x109c407 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x109c40c; continue;
 }
 0x109c40c => {
  { cpu.sub32(cpu.r32(0x11341ccu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x109c42e; continue; }
  pc = 0x109c415; continue;
 }
 0x109c415 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11341ccu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.w32(0x11341ccu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x113420cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x11340ccu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x109c424); cpu.push(0x109c42a); cpu.call(t); if cpu.returned_to != 0x109c42a { return; }
  { let v: u32 = cpu.esi; cpu.esp = v; }
  cpu.poll(); pc = 0x109c40c; continue;
 }
 0x109c42e => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x3cu32); }
  if cpu.cf { cpu.poll(); pc = 0x109c40c; continue; }
  pc = 0x109c436; continue;
 }
 0x109c436 => {
  { cpu.sub32(cpu.r32(0x1134208u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x109c479; continue; }
  pc = 0x109c43f; continue;
 }
 0x109c43f => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1134208u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x1134208u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r16(0x1134c74u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x1134c78u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x113425eu32.wrapping_add(cpu.bases[3]))); cpu.w32(0x113425eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x113424cu32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x1134250u32.wrapping_add(cpu.bases[3])) as u32; cpu.eax = v; }
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
 0x109c479 => {
  { let v = cpu.r16(0x1134c74u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x1134c78u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x113425eu32.wrapping_add(cpu.bases[3]))); cpu.w32(0x113425eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x109c494); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x109c495 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x109c49b); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c49c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c49c); }
 let mut pc: u32 = 0x109c49c;
 loop { match pc {
 0x109c49c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.interrupts_off = true; }
  { let v: u8 = 0x36u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x43u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1134256u32, v); }
  pc = 0x109c4b2; continue;
 }
 0x109c4b2 => {
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x109c4b8; continue;
 }
 0x109c4b8 => {
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { cpu.interrupts_off = true; }
  if cpu.zf { pc = 0x109c4c5; continue; }
  pc = 0x109c4c4; continue;
 }
 0x109c4c4 => {
  { cpu.interrupts_off = false; }
  pc = 0x109c4c5; continue;
 }
 0x109c4c5 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c4cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c4cc); }
 let mut pc: u32 = 0x109c4cc;
 loop { match pc {
 0x109c4cc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0xd68du32); }
  if !cpu.cf { pc = 0x109c4f1; continue; }
  pc = 0x109c4e0; continue;
 }
 0x109c4e0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x20bcu32; cpu.ebx = v; }
  { let v: u32 = 0x2710u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x109c4f1; continue;
 }
 0x109c4f1 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109c4f7); f_0109c49c(cpu); if cpu.returned_to != 0x109c4f7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c4ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c4ff); }
 let mut pc: u32 = 0x109c4ff;
 loop { match pc {
 0x109c4ff => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.flat_ds(0x109c503); cpu.push(0x109c508); f_0108fc12(cpu); if cpu.returned_to != 0x109c508 { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x109c515; continue;
 }
 0x109c515 => {
  { cpu.sub32(cpu.r32(0x113410cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x109c52a; continue; }
  pc = 0x109c51e; continue;
 }
 0x109c51e => {
  { let v: u32 = cpu.r32(0x113418cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x109c52a; continue; }
  pc = 0x109c528; continue;
 }
 0x109c528 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x109c52a; continue;
 }
 0x109c52a => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if cpu.cf { cpu.poll(); pc = 0x109c515; continue; }
  pc = 0x109c532; continue;
 }
 0x109c532 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x113425au32.wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x109c55a; continue; }
  pc = 0x109c53a; continue;
 }
 0x109c53a => {
  { let v: u32 = cpu.ecx; cpu.w32(0x113425au32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x109c541); cpu.push(0x109c546); f_0109c4cc(cpu); if cpu.returned_to != 0x109c546 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x113414cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  pc = 0x109c55a; continue;
 }
 0x109c55a => {
  cpu.flat_ds(0x109c55a); cpu.push(0x109c55f); f_0108fc17(cpu); if cpu.returned_to != 0x109c55f { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x109c563); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c652(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c652); }
 let mut pc: u32 = 0x109c652;
 loop { match pc {
 0x109c652 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.interrupts_off = true; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.flat_ds(0x109c659); cpu.push(0x109c65e); f_0109c49c(cpu); if cpu.returned_to != 0x109c65e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x113424cu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x1134250u32.wrapping_add(cpu.bases[3])) as u32; cpu.ebx = v; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { cpu.interrupts_off = true; }
  if cpu.zf { pc = 0x109c687; continue; }
  pc = 0x109c686; continue;
 }
 0x109c686 => {
  { cpu.interrupts_off = false; }
  pc = 0x109c687; continue;
 }
 0x109c687 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x109c68c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c68d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c68d); }
 let mut pc: u32 = 0x109c68d;
 loop { match pc {
 0x109c68d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x200u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c6b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c6b9); }
 let mut pc: u32 = 0x109c6b9;
 loop { match pc {
 0x109c6b9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x201u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c6e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109c6e0); } r_0109c6e0(cpu, 0x109c6e0); }
pub fn f_0109c6e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109c6e7); } r_0109c6e0(cpu, 0x109c6e7); }
pub fn f_0109c6f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109c6f6); } r_0109c6e0(cpu, 0x109c6f6); }
pub fn f_0109c6fc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109c6fc); } r_0109c6e0(cpu, 0x109c6fc); }
pub fn f_0109c712(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109c712); } r_0109c6e0(cpu, 0x109c712); }
pub fn f_0109c71c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109c71c); } r_0109c6e0(cpu, 0x109c71c); }
pub fn f_0109c723(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109c723); } r_0109c6e0(cpu, 0x109c723); }
fn r_0109c6e0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x109c6e0 => {
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = 0x9999u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u16 = 0x9999u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x99999999u32; cpu.esp = v; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffau32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x109c6e7 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u16 = 0x9999u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x99999999u32; cpu.esp = v; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffau32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x109c6f6 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x99999999u32; cpu.esp = v; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffau32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x109c6fc => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffau32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x109c712 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x109c71c => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x109c723 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c72a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c72a); }
 let mut pc: u32 = 0x109c72a;
 loop { match pc {
 0x109c72a => {
  { let v = (cpu.ebx as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.ecx; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x109c734); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c94b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c94b); }
 let mut pc: u32 = 0x109c94b;
 loop { match pc {
 0x109c94b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x109c98a; continue; }
  pc = 0x109c95a; continue;
 }
 0x109c95a => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x1134268u32)); }
  if !cpu.zf { pc = 0x109c98a; continue; }
  pc = 0x109c962; continue;
 }
 0x109c962 => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if cpu.cf { pc = 0x109c96a; continue; }
  pc = 0x109c967; continue;
 }
 0x109c967 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x60u32); cpu.ebx = v; }
  pc = 0x109c96a; continue;
 }
 0x109c96a => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x205u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x113426cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1134270u32); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1134268u32, v); }
  pc = 0x109c98a; continue;
 }
 0x109c98a => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c994(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c994); }
 let mut pc: u32 = 0x109c994;
 loop { match pc {
 0x109c994 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { cpu.interrupts_off = true; }
  if cpu.zf { pc = 0x109c9a5; continue; }
  pc = 0x109c9a4; continue;
 }
 0x109c9a4 => {
  { cpu.interrupts_off = false; }
  pc = 0x109c9a5; continue;
 }
 0x109c9a5 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c9a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c9a9); }
 let mut pc: u32 = 0x109c9a9;
 loop { match pc {
 0x109c9a9 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffffffbu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.esp); cpu.ebp = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.poll(); pc = cpu.edx; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109c9fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109c9fb); }
 let mut pc: u32 = 0x109c9fb;
 loop { match pc {
 0x109c9fb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffc4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.flat_ds(0x109ca06); cpu.push(0x109ca0b); f_0108fc12(cpu); if cpu.returned_to != 0x109ca0b { return; }
  { cpu.df = false; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0xffffffc6u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffeau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x109ca6a; continue; }
  pc = 0x109ca4a; continue;
 }
 0x109ca4a => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffdeu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xffffffcau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffc6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x109ca6a; continue;
 }
 0x109ca6a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x32u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xffffffd6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x300u32; cpu.eax = v; }
  { let v: u32 = 0x66u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffc6u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x109cac3; continue; }
  pc = 0x109ca94; continue;
 }
 0x109ca94 => {
  { let v: u16 = cpu.r16(0xffffffe2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xffffffdeu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xffffffdau32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xffffffcau32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xffffffc6u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0xffffffd6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x109cac3; continue;
 }
 0x109cac3 => {
  { let v: u32 = cpu.r16(0xffffffe2u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x109cac8); cpu.push(0x109cacd); f_0108fc17(cpu); if cpu.returned_to != 0x109cacd { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x109cad4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cad5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cad5); }
 let mut pc: u32 = 0x109cad5;
 loop { match pc {
 0x109cad5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = 0x63u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x6u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  if cpu.ecx == 0 { pc = 0x109cb0d; continue; }
  pc = 0x109cafb; continue;
 }
 0x109cafb => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, 0x8u32); }
  if cpu.zf { cpu.poll(); pc = 0x109cafb; continue; }
  pc = 0x109cb03; continue;
 }
 0x109cb03 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, 0x8u32); }
  if !cpu.zf { cpu.poll(); pc = 0x109cb03; continue; }
  pc = 0x109cb0b; continue;
 }
 0x109cb0b => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x109cafb; continue; }
  pc = 0x109cb0d; continue;
 }
 0x109cb0d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x109cb11); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cb12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cb12); }
 let mut pc: u32 = 0x109cb12;
 loop { match pc {
 0x109cb12 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x113425eu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cb1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cb1e); }
 let mut pc: u32 = 0x109cb1e;
 loop { match pc {
 0x109cb1e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109cb29); f_0108fc12(cpu); if cpu.returned_to != 0x109cb29 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x109cb2e; continue;
 }
 0x109cb2e => {
  { cpu.sub32(cpu.r32(0x113410cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x109cb46; continue; }
  pc = 0x109cb37; continue;
 }
 0x109cb37 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3cu32); }
  if cpu.cf { cpu.poll(); pc = 0x109cb2e; continue; }
  pc = 0x109cb3f; continue;
 }
 0x109cb3f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x109cb59; continue;
 }
 0x109cb46 => {
  { let v: u32 = 0x1u32; cpu.w32(0x113410cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11340ccu32.wrapping_add(cpu.eax), v); }
  pc = 0x109cb59; continue;
 }
 0x109cb59 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109cb5f); f_0108fc17(cpu); if cpu.returned_to != 0x109cb5f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cb65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cb65); }
 let mut pc: u32 = 0x109cb65;
 loop { match pc {
 0x109cb65 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109cb70); f_0108fc12(cpu); if cpu.returned_to != 0x109cb70 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x109cb81; continue; }
  pc = 0x109cb78; continue;
 }
 0x109cb78 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let a = cpu.r32(0x113420cu32.wrapping_add(cpu.ebx)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x113420cu32.wrapping_add(cpu.ebx), v); } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x109cb81; continue;
 }
 0x109cb81 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109cb87); f_0108fc17(cpu); if cpu.returned_to != 0x109cb87 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cb8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cb8d); }
 let mut pc: u32 = 0x109cb8d;
 loop { match pc {
 0x109cb8d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109cb98); f_0108fc12(cpu); if cpu.returned_to != 0x109cb98 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109cbaa; continue; }
  pc = 0x109cba0; continue;
 }
 0x109cba0 => {
  { let v: u32 = 0x0u32; cpu.w32(0x113410cu32.wrapping_add(cpu.eax), v); }
  pc = 0x109cbaa; continue;
 }
 0x109cbaa => {
  cpu.push(0x109cbaf); f_0108fc17(cpu); if cpu.returned_to != 0x109cbaf { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cbb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cbb4); }
 let mut pc: u32 = 0x109cbb4;
 loop { match pc {
 0x109cbb4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109cbbc); f_0108fc12(cpu); if cpu.returned_to != 0x109cbbc { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x109cbc1; continue;
 }
 0x109cbc1 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109cbc7); f_0109092e(cpu); if cpu.returned_to != 0x109cbc7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x109cbc1; continue; }
  pc = 0x109cbcf; continue;
 }
 0x109cbcf => {
  cpu.push(0x109cbd4); f_0108fc17(cpu); if cpu.returned_to != 0x109cbd4 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cbd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cbd8); }
 let mut pc: u32 = 0x109cbd8;
 loop { match pc {
 0x109cbd8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109cbe3); f_0108fc12(cpu); if cpu.returned_to != 0x109cbe3 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x109cbfe; continue; }
  pc = 0x109cbeb; continue;
 }
 0x109cbeb => {
  { cpu.sub32(cpu.r32(0x113410cu32.wrapping_add(cpu.ebx)), 0x1u32); }
  if !cpu.zf { pc = 0x109cbfe; continue; }
  pc = 0x109cbf4; continue;
 }
 0x109cbf4 => {
  { let v: u32 = 0x2u32; cpu.w32(0x113410cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x109cbfe; continue;
 }
 0x109cbfe => {
  cpu.push(0x109cc03); f_0108fc17(cpu); if cpu.returned_to != 0x109cc03 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cc08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cc08); }
 let mut pc: u32 = 0x109cc08;
 loop { match pc {
 0x109cc08 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109cc10); f_0108fc12(cpu); if cpu.returned_to != 0x109cc10 { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x109cc15; continue;
 }
 0x109cc15 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109cc1b); f_01090790(cpu); if cpu.returned_to != 0x109cc1b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x109cc15; continue; }
  pc = 0x109cc23; continue;
 }
 0x109cc23 => {
  cpu.push(0x109cc28); f_0108fc17(cpu); if cpu.returned_to != 0x109cc28 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cc80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cc80); }
 let mut pc: u32 = 0x109cc80;
 loop { match pc {
 0x109cc80 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109cc8b); f_0108fc12(cpu); if cpu.returned_to != 0x109cc8b { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x113418cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x113414cu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x109cca6); f_0109c4ff(cpu); if cpu.returned_to != 0x109cca6 { return; }
  cpu.push(0x109ccab); f_0108fc17(cpu); if cpu.returned_to != 0x109ccab { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ccb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ccb0); }
 let mut pc: u32 = 0x109ccb0;
 loop { match pc {
 0x109ccb0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109ccbb); f_0108fc12(cpu); if cpu.returned_to != 0x109ccbb { return; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x109ccd3); f_01090550(cpu); if cpu.returned_to != 0x109ccd3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x109ccdb); f_0108fc17(cpu); if cpu.returned_to != 0x109ccdb { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cce0); }
 let mut pc: u32 = 0x109cce0;
 loop { match pc {
 0x109cce0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109cceb); f_0108fc12(cpu); if cpu.returned_to != 0x109cceb { return; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x109ccf8; continue; }
  pc = 0x109ccf1; continue;
 }
 0x109ccf1 => {
  { let v: u32 = 0xd68du32; cpu.eax = v; }
  pc = 0x109cd0c; continue;
 }
 0x109ccf8 => {
  { let v: u32 = 0x2710u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x2e9cu32; cpu.ebx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x109cd0c; continue;
 }
 0x109cd0c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x109cd15); f_01090550(cpu); if cpu.returned_to != 0x109cd15 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x109cd1d); f_0108fc17(cpu); if cpu.returned_to != 0x109cd1d { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cd22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cd22); }
 let mut pc: u32 = 0x109cd22;
 loop { match pc {
 0x109cd22 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1134256u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cd2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cd2e); }
 let mut pc: u32 = 0x109cd2e;
 loop { match pc {
 0x109cd2e => {
  { let v = 0x1134c7cu32; cpu.push(v); }
  { let v = 0x11340ccu32; cpu.push(v); }
  cpu.push(0x109cd3d); f_01013d5e(cpu); if cpu.returned_to != 0x109cd3d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x109cd2eu32; cpu.push(v); }
  { let v = 0x109c35cu32; cpu.push(v); }
  cpu.push(0x109cd4f); f_01013d5e(cpu); if cpu.returned_to != 0x109cd4f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cd53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cd53); }
 let mut pc: u32 = 0x109cd53;
 loop { match pc {
 0x109cd53 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134c80u32), 0x0u32); }
  if !cpu.zf { pc = 0x109cdd8; continue; }
  pc = 0x109cd5f; continue;
 }
 0x109cd5f => {
  { let v = 0x109d62eu32; cpu.push(v); }
  { let v = 0x109cd53u32; cpu.push(v); }
  cpu.push(0x109cd6e); f_01013d5e(cpu); if cpu.returned_to != 0x109cd6e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x11ee158u32; cpu.push(v); }
  cpu.push(0x109cd7d); f_01013dec(cpu); if cpu.returned_to != 0x109cd7d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4cu32; cpu.push(v); }
  { let v = 0x11ee2b4u32; cpu.push(v); }
  cpu.push(0x109cd8c); f_01013dec(cpu); if cpu.returned_to != 0x109cd8c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x11ee198u32; cpu.push(v); }
  cpu.push(0x109cd9e); f_01013dec(cpu); if cpu.returned_to != 0x109cd9e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v = 0x11ee298u32; cpu.push(v); }
  cpu.push(0x109cdad); f_01013dec(cpu); if cpu.returned_to != 0x109cdad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x1134c7cu32; cpu.push(v); }
  cpu.push(0x109cdbc); f_01013dec(cpu); if cpu.returned_to != 0x109cdbc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee2b0u32; cpu.push(v); }
  cpu.push(0x109cdcb); f_01013dec(cpu); if cpu.returned_to != 0x109cdcb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1134c80u32, v); }
  pc = 0x109cdd8; continue;
 }
 0x109cdd8 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cddc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cddc); }
 let mut pc: u32 = 0x109cddc;
 loop { match pc {
 0x109cddc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134c7cu32), 0x0u32); }
  if !cpu.zf { pc = 0x109ce28; continue; }
  pc = 0x109cde7; continue;
 }
 0x109cde7 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1134c7cu32, v); }
  cpu.push(0x109cdf6); f_0108fe7e(cpu); if cpu.returned_to != 0x109cdf6 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee2b0u32, v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x302u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x109ce0e); f_0108fe91(cpu); if cpu.returned_to != 0x109ce0e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ee2b0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109ce1d); f_0108fe83(cpu); if cpu.returned_to != 0x109ce1d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1134c7cu32, v); }
  pc = 0x109ce28; continue;
 }
 0x109ce28 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ce2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ce2b); }
 let mut pc: u32 = 0x109ce2b;
 loop { match pc {
 0x109ce2b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x109cea7; continue;
 }
 0x109ce49 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x109ce5b; continue; }
  pc = 0x109ce55; continue;
 }
 0x109ce55 => {
  { let v: u32 = cpu.neg32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x109cea6; continue;
 }
 0x109ce5b => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x109cea6; continue; }
  pc = 0x109ce73; continue;
 }
 0x109ce73 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109ce9e; continue;
 }
 0x109ce77 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x11389ecu32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  cpu.push(0x109ce8c); f_0108e3ae(cpu); if cpu.returned_to != 0x109ce8c { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x109ce9d; continue; }
  pc = 0x109ce90; continue;
 }
 0x109ce90 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x109cea2; continue;
 }
 0x109ce9d => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109ce9e; continue;
 }
 0x109ce9e => {
  { cpu.sub32(cpu.edx, cpu.edi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109ce77; continue; }
  pc = 0x109cea2; continue;
 }
 0x109cea2 => {
  { cpu.sub32(cpu.edx, cpu.edi); }
  if cpu.zf { pc = 0x109ceb2; continue; }
  pc = 0x109cea6; continue;
 }
 0x109cea6 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x109cea7; continue;
 }
 0x109cea7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109ceae); f_0108e46c(cpu); if cpu.returned_to != 0x109ceae { return; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.cf { cpu.poll(); pc = 0x109ce49; continue; }
  pc = 0x109ceb2; continue;
 }
 0x109ceb2 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109cebe; continue; }
  pc = 0x109ceba; continue;
 }
 0x109ceba => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x109cebe; continue;
 }
 0x109cebe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109cecc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109cecc); }
 let mut pc: u32 = 0x109cecc;
 loop { match pc {
 0x109cecc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x168u32); cpu.esp = v; }
  { let v: u32 = 0x118u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x109cee3); f_0108f030(cpu); if cpu.returned_to != 0x109cee3 { return; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x109cef9); f_0108f030(cpu); if cpu.returned_to != 0x109cef9 { return; }
  { let v: u32 = 0x11389fdu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x17cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x109cf0a); f_0108e8a5(cpu); if cpu.returned_to != 0x109cf0a { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109d030; continue; }
  pc = 0x109cf14; continue;
 }
 0x109cf14 => {
  pc = 0x109d18a; continue;
 }
 0x109cf19 => {
  { let v: u8 = cpu.r8(0x118u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109cf44; continue; }
  pc = 0x109cf36; continue;
 }
 0x109cf36 => {
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x118u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x109cf40; continue;
 }
 0x109cf40 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x109cf19; continue; }
  pc = 0x109cf44; continue;
 }
 0x109cf44 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x109cf66; continue;
 }
 0x109cf48 => {
  { let v: u8 = cpu.r8(0x118u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109cf76; continue; }
  pc = 0x109cf65; continue;
 }
 0x109cf65 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x109cf66; continue;
 }
 0x109cf66 => {
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x109cf72); f_0108e46c(cpu); if cpu.returned_to != 0x109cf72 { return; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.cf { cpu.poll(); pc = 0x109cf48; continue; }
  pc = 0x109cf76; continue;
 }
 0x109cf76 => {
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  pc = 0x109cf9f; continue;
 }
 0x109cf81 => {
  { let v: u8 = cpu.r8(0x118u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x109cfaf; continue; }
  pc = 0x109cf9e; continue;
 }
 0x109cf9e => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x109cf9f; continue;
 }
 0x109cf9f => {
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x109cfab); f_0108e46c(cpu); if cpu.returned_to != 0x109cfab { return; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.cf { cpu.poll(); pc = 0x109cf81; continue; }
  pc = 0x109cfaf; continue;
 }
 0x109cfaf => {
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  pc = 0x109cfd8; continue;
 }
 0x109cfba => {
  { let v: u8 = cpu.r8(0x118u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139178u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109cfe8; continue; }
  pc = 0x109cfd7; continue;
 }
 0x109cfd7 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x109cfd8; continue;
 }
 0x109cfd8 => {
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x109cfe4); f_0108e46c(cpu); if cpu.returned_to != 0x109cfe4 { return; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.cf { cpu.poll(); pc = 0x109cfba; continue; }
  pc = 0x109cfe8; continue;
 }
 0x109cfe8 => {
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x109cffd); f_0108e46c(cpu); if cpu.returned_to != 0x109cffd { return; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.cf { pc = 0x109d030; continue; }
  pc = 0x109d001; continue;
 }
 0x109d001 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3bu32); }
  if cpu.zf { pc = 0x109d030; continue; }
  pc = 0x109d00d; continue;
 }
 0x109d00d => {
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u32 = 0x1138a00u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109d01e); f_0108f6fc(cpu); if cpu.returned_to != 0x109d01e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109d069; continue; }
  pc = 0x109d022; continue;
 }
 0x109d022 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x109d02b; continue;
 }
 0x109d02b => {
  cpu.push(0x109d030); f_0108e30e(cpu); if cpu.returned_to != 0x109d030 { return; }
  pc = 0x109d030; continue;
 }
 0x109d030 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = 0x50u32; cpu.edx = v; }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x109d043); f_0108f61e(cpu); if cpu.returned_to != 0x109d043 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109d13d; continue; }
  pc = 0x109d04b; continue;
 }
 0x109d04b => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x109d13d; continue; }
  pc = 0x109d055; continue;
 }
 0x109d055 => {
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x109d061); f_0108e46c(cpu); if cpu.returned_to != 0x109d061 { return; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  cpu.poll(); pc = 0x109cf40; continue;
 }
 0x109d069 => {
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u32 = 0x1138a07u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109d07a); f_0108f6fc(cpu); if cpu.returned_to != 0x109d07a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109d084; continue; }
  pc = 0x109d07e; continue;
 }
 0x109d07e => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.poll(); pc = 0x109d02b; continue;
 }
 0x109d084 => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x1138a0eu32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109d095); f_0108f6fc(cpu); if cpu.returned_to != 0x109d095 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109d0af; continue; }
  pc = 0x109d099; continue;
 }
 0x109d099 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109d0a2); f_0109ce2b(cpu); if cpu.returned_to != 0x109d0a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x100u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x109d030; continue;
 }
 0x109d0af => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x1138a16u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109d0c0); f_0108f6fc(cpu); if cpu.returned_to != 0x109d0c0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109d0dd; continue; }
  pc = 0x109d0c4; continue;
 }
 0x109d0c4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109d0cd); f_0109ce2b(cpu); if cpu.returned_to != 0x109d0cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x102u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x109d030; continue;
 }
 0x109d0dd => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = 0x1138a1au32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109d0ee); f_0108f6fc(cpu); if cpu.returned_to != 0x109d0ee { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109d10b; continue; }
  pc = 0x109d0f2; continue;
 }
 0x109d0f2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109d0fb); f_0109ce2b(cpu); if cpu.returned_to != 0x109d0fb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x104u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x109d030; continue;
 }
 0x109d10b => {
  { let v: u32 = 0xbu32; cpu.ebx = v; }
  { let v: u32 = 0x1138a24u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109d11c); f_0108f6fc(cpu); if cpu.returned_to != 0x109d11c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x109d030; continue; }
  pc = 0x109d124; continue;
 }
 0x109d124 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109d12d); f_0109ce2b(cpu); if cpu.returned_to != 0x109d12d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x106u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x109d030; continue;
 }
 0x109d13d => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x109d144); f_0108f1c8(cpu); if cpu.returned_to != 0x109d144 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x109d150); f_0108e46c(cpu); if cpu.returned_to != 0x109d150 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109d15f; continue; }
  pc = 0x109d154; continue;
 }
 0x109d154 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x109d15b); f_0108e46c(cpu); if cpu.returned_to != 0x109d15b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109d175; continue; }
  pc = 0x109d15f; continue;
 }
 0x109d15f => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138a2fu32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109d18a; continue;
 }
 0x109d175 => {
  { let v: u32 = 0x46u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x178u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109d18a; continue;
 }
 0x109d18a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x168u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d194(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d194); }
 let mut pc: u32 = 0x109d194;
 loop { match pc {
 0x109d194 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee2b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11ee2b4u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d1ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d1ad); }
 let mut pc: u32 = 0x109d1ad;
 loop { match pc {
 0x109d1ad => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x2cu32; cpu.push(v); }
  cpu.push(0x109d1bb); f_01013c6b(cpu); if cpu.returned_to != 0x109d1bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109d1dc; continue; }
  pc = 0x109d1c4; continue;
 }
 0x109d1c4 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138a44u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109d1dc => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xfu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109d1f6); f_01013cad(cpu); if cpu.returned_to != 0x109d1f6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109d216; continue; }
  pc = 0x109d1fd; continue;
 }
 0x109d1fd => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138a6fu32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(1, false); }
  { let v = 0x2cu32; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x109d288; continue;
 }
 0x109d216 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x109d226); f_010a6310(cpu); if cpu.returned_to != 0x109d226 { return; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u32 = 0x1138a88u32; cpu.edx = v; }
  cpu.push(0x109d23f); f_0108f6fc(cpu); if cpu.returned_to != 0x109d23f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109d248; continue; }
  pc = 0x109d243; continue;
 }
 0x109d243 => {
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109d29d; continue;
 }
 0x109d248 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v: u32 = 0x1138a90u32; cpu.edx = v; }
  cpu.push(0x109d25a); f_0108f6fc(cpu); if cpu.returned_to != 0x109d25a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109d296; continue; }
  pc = 0x109d25e; continue;
 }
 0x109d25e => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138a98u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x109d27d; continue;
 }
 0x109d27d => {
  cpu.push(0x109d282); f_01013d32(cpu); if cpu.returned_to != 0x109d282 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x2cu32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x109d288; continue;
 }
 0x109d288 => {
  cpu.push(0x109d28d); f_01013c8c(cpu); if cpu.returned_to != 0x109d28d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109d296 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109d29d; continue;
 }
 0x109d29d => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x109d2bf; continue;
 }
 0x109d2a1 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11ee158u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x109d2be; continue; }
  pc = 0x109d2af; continue;
 }
 0x109d2af => {
  { let v: u32 = cpu.ebp; cpu.w32(0x11ee158u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x32u32.wrapping_add(cpu.eax), v); }
  pc = 0x109d2c4; continue;
 }
 0x109d2be => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x109d2bf; continue;
 }
 0x109d2bf => {
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if cpu.cf { cpu.poll(); pc = 0x109d2a1; continue; }
  pc = 0x109d2c4; continue;
 }
 0x109d2c4 => {
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if !cpu.zf { pc = 0x109d2eb; continue; }
  pc = 0x109d2c9; continue;
 }
 0x109d2c9 => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138aadu32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.poll(); pc = 0x109d27d; continue;
 }
 0x109d2eb => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x109d309); f_0108fc1c(cpu); if cpu.returned_to != 0x109d309 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x36u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x34u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x109d32b); f_0108fd07(cpu); if cpu.returned_to != 0x109d32b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x300u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109d34b); f_0108fe91(cpu); if cpu.returned_to != 0x109d34b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.eax)), 0x0u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109d361; continue; }
  pc = 0x109d358; continue;
 }
 0x109d358 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x109d3d5; continue;
 }
 0x109d361 => {
  { let v = 0x109cddcu32; cpu.push(v); }
  cpu.push(0x109d36b); f_01090372(cpu); if cpu.returned_to != 0x109d36b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x109d3ab; continue; }
  pc = 0x109d376; continue;
 }
 0x109d376 => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138ac4u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x36u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x109d397); f_0108fd07(cpu); if cpu.returned_to != 0x109d397 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.poll(); pc = 0x109d27d; continue;
 }
 0x109d3ab => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109d3b2); f_0109045d(cpu); if cpu.returned_to != 0x109d3b2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109d3c6); f_010905c6(cpu); if cpu.returned_to != 0x109d3c6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109d3d2); f_01090790(cpu); if cpu.returned_to != 0x109d3d2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109d3d5; continue;
 }
 0x109d3d5 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d3db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d3db); }
 let mut pc: u32 = 0x109d3db;
 loop { match pc {
 0x109d3db => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x109d3f2; continue; }
  pc = 0x109d3e8; continue;
 }
 0x109d3e8 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.push(0x109d3ef); cpu.call(t); if cpu.returned_to != 0x109d3ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109d3f2; continue;
 }
 0x109d3f2 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x109d403; continue; }
  pc = 0x109d3fa; continue;
 }
 0x109d3fa => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x109d400); f_0109092e(cpu); if cpu.returned_to != 0x109d400 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109d403; continue;
 }
 0x109d403 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x109d42c; continue; }
  pc = 0x109d409; continue;
 }
 0x109d409 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109d41a; continue; }
  pc = 0x109d411; continue;
 }
 0x109d411 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109d417); f_0108fe0a(cpu); if cpu.returned_to != 0x109d417 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109d41a; continue;
 }
 0x109d41a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109d429); f_0108fe91(cpu); if cpu.returned_to != 0x109d429 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x109d42c; continue;
 }
 0x109d42c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x36u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x109d43a); f_0108fd07(cpu); if cpu.returned_to != 0x109d43a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109d44d); f_01013d32(cpu); if cpu.returned_to != 0x109d44d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x2cu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109d458); f_01013c8c(cpu); if cpu.returned_to != 0x109d458 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x109d475; continue;
 }
 0x109d45f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x11ee158u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x109d474; continue; }
  pc = 0x109d46c; continue;
 }
 0x109d46c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11ee158u32.wrapping_add(cpu.eax), v); }
  pc = 0x109d474; continue;
 }
 0x109d474 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x109d475; continue;
 }
 0x109d475 => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109d45f; continue; }
  pc = 0x109d47a; continue;
 }
 0x109d47a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d47e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d47e); }
 let mut pc: u32 = 0x109d47e;
 loop { match pc {
 0x109d47e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  if cpu.zf { pc = 0x109d4ec; continue; }
  pc = 0x109d49c; continue;
 }
 0x109d49c => {
  cpu.push(0x109d4a1); f_0108cfa7(cpu); if cpu.returned_to != 0x109d4a1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109d4ec; continue; }
  pc = 0x109d4a5; continue;
 }
 0x109d4a5 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x3au32); cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x109d4b9); f_01098c3e(cpu); if cpu.returned_to != 0x109d4b9 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x303u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109d4c8); f_0108fe91(cpu); if cpu.returned_to != 0x109d4c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x109d4d5; continue; }
  pc = 0x109d4d0; continue;
 }
 0x109d4d0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109d4d5 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee300u32; cpu.edi = v; }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x11ee300u32; cpu.eax = v; }
  pc = 0x109d4ec; continue;
 }
 0x109d4ec => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d4ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d4ef); }
 let mut pc: u32 = 0x109d4ef;
 loop { match pc {
 0x109c564 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.interrupts_off = true; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x1134266u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r16(0x1134266u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x109c57b); cpu.push(0x109c580); f_0109cd2e(cpu); if cpu.returned_to != 0x109c580 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x113425eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1134262u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x113425au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1134268u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x113410cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x113414cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x113418cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x11341ccu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x200u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x113424cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1134250u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1134252u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x109c376u32; cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x2u32; cpu.w32(0x1134148u32.wrapping_add(cpu.bases[3]), v); }
  { let v = 0xd68du32; cpu.push(v); }
  { let v = 0x3cu32; cpu.push(v); }
  cpu.flat_ds(0x109c638); cpu.push(0x109c63d); f_01090550(cpu); if cpu.returned_to != 0x109c63d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  { cpu.interrupts_off = true; }
  if cpu.zf { pc = 0x109c64b; continue; }
  pc = 0x109c64a; continue;
 }
 0x109c64a => {
  { cpu.interrupts_off = false; }
  pc = 0x109c64b; continue;
 }
 0x109c64b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x109c651); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109d4ef => {
  cpu.flat_ds(0x109d4ef); cpu.push(0x109d4f4); f_0109cd53(cpu); if cpu.returned_to != 0x109d4f4 { return; }
  { let v = 0xc8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.flat_ds(0x109d4fb); cpu.push(0x109d500); f_0108fb18(cpu); if cpu.returned_to != 0x109d500 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.flat_ds(0x109d507); cpu.push(0x109d50c); f_0108fb18(cpu); if cpu.returned_to != 0x109d50c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8000u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.flat_ds(0x109d516); cpu.push(0x109d51b); f_0108fb18(cpu); if cpu.returned_to != 0x109d51b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.flat_ds(0x109d522); cpu.push(0x109d527); f_0108fb18(cpu); if cpu.returned_to != 0x109d527 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.flat_ds(0x109d52e); cpu.push(0x109d533); f_0108fb18(cpu); if cpu.returned_to != 0x109d533 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  cpu.flat_ds(0x109d53a); cpu.push(0x109d53f); f_0108fb18(cpu); if cpu.returned_to != 0x109d53f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x28fu32; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  cpu.flat_ds(0x109d549); cpu.push(0x109d54e); f_0108fb18(cpu); if cpu.returned_to != 0x109d54e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  cpu.flat_ds(0x109d555); cpu.push(0x109d55a); f_0108fb18(cpu); if cpu.returned_to != 0x109d55a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.flat_ds(0x109d561); cpu.push(0x109d566); f_0108fb18(cpu); if cpu.returned_to != 0x109d566 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  cpu.flat_ds(0x109d56d); cpu.push(0x109d572); f_0108fb18(cpu); if cpu.returned_to != 0x109d572 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  cpu.flat_ds(0x109d579); cpu.push(0x109d57e); f_0108fb18(cpu); if cpu.returned_to != 0x109d57e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x78u32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  cpu.flat_ds(0x109d585); cpu.push(0x109d58a); f_0108fb18(cpu); if cpu.returned_to != 0x109d58a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.flat_ds(0x109d591); cpu.push(0x109d596); f_0108fb18(cpu); if cpu.returned_to != 0x109d596 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0xdu32; cpu.push(v); }
  cpu.flat_ds(0x109d59d); cpu.push(0x109d5a2); f_0108fb18(cpu); if cpu.returned_to != 0x109d5a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xeu32; cpu.push(v); }
  cpu.flat_ds(0x109d5a9); cpu.push(0x109d5ae); f_0108fb18(cpu); if cpu.returned_to != 0x109d5ae { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  cpu.flat_ds(0x109d5b5); cpu.push(0x109d5ba); f_0108fb18(cpu); if cpu.returned_to != 0x109d5ba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.flat_ds(0x109d5c1); cpu.push(0x109d5c6); f_0108fb18(cpu); if cpu.returned_to != 0x109d5c6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x11u32; cpu.push(v); }
  cpu.flat_ds(0x109d5cd); cpu.push(0x109d5d2); f_0108fb18(cpu); if cpu.returned_to != 0x109d5d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12u32; cpu.push(v); }
  cpu.flat_ds(0x109d5d9); cpu.push(0x109d5de); f_0108fb18(cpu); if cpu.returned_to != 0x109d5de { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109d5ef; continue;
 }
 0x109d5e5 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11ee158u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x109d5ef; continue;
 }
 0x109d5ef => {
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109d5e5; continue; }
  pc = 0x109d5f4; continue;
 }
 0x109d5f4 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x11ee198u32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x109c564; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d601(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d601); }
 let mut pc: u32 = 0x109d601;
 loop { match pc {
 0x109d601 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xfu32; cpu.esi = v; }
  pc = 0x109d61e; continue;
 }
 0x109d609 => {
  { let v: u32 = cpu.r32(0x11ee158u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109d61d; continue; }
  pc = 0x109d614; continue;
 }
 0x109d614 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109d61a); f_01090bd5(cpu); if cpu.returned_to != 0x109d61a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109d61d; continue;
 }
 0x109d61d => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  pc = 0x109d61e; continue;
 }
 0x109d61e => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x109d609; continue; }
  pc = 0x109d622; continue;
 }
 0x109d622 => {
  cpu.push(0x109d627); f_0109099b(cpu); if cpu.returned_to != 0x109d627 { return; }
  cpu.push(0x109d62c); f_0109c652(cpu); if cpu.returned_to != 0x109d62c { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d62e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d62e); }
 let mut pc: u32 = 0x109d62e;
 loop { match pc {
 0x109d62e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134c80u32), 0x0u32); }
  if cpu.zf { pc = 0x109d6b1; continue; }
  pc = 0x109d63a; continue;
 }
 0x109d63a => {
  { let v = 0x109d62eu32; cpu.push(v); }
  { let v = 0x109cd53u32; cpu.push(v); }
  cpu.push(0x109d649); f_01013dc7(cpu); if cpu.returned_to != 0x109d649 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x11ee158u32; cpu.push(v); }
  cpu.push(0x109d658); f_01013e03(cpu); if cpu.returned_to != 0x109d658 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4cu32; cpu.push(v); }
  { let v = 0x11ee2b4u32; cpu.push(v); }
  cpu.push(0x109d667); f_01013e03(cpu); if cpu.returned_to != 0x109d667 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x11ee198u32; cpu.push(v); }
  cpu.push(0x109d679); f_01013e03(cpu); if cpu.returned_to != 0x109d679 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v = 0x11ee298u32; cpu.push(v); }
  cpu.push(0x109d688); f_01013e03(cpu); if cpu.returned_to != 0x109d688 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x1134c7cu32; cpu.push(v); }
  cpu.push(0x109d697); f_01013e03(cpu); if cpu.returned_to != 0x109d697 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee2b0u32; cpu.push(v); }
  cpu.push(0x109d6a6); f_01013e03(cpu); if cpu.returned_to != 0x109d6a6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1134c80u32, v); }
  pc = 0x109d6b1; continue;
 }
 0x109d6b1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d6b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d6b5); }
 let mut pc: u32 = 0x109d6b5;
 loop { match pc {
 0x109d6b5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x109d6c5; continue; }
  pc = 0x109d6c0; continue;
 }
 0x109d6c0 => {
  { let v: u32 = 0x400u32; cpu.ebx = v; }
  pc = 0x109d6c5; continue;
 }
 0x109d6c5 => {
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  cpu.push(0x109d6cf); f_010aa90c(cpu); if cpu.returned_to != 0x109d6cf { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d6d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d6d2); }
 let mut pc: u32 = 0x109d6d2;
 loop { match pc {
 0x109d6d2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x109d6e0); f_010aa973(cpu); if cpu.returned_to != 0x109d6e0 { return; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if (cpu.sf != cpu.of) { pc = 0x109d6ea; continue; }
  pc = 0x109d6e7; continue;
 }
 0x109d6e7 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x109d6ea; continue;
 }
 0x109d6ea => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x109d6f1); f_010aaa36(cpu); if cpu.returned_to != 0x109d6f1 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x109d6f9; continue; }
  pc = 0x109d6f7; continue;
 }
 0x109d6f7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x109d6f9; continue;
 }
 0x109d6f9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d701(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d701); }
 let mut pc: u32 = 0x109d701;
 loop { match pc {
 0x109d701 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  cpu.push(0x109d70e); f_010aabf6(cpu); if cpu.returned_to != 0x109d70e { return; }
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1134f1cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x109d724); f_010a5fca(cpu); if cpu.returned_to != 0x109d724 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x109d72b); f_010ab0ae(cpu); if cpu.returned_to != 0x109d72b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109d74b; continue; }
  pc = 0x109d72f; continue;
 }
 0x109d72f => {
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1134f1cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1134f20u32); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x109d74b); f_010a5fca(cpu); if cpu.returned_to != 0x109d74b { return; }
  pc = 0x109d74b; continue;
 }
 0x109d74b => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d752(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d752); }
 let mut pc: u32 = 0x109d752;
 loop { match pc {
 0x109d752 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x11ee318u32; cpu.edx = v; }
  cpu.push(0x109d75d); f_0109d701(cpu); if cpu.returned_to != 0x109d75d { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d75f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d75f); }
 let mut pc: u32 = 0x109d75f;
 loop { match pc {
 0x109d75f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d792(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d792); }
 let mut pc: u32 = 0x109d792;
 loop { match pc {
 0x109d792 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1139160u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1139167u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x113916eu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x20u8; cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x113913cu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1139148u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1139154u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x20u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x109d7ef); f_0109d75f(cpu); if cpu.returned_to != 0x109d7ef { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x109d7fd; continue; }
  pc = 0x109d7f9; continue;
 }
 0x109d7f9 => {
  { let v: u8 = 0x20u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x109d7fd; continue;
 }
 0x109d7fd => {
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x109d810); f_0109d75f(cpu); if cpu.returned_to != 0x109d810 { return; }
  { let v: u32 = 0xeu32; cpu.edx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x109d823); f_0109d75f(cpu); if cpu.returned_to != 0x109d823 { return; }
  { let v: u32 = 0x11u32; cpu.edx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x109d835); f_0109d75f(cpu); if cpu.returned_to != 0x109d835 { return; }
  { let v: u8 = 0x20u8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x14u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x13u32); cpu.eax = v; }
  cpu.push(0x109d85d); f_0109d75f(cpu); if cpu.returned_to != 0x109d85d { return; }
  { let v: u32 = 0x16u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x109d86d); f_0109d75f(cpu); if cpu.returned_to != 0x109d86d { return; }
  { let v: u8 = 0xau8; cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d89d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d89d); }
 let mut pc: u32 = 0x109d89d;
 loop { match pc {
 0x109d89d => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x11ee33cu32; cpu.edx = v; }
  cpu.push(0x109d8a8); f_0109d792(cpu); if cpu.returned_to != 0x109d8a8 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d8b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d8b0); }
 let mut pc: u32 = 0x109d8b0;
 loop { match pc {
 0x109d8b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1134d08u32), 0x0u32); }
  if !cpu.zf { pc = 0x109d96b; continue; }
  pc = 0x109d8c0; continue;
 }
 0x109d8c0 => {
  { let v = 0x109f3a0u32; cpu.push(v); }
  { let v = 0x109d8b0u32; cpu.push(v); }
  cpu.push(0x109d8cf); f_01013d5e(cpu); if cpu.returned_to != 0x109d8cf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x1134c84u32; cpu.push(v); }
  cpu.push(0x109d8e1); f_01013dec(cpu); if cpu.returned_to != 0x109d8e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x1134d04u32; cpu.push(v); }
  cpu.push(0x109d8f0); f_01013dec(cpu); if cpu.returned_to != 0x109d8f0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee36cu32; cpu.push(v); }
  cpu.push(0x109d8ff); f_01013dec(cpu); if cpu.returned_to != 0x109d8ff { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x11ee358u32; cpu.push(v); }
  cpu.push(0x109d90e); f_01013dec(cpu); if cpu.returned_to != 0x109d90e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee374u32; cpu.push(v); }
  cpu.push(0x109d91d); f_01013dec(cpu); if cpu.returned_to != 0x109d91d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee378u32; cpu.push(v); }
  cpu.push(0x109d92c); f_01013dec(cpu); if cpu.returned_to != 0x109d92c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee368u32; cpu.push(v); }
  cpu.push(0x109d93b); f_01013dec(cpu); if cpu.returned_to != 0x109d93b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee370u32; cpu.push(v); }
  cpu.push(0x109d94a); f_01013dec(cpu); if cpu.returned_to != 0x109d94a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ee364u32; cpu.push(v); }
  cpu.push(0x109d959); f_01013dec(cpu); if cpu.returned_to != 0x109d959 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x109d961); f_010ad4aa(cpu); if cpu.returned_to != 0x109d961 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x1134d08u32, v); }
  pc = 0x109d96b; continue;
 }
 0x109d96b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d970); }
 let mut pc: u32 = 0x109d970;
 loop { match pc {
 0x109d970 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x109d9ae; continue; }
  pc = 0x109d97f; continue;
 }
 0x109d97f => {
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x109d9a4); f_0108fe91(cpu); if cpu.returned_to != 0x109d9a4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x54u32.wrapping_add(cpu.esi), v); }
  pc = 0x109d9ae; continue;
 }
 0x109d9ae => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109d9f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109d9f0); }
 let mut pc: u32 = 0x109d9f0;
 loop { match pc {
 0x109d9f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109da0b; continue; }
  pc = 0x109da02; continue;
 }
 0x109da02 => {
  { let v: u32 = 0x7fu32; cpu.w32(0x40u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109da16; continue;
 }
 0x109da0b => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x109da16; continue; }
  pc = 0x109da0f; continue;
 }
 0x109da0f => {
  { let v: u32 = 0x0u32; cpu.w32(0x40u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109da16; continue;
 }
 0x109da16 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109da27; continue; }
  pc = 0x109da1e; continue;
 }
 0x109da1e => {
  { let v: u32 = 0x7fu32; cpu.w32(0x44u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109da32; continue;
 }
 0x109da27 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x109da32; continue; }
  pc = 0x109da2b; continue;
 }
 0x109da2b => {
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109da32; continue;
 }
 0x109da32 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x68u32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x109da60; continue; }
  pc = 0x109da5a; continue;
 }
 0x109da5a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x109da60; continue;
 }
 0x109da60 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109da6f; continue; }
  pc = 0x109da67; continue;
 }
 0x109da67 => {
  { let v: u32 = 0x7fu32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x109da6f; continue;
 }
 0x109da6f => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { pc = 0x109da7c; continue; }
  pc = 0x109da77; continue;
 }
 0x109da77 => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if !cpu.zf { pc = 0x109dac5; continue; }
  pc = 0x109da7c; continue;
 }
 0x109da7c => {
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1134c84u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1134c84u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x448u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109dcfb; continue;
 }
 0x109dac5 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x109dad4; continue; }
  pc = 0x109dacd; continue;
 }
 0x109dacd => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x109dad4; continue;
 }
 0x109dad4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x109db01; continue; }
  pc = 0x109dade; continue;
 }
 0x109dade => {
  { cpu.sub32(cpu.esi, 0x3u32); }
  if cpu.zf { pc = 0x109db01; continue; }
  pc = 0x109dae3; continue;
 }
 0x109dae3 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x109daf0; continue; }
  pc = 0x109dae7; continue;
 }
 0x109dae7 => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if !cpu.zf { pc = 0x109dc4c; continue; }
  pc = 0x109daf0; continue;
 }
 0x109daf0 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x109db01; continue; }
  pc = 0x109daf8; continue;
 }
 0x109daf8 => {
  { cpu.sub32(cpu.esi, 0x3u32); }
  if !cpu.zf { pc = 0x109dc4c; continue; }
  pc = 0x109db01; continue;
 }
 0x109db01 => {
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = 0x48u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x448u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1134c84u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x1134c84u32.wrapping_add(cpu.edi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x38u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x109dbe8; continue; }
  pc = 0x109db39; continue;
 }
 0x109db39 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, 0x0u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x109db50; continue;
 }
 0x109db50 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109db50; continue; }
  pc = 0x109db86; continue;
 }
 0x109db86 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 17u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0xffff8000u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 15u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x109dbb0; continue;
 }
 0x109dbb0 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x109dcfb; continue; }
  pc = 0x109dbe6; continue;
 }
 0x109dbe6 => {
  cpu.poll(); pc = 0x109dbb0; continue;
 }
 0x109dbe8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 17u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0xffff8000u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 15u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  {  }
  pc = 0x109dc10; continue;
 }
 0x109dc10 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x8000u32); }
  if (cpu.sf == cpu.of) { pc = 0x109dcfb; continue; }
  pc = 0x109dc4a; continue;
 }
 0x109dc4a => {
  cpu.poll(); pc = 0x109dc10; continue;
 }
 0x109dc4c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x48u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if cpu.zf { pc = 0x109dcc9; continue; }
  pc = 0x109dc62; continue;
 }
 0x109dc62 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x0u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x109dc70; continue;
 }
 0x109dc70 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x100u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109dc70; continue; }
  pc = 0x109dc8b; continue;
 }
 0x109dc8b => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0xffff8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 17u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 8u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ebx = v; }
  pc = 0x109dcb0; continue;
 }
 0x109dcb0 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x100u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x109dcfb; continue; }
  pc = 0x109dcc7; continue;
 }
 0x109dcc7 => {
  cpu.poll(); pc = 0x109dcb0; continue;
 }
 0x109dcc9 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 17u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0xffff8000u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.eax = v; }
  pc = 0x109dce0; continue;
 }
 0x109dce0 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x100u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109dce0; continue; }
  pc = 0x109dcfb; continue;
 }
 0x109dcfb => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109dd10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109dd10); }
 let mut pc: u32 = 0x109dd10;
 loop { match pc {
 0x109dd10 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ee368u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x1134d04u32), 0x0u32); }
  if !cpu.zf { pc = 0x109df0e; continue; }
  pc = 0x109dd26; continue;
 }
 0x109dd26 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee2dcu32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1134d04u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee36cu32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x109dd4e; continue; }
  pc = 0x109dd44; continue;
 }
 0x109dd44 => {
  cpu.push(0x109dd49); f_0108fe7e(cpu); if cpu.returned_to != 0x109dd49 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee374u32, v); }
  pc = 0x109dd4e; continue;
 }
 0x109dd4e => {
  { let v: u32 = cpu.r32(0x11ee36cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11ee368u32); cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x109dd8b; continue; }
  pc = 0x109dd60; continue;
 }
 0x109dd60 => {
  { cpu.sub32(cpu.r32(0x11ee2dcu32), 0x0u32); }
  if cpu.zf { pc = 0x109dd78; continue; }
  pc = 0x109dd69; continue;
 }
 0x109dd69 => {
  { let v: u32 = cpu.r32(0x11ee374u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x109dd75); f_0108fe83(cpu); if cpu.returned_to != 0x109dd75 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109dd78; continue;
 }
 0x109dd78 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11ee368u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1134d04u32, v); }
  pc = 0x109df0e; continue;
 }
 0x109dd8b => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11ee370u32, v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x109dda1; continue; }
  pc = 0x109dd9c; continue;
 }
 0x109dd9c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x38u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x109ddd8; continue; }
  pc = 0x109dda1; continue;
 }
 0x109dda1 => {
  { let v: u32 = cpu.r32(0x11ee2dcu32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x11ee368u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x109ddc0; continue; }
  pc = 0x109ddb1; continue;
 }
 0x109ddb1 => {
  { let v: u32 = cpu.r32(0x11ee374u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109ddbd); f_0108fe83(cpu); if cpu.returned_to != 0x109ddbd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109ddc0; continue;
 }
 0x109ddc0 => {
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x11ee368u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1134d04u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x11ee368u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ddd8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x109dde1); f_010ad47e(cpu); if cpu.returned_to != 0x109dde1 { return; }
  { let v: u32 = cpu.r32(0x11ee36cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11ee378u32, v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x11ee368u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee364u32, v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x109de43; continue; }
  pc = 0x109de06; continue;
 }
 0x109de06 => {
  { let v: u32 = cpu.r32(0x11ee364u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x109de20; continue; }
  pc = 0x109de11; continue;
 }
 0x109de11 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11ee378u32)); cpu.w32(0x11ee378u32, v); }
  cpu.push(0x109de1d); f_010ad1d0(cpu); if cpu.returned_to != 0x109de1d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109de20; continue;
 }
 0x109de20 => {
  { let v: u32 = cpu.r32(0x11ee368u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11ee364u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x894u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x11ee368u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x11ee364u32, v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x109de06; continue; }
  pc = 0x109de43; continue;
 }
 0x109de43 => {
  { let v: u32 = cpu.r32(0x11ee36cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee378u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x11ee370u32); cpu.esi = v; }
  { let v: u16 = (cpu.esi as u16) ^ 0x1u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109de62); f_010ad3eb(cpu); if cpu.returned_to != 0x109de62 { return; }
  { let v: u32 = cpu.r32(0x11ee36cu32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x109dea3; continue; }
  pc = 0x109de72; continue;
 }
 0x109de72 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = cpu.r16(0x11ee370u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v = 0x11ee358u32; cpu.push(v); }
  { let v: u32 = cpu.esi ^ 0x1u32; cpu.esi = v; }
  { let v = 0x405u32; cpu.push(v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x11ee35eu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11ee35cu32, v); }
  cpu.push(0x109dea0); f_0108fe91(cpu); if cpu.returned_to != 0x109dea0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x109dea3; continue;
 }
 0x109dea3 => {
  { let v: u32 = cpu.r32(0x11ee36cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x11ee368u32); cpu.edi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x109debf; continue; }
  pc = 0x109deb6; continue;
 }
 0x109deb6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x109dee8; continue;
 }
 0x109debf => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x109dee8; continue; }
  pc = 0x109decd; continue;
 }
 0x109decd => {
  { cpu.sub32(cpu.r32(0x54u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x109dee8; continue; }
  pc = 0x109ded3; continue;
 }
 0x109ded3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x109dee2); f_0108fe91(cpu); if cpu.returned_to != 0x109dee2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x54u32.wrapping_add(cpu.esi), v); }
  pc = 0x109dee8; continue;
 }
 0x109dee8 => {
  { cpu.sub32(cpu.r32(0x11ee2dcu32), 0x0u32); }
  if cpu.zf { pc = 0x109df00; continue; }
  pc = 0x109def1; continue;
 }
 0x109def1 => {
  { let v: u32 = cpu.r32(0x11ee374u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x109defd); f_0108fe83(cpu); if cpu.returned_to != 0x109defd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109df00; continue;
 }
 0x109df00 => {
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11ee368u32); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1134d04u32, v); }
  pc = 0x109df0e; continue;
 }
 0x109df0e => {
  { let v: u32 = cpu.edi; cpu.w32(0x11ee368u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109df18(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109df18); } r_0109df18(cpu, 0x109df18); }
pub fn f_0109df28(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109df28); } r_0109df18(cpu, 0x109df28); }
pub fn f_0109df40(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109df40); } r_0109df18(cpu, 0x109df40); }
fn r_0109df18(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x109df18 => {
  cpu.unimplemented(0x109df18, "Int1");
  cpu.unimplemented(0x109df19, "Fisttp");
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x109df28; continue; }
  pc = 0x109df1f; continue;
 }
 0x109df1f => {
  { let v: u32 = cpu.add32(cpu.r32(0x570109e0u32), cpu.esi); cpu.w32(0x570109e0u32, v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x109df30; continue; }
  pc = 0x109df27; continue;
 }
 0x109df27 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x109df34; continue; }
  pc = 0x109df2b; continue;
 }
 0x109df28 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 9u32); cpu.ecx = v; }
  pc = 0x109df2b; continue;
 }
 0x109df2b => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.esp); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x109df38; continue; }
  pc = 0x109df2f; continue;
 }
 0x109df2f => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x109df3c; continue; }
  pc = 0x109df33; continue;
 }
 0x109df30 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109df34; continue;
 }
 0x109df33 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.ebp); cpu.edx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x109df40; continue; }
  pc = 0x109df37; continue;
 }
 0x109df34 => {
  cpu.call(0x8d0109e1); return;
 }
 0x109df37 => {
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.ecx); cpu.w32(0x80u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x555756d2u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x555756d2u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109dfd1; continue; }
  pc = 0x109df59; continue;
 }
 0x109df38 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  pc = 0x109df40; continue;
 }
 0x109df3c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  pc = 0x109df40; continue;
 }
 0x109df40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109dfd1; continue; }
  pc = 0x109df59; continue;
 }
 0x109df59 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109df91; continue; }
  pc = 0x109df62; continue;
 }
 0x109df62 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109df64; continue;
 }
 0x109df64 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x890u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), 0x4u32); }
  if !cpu.zf { pc = 0x109df84; continue; }
  pc = 0x109df7d; continue;
 }
 0x109df7d => {
  { let v: u32 = 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109df84; continue;
 }
 0x109df84 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x894u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109df64; continue; }
  pc = 0x109df91; continue;
 }
 0x109df91 => {
  { cpu.sub32(cpu.r32(0x54u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x109dfb2; continue; }
  pc = 0x109df97; continue;
 }
 0x109df97 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x109dfa8); f_0108fe91(cpu); if cpu.returned_to != 0x109dfa8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.esi), v); }
  pc = 0x109dfb2; continue;
 }
 0x109dfb2 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109dfb8); f_010ad47e(cpu); if cpu.returned_to != 0x109dfb8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109dfc3); f_010ad3eb(cpu); if cpu.returned_to != 0x109dfc3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109dfce); f_010ad3eb(cpu); if cpu.returned_to != 0x109dfce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x109dfd1; continue;
 }
 0x109dfd1 => {
  { let v: u32 = cpu.r32(0x11ee2d0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee2d4u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebp; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x109e077; continue; }
  pc = 0x109dfe9; continue;
 }
 0x109dfe9 => {
  cpu.poll(); pc = cpu.r32(0x109df18u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x109dff1 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x109e077; continue;
 }
 0x109e013 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x109e077; continue;
 }
 0x109e035 => {
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x109e077; continue;
 }
 0x109e057 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x109e077; continue;
 }
 0x109e077 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109e07c; continue;
 }
 0x109e07c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))), 0x0u8); }
  if cpu.zf { pc = 0x109e08a; continue; }
  pc = 0x109e085; continue;
 }
 0x109e085 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x109e092; continue;
 }
 0x109e08a => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109e07c; continue; }
  pc = 0x109e092; continue;
 }
 0x109e092 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1au32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x11ee2b8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x109e0bb; continue; }
  pc = 0x109e0b2; continue;
 }
 0x109e0b2 => {
  if (cpu.cf || cpu.zf) { pc = 0x109e0dd; continue; }
  pc = 0x109e0b4; continue;
 }
 0x109e0b4 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x109e0fb; continue; }
  pc = 0x109e0b9; continue;
 }
 0x109e0b9 => {
  pc = 0x109e119; continue;
 }
 0x109e0bb => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109e119; continue; }
  pc = 0x109e0bf; continue;
 }
 0x109e0bf => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x109e1ae; continue;
 }
 0x109e0dd => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x109e1ae; continue;
 }
 0x109e0fb => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x109e1ae; continue;
 }
 0x109e119 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11ee2b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x7fffffffu32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x109e182; continue;
 }
 0x109e182 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1)))); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x24u32.wrapping_add(cpu.esp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109e1a2; continue; }
  pc = 0x109e192; continue;
 }
 0x109e192 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1)))); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x109e1a2; continue;
 }
 0x109e1a2 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x3u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109e182; continue; }
  pc = 0x109e1ab; continue;
 }
 0x109e1ab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  pc = 0x109e1ae; continue;
 }
 0x109e1ae => {
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x109e1f8; continue; }
  pc = 0x109e1b9; continue;
 }
 0x109e1b9 => {
  cpu.poll(); pc = cpu.r32(0x109df28u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x109e1c1 => {
  { let v: u32 = 0x1u32; cpu.w32(0x3cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x109e1f8; continue;
 }
 0x109e1d1 => {
  { let v: u32 = 0x2u32; cpu.w32(0x3cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x109e1f8; continue;
 }
 0x109e1e1 => {
  { let v: u32 = 0x1u32; cpu.w32(0x3cu32.wrapping_add(cpu.esi), v); }
  pc = 0x109e1f1; continue;
 }
 0x109e1ea => {
  { let v: u32 = 0x2u32; cpu.w32(0x3cu32.wrapping_add(cpu.esi), v); }
  pc = 0x109e1f1; continue;
 }
 0x109e1f1 => {
  { let v: u32 = 0x2u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x109e1f8; continue;
 }
 0x109e1f8 => {
  { let v: u32 = cpu.r32(0x11ee2c0u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x3e8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x11ee2fcu32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109e27b; continue; }
  pc = 0x109e249; continue;
 }
 0x109e249 => {
  { cpu.sub32(cpu.ebp, 0x800u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109e256; continue; }
  pc = 0x109e251; continue;
 }
 0x109e251 => {
  { let v: u32 = 0x800u32; cpu.ebp = v; }
  pc = 0x109e256; continue;
 }
 0x109e256 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x800u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109e27b; continue; }
  pc = 0x109e260; continue;
 }
 0x109e260 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138adcu32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  pc = 0x109e3b6; continue;
 }
 0x109e27b => {
  { let v: u32 = 0x7fffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x109e2a5; continue;
 }
 0x109e287 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109e2a3; continue; }
  pc = 0x109e295; continue;
 }
 0x109e295 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x109e2a3; continue;
 }
 0x109e2a3 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  pc = 0x109e2a5; continue;
 }
 0x109e2a5 => {
  { let v: u32 = cpu.r32(0x11ee2bcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x109e287; continue; }
  pc = 0x109e2b7; continue;
 }
 0x109e2b7 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x109e2c5; continue; }
  pc = 0x109e2c2; continue;
 }
 0x109e2c2 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x109e2c5; continue;
 }
 0x109e2c5 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x10u32.wrapping_add(cpu.esi))); }
  if (cpu.sf == cpu.of) { pc = 0x109e2cd; continue; }
  pc = 0x109e2ca; continue;
 }
 0x109e2ca => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x109e2cd; continue;
 }
 0x109e2cd => {
  { let v: u32 = cpu.r32(0x11ee2fcu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x109e2ea; continue; }
  pc = 0x109e2da; continue;
 }
 0x109e2da => {
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xff000u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 12u32); cpu.eax = v; }
  pc = 0x109e2ea; continue;
 }
 0x109e2ea => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109e3b1; continue; }
  pc = 0x109e336; continue;
 }
 0x109e336 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109e35c; continue; }
  pc = 0x109e33f; continue;
 }
 0x109e33f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109e341; continue;
 }
 0x109e341 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x890u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x894u32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109e341; continue; }
  pc = 0x109e35c; continue;
 }
 0x109e35c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109e362); f_010ad47e(cpu); if cpu.returned_to != 0x109e362 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109e36d); f_010ad3eb(cpu); if cpu.returned_to != 0x109e36d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109e378); f_010ad3eb(cpu); if cpu.returned_to != 0x109e378 { return; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x109e3b1; continue; }
  pc = 0x109e382; continue;
 }
 0x109e382 => {
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109e3a7); f_0108fe91(cpu); if cpu.returned_to != 0x109e3a7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x54u32.wrapping_add(cpu.esi), v); }
  pc = 0x109e3b1; continue;
 }
 0x109e3b1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109e3b6; continue;
 }
 0x109e3b6 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109e3c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109e3c0); }
 let mut pc: u32 = 0x109e3c0;
 loop { match pc {
 0x109e3c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x109e459; continue; }
  pc = 0x109e3e8; continue;
 }
 0x109e3e8 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x109e40a; continue; }
  pc = 0x109e403; continue;
 }
 0x109e403 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x109e40a; continue;
 }
 0x109e40a => {
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x109e41c; continue; }
  pc = 0x109e413; continue;
 }
 0x109e413 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  pc = 0x109e41c; continue;
 }
 0x109e41c => {
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x109e42e; continue; }
  pc = 0x109e425; continue;
 }
 0x109e425 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x109e42e; continue;
 }
 0x109e42e => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esp)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x109e440; continue; }
  pc = 0x109e437; continue;
 }
 0x109e437 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x109e440; continue;
 }
 0x109e440 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109e442; continue;
 }
 0x109e442 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x109e451; continue; }
  pc = 0x109e449; continue;
 }
 0x109e449 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x109e451; continue;
 }
 0x109e451 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x109e442; continue; }
  pc = 0x109e459; continue;
 }
 0x109e459 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11ee2d8u32); cpu.ebx = v; }
  { cpu.movs(4, true); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x109e490; continue; }
  pc = 0x109e475; continue;
 }
 0x109e475 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x109e4aa; continue;
 }
 0x109e490 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x109e4aa; continue;
 }
 0x109e4aa => {
  cpu.push(0x109e4af); f_0108fe91(cpu); if cpu.returned_to != 0x109e4af { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109e4c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109e4c0); }
 let mut pc: u32 = 0x109e4c0;
 loop { match pc {
 0x109e4c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x109e514; continue; }
  pc = 0x109e4ce; continue;
 }
 0x109e4ce => {
  if cpu.zf { pc = 0x109e4eb; continue; }
  pc = 0x109e4d0; continue;
 }
 0x109e4d0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x109e4e1); f_0108fe91(cpu); if cpu.returned_to != 0x109e4e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.esi), v); }
  pc = 0x109e4eb; continue;
 }
 0x109e4eb => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109e4f1); f_010ad47e(cpu); if cpu.returned_to != 0x109e4f1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109e4fc); f_010ad3eb(cpu); if cpu.returned_to != 0x109e4fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109e507); f_010ad3eb(cpu); if cpu.returned_to != 0x109e507 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xau32; cpu.push(v); }
  cpu.push(0x109e511); f_0108ffa7(cpu); if cpu.returned_to != 0x109e511 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109e514; continue;
 }
 0x109e514 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109e51d); f_0109092e(cpu); if cpu.returned_to != 0x109e51d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x109e531); f_01013d32(cpu); if cpu.returned_to != 0x109e531 { return; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x60u32.wrapping_add(cpu.esi)), 0x894u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x109e545); f_01013c8c(cpu); if cpu.returned_to != 0x109e545 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x109e555); f_01013c8c(cpu); if cpu.returned_to != 0x109e555 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8cu32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109e563); f_01013c8c(cpu); if cpu.returned_to != 0x109e563 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109e570(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109e570); } r_0109e570(cpu, 0x109e570); }
pub fn f_0109e800(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109e800); } r_0109e570(cpu, 0x109e800); }
pub fn f_0109e905(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109e905); } r_0109e570(cpu, 0x109e905); }
fn r_0109e570(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x109e570 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  cpu.flat_ds(0x109e576); cpu.push(0x109e57b); f_0109d8b0(cpu); if cpu.returned_to != 0x109e57b { return; }
  { let v = 0x8cu32; cpu.push(v); }
  cpu.flat_ds(0x109e580); cpu.push(0x109e585); f_01013c6b(cpu); if cpu.returned_to != 0x109e585 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109e5a6; continue; }
  pc = 0x109e58e; continue;
 }
 0x109e58e => {
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138b08u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  pc = 0x109eb7e; continue;
 }
 0x109e5a6 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x109e5e1; continue; }
  pc = 0x109e5b5; continue;
 }
 0x109e5b5 => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138b30u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v = 0x8cu32; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.flat_ds(0x109e5d0); cpu.push(0x109e5d5); f_01013c8c(cpu); if cpu.returned_to != 0x109e5d5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109e5e0); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109e5e1 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x301u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x109e5ed); cpu.push(0x109e5f2); f_0108fe91(cpu); if cpu.returned_to != 0x109e5f2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x38u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x64u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x7fu32; cpu.w32(0x68u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x34u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x11ee298u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  cpu.flat_ds(0x109e66b); cpu.push(0x109e670); f_0108f030(cpu); if cpu.returned_to != 0x109e670 { return; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x109e6aa; continue; }
  pc = 0x109e674; continue;
 }
 0x109e674 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee298u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x109e688); cpu.push(0x109e68d); f_0109e3c0(cpu); if cpu.returned_to != 0x109e68d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109e6aa; continue; }
  pc = 0x109e694; continue;
 }
 0x109e694 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x109e6aa; continue;
 }
 0x109e6aa => {
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x109e6f7; continue; }
  pc = 0x109e6b1; continue;
 }
 0x109e6b1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x109e6b5); cpu.push(0x109e6ba); f_010909fd(cpu); if cpu.returned_to != 0x109e6ba { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109e6f7; continue; }
  pc = 0x109e6c5; continue;
 }
 0x109e6c5 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11ee298u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.flat_ds(0x109e6d5); cpu.push(0x109e6da); f_0109e3c0(cpu); if cpu.returned_to != 0x109e6da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109e6f7; continue; }
  pc = 0x109e6e1; continue;
 }
 0x109e6e1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x109e6f7; continue;
 }
 0x109e6f7 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x109e7a3; continue; }
  pc = 0x109e703; continue;
 }
 0x109e703 => {
  { cpu.sub32(cpu.r32(0x11ee2f8u32.wrapping_add(cpu.bases[3])), 0x1u32); }
  if !cpu.zf { pc = 0x109e7a3; continue; }
  pc = 0x109e710; continue;
 }
 0x109e710 => {
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  pc = 0x109e78b; continue;
 }
 0x109e71a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x109e74d; continue; }
  pc = 0x109e73f; continue;
 }
 0x109e73f => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee298u32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x109e74d; continue;
 }
 0x109e74d => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x109e753); cpu.push(0x109e758); f_0109e3c0(cpu); if cpu.returned_to != 0x109e758 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109e777; continue; }
  pc = 0x109e75f; continue;
 }
 0x109e75f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x109e7a3; continue;
 }
 0x109e777 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x18u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  pc = 0x109e78b; continue;
 }
 0x109e78b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x109e71a; continue; }
  pc = 0x109e7a3; continue;
 }
 0x109e7a3 => {
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x109e7d4; continue; }
  pc = 0x109e7aa; continue;
 }
 0x109e7aa => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v = 0x8cu32; cpu.push(v); }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138b48u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  cpu.flat_ds(0x109e7c3); cpu.push(0x109e7c8); f_01013c8c(cpu); if cpu.returned_to != 0x109e7c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109e7d3); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109e7d4 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x11ee298u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11ee2fcu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.movs(4, true); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x109e7f3; continue; }
  pc = 0x109e7ec; continue;
 }
 0x109e7ec => {
  { let v: u32 = 0x2000u32; cpu.esi = v; }
  pc = 0x109e7f9; continue;
 }
 0x109e7f3 => {
  { let v: u32 = cpu.r32(0x11ee2bcu32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  pc = 0x109e7f9; continue;
 }
 0x109e7f9 => {
  { let v: u32 = 0xfu32.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x109e812; continue;
 }
 0x109e800 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x109e812; continue;
 }
 0x109e812 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.flat_ds(0x109e822); cpu.push(0x109e827); f_01013cad(cpu); if cpu.returned_to != 0x109e827 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109e856; continue; }
  pc = 0x109e82e; continue;
 }
 0x109e82e => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v = 0x8cu32; cpu.push(v); }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138b6cu32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.flat_ds(0x109e845); cpu.push(0x109e84a); f_01013c8c(cpu); if cpu.returned_to != 0x109e84a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109e855); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109e856 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x109e86e; continue; }
  pc = 0x109e85d; continue;
 }
 0x109e85d => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x109e866); cpu.push(0x109e86b); f_01013d32(cpu); if cpu.returned_to != 0x109e86b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x109e86e; continue;
 }
 0x109e86e => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xf0000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xf0000u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x109e812; continue; }
  pc = 0x109e8a2; continue;
 }
 0x109e8a2 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x109e8a3); cpu.push(0x109e8a8); f_0109df40(cpu); if cpu.returned_to != 0x109e8a8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109e8da; continue; }
  pc = 0x109e8af; continue;
 }
 0x109e8af => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.flat_ds(0x109e8bb); cpu.push(0x109e8c0); f_01013d32(cpu); if cpu.returned_to != 0x109e8c0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x8cu32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x109e8c9); cpu.push(0x109e8ce); f_01013c8c(cpu); if cpu.returned_to != 0x109e8ce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109e8d9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109e8da => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x305u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x109e8e7); cpu.push(0x109e8ec); f_0108fe91(cpu); if cpu.returned_to != 0x109e8ec { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x40u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  pc = 0x109e905; continue;
 }
 0x109e905 => {
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.edi; cpu.idiv32(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x44u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x109e928); cpu.push(0x109e92d); f_01013c6b(cpu); if cpu.returned_to != 0x109e92d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109e9a8; continue; }
  pc = 0x109e937; continue;
 }
 0x109e937 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138b8cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.movs(4, true); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x109e95e; continue; }
  pc = 0x109e955; continue;
 }
 0x109e955 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.flat_ds(0x109e956); cpu.push(0x109e95b); f_0108fe0a(cpu); if cpu.returned_to != 0x109e95b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109e95e; continue;
 }
 0x109e95e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.flat_ds(0x109e96b); cpu.push(0x109e970); f_0108fe91(cpu); if cpu.returned_to != 0x109e970 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x109e989); cpu.push(0x109e98e); f_01013d32(cpu); if cpu.returned_to != 0x109e98e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x8cu32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x109e997); cpu.push(0x109e99c); f_01013c8c(cpu); if cpu.returned_to != 0x109e99c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109e9a7); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109e9a8 => {
  { let v: u32 = cpu.r32(0x11ee2c4u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x894u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x109e9b7); cpu.push(0x109e9bc); f_01013c6b(cpu); if cpu.returned_to != 0x109e9bc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ea4c; continue; }
  pc = 0x109e9ca; continue;
 }
 0x109e9ca => {
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138bb0u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x109e9f2; continue; }
  pc = 0x109e9e9; continue;
 }
 0x109e9e9 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.flat_ds(0x109e9ea); cpu.push(0x109e9ef); f_0108fe0a(cpu); if cpu.returned_to != 0x109e9ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109e9f2; continue;
 }
 0x109e9f2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.flat_ds(0x109e9ff); cpu.push(0x109ea04); f_0108fe91(cpu); if cpu.returned_to != 0x109ea04 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.flat_ds(0x109ea1d); cpu.push(0x109ea22); f_01013d32(cpu); if cpu.returned_to != 0x109ea22 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x109ea2d); cpu.push(0x109ea32); f_01013c8c(cpu); if cpu.returned_to != 0x109ea32 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8cu32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x109ea3b); cpu.push(0x109ea40); f_01013c8c(cpu); if cpu.returned_to != 0x109ea40 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109ea4b); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ea4c => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109ea75; continue; }
  pc = 0x109ea55; continue;
 }
 0x109ea55 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109ea57; continue;
 }
 0x109ea57 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x894u32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109ea57; continue; }
  pc = 0x109ea75; continue;
 }
 0x109ea75 => {
  { let v = 0x109dd10u32; cpu.push(v); }
  cpu.flat_ds(0x109ea7a); cpu.push(0x109ea7f); f_01090372(cpu); if cpu.returned_to != 0x109ea7f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x109eb24; continue; }
  pc = 0x109ea8e; continue;
 }
 0x109ea8e => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138bd8u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x109eab6; continue; }
  pc = 0x109eaad; continue;
 }
 0x109eaad => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.flat_ds(0x109eaae); cpu.push(0x109eab3); f_0108fe0a(cpu); if cpu.returned_to != 0x109eab3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x109eab6; continue;
 }
 0x109eab6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.flat_ds(0x109eac3); cpu.push(0x109eac8); f_0108fe91(cpu); if cpu.returned_to != 0x109eac8 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.flat_ds(0x109eae1); cpu.push(0x109eae6); f_01013d32(cpu); if cpu.returned_to != 0x109eae6 { return; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x60u32.wrapping_add(cpu.ebp)), 0x894u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.flat_ds(0x109eaf5); cpu.push(0x109eafa); f_01013c8c(cpu); if cpu.returned_to != 0x109eafa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.flat_ds(0x109eb05); cpu.push(0x109eb0a); f_01013c8c(cpu); if cpu.returned_to != 0x109eb0a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8cu32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x109eb13); cpu.push(0x109eb18); f_01013c8c(cpu); if cpu.returned_to != 0x109eb18 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109eb23); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109eb24 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x109eb26); cpu.push(0x109eb2b); f_0109045d(cpu); if cpu.returned_to != 0x109eb2b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ee2b4u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x109eb39); cpu.push(0x109eb3e); f_010905c6(cpu); if cpu.returned_to != 0x109eb3e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.flat_ds(0x109eb45); cpu.push(0x109eb4a); f_01090790(cpu); if cpu.returned_to != 0x109eb4a { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x109e4c0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x109eb5e); cpu.push(0x109eb63); f_010ad47e(cpu); if cpu.returned_to != 0x109eb63 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x109eb69); cpu.push(0x109eb6e); f_010ad3eb(cpu); if cpu.returned_to != 0x109eb6e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x109eb74); cpu.push(0x109eb79); f_010ad3eb(cpu); if cpu.returned_to != 0x109eb79 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x109eb7e; continue;
 }
 0x109eb7e => {
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109eb84); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109eb90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109eb90); }
 let mut pc: u32 = 0x109eb90;
 loop { match pc {
 0x109eb90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109eb9e); f_01014423(cpu); if cpu.returned_to != 0x109eb9e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ebbe; continue; }
  pc = 0x109eba7; continue;
 }
 0x109eba7 => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138bf0u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ebbe => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109ebc4); f_010143d7(cpu); if cpu.returned_to != 0x109ebc4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109ebce); f_01090ae2(cpu); if cpu.returned_to != 0x109ebce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  let t = cpu.r32(0x110de9cu32); cpu.push(0x109ebdb); cpu.call(t); if cpu.returned_to != 0x109ebdb { return; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x109ebe4; continue; }
  pc = 0x109ebdf; continue;
 }
 0x109ebdf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ebe4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109ebef); f_0109e570(cpu); if cpu.returned_to != 0x109ebef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ec01; continue; }
  pc = 0x109ebf8; continue;
 }
 0x109ebf8 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x109ebfe); f_01090bd5(cpu); if cpu.returned_to != 0x109ebfe { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109ec01; continue;
 }
 0x109ec01 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ec10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ec10); }
 let mut pc: u32 = 0x109ec10;
 loop { match pc {
 0x109ec10 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x109ec21); f_01090ae2(cpu); if cpu.returned_to != 0x109ec21 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109ec49; continue; }
  pc = 0x109ec2a; continue;
 }
 0x109ec2a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109ec35); f_0109e570(cpu); if cpu.returned_to != 0x109ec35 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ec47; continue; }
  pc = 0x109ec3e; continue;
 }
 0x109ec3e => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109ec44); f_01090bd5(cpu); if cpu.returned_to != 0x109ec44 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x109ec47; continue;
 }
 0x109ec47 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x109ec49; continue;
 }
 0x109ec49 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ec50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ec50); }
 let mut pc: u32 = 0x109ec50;
 loop { match pc {
 0x109ec50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x118u32); cpu.esp = v; }
  { let v = 0x1138c08u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109ec67); f_01090019(cpu); if cpu.returned_to != 0x109ec67 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ec87; continue; }
  pc = 0x109ec6e; continue;
 }
 0x109ec6e => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138c10u32; cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.movs(4, true); }
  { cpu.movs(1, false); }
  pc = 0x109ecbc; continue;
 }
 0x109ec87 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109ec9c); f_01090d33(cpu); if cpu.returned_to != 0x109ec9c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109ecba; continue; }
  pc = 0x109ecac; continue;
 }
 0x109ecac => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ecba => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109ecbc; continue;
 }
 0x109ecbc => {
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ecd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ecd0); }
 let mut pc: u32 = 0x109ecd0;
 loop { match pc {
 0x109ecd0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x109ecdc); f_01090bd5(cpu); if cpu.returned_to != 0x109ecdc { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ed10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ed10); }
 let mut pc: u32 = 0x109ed10;
 loop { match pc {
 0x109ed10 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x109ed1b); f_0108fc12(cpu); if cpu.returned_to != 0x109ed1b { return; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x109ed3b; continue; }
  pc = 0x109ed24; continue;
 }
 0x109ed24 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x109ed27; continue;
 }
 0x109ed27 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x1u32); }
  if cpu.zf { pc = 0x109ed3b; continue; }
  pc = 0x109ed2d; continue;
 }
 0x109ed2d => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x894u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x109ed27; continue; }
  pc = 0x109ed3b; continue;
 }
 0x109ed3b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x60u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x109ed5e; continue; }
  pc = 0x109ed40; continue;
 }
 0x109ed40 => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x11ee198u32; cpu.edi = v; }
  { let v: u32 = 0x1138c30u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  cpu.push(0x109ed59); f_0108fc17(cpu); if cpu.returned_to != 0x109ed59 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109ed5e => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x894u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x109ed6f); f_010910de(cpu); if cpu.returned_to != 0x109ed6f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x109ed77); f_0108fc17(cpu); if cpu.returned_to != 0x109ed77 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109ed80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109ed80); }
 let mut pc: u32 = 0x109ed80;
 loop { match pc {
 0x109ed80 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109ed8f; continue; }
  pc = 0x109ed88; continue;
 }
 0x109ed88 => {
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x109ed8f; continue;
 }
 0x109ed8f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
