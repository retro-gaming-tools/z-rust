//! The Miles Sound System (AIL 3) in `ZED.EXE`, played by the runtime.
//!
//! Miles loads real-mode drivers for the sound card that `SETSOUND.EXE` set up
//! (`MDI.INI`, `DIG.INI`) and calls them through `int 66h`. None can run here,
//! and players shouldn't set up drivers. So the API functions the game calls
//! are replaced by the runtime's `host::miles`: samples go to its mixer, and
//! XMIDI music to its General MIDI synthesizer, as with a General MIDI card
//! and a Sound Blaster set up. Miles' start-up, timers and shut-down stay the
//! original code: the game's timer runs through them.
//!
//! Addresses: each function was found by the debug text it prints (such as
//! "AIL_start_sample(0x%X)"). The linked address + 0xFF0000 is the name.
//! The game picks its music by `AIL_MDI_driver_type`: General MIDI is type 3,
//! which it reads as the `F*.XMI` files.

use recomp_runtime::host::miles;
use recomp_runtime::Cpu;

/// AIL_install_DIG_INI, 0xa0c42.
pub fn f_01090c42(cpu: &mut Cpu) { miles::install_dig_ini(cpu) }
/// AIL_allocate_sample_handle, 0xa0e8d.
pub fn f_01090e8d(cpu: &mut Cpu) { miles::allocate_sample_handle(cpu) }
/// AIL_release_sample_handle, 0xa1071.
pub fn f_01091071(cpu: &mut Cpu) { miles::release_sample_handle(cpu) }
/// AIL_init_sample, 0xa10de.
pub fn f_010910de(cpu: &mut Cpu) { miles::init_sample(cpu) }
/// AIL_set_sample_address, 0xa1251.
pub fn f_01091251(cpu: &mut Cpu) { miles::set_sample_address(cpu) }
/// AIL_set_sample_type, 0xa12d3.
pub fn f_010912d3(cpu: &mut Cpu) { miles::set_sample_type(cpu) }
/// AIL_start_sample, 0xa1355.
pub fn f_01091355(cpu: &mut Cpu) { miles::start_sample(cpu) }
/// AIL_end_sample, 0xa149c.
pub fn f_0109149c(cpu: &mut Cpu) { miles::end_sample(cpu) }
/// AIL_set_sample_playback_rate, 0xa1509.
pub fn f_01091509(cpu: &mut Cpu) { miles::set_sample_playback_rate(cpu) }
/// AIL_set_sample_volume, 0xa157f.
pub fn f_0109157f(cpu: &mut Cpu) { miles::set_sample_volume(cpu) }
/// AIL_set_sample_pan, 0xa15f5.
pub fn f_010915f5(cpu: &mut Cpu) { miles::set_sample_pan(cpu) }
/// AIL_set_sample_loop_count, 0xa166b.
pub fn f_0109166b(cpu: &mut Cpu) { miles::set_sample_loop_count(cpu) }
/// AIL_sample_status, 0xa16e1.
pub fn f_010916e1(cpu: &mut Cpu) { miles::sample_status(cpu) }
/// AIL_set_digital_master_volume, 0xa1b5a.
pub fn f_01091b5a(cpu: &mut Cpu) { miles::set_digital_master_volume(cpu) }
/// AIL_sample_buffer_ready, 0xa1ea7.
pub fn f_01091ea7(cpu: &mut Cpu) { miles::sample_buffer_ready(cpu) }
/// AIL_load_sample_buffer, 0xa1f8c.
pub fn f_01091f8c(cpu: &mut Cpu) { miles::load_sample_buffer(cpu) }
/// AIL_sample_position, 0xa2091.
pub fn f_01092091(cpu: &mut Cpu) { miles::sample_position(cpu) }
/// AIL_register_EOB_callback, 0xa2263.
pub fn f_01092263(cpu: &mut Cpu) { miles::register_eob_callback(cpu) }
/// AIL_install_MDI_INI, 0xa277e.
pub fn f_0109277e(cpu: &mut Cpu) { miles::install_mdi_ini(cpu) }
/// AIL_allocate_sequence_handle, 0xa29bd.
pub fn f_010929bd(cpu: &mut Cpu) { miles::allocate_sequence_handle(cpu) }
/// AIL_release_sequence_handle, 0xa2aa2.
pub fn f_01092aa2(cpu: &mut Cpu) { miles::release_sequence_handle(cpu) }
/// AIL_init_sequence, 0xa2b0f.
pub fn f_01092b0f(cpu: &mut Cpu) { miles::init_sequence(cpu) }
/// AIL_start_sequence, 0xa2c08.
pub fn f_01092c08(cpu: &mut Cpu) { miles::start_sequence(cpu) }
/// AIL_stop_sequence, 0xa2c75.
pub fn f_01092c75(cpu: &mut Cpu) { miles::stop_sequence(cpu) }
/// AIL_end_sequence, 0xa2d4f.
pub fn f_01092d4f(cpu: &mut Cpu) { miles::end_sequence(cpu) }
/// AIL_set_sequence_volume, 0xa2e3e.
pub fn f_01092e3e(cpu: &mut Cpu) { miles::set_sequence_volume(cpu) }
/// AIL_sequence_status, 0xa2f36.
pub fn f_01092f36(cpu: &mut Cpu) { miles::sequence_status(cpu) }
/// AIL_set_XMIDI_master_volume, 0xa32ca.
pub fn f_010932ca(cpu: &mut Cpu) { miles::set_xmidi_master_volume(cpu) }
/// AIL_MDI_driver_type, 0xa351e.
pub fn f_0109351e(cpu: &mut Cpu) { miles::mdi_driver_type(cpu) }
/// AIL_branch_index, 0xa3d39.
pub fn f_01093d39(cpu: &mut Cpu) { miles::branch_index(cpu) }
/// AIL_register_trigger_callback, 0xa3e9c.
pub fn f_01093e9c(cpu: &mut Cpu) { miles::register_trigger_callback(cpu) }
