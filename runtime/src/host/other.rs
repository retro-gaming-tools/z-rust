//! Small DLLs.

use super::{ddraw, table};
use crate::cpu::{Cpu, GuestFn};

const DSERR_GENERIC: u32 = 0x8000_4005;

/// The game needs a lobby object to start, and only asks it how it was launched.
fn create_lobby(cpu: &mut Cpu) -> u32 {
    const SLOTS: u32 = 16;
    const GET_CONNECTION_SETTINGS: u32 = 0x20;
    const DPERR_NOTLOBBIED: u32 = 0x8877_042e;
    const ENUM_ADDRESS_TYPES: u32 = 0x18;
    let methods: [ddraw::Method; 4] = [
        (0x04, 1, (|cpu: &mut Cpu| cpu.finish(1, 1)) as GuestFn),
        (0x08, 1, (|cpu: &mut Cpu| cpu.finish(0, 1)) as GuestFn),
        // Multiplayer isn't included: no service provider has address types,
        // so the game's list of links stays empty.
        (ENUM_ADDRESS_TYPES, 5, (|cpu: &mut Cpu| cpu.finish(0, 5)) as GuestFn),
        (GET_CONNECTION_SETTINGS, 4, (|cpu: &mut Cpu| cpu.finish(DPERR_NOTLOBBIED, 4)) as GuestFn),
    ];
    let table = ddraw::build_vtable(cpu, "IDirectPlayLobby", SLOTS, &methods);
    let lobby = ddraw::object(cpu, table);
    let out = cpu.arg(1);
    cpu.w32(out, lobby);
    0
}

/// Multiplayer isn't included, so DirectPlay itself reports that it isn't available.
pub fn find_dplay(name: &str) -> Option<GuestFn> {
    table! { name;
        "#1" 3 |_cpu| { DSERR_GENERIC }
        "#2" 2 |_cpu| { DSERR_GENERIC }
        "#4" 5 |cpu| { create_lobby(cpu) }
    }
}

/// MIDI streams, played by the synthesizer in `midi`.
pub fn find_winmm(name: &str) -> Option<GuestFn> {
    use super::midi;
    const HEADER_FLAGS: u32 = 16;
    const PREPARED: u32 = 2;
    table! { name;
        "midiOutGetDevCapsA" 3 |cpu| { let (at, size) = (cpu.arg(1), cpu.arg(2)); midi::capabilities(cpu, at, size) }
        "midiOutGetNumDevs" 0 |_cpu| { 1 }
        "midiStreamOpen" 6 |cpu| {
            let (out, callback, instance, flags) = (cpu.arg(0), cpu.arg(3), cpu.arg(4), cpu.arg(5));
            midi::open(cpu, out, callback, instance, flags)
        }
        "midiStreamProperty" 3 |cpu| { let (handle, data, flags) = (cpu.arg(0), cpu.arg(1), cpu.arg(2)); midi::property(cpu, handle, data, flags) }
        "midiStreamOut" 3 |cpu| { let (handle, header) = (cpu.arg(0), cpu.arg(1)); midi::out(cpu, handle, header) }
        "midiStreamRestart" 1 |cpu| { let handle = cpu.arg(0); midi::set_playing(cpu, handle, true) }
        "midiStreamPause" 1 |cpu| { let handle = cpu.arg(0); midi::set_playing(cpu, handle, false) }
        "midiStreamStop" 1 |cpu| { let handle = cpu.arg(0); midi::stop(cpu, handle, false) }
        "midiStreamClose" 1 |cpu| { let handle = cpu.arg(0); midi::close(cpu, handle) }
        "midiOutGetVolume" 2 |cpu| { let at = cpu.arg(1); midi::volume(cpu, at) }
        "midiOutPrepareHeader" 3 |cpu| { let at = cpu.arg(1) + HEADER_FLAGS; let flags = cpu.r32(at); cpu.w32(at, flags | PREPARED); 0 }
        "midiOutUnprepareHeader" 3 |cpu| { let at = cpu.arg(1) + HEADER_FLAGS; let flags = cpu.r32(at); cpu.w32(at, flags & !PREPARED); 0 }
        "midiOutReset" 1 |cpu| { let handle = cpu.arg(0); midi::stop(cpu, handle, true) }
        "midiOutSetVolume" 2 |cpu| { let volume = cpu.arg(1); midi::set_volume(cpu, volume) }
        "timeSetEvent" 5 |cpu| { super::timers::set_event(cpu) }
        "timeKillEvent" 1 |cpu| { super::timers::kill_event(cpu) }
    }
}

/// Video for Windows draws pictures into the window with GDI. The window only
/// shows DirectDraw's frames so far, so these succeed and draw nothing.
pub fn find_video_for_windows(name: &str) -> Option<GuestFn> {
    table! { name;
        "DrawDibOpen" 0 |cpu| { cpu.host.handle() }
        "DrawDibBegin" 8 |_cpu| { 1 }
        "DrawDibDraw" 13 |cpu| {
            let (header, bits) = (cpu.arg(6), cpu.arg(7));
            // A picture on the window is where a game with a menu bar waits.
            cpu.host.menu_due = true;
            u32::from(super::ddraw::draw_dib(cpu, header, bits))
        }
        "DrawDibClose" 1 |_cpu| { 1 }
    }
}

/// Common controls are never drawn: the game gets handles, and nothing is shown.
pub fn find_comctl(name: &str) -> Option<GuestFn> {
    table! { name;
        "#17" 0 |_cpu| { 0 } // InitCommonControls
        "ImageList_Create" 5 |cpu| { cpu.host.handle() }
        "ImageList_ReplaceIcon" 3 |_cpu| { 0 }
    }
}

pub fn find_audio(name: &str) -> Option<GuestFn> {
    if let Some(function) = super::audio::find_mci(name) {
        return Some(function);
    }
    table! { name;
        "mciGetErrorStringA" 3 |cpu| { let at = cpu.arg(1); cpu.w8(at, 0); 1 }
    }
}
