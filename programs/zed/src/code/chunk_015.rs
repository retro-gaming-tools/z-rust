#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_01097f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f40); }
 let mut pc: u32 = 0x1097f40;
 loop { match pc {
 0x1097f40 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f44); }
 let mut pc: u32 = 0x1097f44;
 loop { match pc {
 0x1097f44 => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f48); }
 let mut pc: u32 = 0x1097f48;
 loop { match pc {
 0x1097f48 => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f4c); }
 let mut pc: u32 = 0x1097f4c;
 loop { match pc {
 0x1097f4c => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f50); }
 let mut pc: u32 = 0x1097f50;
 loop { match pc {
 0x1097f50 => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f54); }
 let mut pc: u32 = 0x1097f54;
 loop { match pc {
 0x1097f54 => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f58); }
 let mut pc: u32 = 0x1097f58;
 loop { match pc {
 0x1097f58 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f5c); }
 let mut pc: u32 = 0x1097f5c;
 loop { match pc {
 0x1097f5c => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f60); }
 let mut pc: u32 = 0x1097f60;
 loop { match pc {
 0x1097f60 => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f64); }
 let mut pc: u32 = 0x1097f64;
 loop { match pc {
 0x1097f64 => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f68); }
 let mut pc: u32 = 0x1097f68;
 loop { match pc {
 0x1097f68 => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f6c); }
 let mut pc: u32 = 0x1097f6c;
 loop { match pc {
 0x1097f6c => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f70); }
 let mut pc: u32 = 0x1097f70;
 loop { match pc {
 0x1097f70 => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f74); }
 let mut pc: u32 = 0x1097f74;
 loop { match pc {
 0x1097f74 => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f78); }
 let mut pc: u32 = 0x1097f78;
 loop { match pc {
 0x1097f78 => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f7c); }
 let mut pc: u32 = 0x1097f7c;
 loop { match pc {
 0x1097f7c => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f80); }
 let mut pc: u32 = 0x1097f80;
 loop { match pc {
 0x1097f80 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f84); }
 let mut pc: u32 = 0x1097f84;
 loop { match pc {
 0x1097f84 => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f88); }
 let mut pc: u32 = 0x1097f88;
 loop { match pc {
 0x1097f88 => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f8c); }
 let mut pc: u32 = 0x1097f8c;
 loop { match pc {
 0x1097f8c => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f90); }
 let mut pc: u32 = 0x1097f90;
 loop { match pc {
 0x1097f90 => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f94); }
 let mut pc: u32 = 0x1097f94;
 loop { match pc {
 0x1097f94 => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f98); }
 let mut pc: u32 = 0x1097f98;
 loop { match pc {
 0x1097f98 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097f9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097f9c); }
 let mut pc: u32 = 0x1097f9c;
 loop { match pc {
 0x1097f9c => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fa0); }
 let mut pc: u32 = 0x1097fa0;
 loop { match pc {
 0x1097fa0 => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fa4); }
 let mut pc: u32 = 0x1097fa4;
 loop { match pc {
 0x1097fa4 => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fa8); }
 let mut pc: u32 = 0x1097fa8;
 loop { match pc {
 0x1097fa8 => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fac); }
 let mut pc: u32 = 0x1097fac;
 loop { match pc {
 0x1097fac => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fb0); }
 let mut pc: u32 = 0x1097fb0;
 loop { match pc {
 0x1097fb0 => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fb4); }
 let mut pc: u32 = 0x1097fb4;
 loop { match pc {
 0x1097fb4 => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fb8); }
 let mut pc: u32 = 0x1097fb8;
 loop { match pc {
 0x1097fb8 => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fbc); }
 let mut pc: u32 = 0x1097fbc;
 loop { match pc {
 0x1097fbc => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fc0); }
 let mut pc: u32 = 0x1097fc0;
 loop { match pc {
 0x1097fc0 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fc4); }
 let mut pc: u32 = 0x1097fc4;
 loop { match pc {
 0x1097fc4 => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fc8); }
 let mut pc: u32 = 0x1097fc8;
 loop { match pc {
 0x1097fc8 => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fcc); }
 let mut pc: u32 = 0x1097fcc;
 loop { match pc {
 0x1097fcc => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fd0); }
 let mut pc: u32 = 0x1097fd0;
 loop { match pc {
 0x1097fd0 => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fd4); }
 let mut pc: u32 = 0x1097fd4;
 loop { match pc {
 0x1097fd4 => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fd8); }
 let mut pc: u32 = 0x1097fd8;
 loop { match pc {
 0x1097fd8 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fdc); }
 let mut pc: u32 = 0x1097fdc;
 loop { match pc {
 0x1097fdc => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fe0); }
 let mut pc: u32 = 0x1097fe0;
 loop { match pc {
 0x1097fe0 => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fe4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fe4); }
 let mut pc: u32 = 0x1097fe4;
 loop { match pc {
 0x1097fe4 => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fe8); }
 let mut pc: u32 = 0x1097fe8;
 loop { match pc {
 0x1097fe8 => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097fec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097fec); }
 let mut pc: u32 = 0x1097fec;
 loop { match pc {
 0x1097fec => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ff0); }
 let mut pc: u32 = 0x1097ff0;
 loop { match pc {
 0x1097ff0 => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ff4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ff4); }
 let mut pc: u32 = 0x1097ff4;
 loop { match pc {
 0x1097ff4 => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ff8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ff8); }
 let mut pc: u32 = 0x1097ff8;
 loop { match pc {
 0x1097ff8 => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01097ffc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1097ffc); }
 let mut pc: u32 = 0x1097ffc;
 loop { match pc {
 0x1097ffc => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098000); }
 let mut pc: u32 = 0x1098000;
 loop { match pc {
 0x1098000 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098004(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098004); }
 let mut pc: u32 = 0x1098004;
 loop { match pc {
 0x1098004 => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098008(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098008); }
 let mut pc: u32 = 0x1098008;
 loop { match pc {
 0x1098008 => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109800c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109800c); }
 let mut pc: u32 = 0x109800c;
 loop { match pc {
 0x109800c => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098010); }
 let mut pc: u32 = 0x1098010;
 loop { match pc {
 0x1098010 => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098014(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098014); }
 let mut pc: u32 = 0x1098014;
 loop { match pc {
 0x1098014 => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098018(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098018); }
 let mut pc: u32 = 0x1098018;
 loop { match pc {
 0x1098018 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109801c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109801c); }
 let mut pc: u32 = 0x109801c;
 loop { match pc {
 0x109801c => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098020); }
 let mut pc: u32 = 0x1098020;
 loop { match pc {
 0x1098020 => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098024(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098024); }
 let mut pc: u32 = 0x1098024;
 loop { match pc {
 0x1098024 => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098028(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098028); }
 let mut pc: u32 = 0x1098028;
 loop { match pc {
 0x1098028 => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109802c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109802c); }
 let mut pc: u32 = 0x109802c;
 loop { match pc {
 0x109802c => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098030); }
 let mut pc: u32 = 0x1098030;
 loop { match pc {
 0x1098030 => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098034(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098034); }
 let mut pc: u32 = 0x1098034;
 loop { match pc {
 0x1098034 => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098038(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098038); }
 let mut pc: u32 = 0x1098038;
 loop { match pc {
 0x1098038 => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109803c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109803c); }
 let mut pc: u32 = 0x109803c;
 loop { match pc {
 0x109803c => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098040); }
 let mut pc: u32 = 0x1098040;
 loop { match pc {
 0x1098040 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098044(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098044); }
 let mut pc: u32 = 0x1098044;
 loop { match pc {
 0x1098044 => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098048(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098048); }
 let mut pc: u32 = 0x1098048;
 loop { match pc {
 0x1098048 => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109804c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109804c); }
 let mut pc: u32 = 0x109804c;
 loop { match pc {
 0x109804c => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098050); }
 let mut pc: u32 = 0x1098050;
 loop { match pc {
 0x1098050 => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098054(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098054); }
 let mut pc: u32 = 0x1098054;
 loop { match pc {
 0x1098054 => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098058(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098058); }
 let mut pc: u32 = 0x1098058;
 loop { match pc {
 0x1098058 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109805c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109805c); }
 let mut pc: u32 = 0x109805c;
 loop { match pc {
 0x109805c => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098060); }
 let mut pc: u32 = 0x1098060;
 loop { match pc {
 0x1098060 => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098064(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098064); }
 let mut pc: u32 = 0x1098064;
 loop { match pc {
 0x1098064 => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098068(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098068); }
 let mut pc: u32 = 0x1098068;
 loop { match pc {
 0x1098068 => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109806c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109806c); }
 let mut pc: u32 = 0x109806c;
 loop { match pc {
 0x109806c => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098070); }
 let mut pc: u32 = 0x1098070;
 loop { match pc {
 0x1098070 => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098074(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098074); }
 let mut pc: u32 = 0x1098074;
 loop { match pc {
 0x1098074 => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098078(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098078); }
 let mut pc: u32 = 0x1098078;
 loop { match pc {
 0x1098078 => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109807c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109807c); }
 let mut pc: u32 = 0x109807c;
 loop { match pc {
 0x109807c => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098080); }
 let mut pc: u32 = 0x1098080;
 loop { match pc {
 0x1098080 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098084(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098084); }
 let mut pc: u32 = 0x1098084;
 loop { match pc {
 0x1098084 => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098088(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098088); }
 let mut pc: u32 = 0x1098088;
 loop { match pc {
 0x1098088 => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109808c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109808c); }
 let mut pc: u32 = 0x109808c;
 loop { match pc {
 0x109808c => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098090); }
 let mut pc: u32 = 0x1098090;
 loop { match pc {
 0x1098090 => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098094(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098094); }
 let mut pc: u32 = 0x1098094;
 loop { match pc {
 0x1098094 => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098098(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098098); }
 let mut pc: u32 = 0x1098098;
 loop { match pc {
 0x1098098 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109809c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109809c); }
 let mut pc: u32 = 0x109809c;
 loop { match pc {
 0x109809c => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980a0); }
 let mut pc: u32 = 0x10980a0;
 loop { match pc {
 0x10980a0 => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980a4); }
 let mut pc: u32 = 0x10980a4;
 loop { match pc {
 0x10980a4 => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980a8); }
 let mut pc: u32 = 0x10980a8;
 loop { match pc {
 0x10980a8 => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980ac); }
 let mut pc: u32 = 0x10980ac;
 loop { match pc {
 0x10980ac => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980b0); }
 let mut pc: u32 = 0x10980b0;
 loop { match pc {
 0x10980b0 => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980b4); }
 let mut pc: u32 = 0x10980b4;
 loop { match pc {
 0x10980b4 => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980b8); }
 let mut pc: u32 = 0x10980b8;
 loop { match pc {
 0x10980b8 => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980bc); }
 let mut pc: u32 = 0x10980bc;
 loop { match pc {
 0x10980bc => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980c0); }
 let mut pc: u32 = 0x10980c0;
 loop { match pc {
 0x10980c0 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980c4); }
 let mut pc: u32 = 0x10980c4;
 loop { match pc {
 0x10980c4 => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980c8); }
 let mut pc: u32 = 0x10980c8;
 loop { match pc {
 0x10980c8 => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980cc); }
 let mut pc: u32 = 0x10980cc;
 loop { match pc {
 0x10980cc => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980d0); }
 let mut pc: u32 = 0x10980d0;
 loop { match pc {
 0x10980d0 => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980d4); }
 let mut pc: u32 = 0x10980d4;
 loop { match pc {
 0x10980d4 => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980d8); }
 let mut pc: u32 = 0x10980d8;
 loop { match pc {
 0x10980d8 => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980dc); }
 let mut pc: u32 = 0x10980dc;
 loop { match pc {
 0x10980dc => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980e0); }
 let mut pc: u32 = 0x10980e0;
 loop { match pc {
 0x10980e0 => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980e4); }
 let mut pc: u32 = 0x10980e4;
 loop { match pc {
 0x10980e4 => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980e8); }
 let mut pc: u32 = 0x10980e8;
 loop { match pc {
 0x10980e8 => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980ec); }
 let mut pc: u32 = 0x10980ec;
 loop { match pc {
 0x10980ec => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980f0); }
 let mut pc: u32 = 0x10980f0;
 loop { match pc {
 0x10980f0 => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980f4); }
 let mut pc: u32 = 0x10980f4;
 loop { match pc {
 0x10980f4 => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980f8); }
 let mut pc: u32 = 0x10980f8;
 loop { match pc {
 0x10980f8 => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010980fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10980fc); }
 let mut pc: u32 = 0x10980fc;
 loop { match pc {
 0x10980fc => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098100); }
 let mut pc: u32 = 0x1098100;
 loop { match pc {
 0x1098100 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098104(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098104); }
 let mut pc: u32 = 0x1098104;
 loop { match pc {
 0x1098104 => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098108(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098108); }
 let mut pc: u32 = 0x1098108;
 loop { match pc {
 0x1098108 => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109810c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109810c); }
 let mut pc: u32 = 0x109810c;
 loop { match pc {
 0x109810c => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098110); }
 let mut pc: u32 = 0x1098110;
 loop { match pc {
 0x1098110 => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098114(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098114); }
 let mut pc: u32 = 0x1098114;
 loop { match pc {
 0x1098114 => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098118(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098118); }
 let mut pc: u32 = 0x1098118;
 loop { match pc {
 0x1098118 => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109811c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109811c); }
 let mut pc: u32 = 0x109811c;
 loop { match pc {
 0x109811c => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098120); }
 let mut pc: u32 = 0x1098120;
 loop { match pc {
 0x1098120 => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098124(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098124); }
 let mut pc: u32 = 0x1098124;
 loop { match pc {
 0x1098124 => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098128(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098128); }
 let mut pc: u32 = 0x1098128;
 loop { match pc {
 0x1098128 => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109812c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109812c); }
 let mut pc: u32 = 0x109812c;
 loop { match pc {
 0x109812c => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098130); }
 let mut pc: u32 = 0x1098130;
 loop { match pc {
 0x1098130 => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098134(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098134); }
 let mut pc: u32 = 0x1098134;
 loop { match pc {
 0x1098134 => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098138(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098138); }
 let mut pc: u32 = 0x1098138;
 loop { match pc {
 0x1098138 => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109813c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109813c); }
 let mut pc: u32 = 0x109813c;
 loop { match pc {
 0x109813c => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098140); }
 let mut pc: u32 = 0x1098140;
 loop { match pc {
 0x1098140 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098144(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098144); }
 let mut pc: u32 = 0x1098144;
 loop { match pc {
 0x1098144 => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098148(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098148); }
 let mut pc: u32 = 0x1098148;
 loop { match pc {
 0x1098148 => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109814c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109814c); }
 let mut pc: u32 = 0x109814c;
 loop { match pc {
 0x109814c => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098150); }
 let mut pc: u32 = 0x1098150;
 loop { match pc {
 0x1098150 => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098154(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098154); }
 let mut pc: u32 = 0x1098154;
 loop { match pc {
 0x1098154 => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098158(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098158); }
 let mut pc: u32 = 0x1098158;
 loop { match pc {
 0x1098158 => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109815c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109815c); }
 let mut pc: u32 = 0x109815c;
 loop { match pc {
 0x109815c => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098160); }
 let mut pc: u32 = 0x1098160;
 loop { match pc {
 0x1098160 => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098164(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098164); }
 let mut pc: u32 = 0x1098164;
 loop { match pc {
 0x1098164 => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098168(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098168); }
 let mut pc: u32 = 0x1098168;
 loop { match pc {
 0x1098168 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1097d68u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109817e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109817e); }
 let mut pc: u32 = 0x109817e;
 loop { match pc {
 0x109817e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.set_segment(4, v as u16); }
  { let v = cpu.r16(0xau32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.cf = false; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x10981bc); cpu.push(0x10981c1); f_01098168(cpu); if cpu.returned_to != 0x10981c1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.pop(); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0x1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[4] as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0xau32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x109820f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098210); }
 let mut pc: u32 = 0x1098210;
 loop { match pc {
 0x1098210 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x1098220); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098221(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098221); }
 let mut pc: u32 = 0x1098221;
 loop { match pc {
 0x1098221 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x1098232); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098233(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098233); }
 let mut pc: u32 = 0x1098233;
 loop { match pc {
 0x1098233 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x1098243); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098244(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098244); }
 let mut pc: u32 = 0x1098244;
 loop { match pc {
 0x1098244 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x1098257); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098258(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098258); }
 let mut pc: u32 = 0x1098258;
 loop { match pc {
 0x1098258 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x109826d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109826e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109826e); }
 let mut pc: u32 = 0x109826e;
 loop { match pc {
 0x109826e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x1098281); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098282(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098282); }
 let mut pc: u32 = 0x1098282;
 loop { match pc {
 0x1098282 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x10982ac); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010982ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10982ad); }
 let mut pc: u32 = 0x10982ad;
 loop { match pc {
 0x10982ad => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
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
pub fn f_010982d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10982d8); }
 let mut pc: u32 = 0x10982d8;
 loop { match pc {
 0x10982d8 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x1133eb4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010982df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10982df); }
 let mut pc: u32 = 0x10982df;
 loop { match pc {
 0x10982df => {
  { let v = cpu.r16(0x1133eb4u32); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x10982e6); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010982f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10982f8); }
 let mut pc: u32 = 0x10982f8;
 loop { match pc {
 0x10982f8 => {
  { cpu.sub32(cpu.r32(0x1133ebcu32), 0x0u32); }
  if !cpu.zf { pc = 0x1098318; continue; }
  pc = 0x1098301; continue;
 }
 0x1098301 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x400u32; cpu.push(v); }
  cpu.push(0x1098310); f_01098b68(cpu); if cpu.returned_to != 0x1098310 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133ebcu32, v); }
  pc = 0x1098318; continue;
 }
 0x1098318 => {
  { let v: u32 = cpu.r32(0x1133ebcu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098320); }
 let mut pc: u32 = 0x1098320;
 loop { match pc {
 0x1098320 => {
  { let v = 0x49u32; cpu.push(v); }
  cpu.push(0x1098327); f_010982f8(cpu); if cpu.returned_to != 0x1098327 { return; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109832d); f_01098210(cpu); if cpu.returned_to != 0x109832d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.cf { pc = 0x109838c; continue; }
  pc = 0x1098334; continue;
 }
 0x1098334 => {
  if (cpu.cf || cpu.zf) { pc = 0x109833c; continue; }
  pc = 0x1098336; continue;
 }
 0x1098336 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x1098364; continue; }
  pc = 0x109833a; continue;
 }
 0x109833a => {
  pc = 0x109838c; continue;
 }
 0x109833c => {
  { cpu.sub32(cpu.r32(0x1133ec0u32), 0x0u32); }
  if !cpu.zf { pc = 0x109835c; continue; }
  pc = 0x1098345; continue;
 }
 0x1098345 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0xb8000u32; cpu.push(v); }
  cpu.push(0x1098354); f_01098b68(cpu); if cpu.returned_to != 0x1098354 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133ec0u32, v); }
  pc = 0x109835c; continue;
 }
 0x109835c => {
  { let v: u32 = cpu.r32(0x1133ec0u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1098364 => {
  { cpu.sub32(cpu.r32(0x1133ec4u32), 0x0u32); }
  if !cpu.zf { pc = 0x1098384; continue; }
  pc = 0x109836d; continue;
 }
 0x109836d => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0xb0000u32; cpu.push(v); }
  cpu.push(0x109837c); f_01098b68(cpu); if cpu.returned_to != 0x109837c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133ec4u32, v); }
  pc = 0x1098384; continue;
 }
 0x1098384 => {
  { let v: u32 = cpu.r32(0x1133ec4u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109838c => {
  { cpu.sub32(cpu.r32(0x1133ec8u32), 0x0u32); }
  if !cpu.zf { pc = 0x10983ac; continue; }
  pc = 0x1098395; continue;
 }
 0x1098395 => {
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0xa0000u32; cpu.push(v); }
  cpu.push(0x10983a4); f_01098b68(cpu); if cpu.returned_to != 0x10983a4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133ec8u32, v); }
  pc = 0x10983ac; continue;
 }
 0x10983ac => {
  { let v: u32 = cpu.r32(0x1133ec8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010983b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10983b4); }
 let mut pc: u32 = 0x10983b4;
 loop { match pc {
 0x10983b4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10983c0); f_01097d44(cpu); if cpu.returned_to != 0x10983c0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10983da); f_0109817e(cpu); if cpu.returned_to != 0x10983da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010983e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10983e4); }
 let mut pc: u32 = 0x10983e4;
 loop { match pc {
 0x10983e4 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  cpu.push(0x10983ec); f_010982d8(cpu); if cpu.returned_to != 0x10983ec { return; }
  { let v: u8 = 0x35u8; cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x21u32; cpu.push(v); }
  cpu.push(0x109840a); f_01098a48(cpu); if cpu.returned_to != 0x109840a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx | cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109842c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109842c); }
 let mut pc: u32 = 0x109842c;
 loop { match pc {
 0x109842c => {
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  cpu.push(0x1098434); f_010982d8(cpu); if cpu.returned_to != 0x1098434 { return; }
  { let v: u8 = 0x25u8; cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x21u32; cpu.push(v); }
  cpu.push(0x1098467); f_01098a48(cpu); if cpu.returned_to != 0x1098467 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098470(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098470); }
 let mut pc: u32 = 0x1098470;
 loop { match pc {
 0x1098470 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u32 = 0x800u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x1000u32); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xf0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x20u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10984c2); f_010983b4(cpu); if cpu.returned_to != 0x10984c2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  if cpu.zf { pc = 0x10984d1; continue; }
  pc = 0x10984cd; continue;
 }
 0x10984cd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10984e6; continue;
 }
 0x10984d1 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  pc = 0x10984e6; continue;
 }
 0x10984e6 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010984ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10984ec); }
 let mut pc: u32 = 0x10984ec;
 loop { match pc {
 0x10984ec => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x109851a); f_010983b4(cpu); if cpu.returned_to != 0x109851a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098534(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098534); }
 let mut pc: u32 = 0x1098534;
 loop { match pc {
 0x1098534 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.edx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1098553); f_010983b4(cpu); if cpu.returned_to != 0x1098553 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098570); }
 let mut pc: u32 = 0x1098570;
 loop { match pc {
 0x1098570 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x109859e); f_010983b4(cpu); if cpu.returned_to != 0x109859e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098620); }
 let mut pc: u32 = 0x1098620;
 loop { match pc {
 0x1098620 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = 0x600u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1098657); f_010983b4(cpu); if cpu.returned_to != 0x1098657 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010986c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10986c0); }
 let mut pc: u32 = 0x10986c0;
 loop { match pc {
 0x10986c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1133ed8u32), 0x0u32); }
  if !cpu.zf { pc = 0x109870b; continue; }
  pc = 0x10986d0; continue;
 }
 0x10986d0 => {
  { let v = 0x11edd44u32; cpu.push(v); }
  { let v = 0x11edd48u32; cpu.push(v); }
  { let v = 0x11edd40u32; cpu.push(v); }
  { let v = 0x1133ed8u32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  cpu.push(0x10986eb); f_010988c4(cpu); if cpu.returned_to != 0x10986eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0xbu32; cpu.push(v); }
  { let v = 0x1133eccu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edd40u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133ed8u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1098708); f_010982ad(cpu); if cpu.returned_to != 0x1098708 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x109870b; continue;
 }
 0x109870b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edd40u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133ed8u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1098726); f_01098258(cpu); if cpu.returned_to != 0x1098726 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11edd40u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133ed8u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1098746); f_01098258(cpu); if cpu.returned_to != 0x1098746 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x1a8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109875f); f_01098888(cpu); if cpu.returned_to != 0x109875f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1098770); f_01098221(cpu); if cpu.returned_to != 0x1098770 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1098786); f_01098221(cpu); if cpu.returned_to != 0x1098786 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x11edd44u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10987ab); f_01098258(cpu); if cpu.returned_to != 0x10987ab { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0x11edd48u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10987c9); f_01098258(cpu); if cpu.returned_to != 0x10987c9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x6au32; cpu.push(v); }
  cpu.push(0x10987de); f_01098a48(cpu); if cpu.returned_to != 0x10987de { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10987f6); f_01098258(cpu); if cpu.returned_to != 0x10987f6 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1098811); f_01098258(cpu); if cpu.returned_to != 0x1098811 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109881c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109881c); }
 let mut pc: u32 = 0x109881c;
 loop { match pc {
 0x109881c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x109882d); f_01097d44(cpu); if cpu.returned_to != 0x109882d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x109883c); f_01098534(cpu); if cpu.returned_to != 0x109883c { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.ebx, 0x100000u32); }
  if !cpu.cf { pc = 0x109884e; continue; }
  pc = 0x1098846; continue;
 }
 0x1098846 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109884e; continue; }
  pc = 0x109884a; continue;
 }
 0x109884a => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x1098882; continue;
 }
 0x109884e => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x1098859); f_01098470(cpu); if cpu.returned_to != 0x1098859 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1098869; continue; }
  pc = 0x109885f; continue;
 }
 0x109885f => {
  { cpu.sub32(cpu.ebx, 0x100000u32); }
  if !cpu.cf { pc = 0x1098882; continue; }
  pc = 0x1098867; continue;
 }
 0x1098867 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x1098869; continue;
 }
 0x1098869 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x109887a); f_01098570(cpu); if cpu.returned_to != 0x109887a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1098882; continue; }
  pc = 0x109887e; continue;
 }
 0x109887e => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  pc = 0x1098882; continue;
 }
 0x1098882 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098888(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098888); }
 let mut pc: u32 = 0x1098888;
 loop { match pc {
 0x1098888 => {
  { let v: u32 = cpu.r32(0x1133edcu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10988a5; continue; }
  pc = 0x1098892; continue;
 }
 0x1098892 => {
  { let v = 0xfffffu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x109889d); f_01098b68(cpu); if cpu.returned_to != 0x109889d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133edcu32, v); }
  pc = 0x10988a5; continue;
 }
 0x10988a5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133edcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010988c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10988c4); }
 let mut pc: u32 = 0x10988c4;
 loop { match pc {
 0x10988c4 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x100u32; cpu.edx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10988eb); f_010983b4(cpu); if cpu.returned_to != 0x10988eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  if cpu.zf { pc = 0x10988fc; continue; }
  pc = 0x10988f6; continue;
 }
 0x10988f6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10988fc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098934(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098934); }
 let mut pc: u32 = 0x1098934;
 loop { match pc {
 0x1098934 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x101u32; cpu.edx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1098955); f_010983b4(cpu); if cpu.returned_to != 0x1098955 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109895c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109895c); }
 let mut pc: u32 = 0x109895c;
 loop { match pc {
 0x109895c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x5cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x1098977); f_0108f030(cpu); if cpu.returned_to != 0x1098977 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10989bb); f_01097d44(cpu); if cpu.returned_to != 0x10989bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x68u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x39u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x56u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x300u32; cpu.edx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x44u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4cu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10989fe); f_0109817e(cpu); if cpu.returned_to != 0x10989fe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098a48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098a48); }
 let mut pc: u32 = 0x1098a48;
 loop { match pc {
 0x1098a48 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x5cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x1098a68); f_0108f030(cpu); if cpu.returned_to != 0x1098a68 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1098abd); f_01097d44(cpu); if cpu.returned_to != 0x1098abd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x6cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x39u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x56u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x50u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x300u32; cpu.edx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x44u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4cu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1098b00); f_0109817e(cpu); if cpu.returned_to != 0x1098b00 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098b68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098b68); }
 let mut pc: u32 = 0x1098b68;
 loop { match pc {
 0x10985b5 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x100000u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10985f5; continue; }
  pc = 0x10985c3; continue;
 }
 0x10985c3 => {
  { let v: u16 = 0x9u16; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8092u32; cpu.edx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10985e3); f_010983b4(cpu); if cpu.returned_to != 0x10985e3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10985ef); f_01098470(cpu); if cpu.returned_to != 0x10985ef { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1098616; continue; }
  pc = 0x10985f5; continue;
 }
 0x10985f5 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = (cpu.edi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x1098603); f_010984ec(cpu); if cpu.returned_to != 0x1098603 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1098616; continue; }
  pc = 0x1098607; continue;
 }
 0x1098607 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x1098610); f_01098570(cpu); if cpu.returned_to != 0x1098610 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1098616; continue; }
  pc = 0x1098614; continue;
 }
 0x1098614 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x1098616; continue;
 }
 0x1098616 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1098b68 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1098b97); f_010983b4(cpu); if cpu.returned_to != 0x1098b97 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x10985b5; continue; }
  pc = 0x1098ba6; continue;
 }
 0x1098ba6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098bb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098bb0); }
 let mut pc: u32 = 0x1098bb0;
 loop { match pc {
 0x1098bb0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x1098bd3); f_010983b4(cpu); if cpu.returned_to != 0x1098bd3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098c3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098c3e); }
 let mut pc: u32 = 0x1098c3e;
 loop { match pc {
 0x1098c3e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x1098c4f; continue;
 }
 0x1098c44 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x1098c53; continue; }
  pc = 0x1098c4a; continue;
 }
 0x1098c4a => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x1098c4f; continue;
 }
 0x1098c4f => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x1098c44; continue; }
  pc = 0x1098c53; continue;
 }
 0x1098c53 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1098c5e; continue; }
  pc = 0x1098c57; continue;
 }
 0x1098c57 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x1098c53; continue;
 }
 0x1098c5e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098c63(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098c63); }
 let mut pc: u32 = 0x1098c63;
 loop { match pc {
 0x1098c63 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x41u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1098c80; continue; }
  pc = 0x1098c74; continue;
 }
 0x1098c74 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1098c7e); f_0109995d(cpu); if cpu.returned_to != 0x1098c7e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1098c80 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098c84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098c84); }
 let mut pc: u32 = 0x1098c84;
 loop { match pc {
 0x1098c84 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x1098c8a; continue;
 }
 0x1098c8a => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x1098c90; continue; }
  pc = 0x1098c8e; continue;
 }
 0x1098c8e => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x1098c90; continue;
 }
 0x1098c90 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x1098c8a; continue; }
  pc = 0x1098c99; continue;
 }
 0x1098c99 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01098c9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1098c9e); }
 let mut pc: u32 = 0x1098c9e;
 loop { match pc {
 0x1098c9e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x124u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x120u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x114u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x110u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  pc = 0x1098d70; continue;
 }
 0x1098cdd => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x1098ceb); f_0109903d(cpu); if cpu.returned_to != 0x1098ceb { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x115u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x109902b; continue; }
  pc = 0x1098cff; continue;
 }
 0x1098cff => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x1098e3a; continue; }
  pc = 0x1098d11; continue;
 }
 0x1098d11 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1098da7; continue; }
  pc = 0x1098d39; continue;
 }
 0x1098d39 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1098d57; continue; }
  pc = 0x1098d41; continue;
 }
 0x1098d41 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x1098d4b; continue;
 }
 0x1098d4b => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x1098d70; continue;
 }
 0x1098d57 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x1098d9c; continue; }
  pc = 0x1098d5e; continue;
 }
 0x1098d5e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x1098d67; continue;
 }
 0x1098d67 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x1098d70; continue;
 }
 0x1098d70 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x109902b; continue; }
  pc = 0x1098d7a; continue;
 }
 0x1098d7a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x25u32); }
  if cpu.zf { cpu.poll(); pc = 0x1098cdd; continue; }
  pc = 0x1098d8a; continue;
 }
 0x1098d8a => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x1098d9a); cpu.call(t); if cpu.returned_to != 0x1098d9a { return; }
  cpu.poll(); pc = 0x1098d70; continue;
 }
 0x1098d9c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x1098d67; continue;
 }
 0x1098da7 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x1098e02; continue; }
  pc = 0x1098db9; continue;
 }
 0x1098db9 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1098dd8; continue; }
  pc = 0x1098dc1; continue;
 }
 0x1098dc1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.eax); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x1098d70; continue;
 }
 0x1098dd8 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x1098df7; continue; }
  pc = 0x1098ddf; continue;
 }
 0x1098ddf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  pc = 0x1098de8; continue;
 }
 0x1098de8 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x1098d70; continue;
 }
 0x1098df7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.poll(); pc = 0x1098de8; continue;
 }
 0x1098e02 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1098e17; continue; }
  pc = 0x1098e08; continue;
 }
 0x1098e08 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  cpu.poll(); pc = 0x1098d4b; continue;
 }
 0x1098e17 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x1098e2c; continue; }
  pc = 0x1098e1e; continue;
 }
 0x1098e1e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  cpu.poll(); pc = 0x1098d67; continue;
 }
 0x1098e2c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  cpu.poll(); pc = 0x1098d67; continue;
 }
 0x1098e3a => {
  { let v: u32 = 0x120u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x1098e51); f_0109937b(cpu); if cpu.returned_to != 0x1098e51 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1098e9d; continue; }
  pc = 0x1098e65; continue;
 }
 0x1098e65 => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x1098e9d; continue; }
  pc = 0x1098e71; continue;
 }
 0x1098e71 => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1098e9d; continue; }
  pc = 0x1098e84; continue;
 }
 0x1098e84 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x1098e9b); cpu.call(t); if cpu.returned_to != 0x1098e9b { return; }
  cpu.poll(); pc = 0x1098e71; continue;
 }
 0x1098e9d => {
  { let v: u32 = 0x117u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x1098ea4; continue;
 }
 0x1098ea4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1098ebf; continue; }
  pc = 0x1098eaa; continue;
 }
 0x1098eaa => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x1098ebd); cpu.call(t); if cpu.returned_to != 0x1098ebd { return; }
  cpu.poll(); pc = 0x1098ea4; continue;
 }
 0x1098ebf => {
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1098ee9; continue; }
  pc = 0x1098ed4; continue;
 }
 0x1098ed4 => {
  { let v: u32 = 0x30u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x1098ee7); cpu.call(t); if cpu.returned_to != 0x1098ee7 { return; }
  cpu.poll(); pc = 0x1098ebf; continue;
 }
 0x1098ee9 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1098f31; continue; }
  pc = 0x1098ef9; continue;
 }
 0x1098ef9 => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if cpu.zf { pc = 0x1098f31; continue; }
  pc = 0x1098f05; continue;
 }
 0x1098f05 => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1098f31; continue; }
  pc = 0x1098f18; continue;
 }
 0x1098f18 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x1098f2f); cpu.call(t); if cpu.returned_to != 0x1098f2f { return; }
  cpu.poll(); pc = 0x1098f05; continue;
 }
 0x1098f31 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.zf { pc = 0x1098f44; continue; }
  pc = 0x1098f3f; continue;
 }
 0x1098f3f => {
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x1098fb0; continue; }
  pc = 0x1098f44; continue;
 }
 0x1098f44 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x1098f85; continue; }
  pc = 0x1098f54; continue;
 }
 0x1098f54 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1098fdb; continue; }
  pc = 0x1098f6d; continue;
 }
 0x1098f6d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x1098f80); cpu.call(t); if cpu.returned_to != 0x1098f80 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x1098f54; continue;
 }
 0x1098f85 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1098fdb; continue; }
  pc = 0x1098f9a; continue;
 }
 0x1098f9a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x1098fad); cpu.call(t); if cpu.returned_to != 0x1098fad { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x1098f85; continue;
 }
 0x1098fb0 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1098fdb; continue; }
  pc = 0x1098fc5; continue;
 }
 0x1098fc5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x1098fd8); cpu.call(t); if cpu.returned_to != 0x1098fd8 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x1098fb0; continue;
 }
 0x1098fdb => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x1098d70; continue; }
  pc = 0x1098fef; continue;
 }
 0x1098fef => {
  { cpu.sub32(cpu.r32(0x104u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x1098d70; continue; }
  pc = 0x1098ffd; continue;
 }
 0x1098ffd => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x1098d70; continue; }
  pc = 0x1099016; continue;
 }
 0x1099016 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x1099029); cpu.call(t); if cpu.returned_to != 0x1099029 { return; }
  cpu.poll(); pc = 0x1098ffd; continue;
 }
 0x109902b => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x124u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109903d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109903d); }
 let mut pc: u32 = 0x109903d;
 loop { match pc {
 0x109903d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x1099051); f_0109917b(cpu); if cpu.returned_to != 0x1099051 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x109908b; continue; }
  pc = 0x1099065; continue;
 }
 0x1099065 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109908b; continue; }
  pc = 0x109906a; continue;
 }
 0x109906a => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0xau32); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x10990b6; continue; }
  pc = 0x1099084; continue;
 }
 0x1099084 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x109906a; continue; }
  pc = 0x1099089; continue;
 }
 0x1099089 => {
  pc = 0x10990b6; continue;
 }
 0x109908b => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x10990b6; continue; }
  pc = 0x1099094; continue;
 }
 0x1099094 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10990b5; continue; }
  pc = 0x10990a5; continue;
 }
 0x10990a5 => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8) | 0x8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10990b5; continue;
 }
 0x10990b5 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10990b6; continue;
 }
 0x10990b6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x109910f; continue; }
  pc = 0x10990c6; continue;
 }
 0x10990c6 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x10990f2; continue; }
  pc = 0x10990d7; continue;
 }
 0x10990d7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10990ef; continue; }
  pc = 0x10990e8; continue;
 }
 0x10990e8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10990ef; continue;
 }
 0x10990ef => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x109910f; continue;
 }
 0x10990f2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x109910f; continue; }
  pc = 0x10990fb; continue;
 }
 0x10990fb => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109910f; continue; }
  pc = 0x1099100; continue;
 }
 0x1099100 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0xau32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x10990f2; continue;
 }
 0x109910f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8((cpu.edx as u8), 0x4eu8); }
  if cpu.cf { pc = 0x1099134; continue; }
  pc = 0x1099119; continue;
 }
 0x1099119 => {
  if (cpu.cf || cpu.zf) { pc = 0x1099171; continue; }
  pc = 0x109911b; continue;
 }
 0x109911b => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x109912b; continue; }
  pc = 0x1099120; continue;
 }
 0x1099120 => {
  if (cpu.cf || cpu.zf) { pc = 0x1099144; continue; }
  pc = 0x1099122; continue;
 }
 0x1099122 => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x1099144; continue; }
  pc = 0x1099127; continue;
 }
 0x1099127 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109912b => {
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.zf { pc = 0x1099152; continue; }
  pc = 0x1099130; continue;
 }
 0x1099130 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1099134 => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x1099177; continue; }
  pc = 0x1099139; continue;
 }
 0x1099139 => {
  if (cpu.cf || cpu.zf) { pc = 0x1099167; continue; }
  pc = 0x109913b; continue;
 }
 0x109913b => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x1099158; continue; }
  pc = 0x1099140; continue;
 }
 0x1099140 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1099144 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x109914b; continue;
 }
 0x109914b => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1099152 => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x10u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1099175; continue;
 }
 0x1099158 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), 0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1099167 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x80u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.poll(); pc = 0x109914b; continue;
 }
 0x1099171 => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x40u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1099175; continue;
 }
 0x1099175 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x1099177; continue;
 }
 0x1099177 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109917b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109917b); }
 let mut pc: u32 = 0x109917b;
 loop { match pc {
 0x109917b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x1099183; continue;
 }
 0x1099183 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x1099192; continue; }
  pc = 0x109918c; continue;
 }
 0x109918c => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x8u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10991d2; continue;
 }
 0x1099192 => {
  { cpu.sub32(cpu.eax, 0x23u32); }
  if !cpu.zf { pc = 0x109919d; continue; }
  pc = 0x1099197; continue;
 }
 0x1099197 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10991d2; continue;
 }
 0x109919d => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x10991b2; continue; }
  pc = 0x10991a2; continue;
 }
 0x10991a2 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10991d2; continue;
 }
 0x10991b2 => {
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x10991c9; continue; }
  pc = 0x10991b7; continue;
 }
 0x10991b7 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x10991d2; continue; }
  pc = 0x10991c3; continue;
 }
 0x10991c3 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x2u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10991d2; continue;
 }
 0x10991c9 => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x10991d5; continue; }
  pc = 0x10991ce; continue;
 }
 0x10991ce => {
  { let v: u8 = 0x30u8; cpu.w8(0x16u32.wrapping_add(cpu.edx), v); }
  pc = 0x10991d2; continue;
 }
 0x10991d2 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x1099183; continue;
 }
 0x10991d5 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010991da(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10991da); } r_010991da(cpu, 0x10991da); }
pub fn f_010991fe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10991fe); } r_010991da(cpu, 0x10991fe); }
fn r_010991da(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10991da => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10991e8; continue;
 }
 0x10991e8 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x10991f9; continue; }
  pc = 0x10991f2; continue;
 }
 0x10991f2 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x10991f9; continue; }
  pc = 0x10991f6; continue;
 }
 0x10991f6 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10991e8; continue;
 }
 0x10991f9 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10991fe => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109920c; continue;
 }
 0x109920c => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { cpu.poll(); pc = 0x10991f9; continue; }
  pc = 0x109921a; continue;
 }
 0x109921a => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { cpu.poll(); pc = 0x10991f9; continue; }
  pc = 0x109921e; continue;
 }
 0x109921e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x109920c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099221(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099221); }
 let mut pc: u32 = 0x1099221;
 loop { match pc {
 0x1099221 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x1099239); f_010a63f4(cpu); if cpu.returned_to != 0x1099239 { return; }
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x109925e; continue;
 }
 0x1099257 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x109925e; continue;
 }
 0x109925e => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x1099257; continue; }
  pc = 0x1099262; continue;
 }
 0x1099262 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  pc = 0x1099265; continue;
 }
 0x1099265 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1099270; continue; }
  pc = 0x1099269; continue;
 }
 0x1099269 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x1099265; continue;
 }
 0x1099270 => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
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
pub fn f_0109927f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109927f); }
 let mut pc: u32 = 0x109927f;
 loop { match pc {
 0x109927f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x109929b; continue; }
  pc = 0x1099290; continue;
 }
 0x1099290 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x109929b; continue;
 }
 0x109929b => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10992a8; continue; }
  pc = 0x10992a1; continue;
 }
 0x10992a1 => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x10992a8; continue;
 }
 0x10992a8 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10992bb); f_010a63f4(cpu); if cpu.returned_to != 0x10992bb { return; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x10992bd; continue;
 }
 0x10992bd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x10992ca; continue; }
  pc = 0x10992c6; continue;
 }
 0x10992c6 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.poll(); pc = 0x10992bd; continue;
 }
 0x10992ca => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10992fc; continue; }
  pc = 0x10992d0; continue;
 }
 0x10992d0 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x10992f4; continue;
 }
 0x10992d9 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, 0xau32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x10992f4; continue;
 }
 0x10992f4 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10992d9; continue; }
  pc = 0x10992f9; continue;
 }
 0x10992f9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10992fc; continue;
 }
 0x10992fc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109936d; continue; }
  pc = 0x109930f; continue;
 }
 0x109930f => {
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if !cpu.zf { pc = 0x109934c; continue; }
  pc = 0x1099313; continue;
 }
 0x1099313 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u8 = 0x31u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1099319; continue;
 }
 0x1099319 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if !cpu.zf { pc = 0x1099329; continue; }
  pc = 0x1099325; continue;
 }
 0x1099325 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x1099319; continue;
 }
 0x1099329 => {
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x1099343; continue; }
  pc = 0x109932e; continue;
 }
 0x109932e => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1099337; continue;
 }
 0x1099337 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x1099343; continue; }
  pc = 0x1099340; continue;
 }
 0x1099340 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x1099337; continue;
 }
 0x1099343 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109936d; continue;
 }
 0x109934c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x1099357; continue; }
  pc = 0x1099356; continue;
 }
 0x1099356 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1099357; continue;
 }
 0x1099357 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if cpu.zf { pc = 0x1099368; continue; }
  pc = 0x1099360; continue;
 }
 0x1099360 => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x109936d; continue;
 }
 0x1099368 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.poll(); pc = 0x109930f; continue;
 }
 0x109936d => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099374(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099374); }
 let mut pc: u32 = 0x1099374;
 loop { match pc {
 0x1099374 => {
  let t = cpu.r32(0x1134db8u32); cpu.push(0x109937a); cpu.call(t); if cpu.returned_to != 0x109937a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109937b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109937b); }
 let mut pc: u32 = 0x109937b;
 loop { match pc {
 0x109937b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { cpu.sub8((cpu.edx as u8), 0x69u8); }
  if cpu.cf { pc = 0x10993c4; continue; }
  pc = 0x10993a8; continue;
 }
 0x10993a8 => {
  if (cpu.cf || cpu.zf) { pc = 0x1099415; continue; }
  pc = 0x10993ae; continue;
 }
 0x10993ae => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x10993bf; continue; }
  pc = 0x10993b3; continue;
 }
 0x10993b3 => {
  if (cpu.cf || cpu.zf) { pc = 0x10993d2; continue; }
  pc = 0x10993b5; continue;
 }
 0x10993b5 => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  pc = 0x10993b8; continue;
 }
 0x10993b8 => {
  if cpu.zf { pc = 0x10993d2; continue; }
  pc = 0x10993ba; continue;
 }
 0x10993ba => {
  pc = 0x109941f; continue;
 }
 0x10993bf => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  cpu.poll(); pc = 0x10993b8; continue;
 }
 0x10993c4 => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x109941f; continue; }
  pc = 0x10993c9; continue;
 }
 0x10993c9 => {
  if (cpu.cf || cpu.zf) { pc = 0x10993d2; continue; }
  pc = 0x10993cb; continue;
 }
 0x10993cb => {
  { cpu.sub8((cpu.edx as u8), 0x64u8); }
  if cpu.zf { pc = 0x1099415; continue; }
  pc = 0x10993d0; continue;
 }
 0x10993d0 => {
  pc = 0x109941f; continue;
 }
 0x10993d2 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10993ec; continue; }
  pc = 0x10993e0; continue;
 }
 0x10993e0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  pc = 0x1099411; continue;
 }
 0x10993ec => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x1099407; continue; }
  pc = 0x10993fa; continue;
 }
 0x10993fa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  pc = 0x1099411; continue;
 }
 0x1099407 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  pc = 0x1099411; continue;
 }
 0x1099411 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)) & 0xf9u8; cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x1099415; continue;
 }
 0x1099415 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x109941f; continue; }
  pc = 0x109941b; continue;
 }
 0x109941b => {
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.esi), v); }
  pc = 0x109941f; continue;
 }
 0x109941f => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.cf { pc = 0x109948c; continue; }
  pc = 0x109942c; continue;
 }
 0x109942c => {
  if (cpu.cf || cpu.zf) { pc = 0x109950c; continue; }
  pc = 0x1099432; continue;
 }
 0x1099432 => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  if cpu.cf { pc = 0x1099473; continue; }
  pc = 0x1099437; continue;
 }
 0x1099437 => {
  if (cpu.cf || cpu.zf) { pc = 0x1099522; continue; }
  pc = 0x109943d; continue;
 }
 0x109943d => {
  { cpu.sub8((cpu.edx as u8), 0x73u8); }
  if cpu.cf { pc = 0x1099465; continue; }
  pc = 0x1099442; continue;
 }
 0x1099442 => {
  if (cpu.cf || cpu.zf) { pc = 0x10995b7; continue; }
  pc = 0x1099448; continue;
 }
 0x1099448 => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x1099789; continue; }
  pc = 0x1099451; continue;
 }
 0x1099451 => {
  if (cpu.cf || cpu.zf) { pc = 0x10996a3; continue; }
  pc = 0x1099457; continue;
 }
 0x1099457 => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  if cpu.zf { pc = 0x1099680; continue; }
  pc = 0x1099460; continue;
 }
 0x1099460 => {
  pc = 0x1099789; continue;
 }
 0x1099465 => {
  { cpu.sub8((cpu.edx as u8), 0x70u8); }
  if cpu.zf { pc = 0x10996c6; continue; }
  pc = 0x109946e; continue;
 }
 0x109946e => {
  pc = 0x1099789; continue;
 }
 0x1099473 => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.cf { pc = 0x10994da; continue; }
  pc = 0x1099478; continue;
 }
 0x1099478 => {
  if (cpu.cf || cpu.zf) { pc = 0x109950c; continue; }
  pc = 0x109947e; continue;
 }
 0x109947e => {
  { cpu.sub8((cpu.edx as u8), 0x69u8); }
  if cpu.zf { pc = 0x1099566; continue; }
  pc = 0x1099487; continue;
 }
 0x1099487 => {
  pc = 0x1099789; continue;
 }
 0x109948c => {
  { cpu.sub8((cpu.edx as u8), 0x50u8); }
  if cpu.cf { pc = 0x10994c4; continue; }
  pc = 0x1099491; continue;
 }
 0x1099491 => {
  if (cpu.cf || cpu.zf) { pc = 0x10996c6; continue; }
  pc = 0x1099497; continue;
 }
 0x1099497 => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x10994b6; continue; }
  pc = 0x109949c; continue;
 }
 0x109949c => {
  if (cpu.cf || cpu.zf) { pc = 0x1099680; continue; }
  pc = 0x10994a2; continue;
 }
 0x10994a2 => {
  { cpu.sub8((cpu.edx as u8), 0x63u8); }
  if cpu.cf { pc = 0x1099789; continue; }
  pc = 0x10994ab; continue;
 }
 0x10994ab => {
  if (cpu.cf || cpu.zf) { pc = 0x109975d; continue; }
  pc = 0x10994b1; continue;
 }
 0x10994b1 => {
  pc = 0x1099566; continue;
 }
 0x10994b6 => {
  { cpu.sub8((cpu.edx as u8), 0x53u8); }
  if cpu.zf { pc = 0x10995b7; continue; }
  pc = 0x10994bf; continue;
 }
 0x10994bf => {
  pc = 0x1099789; continue;
 }
 0x10994c4 => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x10994d5; continue; }
  pc = 0x10994c9; continue;
 }
 0x10994c9 => {
  if (cpu.cf || cpu.zf) { pc = 0x10994da; continue; }
  pc = 0x10994cb; continue;
 }
 0x10994cb => {
  { cpu.sub8((cpu.edx as u8), 0x47u8); }
  pc = 0x10994ce; continue;
 }
 0x10994ce => {
  if cpu.zf { pc = 0x109950c; continue; }
  pc = 0x10994d0; continue;
 }
 0x10994d0 => {
  pc = 0x1099789; continue;
 }
 0x10994d5 => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  cpu.poll(); pc = 0x10994ce; continue;
 }
 0x10994da => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x109950c; continue; }
  pc = 0x10994e8; continue;
 }
 0x10994e8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x10994fb); f_0109927f(cpu); if cpu.returned_to != 0x10994fb { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x109955c; continue;
 }
 0x109950c => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x1099517); f_01099374(cpu); if cpu.returned_to != 0x1099517 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x109955c; continue;
 }
 0x1099522 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x1099537; continue; }
  pc = 0x1099533; continue;
 }
 0x1099533 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x1099537; continue;
 }
 0x1099537 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x1099540); f_010a6490(cpu); if cpu.returned_to != 0x1099540 { return; }
  pc = 0x1099540; continue;
 }
 0x1099540 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x1099553; continue; }
  pc = 0x1099546; continue;
 }
 0x1099546 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x1099553; continue; }
  pc = 0x109954f; continue;
 }
 0x109954f => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x1099553; continue;
 }
 0x1099553 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x109955c; continue;
 }
 0x109955c => {
  cpu.push(0x1099561); f_010991da(cpu); if cpu.returned_to != 0x1099561 { return; }
  pc = 0x10997b2; continue;
 }
 0x1099566 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x1099587; continue; }
  pc = 0x1099574; continue;
 }
 0x1099574 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x1099580; continue;
 }
 0x1099580 => {
  cpu.push(0x1099585); f_010a6490(cpu); if cpu.returned_to != 0x1099585 { return; }
  cpu.poll(); pc = 0x1099540; continue;
 }
 0x1099587 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10995a4; continue; }
  pc = 0x1099595; continue;
 }
 0x1099595 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.poll(); pc = 0x1099580; continue;
 }
 0x10995a4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x10995b5); f_010a63f4(cpu); if cpu.returned_to != 0x10995b5 { return; }
  cpu.poll(); pc = 0x1099540; continue;
 }
 0x10995b7 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10995e8; continue; }
  pc = 0x10995c5; continue;
 }
 0x10995c5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10995dc; continue; }
  pc = 0x10995d7; continue;
 }
 0x10995d7 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x10995e0; continue; }
  pc = 0x10995dc; continue;
 }
 0x10995dc => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  pc = 0x109961e; continue;
 }
 0x10995e0 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x109961c; continue;
 }
 0x10995e8 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x40u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x1099608; continue; }
  pc = 0x10995f6; continue;
 }
 0x10995f6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x10995e0; continue; }
  pc = 0x1099604; continue;
 }
 0x1099604 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  pc = 0x109961c; continue;
 }
 0x1099608 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x1099604; continue; }
  pc = 0x1099616; continue;
 }
 0x1099616 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x109961c; continue;
 }
 0x109961c => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  pc = 0x109961e; continue;
 }
 0x109961e => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) & 0xf9u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x1099658; continue; }
  pc = 0x1099636; continue;
 }
 0x1099636 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109964f; continue; }
  pc = 0x1099641; continue;
 }
 0x1099641 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x10997b2; continue;
 }
 0x109964f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x10997b2; continue;
 }
 0x1099658 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x1099674; continue; }
  pc = 0x1099663; continue;
 }
 0x1099663 => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x109966f); f_010991fe(cpu); if cpu.returned_to != 0x109966f { return; }
  pc = 0x10997b2; continue;
 }
 0x1099674 => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.poll(); pc = 0x109955c; continue;
 }
 0x1099680 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x109969e; continue; }
  pc = 0x109968c; continue;
 }
 0x109968c => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x109969e; continue; }
  pc = 0x1099690; continue;
 }
 0x1099690 => {
  { let v: u8 = 0x30u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x109969e; continue;
 }
 0x109969e => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x10996a3; continue;
 }
 0x10996a3 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x10996ac); f_010a6446(cpu); if cpu.returned_to != 0x10996ac { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1099540; continue; }
  pc = 0x10996ba; continue;
 }
 0x10996ba => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10996c1); f_01099864(cpu); if cpu.returned_to != 0x10996c1 { return; }
  cpu.poll(); pc = 0x1099540; continue;
 }
 0x10996c6 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x10996ea; continue; }
  pc = 0x10996cc; continue;
 }
 0x10996cc => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10996e3; continue; }
  pc = 0x10996da; continue;
 }
 0x10996da => {
  { let v: u32 = 0xdu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x10996ea; continue;
 }
 0x10996e3 => {
  { let v: u32 = 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x10996ea; continue;
 }
 0x10996ea => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)) & 0xf9u8; cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x1099735; continue; }
  pc = 0x109970c; continue;
 }
 0x109970c => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  cpu.push(0x1099725); f_01099221(cpu); if cpu.returned_to != 0x1099725 { return; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x109973e; continue;
 }
 0x1099735 => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x109973e; continue;
 }
 0x109973e => {
  cpu.push(0x1099743); f_01099221(cpu); if cpu.returned_to != 0x1099743 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x50u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1099553; continue; }
  pc = 0x1099751; continue;
 }
 0x1099751 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x1099758); f_01099864(cpu); if cpu.returned_to != 0x1099758 { return; }
  cpu.poll(); pc = 0x1099553; continue;
 }
 0x109975d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x10997b2; continue;
 }
 0x1099789 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) & 0xf9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10997b2; continue;
 }
 0x10997b2 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1099807; continue; }
  pc = 0x10997b8; continue;
 }
 0x10997b8 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x10997ca; continue; }
  pc = 0x10997c2; continue;
 }
 0x10997c2 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x10997f0; continue;
 }
 0x10997ca => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10997de; continue; }
  pc = 0x10997d8; continue;
 }
 0x10997d8 => {
  { let v: u8 = 0x20u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x10997f0; continue;
 }
 0x10997de => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10997f4; continue; }
  pc = 0x10997ec; continue;
 }
 0x10997ec => {
  { let v: u8 = 0x2bu8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x10997f0; continue;
 }
 0x10997f0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x10997f4; continue;
 }
 0x10997f4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1099800; continue; }
  pc = 0x10997fb; continue;
 }
 0x10997fb => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x1099807; continue;
 }
 0x1099800 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x1099807; continue;
 }
 0x1099807 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x109981e; continue; }
  pc = 0x1099811; continue;
 }
 0x1099811 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x109981e; continue;
 }
 0x109981e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x109982a; continue; }
  pc = 0x1099826; continue;
 }
 0x1099826 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x1099837; continue; }
  pc = 0x109982a; continue;
 }
 0x109982a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x63u32); }
  if cpu.zf { pc = 0x1099837; continue; }
  pc = 0x1099834; continue;
 }
 0x1099834 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x1099837; continue;
 }
 0x1099837 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x17u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x1099846); f_010991da(cpu); if cpu.returned_to != 0x1099846 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099864(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099864); }
 let mut pc: u32 = 0x1099864;
 loop { match pc {
 0x1099864 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x1099867; continue;
 }
 0x1099867 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x109987a; continue; }
  pc = 0x109986c; continue;
 }
 0x109986c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1099875); f_0108e3ae(cpu); if cpu.returned_to != 0x1099875 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x1099867; continue;
 }
 0x109987a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109987c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109987c); }
 let mut pc: u32 = 0x109987c;
 loop { match pc {
 0x109987c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.cf { pc = 0x10998be; continue; }
  pc = 0x109988d; continue;
 }
 0x109988d => {
  { cpu.sub8((cpu.edx as u8), 0x3u8); }
  if (cpu.cf || cpu.zf) { pc = 0x1099899; continue; }
  pc = 0x1099892; continue;
 }
 0x1099892 => {
  { cpu.sub8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x10998aa; continue; }
  pc = 0x1099897; continue;
 }
 0x1099897 => {
  pc = 0x10998be; continue;
 }
 0x1099899 => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10998e2; continue;
 }
 0x10998aa => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if (cpu.cf || cpu.zf) { pc = 0x10998be; continue; }
  pc = 0x10998b4; continue;
 }
 0x10998b4 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10998ea; continue;
 }
 0x10998be => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | 0x81u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub32(cpu.edx, 0x6u32); }
  if !cpu.zf { pc = 0x10998dd; continue; }
  pc = 0x10998ce; continue;
 }
 0x10998ce => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if (cpu.cf || cpu.zf) { pc = 0x10998dd; continue; }
  pc = 0x10998d8; continue;
 }
 0x10998d8 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x11u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x10998dd; continue;
 }
 0x10998dd => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x10998e2; continue;
 }
 0x10998e2 => {
  cpu.push(0x10998e7); f_010a64ab(cpu); if cpu.returned_to != 0x10998e7 { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x10998ea; continue;
 }
 0x10998ea => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010998f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10998f7); }
 let mut pc: u32 = 0x10998f7;
 loop { match pc {
 0x10998f7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109991e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109991e); }
 let mut pc: u32 = 0x109991e;
 loop { match pc {
 0x109991e => {
  { let v: u32 = 0xdu32; cpu.eax = v; }
  f_01099923(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099923(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099923); }
 let mut pc: u32 = 0x1099923;
 loop { match pc {
 0x1099923 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x109992b); f_010a4ca4(cpu); if cpu.returned_to != 0x109992b { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109992f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109992f); }
 let mut pc: u32 = 0x109992f;
 loop { match pc {
 0x109992f => {
  { let v: u32 = 0xeu32; cpu.eax = v; }
  f_01099923(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099936(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099936); }
 let mut pc: u32 = 0x1099936;
 loop { match pc {
 0x1099936 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x1099940); f_01099923(cpu); if cpu.returned_to != 0x1099940 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099946(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099946); }
 let mut pc: u32 = 0x1099946;
 loop { match pc {
 0x1099946 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x109994e); f_010a4caa(cpu); if cpu.returned_to != 0x109994e { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099959(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099959); }
 let mut pc: u32 = 0x1099959;
 loop { match pc {
 0x1099958 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1099959 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x1099958; continue; }
  f_0109995d(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109995d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109995d); }
 let mut pc: u32 = 0x109995d;
 loop { match pc {
 0x109995d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x109996c); f_01099946(cpu); if cpu.returned_to != 0x109996c { return; }
  { cpu.sub32(cpu.ebx, 0x100u32); }
  if !cpu.cf { pc = 0x10999b7; continue; }
  pc = 0x1099974; continue;
 }
 0x1099974 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e87u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x109999d; continue; }
  pc = 0x1099980; continue;
 }
 0x1099980 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x50u32); }
  if !cpu.zf { pc = 0x109998d; continue; }
  pc = 0x1099989; continue;
 }
 0x1099989 => {
  { let v: u8 = 0xeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x109999d; continue;
 }
 0x109998d => {
  { cpu.sub32(cpu.ebx, 0x22u32); }
  if (cpu.sf != cpu.of) { pc = 0x1099996; continue; }
  pc = 0x1099992; continue;
 }
 0x1099992 => {
  { let v: u8 = 0x13u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x109999d; continue;
 }
 0x1099996 => {
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { pc = 0x109999d; continue; }
  pc = 0x109999b; continue;
 }
 0x109999b => {
  { let v: u8 = 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x109999d; continue;
 }
 0x109999d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x13u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10999a8; continue; }
  pc = 0x10999a6; continue;
 }
 0x10999a6 => {
  { let v: u8 = 0x13u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10999a8; continue;
 }
 0x10999a8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1133eddu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 24u32); cpu.eax = v; }
  pc = 0x10999c1; continue;
 }
 0x10999b7 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  pc = 0x10999c1; continue;
 }
 0x10999c1 => {
  cpu.push(0x10999c6); f_01099923(cpu); if cpu.returned_to != 0x10999c6 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010999ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10999ce); }
 let mut pc: u32 = 0x10999ce;
 loop { match pc {
 0x10999ce => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x1133ef8u32)); }
  if cpu.cf { pc = 0x10999e1; continue; }
  pc = 0x10999db; continue;
 }
 0x10999db => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10999e1 => {
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1099a17; continue; }
  pc = 0x10999e6; continue;
 }
 0x10999e6 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1133f4cu32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x1099a17; continue; }
  pc = 0x10999fa; continue;
 }
 0x10999fa => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1099a09); f_010998f7(cpu); if cpu.returned_to != 0x1099a09 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1099a17; continue; }
  pc = 0x1099a0d; continue;
 }
 0x1099a0d => {
  { let v: u32 = cpu.r32(0x1133f4cu32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x20u8); cpu.w8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x1099a17; continue;
 }
 0x1099a17 => {
  { let v: u32 = cpu.r32(0x1133f4cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099a23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099a23); }
 let mut pc: u32 = 0x1099a23;
 loop { match pc {
 0x1099a23 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x40u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133f4cu32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099a38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099a38); }
 let mut pc: u32 = 0x1099a38;
 loop { match pc {
 0x1099a38 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1099a5a; continue; }
  pc = 0x1099a4b; continue;
 }
 0x1099a4b => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x1099a54); f_01099a23(cpu); if cpu.returned_to != 0x1099a54 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1099a5a => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x1099a64); f_01099923(cpu); if cpu.returned_to != 0x1099a64 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099a6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099a6e); }
 let mut pc: u32 = 0x1099a6e;
 loop { match pc {
 0x1099a6e => {
  if !cpu.cf { pc = 0x1099a7e; continue; }
  pc = 0x1099a70; continue;
 }
 0x1099a70 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1099a7b); f_0109995d(cpu); if cpu.returned_to != 0x1099a7b { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1099a80; continue;
 }
 0x1099a7e => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1099a80; continue;
 }
 0x1099a80 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099a87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099a87); }
 let mut pc: u32 = 0x1099a87;
 loop { match pc {
 0x1099a87 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x11389acu32; cpu.eax = v; }
  cpu.push(0x1099a97); f_0109aa8f(cpu); if cpu.returned_to != 0x1099a97 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099a99(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1099a99); } r_01099a99(cpu, 0x1099a99); }
pub fn f_01099b2e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1099b2e); } r_01099a99(cpu, 0x1099b2e); }
fn r_01099a99(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1099a99 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x1099aa3); cpu.push(0x1099aa8); f_0108ee7c(cpu); if cpu.returned_to != 0x1099aa8 { return; }
  { let v: u32 = 0x11eed3cu32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1133e60u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1133e78u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x11edd4cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1133e60u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1099afd; continue; }
  pc = 0x1099adc; continue;
 }
 0x1099adc => {
  { let v: u32 = cpu.r32(0x1133bbcu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x108dd4au32); }
  if cpu.zf { pc = 0x1099afd; continue; }
  pc = 0x1099ae8; continue;
 }
 0x1099ae8 => {
  { cpu.sub32(cpu.eax, 0x1099df0u32); }
  if cpu.zf { pc = 0x1099afd; continue; }
  pc = 0x1099aef; continue;
 }
 0x1099aef => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { cpu.interrupts_off = false; }
  { cpu.df = false; }
  cpu.flat_ds(0x1099af6); cpu.push(0x1099afb); f_0108e579(cpu); if cpu.returned_to != 0x1099afb { return; }
  pc = 0x1099b0e; continue;
 }
 0x1099afd => {
  { let v: u16 = cpu.r16(0x1133f58u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f54u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x1099b09; continue;
 }
 0x1099b09 => {
  cpu.flat_ds(0x1099b09); cpu.push(0x1099b0e); f_010a6590(cpu); if cpu.returned_to != 0x1099b0e { return; }
  pc = 0x1099b0e; continue;
 }
 0x1099b0e => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1133e60u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x1099b2d); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x1099b2e => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x1099b38); cpu.push(0x1099b3d); f_0108ee7c(cpu); if cpu.returned_to != 0x1099b3d { return; }
  { let v: u32 = 0x11eed3cu32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1133e60u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1133e78u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x11edd4cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1133e60u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x1099b95; continue; }
  pc = 0x1099b71; continue;
 }
 0x1099b71 => {
  { let v: u32 = cpu.r32(0x1133bbcu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x108dd4au32); }
  if cpu.zf { pc = 0x1099b95; continue; }
  pc = 0x1099b7d; continue;
 }
 0x1099b7d => {
  { cpu.sub32(cpu.eax, 0x1099c3au32); }
  if cpu.zf { pc = 0x1099b95; continue; }
  pc = 0x1099b84; continue;
 }
 0x1099b84 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { cpu.interrupts_off = false; }
  { cpu.df = false; }
  cpu.flat_ds(0x1099b8b); cpu.push(0x1099b90); f_0108e579(cpu); if cpu.returned_to != 0x1099b90 { return; }
  cpu.poll(); pc = 0x1099b0e; continue;
 }
 0x1099b95 => {
  { let v: u16 = cpu.r16(0x1133f5eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f5au32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  cpu.poll(); pc = 0x1099b09; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099ba6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099ba6); }
 let mut pc: u32 = 0x1099ba6;
 loop { match pc {
 0x1099ba6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x1133f6cu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x1099c2d; continue; }
  pc = 0x1099bba; continue;
 }
 0x1099bba => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1099bd1; continue; }
  pc = 0x1099bc6; continue;
 }
 0x1099bc6 => {
  { let v: u16 = 0x1686u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x2f); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x1099c23; continue; }
  pc = 0x1099bd1; continue;
 }
 0x1099bd1 => {
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x400u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x6u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.esp)) as u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1099c23; continue; }
  pc = 0x1099c17; continue;
 }
 0x1099c17 => {
  { let v: u32 = 0x1u32; cpu.w32(0x1133f6cu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1099c2d; continue;
 }
 0x1099c23 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1133f6cu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1099c2d; continue;
 }
 0x1099c2d => {
  { let v: u32 = cpu.r32(0x1133f6cu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x1099c39); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099c3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099c3a); }
 let mut pc: u32 = 0x1099c3a;
 loop { match pc {
 0x1099c3a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1133f54u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x1099c55; continue; }
  pc = 0x1099c47; continue;
 }
 0x1099c47 => {
  { cpu.sub16(cpu.r16(0x1133f58u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x1099d28; continue; }
  pc = 0x1099c55; continue;
 }
 0x1099c55 => {
  { let v: u32 = cpu.r32(0x1133bbcu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1099debu32); }
  if !cpu.zf { pc = 0x1099c6f; continue; }
  pc = 0x1099c63; continue;
 }
 0x1099c63 => {
  { let v: u32 = 0x1099df0u32; cpu.w32(0x1133bbcu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1099c81; continue;
 }
 0x1099c6f => {
  { cpu.sub32(cpu.ebx, 0x1099c3au32); }
  if !cpu.zf { pc = 0x1099c81; continue; }
  pc = 0x1099c77; continue;
 }
 0x1099c77 => {
  { let v: u32 = 0x108dd4au32; cpu.w32(0x1133bbcu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1099c81; continue;
 }
 0x1099c81 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x1099cca; continue; }
  pc = 0x1099c8d; continue;
 }
 0x1099c8d => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1099cca; continue; }
  pc = 0x1099c92; continue;
 }
 0x1099c92 => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1133f58u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f54u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1133f64u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f60u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x1099d17; continue;
 }
 0x1099cca => {
  cpu.flat_ds(0x1099cca); cpu.push(0x1099ccf); f_01099ba6(cpu); if cpu.returned_to != 0x1099ccf { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x1099d00; continue; }
  pc = 0x1099cd4; continue;
 }
 0x1099cd4 => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1133f58u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f54u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1133f64u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f60u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x1099d17; continue;
 }
 0x1099d00 => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1133f58u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f54u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x1099d12); cpu.push(0x1099d17); f_010a65a9(cpu); if cpu.returned_to != 0x1099d17 { return; }
  pc = 0x1099d17; continue;
 }
 0x1099d17 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1133f58u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x1133f54u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1099d28; continue;
 }
 0x1099d28 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x1099d2c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099deb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099deb); }
 let mut pc: u32 = 0x1099deb;
 loop { match pc {
 0x1099deb => {
  cpu.push(0x1099df0); f_01099c3a(cpu); if cpu.returned_to != 0x1099df0 { return; }
  f_01099df0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099df0); }
 let mut pc: u32 = 0x1099df0;
 loop { match pc {
 0x1099d2d => {
  { cpu.sub32(cpu.ebx, 0x1099df0u32); }
  if !cpu.zf { pc = 0x1099d3f; continue; }
  pc = 0x1099d35; continue;
 }
 0x1099d35 => {
  { let v: u32 = 0x108dd4au32; cpu.w32(0x1133bbcu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1099d3f; continue;
 }
 0x1099d3f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x1099d88; continue; }
  pc = 0x1099d4b; continue;
 }
 0x1099d4b => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1099d88; continue; }
  pc = 0x1099d50; continue;
 }
 0x1099d50 => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1133f5eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f5au32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1133f6au32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f66u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x1099dd5; continue;
 }
 0x1099d88 => {
  cpu.flat_ds(0x1099d88); cpu.push(0x1099d8d); f_01099ba6(cpu); if cpu.returned_to != 0x1099d8d { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x1099dbe; continue; }
  pc = 0x1099d92; continue;
 }
 0x1099d92 => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1133f5eu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f5au32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1133f6au32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f66u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x1099dd5; continue;
 }
 0x1099dbe => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1133f5eu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1133f5au32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x1099dd0); cpu.push(0x1099dd5); f_010a65a9(cpu); if cpu.returned_to != 0x1099dd5 { return; }
  pc = 0x1099dd5; continue;
 }
 0x1099dd5 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1133f5eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x1133f5au32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1099de6; continue;
 }
 0x1099de6 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x1099dea); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1099df0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1133f5au32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x1099e07; continue; }
  pc = 0x1099dfd; continue;
 }
 0x1099dfd => {
  { cpu.sub16(cpu.r16(0x1133f5eu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x1099de6; continue; }
  pc = 0x1099e07; continue;
 }
 0x1099e07 => {
  { let v: u32 = cpu.r32(0x1133bbcu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1099debu32); }
  if !cpu.zf { cpu.poll(); pc = 0x1099d2d; continue; }
  pc = 0x1099e19; continue;
 }
 0x1099e19 => {
  { let v: u32 = 0x1099c3au32; cpu.w32(0x1133bbcu32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x1099d3f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099e28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099e28); }
 let mut pc: u32 = 0x1099e28;
 loop { match pc {
 0x1099e28 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1133f54u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x1099f3f; continue; }
  pc = 0x1099e38; continue;
 }
 0x1099e38 => {
  { cpu.sub16(cpu.r16(0x1133f58u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x1099f3f; continue; }
  pc = 0x1099e46; continue;
 }
 0x1099e46 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x1099ea5; continue; }
  pc = 0x1099e52; continue;
 }
 0x1099e52 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1099ea5; continue; }
  pc = 0x1099e57; continue;
 }
 0x1099e57 => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1133f58u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1133f54u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1099a99u32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x23u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1133f64u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133f60u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x1099f11; continue;
 }
 0x1099ea5 => {
  cpu.flat_ds(0x1099ea5); cpu.push(0x1099eaa); f_01099ba6(cpu); if cpu.returned_to != 0x1099eaa { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x1099eea; continue; }
  pc = 0x1099eaf; continue;
 }
 0x1099eaf => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1133f58u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1133f54u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1133f64u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1133f60u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1099a99u32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x1099f11; continue;
 }
 0x1099eea => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u32 = 0x1099a99u32; cpu.ebx = v; }
  cpu.flat_ds(0x1099ef4); cpu.push(0x1099ef9); f_010a65d6(cpu); if cpu.returned_to != 0x1099ef9 { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133f54u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1133f58u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x1099f0c); cpu.push(0x1099f11); f_010a65a9(cpu); if cpu.returned_to != 0x1099f11 { return; }
  pc = 0x1099f11; continue;
 }
 0x1099f11 => {
  { let v: u32 = cpu.r32(0x1133bbcu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x108dd4au32); }
  if !cpu.zf { pc = 0x1099f2d; continue; }
  pc = 0x1099f1f; continue;
 }
 0x1099f1f => {
  { let v: u32 = 0x1099c3au32; cpu.w32(0x1133bbcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x1099f2c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1099f2d => {
  { cpu.sub32(cpu.ebx, 0x1099df0u32); }
  if !cpu.zf { pc = 0x1099f3f; continue; }
  pc = 0x1099f35; continue;
 }
 0x1099f35 => {
  { let v: u32 = 0x1099debu32; cpu.w32(0x1133bbcu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1099f3f; continue;
 }
 0x1099f3f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x1099f42); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01099f43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1099f43); }
 let mut pc: u32 = 0x1099f43;
 loop { match pc {
 0x1099f43 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1133f5au32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x109a05a; continue; }
  pc = 0x1099f53; continue;
 }
 0x1099f53 => {
  { cpu.sub16(cpu.r16(0x1133f5eu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x109a05a; continue; }
  pc = 0x1099f61; continue;
 }
 0x1099f61 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x1099fc0; continue; }
  pc = 0x1099f6d; continue;
 }
 0x1099f6d => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1099fc0; continue; }
  pc = 0x1099f72; continue;
 }
 0x1099f72 => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1133f5eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1133f5au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1099b2eu32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x1bu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1133f6au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133f66u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x109a02c; continue;
 }
 0x1099fc0 => {
  cpu.flat_ds(0x1099fc0); cpu.push(0x1099fc5); f_01099ba6(cpu); if cpu.returned_to != 0x1099fc5 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x109a005; continue; }
  pc = 0x1099fca; continue;
 }
 0x1099fca => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1133f5eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1133f5au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1133f6au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1133f66u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1099b2eu32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x109a02c; continue;
 }
 0x109a005 => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u32 = 0x1099b2eu32; cpu.ebx = v; }
  cpu.flat_ds(0x109a00f); cpu.push(0x109a014); f_010a65d6(cpu); if cpu.returned_to != 0x109a014 { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133f5au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1133f5eu32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x109a027); cpu.push(0x109a02c); f_010a65a9(cpu); if cpu.returned_to != 0x109a02c { return; }
  pc = 0x109a02c; continue;
 }
 0x109a02c => {
  { let v: u32 = cpu.r32(0x1133bbcu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x108dd4au32); }
  if !cpu.zf { pc = 0x109a048; continue; }
  pc = 0x109a03a; continue;
 }
 0x109a03a => {
  { let v: u32 = 0x1099df0u32; cpu.w32(0x1133bbcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x109a047); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109a048 => {
  { cpu.sub32(cpu.ebx, 0x1099c3au32); }
  if !cpu.zf { pc = 0x109a05a; continue; }
  pc = 0x109a050; continue;
 }
 0x109a050 => {
  { let v: u32 = 0x1099debu32; cpu.w32(0x1133bbcu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x109a05a; continue;
 }
 0x109a05a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x109a05d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a05e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a05e); }
 let mut pc: u32 = 0x109a05e;
 loop { match pc {
 0x109a05e => {
  pc = 0x10a6608; continue;
 }
 0x10a6608 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1133bc0u32), 0x10ad747u32); }
  if cpu.zf { pc = 0x10a6631; continue; }
  pc = 0x10a6616; continue;
 }
 0x10a6616 => {
  { let v: u32 = 0x108e48fu32; cpu.ebx = v; }
  { let v: u32 = 0x10ad747u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133e89u32, v); }
  cpu.push(0x10a662b); f_010ad6ea(cpu); if cpu.returned_to != 0x10a662b { return; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1133bc0u32, v); }
  pc = 0x10a6631; continue;
 }
 0x10a6631 => {
  cpu.push(0x10a6636); f_010ad7a4(cpu); if cpu.returned_to != 0x10a6636 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a063(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a063); }
 let mut pc: u32 = 0x109a063;
 loop { match pc {
 0x109a063 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a064(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a064); }
 let mut pc: u32 = 0x109a064;
 loop { match pc {
 0x109a064 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = 0x2cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x109a077); f_0108efc0(cpu); if cpu.returned_to != 0x109a077 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a0bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a0bf); }
 let mut pc: u32 = 0x109a0bf;
 loop { match pc {
 0x109a0bf => {
  cpu.push(0x109a0c4); f_0109a064(cpu); if cpu.returned_to != 0x109a0c4 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee14cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a0f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a0f2); }
 let mut pc: u32 = 0x109a0f2;
 loop { match pc {
 0x109a0f2 => {
  { cpu.sub32(cpu.eax, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x109a0ff; continue; }
  pc = 0x109a0f7; continue;
 }
 0x109a0f7 => {
  { cpu.sub32(cpu.eax, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x109a0ff; continue; }
  pc = 0x109a0fc; continue;
 }
 0x109a0fc => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x109a0ff; continue;
 }
 0x109a0ff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a100); }
 let mut pc: u32 = 0x109a100;
 loop { match pc {
 0x109a100 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11eca18u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x109a128; continue; }
  pc = 0x109a10f; continue;
 }
 0x109a10f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi & 0x4003u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11eca18u32, v); }
  { let v: u16 = (cpu.edi as u16) | 0x3u16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x109a16f; continue;
 }
 0x109a128 => {
  { let v: u32 = 0x1133c38u32; cpu.ecx = v; }
  pc = 0x109a14f; continue;
 }
 0x109a12f => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x3u8); }
  if !cpu.zf { pc = 0x109a14c; continue; }
  pc = 0x109a135; continue;
 }
 0x109a135 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x109a13f); f_01094a9a(cpu); if cpu.returned_to != 0x109a13f { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109a194; continue; }
  pc = 0x109a145; continue;
 }
 0x109a145 => {
  { let v: u32 = 0x3u32; cpu.edi = v; }
  pc = 0x109a16f; continue;
 }
 0x109a14c => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  pc = 0x109a14f; continue;
 }
 0x109a14f => {
  { cpu.sub32(cpu.ecx, 0x1133e40u32); }
  if cpu.cf { cpu.poll(); pc = 0x109a12f; continue; }
  pc = 0x109a157; continue;
 }
 0x109a157 => {
  { let v: u32 = 0x22u32; cpu.eax = v; }
  { let v: u32 = 0x4003u32; cpu.edi = v; }
  cpu.push(0x109a166); f_01094a9a(cpu); if cpu.returned_to != 0x109a166 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109a194; continue; }
  pc = 0x109a16c; continue;
 }
 0x109a16c => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x109a16f; continue;
 }
 0x109a16f => {
  { let v: u32 = 0x1au32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x109a17d); f_0108f030(cpu); if cpu.returned_to != 0x109a17d { return; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x11ee150u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x11ee150u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x109a1a0; continue;
 }
 0x109a194 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x109a19e); f_01099923(cpu); if cpu.returned_to != 0x109a19e { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x109a1a0; continue;
 }
 0x109a1a0 => {
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
pub fn f_0109a1a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a1a6); }
 let mut pc: u32 = 0x109a1a6;
 loop { match pc {
 0x109a1a6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x11ee150u32; cpu.edx = v; }
  pc = 0x109a1b0; continue;
 }
 0x109a1b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109a1db; continue; }
  pc = 0x109a1b6; continue;
 }
 0x109a1b6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if cpu.zf { pc = 0x109a1c1; continue; }
  pc = 0x109a1bd; continue;
 }
 0x109a1bd => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.poll(); pc = 0x109a1b0; continue;
 }
 0x109a1c1 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x11eca18u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11eca18u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x109a1db; continue;
 }
 0x109a1db => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a1fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a1fd); }
 let mut pc: u32 = 0x109a1fd;
 loop { match pc {
 0x109a1fd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x20u8); }
  if !cpu.zf { pc = 0x109a22a; continue; }
  pc = 0x109a208; continue;
 }
 0x109a208 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x109a210); f_010998f7(cpu); if cpu.returned_to != 0x109a210 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109a22a; continue; }
  pc = 0x109a214; continue;
 }
 0x109a214 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { cpu.and8((cpu.ebx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x109a22a; continue; }
  pc = 0x109a222; continue;
 }
 0x109a222 => {
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x109a22a; continue;
 }
 0x109a22a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a22e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a22e); }
 let mut pc: u32 = 0x109a22e;
 loop { match pc {
 0x109a22e => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x1133e60u32)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a237(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a237); }
 let mut pc: u32 = 0x109a237;
 loop { match pc {
 0x109a237 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x109a243; continue; }
  pc = 0x109a23e; continue;
 }
 0x109a23e => {
  { let v: u8 = 0x2fu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x109a251; continue;
 }
 0x109a243 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x37u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x109a251; continue;
 }
 0x109a251 => {
  { let v: u8 = 0x63u8; cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a25d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a25d); }
 let mut pc: u32 = 0x109a25d;
 loop { match pc {
 0x109a25d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(44); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  cpu.push(0x109a270); f_0108e28c(cpu); if cpu.returned_to != 0x109a270 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a27c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a27c); }
 let mut pc: u32 = 0x109a27c;
 loop { match pc {
 0x109a27c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(4); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.push(0x109a296); f_010a6860(cpu); if cpu.returned_to != 0x109a296 { return; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x109a2a6); f_010a68b3(cpu); if cpu.returned_to != 0x109a2a6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a2af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a2af); }
 let mut pc: u32 = 0x109a2af;
 loop { match pc {
 0x109a2af => {
  { let v = 0xe4u32; cpu.push(v); }
  cpu.flat_ds(0x109a2b4); cpu.push(0x109a2b9); f_0108ee90(cpu); if cpu.returned_to != 0x109a2b9 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(176); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x82u32); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x6eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.cf { pc = 0x109a2eb; continue; }
  pc = 0x109a2d7; continue;
 }
 0x109a2d7 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.flat_ds(0x109a2dc); cpu.push(0x109a2e1); f_01099923(cpu); if cpu.returned_to != 0x109a2e1 { return; }
  pc = 0x109a2e1; continue;
 }
 0x109a2e1 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x109a5b7; continue;
 }
 0x109a2eb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x52u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x56u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x5au32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5eu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x109a2fe); cpu.push(0x109a303); f_010a6728(cpu); if cpu.returned_to != 0x109a303 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x109a5b7; continue; }
  pc = 0x109a30f; continue;
 }
 0x109a30f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.flat_ds(0x109a311); cpu.push(0x109a316); f_0108e46c(cpu); if cpu.returned_to != 0x109a316 { return; }
  { let v: u32 = 0x9au32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.flat_ds(0x109a31e); cpu.push(0x109a323); f_01094a9a(cpu); if cpu.returned_to != 0x109a323 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109a354; continue; }
  pc = 0x109a32a; continue;
 }
 0x109a32a => {
  { let v: u32 = 0x3u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0xfcu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x109a330); cpu.push(0x109a335); f_0109a22e(cpu); if cpu.returned_to != 0x109a335 { return; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.cf { pc = 0x109a341; continue; }
  pc = 0x109a339; continue;
 }
 0x109a339 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x109a343; continue;
 }
 0x109a341 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109a343; continue;
 }
 0x109a343 => {
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109a357; continue; }
  pc = 0x109a34a; continue;
 }
 0x109a34a => {
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x109a34d); cpu.push(0x109a352); f_01094b7d(cpu); if cpu.returned_to != 0x109a352 { return; }
  cpu.poll(); pc = 0x109a2e1; continue;
 }
 0x109a354 => {
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x109a357; continue;
 }
 0x109a357 => {
  { let v: u32 = 0x42u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x4au32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x46u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3eu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x93u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x7au32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.flat_ds(0x109a370); cpu.push(0x109a375); f_010a69d3(cpu); if cpu.returned_to != 0x109a375 { return; }
  { let v: u32 = 0xffffffd2u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  let t = cpu.r32(0x1134e44u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x109a378); cpu.push(0x109a37e); cpu.call(t); if cpu.returned_to != 0x109a37e { return; }
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x109a381); cpu.push(0x109a386); f_01094a9a(cpu); if cpu.returned_to != 0x109a386 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109a3d6; continue; }
  pc = 0x109a38d; continue;
 }
 0x109a38d => {
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x109a396); cpu.push(0x109a39b); f_0109a22e(cpu); if cpu.returned_to != 0x109a39b { return; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x109a3ad; continue; }
  pc = 0x109a39f; continue;
 }
 0x109a39f => {
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x109a3af; continue;
 }
 0x109a3ad => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109a3af; continue;
 }
 0x109a3af => {
  { let v: u32 = cpu.eax; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109a3d9; continue; }
  pc = 0x109a3b6; continue;
 }
 0x109a3b6 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.flat_ds(0x109a3bb); cpu.push(0x109a3c0); f_01099923(cpu); if cpu.returned_to != 0x109a3c0 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x109a3cc); cpu.push(0x109a3d1); f_01099946(cpu); if cpu.returned_to != 0x109a3d1 { return; }
  pc = 0x109a593; continue;
 }
 0x109a3d6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  pc = 0x109a3d9; continue;
 }
 0x109a3d9 => {
  { let v: u32 = cpu.r8(0x1133e87u32.wrapping_add(cpu.bases[3])) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x109a3fc; continue; }
  pc = 0x109a3e5; continue;
 }
 0x109a3e5 => {
  { let v: u32 = cpu.r32(0x46u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x109a3fc; continue; }
  pc = 0x109a3ed; continue;
 }
 0x109a3ed => {
  { let v: u32 = cpu.r32(0x4au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x109a3fc; continue; }
  pc = 0x109a3f5; continue;
 }
 0x109a3f5 => {
  { let v: u32 = 0x11389c4u32; cpu.w32(0x4au32.wrapping_add(cpu.ebp), v); }
  pc = 0x109a3fc; continue;
 }
 0x109a3fc => {
  { let v = cpu.r32(0x42u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x3eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4au32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x46u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x109a40b); cpu.push(0x109a410); f_010a6ac3(cpu); if cpu.returned_to != 0x109a410 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.flat_ds(0x109a415); cpu.push(0x109a41a); f_01099923(cpu); if cpu.returned_to != 0x109a41a { return; }
  { let v: u32 = cpu.r32(0x42u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x109a46c; continue; }
  pc = 0x109a422; continue;
 }
 0x109a422 => {
  { let v: u32 = 0x11389c7u32; cpu.edx = v; }
  cpu.flat_ds(0x109a427); cpu.push(0x109a42c); f_0108ddc3(cpu); if cpu.returned_to != 0x109a42c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109a44c; continue; }
  pc = 0x109a430; continue;
 }
 0x109a430 => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x109a43a); cpu.push(0x109a43f); f_0109a25d(cpu); if cpu.returned_to != 0x109a43f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109a530; continue; }
  pc = 0x109a447; continue;
 }
 0x109a447 => {
  pc = 0x109a593; continue;
 }
 0x109a44c => {
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x109a454); cpu.push(0x109a459); f_01099923(cpu); if cpu.returned_to != 0x109a459 { return; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x109a462); cpu.push(0x109a467); f_0109a27c(cpu); if cpu.returned_to != 0x109a467 { return; }
  pc = 0x109a590; continue;
 }
 0x109a46c => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x109a472); cpu.push(0x109a477); f_0108e46c(cpu); if cpu.returned_to != 0x109a477 { return; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x6au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x7eu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x109a48e; continue; }
  pc = 0x109a482; continue;
 }
 0x109a482 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.flat_ds(0x109a487); cpu.push(0x109a48c); f_01099923(cpu); if cpu.returned_to != 0x109a48c { return; }
  pc = 0x109a4b7; continue;
 }
 0x109a48e => {
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x11389ccu32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  cpu.flat_ds(0x109a4a1); cpu.push(0x109a4a6); f_01099923(cpu); if cpu.returned_to != 0x109a4a6 { return; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x109a4af); cpu.push(0x109a4b4); f_0109a27c(cpu); if cpu.returned_to != 0x109a4b4 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  pc = 0x109a4b7; continue;
 }
 0x109a4b7 => {
  cpu.flat_ds(0x109a4b7); cpu.push(0x109a4bc); f_010a4ca4(cpu); if cpu.returned_to != 0x109a4bc { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x109a4cf; continue; }
  pc = 0x109a4c1; continue;
 }
 0x109a4c1 => {
  cpu.flat_ds(0x109a4c1); cpu.push(0x109a4c6); f_010a4ca4(cpu); if cpu.returned_to != 0x109a4c6 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x9u32); }
  if !cpu.zf { pc = 0x109a593; continue; }
  pc = 0x109a4cf; continue;
 }
 0x109a4cf => {
  { let v: u32 = cpu.r32(0x6au32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x109a4da); cpu.push(0x109a4df); f_01099923(cpu); if cpu.returned_to != 0x109a4df { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x11389d1u32; cpu.esi = v; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.flat_ds(0x109a4f1); cpu.push(0x109a4f6); f_0109a27c(cpu); if cpu.returned_to != 0x109a4f6 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x109a4f9); cpu.push(0x109a4fe); f_010a4ca4(cpu); if cpu.returned_to != 0x109a4fe { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x109a511; continue; }
  pc = 0x109a503; continue;
 }
 0x109a503 => {
  cpu.flat_ds(0x109a503); cpu.push(0x109a508); f_010a4ca4(cpu); if cpu.returned_to != 0x109a508 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x9u32); }
  if !cpu.zf { pc = 0x109a593; continue; }
  pc = 0x109a511; continue;
 }
 0x109a511 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6au32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.flat_ds(0x109a516); cpu.push(0x109a51b); f_01099923(cpu); if cpu.returned_to != 0x109a51b { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x11389c7u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  cpu.flat_ds(0x109a527); cpu.push(0x109a52c); f_0109a25d(cpu); if cpu.returned_to != 0x109a52c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109a593; continue; }
  pc = 0x109a530; continue;
 }
 0x109a530 => {
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x109a53e); cpu.push(0x109a543); f_01094b7d(cpu); if cpu.returned_to != 0x109a543 { return; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5eu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x109a54d); cpu.push(0x109a552); f_010a6860(cpu); if cpu.returned_to != 0x109a552 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r8(0x7eu32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0x4eu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.flat_ds(0x109a561); cpu.push(0x109a566); f_0109a237(cpu); if cpu.returned_to != 0x109a566 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x7eu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x109a574; continue; }
  pc = 0x109a56d; continue;
 }
 0x109a56d => {
  { let v: u32 = 0x11389deu32; cpu.eax = v; }
  pc = 0x109a579; continue;
 }
 0x109a574 => {
  { let v: u32 = 0x11389e2u32; cpu.eax = v; }
  pc = 0x109a579; continue;
 }
 0x109a579 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11389d6u32; cpu.eax = v; }
  cpu.flat_ds(0x109a57f); cpu.push(0x109a584); f_0108cfa7(cpu); if cpu.returned_to != 0x109a584 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.flat_ds(0x109a588); cpu.push(0x109a58d); f_0108eb78(cpu); if cpu.returned_to != 0x109a58d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x109a590; continue;
 }
 0x109a590 => {
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  pc = 0x109a593; continue;
 }
 0x109a593 => {
  { let v: u32 = cpu.r32(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x109a596); cpu.push(0x109a59b); f_01094b7d(cpu); if cpu.returned_to != 0x109a59b { return; }
  { let v: u32 = cpu.r32(0x66u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x109a59e); cpu.push(0x109a5a3); f_01094b7d(cpu); if cpu.returned_to != 0x109a5a3 { return; }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x109a5a6); cpu.push(0x109a5ab); f_01094b7d(cpu); if cpu.returned_to != 0x109a5ab { return; }
  { let v: u32 = 0xffffffd2u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  let t = cpu.r32(0x1134e48u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x109a5ae); cpu.push(0x109a5b4); cpu.call(t); if cpu.returned_to != 0x109a5b4 { return; }
  { let v: u32 = cpu.r32(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x109a5b7; continue;
 }
 0x109a5b7 => {
  { let v: u32 = 0x82u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x109a5c1); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a5c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a5c2); }
 let mut pc: u32 = 0x109a5c2;
 loop { match pc {
 0x109a5c2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x109a5c7); cpu.push(0x109a5cc); f_0109a5fa(cpu); if cpu.returned_to != 0x109a5cc { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.r16(0xcu32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x14u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x109a5f9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a5fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a5fa); }
 let mut pc: u32 = 0x109a5fa;
 loop { match pc {
 0x109a5fa => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.esi = v; }
  { let v: u32 = 0x109a6a0u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v = cpu.ebp; cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a66c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a66c); }
 let mut pc: u32 = 0x109a66c;
 loop { match pc {
 0x109a66c => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x109a6a0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v = cpu.r16(0x1eu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v = cpu.r16(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.set_segment(4, v as u16); }
  { let v = cpu.r16(0x22u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6a0); }
 let mut pc: u32 = 0x109a6a0;
 loop { match pc {
 0x109a6a0 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6a3); }
 let mut pc: u32 = 0x109a6a3;
 loop { match pc {
 0x109a6a3 => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6ac); }
 let mut pc: u32 = 0x109a6ac;
 loop { match pc {
 0x109a6ac => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6af); }
 let mut pc: u32 = 0x109a6af;
 loop { match pc {
 0x109a6af => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6b2); }
 let mut pc: u32 = 0x109a6b2;
 loop { match pc {
 0x109a6b2 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6b5); }
 let mut pc: u32 = 0x109a6b5;
 loop { match pc {
 0x109a6b5 => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6b8); }
 let mut pc: u32 = 0x109a6b8;
 loop { match pc {
 0x109a6b8 => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6bb); }
 let mut pc: u32 = 0x109a6bb;
 loop { match pc {
 0x109a6bb => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6be); }
 let mut pc: u32 = 0x109a6be;
 loop { match pc {
 0x109a6be => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6c1); }
 let mut pc: u32 = 0x109a6c1;
 loop { match pc {
 0x109a6c1 => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6c4); }
 let mut pc: u32 = 0x109a6c4;
 loop { match pc {
 0x109a6c4 => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6c7); }
 let mut pc: u32 = 0x109a6c7;
 loop { match pc {
 0x109a6c7 => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6ca); }
 let mut pc: u32 = 0x109a6ca;
 loop { match pc {
 0x109a6ca => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6cd); }
 let mut pc: u32 = 0x109a6cd;
 loop { match pc {
 0x109a6cd => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6d0); }
 let mut pc: u32 = 0x109a6d0;
 loop { match pc {
 0x109a6d0 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6d3); }
 let mut pc: u32 = 0x109a6d3;
 loop { match pc {
 0x109a6d3 => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6d6); }
 let mut pc: u32 = 0x109a6d6;
 loop { match pc {
 0x109a6d6 => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6d9); }
 let mut pc: u32 = 0x109a6d9;
 loop { match pc {
 0x109a6d9 => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6dc); }
 let mut pc: u32 = 0x109a6dc;
 loop { match pc {
 0x109a6dc => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6df); }
 let mut pc: u32 = 0x109a6df;
 loop { match pc {
 0x109a6df => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6e2); }
 let mut pc: u32 = 0x109a6e2;
 loop { match pc {
 0x109a6e2 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6e5); }
 let mut pc: u32 = 0x109a6e5;
 loop { match pc {
 0x109a6e5 => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6e8); }
 let mut pc: u32 = 0x109a6e8;
 loop { match pc {
 0x109a6e8 => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6eb); }
 let mut pc: u32 = 0x109a6eb;
 loop { match pc {
 0x109a6eb => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6ee); }
 let mut pc: u32 = 0x109a6ee;
 loop { match pc {
 0x109a6ee => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6f1); }
 let mut pc: u32 = 0x109a6f1;
 loop { match pc {
 0x109a6f1 => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6f4); }
 let mut pc: u32 = 0x109a6f4;
 loop { match pc {
 0x109a6f4 => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6f7); }
 let mut pc: u32 = 0x109a6f7;
 loop { match pc {
 0x109a6f7 => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6fa); }
 let mut pc: u32 = 0x109a6fa;
 loop { match pc {
 0x109a6fa => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a6fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a6fd); }
 let mut pc: u32 = 0x109a6fd;
 loop { match pc {
 0x109a6fd => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a700); }
 let mut pc: u32 = 0x109a700;
 loop { match pc {
 0x109a700 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a703(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a703); }
 let mut pc: u32 = 0x109a703;
 loop { match pc {
 0x109a703 => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a706(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a706); }
 let mut pc: u32 = 0x109a706;
 loop { match pc {
 0x109a706 => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a709(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a709); }
 let mut pc: u32 = 0x109a709;
 loop { match pc {
 0x109a709 => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a70c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a70c); }
 let mut pc: u32 = 0x109a70c;
 loop { match pc {
 0x109a70c => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a70f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a70f); }
 let mut pc: u32 = 0x109a70f;
 loop { match pc {
 0x109a70f => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a712(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a712); }
 let mut pc: u32 = 0x109a712;
 loop { match pc {
 0x109a712 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a715(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a715); }
 let mut pc: u32 = 0x109a715;
 loop { match pc {
 0x109a715 => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a718(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a718); }
 let mut pc: u32 = 0x109a718;
 loop { match pc {
 0x109a718 => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a71b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a71b); }
 let mut pc: u32 = 0x109a71b;
 loop { match pc {
 0x109a71b => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a71e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a71e); }
 let mut pc: u32 = 0x109a71e;
 loop { match pc {
 0x109a71e => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a721(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a721); }
 let mut pc: u32 = 0x109a721;
 loop { match pc {
 0x109a721 => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a724(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a724); }
 let mut pc: u32 = 0x109a724;
 loop { match pc {
 0x109a724 => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a727(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a727); }
 let mut pc: u32 = 0x109a727;
 loop { match pc {
 0x109a727 => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a72a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a72a); }
 let mut pc: u32 = 0x109a72a;
 loop { match pc {
 0x109a72a => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a72d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a72d); }
 let mut pc: u32 = 0x109a72d;
 loop { match pc {
 0x109a72d => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a730); }
 let mut pc: u32 = 0x109a730;
 loop { match pc {
 0x109a730 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a733(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a733); }
 let mut pc: u32 = 0x109a733;
 loop { match pc {
 0x109a733 => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a736(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a736); }
 let mut pc: u32 = 0x109a736;
 loop { match pc {
 0x109a736 => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a739(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a739); }
 let mut pc: u32 = 0x109a739;
 loop { match pc {
 0x109a739 => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a73c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a73c); }
 let mut pc: u32 = 0x109a73c;
 loop { match pc {
 0x109a73c => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a73f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a73f); }
 let mut pc: u32 = 0x109a73f;
 loop { match pc {
 0x109a73f => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a742(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a742); }
 let mut pc: u32 = 0x109a742;
 loop { match pc {
 0x109a742 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a745(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a745); }
 let mut pc: u32 = 0x109a745;
 loop { match pc {
 0x109a745 => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a748(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a748); }
 let mut pc: u32 = 0x109a748;
 loop { match pc {
 0x109a748 => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a74b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a74b); }
 let mut pc: u32 = 0x109a74b;
 loop { match pc {
 0x109a74b => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a74e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a74e); }
 let mut pc: u32 = 0x109a74e;
 loop { match pc {
 0x109a74e => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a751(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a751); }
 let mut pc: u32 = 0x109a751;
 loop { match pc {
 0x109a751 => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a754(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a754); }
 let mut pc: u32 = 0x109a754;
 loop { match pc {
 0x109a754 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a757(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a757); }
 let mut pc: u32 = 0x109a757;
 loop { match pc {
 0x109a757 => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a75a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a75a); }
 let mut pc: u32 = 0x109a75a;
 loop { match pc {
 0x109a75a => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a75d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a75d); }
 let mut pc: u32 = 0x109a75d;
 loop { match pc {
 0x109a75d => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a760); }
 let mut pc: u32 = 0x109a760;
 loop { match pc {
 0x109a760 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a763(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a763); }
 let mut pc: u32 = 0x109a763;
 loop { match pc {
 0x109a763 => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a766(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a766); }
 let mut pc: u32 = 0x109a766;
 loop { match pc {
 0x109a766 => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a769(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a769); }
 let mut pc: u32 = 0x109a769;
 loop { match pc {
 0x109a769 => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a76c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a76c); }
 let mut pc: u32 = 0x109a76c;
 loop { match pc {
 0x109a76c => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a76f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a76f); }
 let mut pc: u32 = 0x109a76f;
 loop { match pc {
 0x109a76f => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a772(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a772); }
 let mut pc: u32 = 0x109a772;
 loop { match pc {
 0x109a772 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a775(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a775); }
 let mut pc: u32 = 0x109a775;
 loop { match pc {
 0x109a775 => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a778(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a778); }
 let mut pc: u32 = 0x109a778;
 loop { match pc {
 0x109a778 => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a77b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a77b); }
 let mut pc: u32 = 0x109a77b;
 loop { match pc {
 0x109a77b => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a77e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a77e); }
 let mut pc: u32 = 0x109a77e;
 loop { match pc {
 0x109a77e => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a781(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a781); }
 let mut pc: u32 = 0x109a781;
 loop { match pc {
 0x109a781 => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a784(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a784); }
 let mut pc: u32 = 0x109a784;
 loop { match pc {
 0x109a784 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a787(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a787); }
 let mut pc: u32 = 0x109a787;
 loop { match pc {
 0x109a787 => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a78a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a78a); }
 let mut pc: u32 = 0x109a78a;
 loop { match pc {
 0x109a78a => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a78d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a78d); }
 let mut pc: u32 = 0x109a78d;
 loop { match pc {
 0x109a78d => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a790); }
 let mut pc: u32 = 0x109a790;
 loop { match pc {
 0x109a790 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0109a793(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x109a793); }
 let mut pc: u32 = 0x109a793;
 loop { match pc {
 0x109a793 => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
