//! The keys the port adds over the game (frame counter, fullscreen and the like), which the
//! player can move in the launcher's "Hotkeys..." page when one clashes with a key the game
//! uses. The choices are kept in `settings.ini` as `hotkey.NAME=KEY`.
//!
//! Every runtime copy has this file. Only `HOTKEYS` and the index constants differ: they list
//! the keys that port's runtime handles. The rest is the same everywhere.

use std::sync::Mutex;

/// A key of the port's, by the name it's saved under and the label the launcher shows.
pub struct Hotkey {
    pub name: &'static str,
    pub label: &'static str,
    /// An SDL keycode.
    pub default: u32,
    /// Whether the game gets the key as well, for a key the game binds itself (such as a
    /// quick load that a mod also counts).
    pub to_game: bool,
}

const KEY_F1: u32 = 0x4000_003a;
const fn f(number: u32) -> u32 {
    KEY_F1 + number - 1
}

pub const STATS: usize = 0;
pub const FULLSCREEN: usize = 1;

pub const HOTKEYS: &[Hotkey] = &[
    Hotkey { name: "stats", label: "Frame counter", default: f(10), to_game: false },
    Hotkey { name: "fullscreen", label: "Fullscreen", default: f(11), to_game: false },
];

/// The keys chosen, one per hotkey; empty until something is loaded or chosen.
static CHOSEN: Mutex<Vec<u32>> = Mutex::new(Vec::new());

/// The keys now, one per hotkey.
pub fn all() -> Vec<u32> {
    let chosen = CHOSEN.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if chosen.len() == HOTKEYS.len() { chosen.clone() } else { HOTKEYS.iter().map(|hotkey| hotkey.default).collect() }
}

fn store(keys: Vec<u32>) {
    *CHOSEN.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = keys;
}

/// The key for a hotkey now.
pub fn key(index: usize) -> u32 {
    all()[index]
}

/// A hotkey with no key: the player turned it off, and no key triggers it.
pub const NONE: u32 = 0;

/// The hotkey a key stands for, if any.
pub fn action(key: u32) -> Option<usize> {
    if key == NONE {
        return None; // SDL's unknown key, never a turned-off hotkey.
    }
    all().iter().position(|chosen| *chosen == key)
}

/// Whether a key is the frame counter's or the fullscreen key, which work in the port's own
/// windows too.
pub fn is_window_key(key: u32) -> bool {
    matches!(action(key), Some(STATS | FULLSCREEN))
}

/// Gives a hotkey a key, or `NONE` to turn it off. A hotkey that had that key takes this
/// one's old key, so no two share.
pub fn set(index: usize, key: u32) {
    let mut keys = all();
    if let Some(other) = keys.iter().position(|chosen| *chosen == key && key != NONE) {
        keys[other] = keys[index];
    }
    keys[index] = key;
    store(keys);
}

pub fn reset() {
    store(HOTKEYS.iter().map(|hotkey| hotkey.default).collect());
}

/// Reads one `settings.ini` line's key and value; true when it was a hotkey's.
pub fn read_setting(name: &str, value: &str) -> bool {
    let Some(name) = name.strip_prefix("hotkey.") else { return false };
    if let (Some(index), Some(key)) = (HOTKEYS.iter().position(|hotkey| hotkey.name == name), from_name(value.trim())) {
        set(index, key);
    }
    true
}

/// The lines `settings.ini` keeps, one per hotkey.
pub fn setting_lines() -> String {
    HOTKEYS.iter().zip(all()).map(|(hotkey, key)| format!("hotkey.{}={}\n", hotkey.name, saved_name(key))).collect()
}

/// A key's name as saved: `none`, `F10`, a letter or digit, or the SDL keycode in hex.
fn saved_name(key: u32) -> String {
    match key {
        NONE => "none".to_string(),
        _ if (KEY_F1..KEY_F1 + 12).contains(&key) => format!("F{}", key - KEY_F1 + 1),
        _ if char::from_u32(key).is_some_and(|c| c.is_ascii_alphanumeric()) => (key as u8 as char).to_ascii_uppercase().to_string(),
        _ => format!("0x{key:x}"),
    }
}

fn from_name(text: &str) -> Option<u32> {
    if text.eq_ignore_ascii_case("none") || text.is_empty() {
        return Some(NONE);
    }
    if let Some(number) = text.strip_prefix(['F', 'f']).and_then(|n| n.parse::<u32>().ok()).filter(|n| (1..=12).contains(n)) {
        return Some(f(number));
    }
    if let Some(hex) = text.strip_prefix("0x") {
        return u32::from_str_radix(hex, 16).ok().filter(|key| *key != 0);
    }
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) if c.is_ascii_alphanumeric() => Some(c.to_ascii_lowercase() as u32),
        _ => None,
    }
}

/// A key's name for the launcher: SDL's, when it has one, else the saved name.
pub fn display_name(key: u32) -> String {
    if key == NONE {
        return "None".to_string();
    }
    let saved = saved_name(key);
    if !saved.starts_with("0x") {
        return saved;
    }
    unsafe {
        let library = crate::sys::open_sdl();
        if library.is_null() {
            return saved;
        }
        let name: unsafe extern "C" fn(u32) -> *const std::ffi::c_char = crate::host::video::symbol(library, "SDL_GetKeyName");
        let text = name(key);
        if text.is_null() {
            return saved;
        }
        let text = std::ffi::CStr::from_ptr(text).to_string_lossy().into_owned();
        if text.is_empty() { saved } else { text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_read_back() {
        for key in [f(1), f(9), f(12), 'q' as u32, '7' as u32, 0x4000_0049] {
            assert_eq!(from_name(&saved_name(key)), Some(key));
        }
        assert_eq!(from_name("f10"), Some(f(10)));
        assert_eq!(from_name("F13"), None);
        assert_eq!(from_name("0x0"), None);
        assert_eq!(from_name("none"), Some(NONE));
        assert_eq!(from_name(""), Some(NONE));
        assert_eq!(saved_name(NONE), "none");
    }

    #[test]
    fn a_hotkey_can_be_turned_off() {
        reset();
        set(FULLSCREEN, NONE);
        set(STATS, NONE);
        // Both off: neither takes the other's key, and no key triggers them.
        assert_eq!((key(FULLSCREEN), key(STATS)), (NONE, NONE));
        assert_eq!(action(f(11)), None);
        assert_eq!(action(NONE), None);
        assert!(setting_lines().contains("hotkey.fullscreen=none\n"));
        assert!(read_setting("hotkey.fullscreen", "F11"));
        assert_eq!(action(f(11)), Some(FULLSCREEN));
        reset();
    }

    #[test]
    fn keys_never_share_and_save() {
        reset();
        // Giving the frame counter the fullscreen key swaps the two.
        set(STATS, f(11));
        assert_eq!((key(STATS), key(FULLSCREEN)), (f(11), f(10)));
        assert_eq!(action(f(10)), Some(FULLSCREEN));
        assert!(setting_lines().contains("hotkey.stats=F11\n"));
        assert!(read_setting("hotkey.stats", "F10"));
        assert!(!read_setting("scale", "2"));
        assert_eq!((key(STATS), key(FULLSCREEN)), (f(10), f(11)));
        reset();
    }
}
