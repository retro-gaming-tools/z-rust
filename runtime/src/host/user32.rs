//! user32 and gdi32: one window, its messages, and what the game asks of GDI.

use super::table;
use crate::cpu::{Cpu, GuestFn};
use crate::host::{audio, dialog_box, gamepad, script, video};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const MSG_SIZE: u32 = 28;
const WM_CREATE: u32 = 1;
const WM_DESTROY: u32 = 2;
const WM_NCDESTROY: u32 = 0x82;
const WM_ACTIVATEAPP: u32 = 0x1c;
const WM_ACTIVATE: u32 = 6;
const WM_SETFOCUS: u32 = 7;
const WM_SHOWWINDOW: u32 = 0x18;
const WM_PAINT: u32 = 0xf;
const WM_TIMER: u32 = 0x113;

/// Ends a function in a `table!` entry early, with a result of FALSE.
macro_rules! return_false {
    () => {
        return 0
    };
}

/// Ends a function in a `table!` entry early, with this result.
macro_rules! return_value {
    ($value:expr) => {
        return $value
    };
}

/// Sends a message to the game's window procedure and returns its result.
pub fn send(cpu: &mut Cpu, message: u32, wparam: u32, lparam: u32) -> u32 {
    let (procedure, window) = (cpu.host.window_procedure, cpu.host.window);
    if procedure == 0 {
        return 0;
    }
    let saved = (cpu.ebx, cpu.esi, cpu.edi, cpu.ebp);
    cpu.push(lparam);
    cpu.push(wparam);
    cpu.push(message);
    cpu.push(window);
    cpu.push(0); // Its `ret 16` pops this and the four arguments.
    cpu.call(procedure);
    super::returned_to_runtime(cpu, procedure);
    (cpu.ebx, cpu.esi, cpu.edi, cpu.ebp) = saved;
    cpu.eax
}

/// The printf that Windows has: no floats, and %ld and %lu mean the same as %d and %u.
fn format(cpu: &Cpu, pattern: &str, mut next: impl FnMut(&Cpu) -> u32) -> String {
    let mut output = String::new();
    let mut characters = pattern.chars().peekable();
    while let Some(character) = characters.next() {
        if character != '%' {
            output.push(character);
            continue;
        }
        let mut left = false;
        let mut zero = false;
        let mut width = 0usize;
        while let Some(&flag) = characters.peek() {
            match flag {
                '-' => left = true,
                '0' => zero = true,
                '+' | ' ' | '#' => {}
                _ => break,
            }
            characters.next();
        }
        while let Some(digit) = characters.peek().and_then(|c| c.to_digit(10)) {
            width = width * 10 + digit as usize;
            characters.next();
        }
        if characters.peek() == Some(&'.') {
            characters.next();
            while characters.peek().is_some_and(|c| c.is_ascii_digit()) {
                characters.next();
            }
        }
        while matches!(characters.peek(), Some('l' | 'h')) {
            characters.next();
        }
        let text = match characters.next() {
            Some('d' | 'i') => (next(cpu) as i32).to_string(),
            Some('u') => next(cpu).to_string(),
            Some('x') => format!("{:x}", next(cpu)),
            Some('X') => format!("{:X}", next(cpu)),
            Some('c') => ((next(cpu) as u8) as char).to_string(),
            Some('s') => cpu.string(next(cpu)),
            Some('%') => "%".to_string(),
            _ => String::new(),
        };
        let padding = width.saturating_sub(text.chars().count());
        if left {
            output.push_str(&text);
            output.push_str(&" ".repeat(padding));
        } else {
            output.push_str(&(if zero { "0" } else { " " }).repeat(padding));
            output.push_str(&text);
        }
    }
    output
}

fn peek_message(cpu: &mut Cpu, at: u32, flags: u32) -> u32 {
    const REMOVE: u32 = 1;
    script::advance(cpu);
    video::pump(cpu);
    gamepad::advance(cpu);
    audio::pump(cpu);
    // A game with a menu bar waits for a command while it shows a picture it
    // drew on its window, as Z does with its title. The menu opens then, and
    // again when the player clicks or presses a key there after closing it.
    for (id, callback) in dialog_box::due_timers(cpu) {
        cpu.host.messages.push_back((WM_TIMER, id, callback));
    }
    let waiting = cpu.host.menu_shown != 0 && cpu.host.draw.window_picture.is_some() && !cpu.host.menu_picked;
    if cpu.host.menu_due && waiting {
        cpu.host.menu_due = false;
        open_menu(cpu);
    }
    // Alt or the controller's Back asked for it, anywhere in the game.
    if std::mem::take(&mut cpu.host.menu_wanted) && cpu.host.menu_shown != 0 {
        open_menu(cpu);
    }
    let Some(&(message, wparam, lparam)) = cpu.host.messages.front() else { return 0 };
    if flags & REMOVE != 0 {
        cpu.host.messages.pop_front();
        const WM_KEYUP: u32 = 0x101;
        const WM_LBUTTONUP: u32 = 0x202;
        if waiting && (message == WM_LBUTTONUP || message == WM_KEYUP && matches!(wparam, 13 | 27 | 32)) {
            cpu.host.menu_due = true;
        }
    }
    cpu.fill(at, MSG_SIZE, 0);
    let window = cpu.host.window;
    cpu.w32(at, window);
    cpu.w32(at + 4, message);
    cpu.w32(at + 8, wparam);
    cpu.w32(at + 12, lparam);
    let ticks = cpu.host.started.elapsed().as_millis() as u32;
    cpu.w32(at + 16, ticks);
    let (x, y) = cpu.host.pointer;
    cpu.w32(at + 20, x);
    cpu.w32(at + 24, y);
    1
}

/// Where a resource of the program is in memory, and its size: `kind` is the
/// resource type's number, such as 9 for a key table, and `id` its number. The
/// first language is taken. Resources with names instead of numbers aren't found.
pub fn find_resource(cpu: &Cpu, kind: u32, id: u32) -> Option<(u32, u32)> {
    const RESOURCES: u32 = 0x88; // The third data directory, counted from "PE".
    const SUBFOLDER: u32 = 0x8000_0000;
    let base = cpu.host.modules.first()?.base;
    let header = base + cpu.r32(base + 0x3c);
    let root = base + cpu.r32(header + RESOURCES);
    if root == base {
        return None;
    }
    // A folder holds named entries, then numbered ones, each of 8 bytes.
    let find = |folder: u32, wanted: Option<u32>| {
        let (named, numbered) = (cpu.r16(folder + 12) as u32, cpu.r16(folder + 14) as u32);
        (named..named + numbered)
            .map(|index| folder + 16 + index * 8)
            .find(|&entry| wanted.is_none_or(|wanted| cpu.r32(entry) == wanted))
            .map(|entry| cpu.r32(entry + 4))
    };
    let kinds = find(root, Some(kind)).filter(|at| at & SUBFOLDER != 0)?;
    let ids = find(root + (kinds & !SUBFOLDER), Some(id)).filter(|at| at & SUBFOLDER != 0)?;
    // Of the languages a resource comes in, the game's own is taken, or else the first.
    let languages = root + (ids & !SUBFOLDER);
    let in_language = (0..cpu.r16(languages + 14) as u32)
        .map(|index| languages + 16 + (cpu.r16(languages + 12) as u32 + index) * 8)
        .find(|&entry| cpu.r32(entry) & PRIMARY_LANGUAGE == LANGUAGE & PRIMARY_LANGUAGE)
        .map(|entry| cpu.r32(entry + 4));
    let data = in_language.or_else(|| find(languages, None)).filter(|at| at & SUBFOLDER == 0)?;
    Some((base + cpu.r32(root + data), cpu.r32(root + data + 4)))
}

/// The language the game is shown in: English. Its low ten bits name the
/// language, and the rest the country.
pub const LANGUAGE: u32 = 0x409;
const PRIMARY_LANGUAGE: u32 = 0x3ff;

/// An entry's words without the `&` that marks its letter on a menu bar, and
/// the key named after a tab, such as F2.
fn menu_words(text: &str) -> (String, String) {
    let (label, shortcut) = text.split_once('\t').unwrap_or((text, ""));
    (label.replace('&', ""), shortcut.to_string())
}

/// Reads the entries of a menu resource from `at`, up to the one marked as the
/// last of its list. Each is flags, a command unless it opens a list, and its
/// text in 16-bit characters. A list's entries follow it.
fn read_menu(cpu: &Cpu, at: &mut u32, end: u32) -> Vec<video::menu::Item> {
    const GREYED_OR_DISABLED: u16 = 3;
    const CHECKED: u16 = 8;
    const LIST: u16 = 0x10;
    const LAST: u16 = 0x80;
    let mut items = Vec::new();
    while *at + 4 <= end {
        let flags = cpu.r16(*at);
        *at += 2;
        let mut command = 0;
        if flags & LIST == 0 {
            command = cpu.r16(*at) as u32;
            *at += 2;
        }
        let text = cpu.wide_string(*at);
        *at += (text.chars().count() as u32 + 1) * 2;
        let children = if flags & LIST != 0 { read_menu(cpu, at, end) } else { Vec::new() };
        let (label, shortcut) = menu_words(&text);
        items.push(video::menu::Item {
            separator: flags & LIST == 0 && command == 0 && label.is_empty(),
            text: label,
            shortcut,
            command,
            children,
            disabled: flags & GREYED_OR_DISABLED != 0,
            checked: flags & CHECKED != 0,
        });
        if flags & LAST != 0 {
            break;
        }
    }
    items
}

/// Loads a menu from the program's resources, and returns its handle.
fn load_menu(cpu: &mut Cpu, id: u32) -> u32 {
    const MENU: u32 = 4;
    const HEADER: u32 = 4;
    let handle = cpu.host.handle();
    if let Some((at, size)) = find_resource(cpu, MENU, id) {
        let mut reader = at + HEADER;
        let items = read_menu(cpu, &mut reader, at + size);
        cpu.host.menus.insert(handle, items);
    }
    handle
}

/// The entry of a loaded menu that sends this command.
fn menu_item(cpu: &mut Cpu, menu: u32, command: u32) -> Option<&mut video::menu::Item> {
    cpu.host.menus.get_mut(&menu)?.iter_mut().find_map(|item| item.find(command))
}

/// Shows the window's menu, and sends the game the command the player picks.
///
/// A last row of the runtime's own, Quit, ends the program at once, as
/// closing the window does. A game's own exit can be out of reach, such as in
/// the middle of a mission, and in fullscreen there's no window to close.
fn open_menu(cpu: &mut Cpu) {
    const WM_COMMAND: u32 = 0x111;
    // No menu command is this large: they're 16 bits.
    const QUIT: u32 = 0xffff_ffff;
    let Some(mut items) = cpu.host.menus.get(&cpu.host.menu_shown).cloned() else { return };
    items.push(video::menu::Item { separator: true, ..Default::default() });
    items.push(video::menu::Item { text: "Quit".to_string(), command: QUIT, ..Default::default() });
    let title = cpu.host.title;
    match video::show_menu(cpu, title, &items) {
        Some(QUIT) => {
            println!("the player quit from the menu");
            std::process::exit(0);
        }
        Some(command) => {
            cpu.host.menu_picked = true;
            cpu.host.post((WM_COMMAND, command, 0));
        }
        None => {}
    }
}

/// Reads a key table: the keys that stand for menu commands, such as F2 for a
/// new game. Each entry is flags, a key, a command and two unused bytes.
fn load_accelerators(cpu: &mut Cpu, id: u32) -> u32 {
    const KEY_TABLE: u32 = 9;
    const ENTRY: u32 = 8;
    if let Some((at, size)) = find_resource(cpu, KEY_TABLE, id) {
        cpu.host.accelerators = (0..size / ENTRY).map(|index| at + index * ENTRY).map(|entry| (cpu.r16(entry), cpu.r16(entry + 2), cpu.r16(entry + 4))).collect();
    }
    0x44
}

/// Turns a key press that is in the key table into its menu command, sent to
/// the window at once, as Windows does. Keys that need Shift, Ctrl or Alt are
/// left alone, since the state of those isn't followed here.
fn translate_accelerator(cpu: &mut Cpu, message_at: u32) -> u32 {
    const WM_KEYDOWN: u32 = 0x100;
    const WM_SYSKEYDOWN: u32 = 0x104;
    const WM_COMMAND: u32 = 0x111;
    const VIRTUAL_KEY: u16 = 1;
    const MODIFIERS: u16 = 0x1c;
    const FROM_KEY: u32 = 1 << 16;
    let (message, key) = (cpu.r32(message_at + 4), cpu.r32(message_at + 8));
    if message != WM_KEYDOWN && message != WM_SYSKEYDOWN {
        return 0;
    }
    let found = cpu.host.accelerators.iter().find(|(flags, wanted, _)| flags & VIRTUAL_KEY != 0 && flags & MODIFIERS == 0 && *wanted as u32 == key).copied();
    match found {
        Some((_, _, command)) => {
            send(cpu, WM_COMMAND, command as u32 | FROM_KEY, 0);
            1
        }
        None => 0,
    }
}

pub fn find_user(name: &str) -> Option<GuestFn> {
    table! { name;
        "wsprintfA" 0 |cpu| {
            let (target, pattern) = (cpu.arg(0), cpu.string(cpu.arg(1)));
            let mut index = 2;
            let text = format(cpu, &pattern, |cpu| { index += 1; cpu.arg(index - 1) });
            cpu.write_string(target, &text);
            text.len() as u32
        }
        "wvsprintfA" 3 |cpu| {
            let (target, pattern, mut list) = (cpu.arg(0), cpu.string(cpu.arg(1)), cpu.arg(2));
            let text = format(cpu, &pattern, |cpu| { list += 4; cpu.r32(list - 4) });
            cpu.write_string(target, &text);
            text.len() as u32
        }
        "RegisterClassA" 1 |cpu| { let class = cpu.arg(0); cpu.host.window_procedure = cpu.r32(class + 4); 1 }
        "CreateWindowExA" 12 |cpu| {
            // Later windows are the game's child controls, such as list boxes.
            // Only the first is the real one: the others just need a handle.
            let window = cpu.host.handle();
            if cpu.host.window != 0 {
                return_value!(window);
            }
            cpu.host.window = window;
            cpu.host.menu_shown = cpu.arg(9);
            cpu.host.video = video::Video::open(cpu.host.title, cpu.host.icon);
            cpu.fill(0x7ffc_0000, 64, 0);
            send(cpu, WM_CREATE, 0, 0x7ffc_0000);
            window
        }
        // At exit the game passes the handle of a launcher that isn't there.
        // Answering that would wake a game that has already freed everything.
        "ShowWindow" 2 |cpu| {
            let (window, shown) = (cpu.arg(0), cpu.arg(1) != 0);
            if dialog_box::show(cpu, window, shown) {
                return_false!();
            }
            if cpu.arg(0) != cpu.host.window || std::mem::replace(&mut cpu.host.window_shown, true) {
                return_false!();
            }
            send(cpu, WM_SHOWWINDOW, 1, 0);
            send(cpu, WM_ACTIVATEAPP, 1, 0);
            send(cpu, WM_ACTIVATE, 1, 0);
            send(cpu, WM_SETFOCUS, 0, 0);
            0
        }
        "UpdateWindow" 1 |cpu| { cpu.host.messages.push_back((WM_PAINT, 0, 0)); 1 }
        // The game ends its message loop from the WM_DESTROY this sends.
        "DestroyWindow" 1 |cpu| {
            if cpu.arg(0) != cpu.host.window {
                return_false!();
            }
            send(cpu, WM_DESTROY, 0, 0);
            send(cpu, WM_NCDESTROY, 0, 0);
            1
        }
        "SetWindowPos" 7 |_cpu| { 1 }
        "SetWindowTextA" 2 |_cpu| { 1 }
        "FindWindowA" 2 |_cpu| { 0 }
        "SetFocus" 1 |cpu| { cpu.host.window }
        "GetActiveWindow" 0 |cpu| { cpu.host.window }
        "EnableWindow" 2 |cpu| { let (window, enabled) = (cpu.arg(0), cpu.arg(1) != 0); dialog_box::enable(cpu, window, enabled); 0 }
        "SetCursor" 1 |_cpu| { 0 }
        "LoadCursorA" 2 |_cpu| { 0x10 }
        "LoadIconA" 2 |_cpu| { 0x14 }
        "GetSystemMetrics" 1 |cpu| {
            match cpu.arg(0) {
                0 => SCREEN_WIDTH,
                1 => SCREEN_HEIGHT,
                _ => 0,
            }
        }
        "SetCursorPos" 2 |cpu| { let (x, y) = (cpu.arg(0), cpu.arg(1)); video::set_cursor(cpu, x, y); 1 }
        "GetCursorPos" 1 |cpu| {
            let at = cpu.arg(0);
            let (x, y) = cpu.host.pointer;
            cpu.w32(at, x);
            cpu.w32(at + 4, y);
            1
        }
        "ClientToScreen" 2 |_cpu| { 1 }
        "ClipCursor" 1 |_cpu| { 1 }
        "GetClipCursor" 1 |cpu| {
            let at = cpu.arg(0);
            cpu.w32(at, 0);
            cpu.w32(at + 4, 0);
            cpu.w32(at + 8, SCREEN_WIDTH);
            cpu.w32(at + 12, SCREEN_HEIGHT);
            1
        }
        "GetWindowRect" 2 |cpu| {
            let at = cpu.arg(1);
            cpu.w32(at, 0);
            cpu.w32(at + 4, 0);
            cpu.w32(at + 8, SCREEN_WIDTH);
            cpu.w32(at + 12, SCREEN_HEIGHT);
            1
        }
        "PeekMessageA" 5 |cpu| { let (at, flags) = (cpu.arg(0), cpu.arg(4)); peek_message(cpu, at, flags) }
        // Waits for a message, and returns 0 for WM_QUIT, which ends the game's loop.
        "GetMessageA" 4 |cpu| {
            const WM_QUIT: u32 = 0x12;
            let at = cpu.arg(0);
            while peek_message(cpu, at, 1) == 0 {
                super::sleep(cpu, std::time::Duration::from_millis(1));
            }
            u32::from(cpu.r32(at + 4) != WM_QUIT)
        }
        // The scan code of the key that types a character, on a US keyboard,
        // with 2 in the high half when it needs Shift.
        "OemKeyScan" 1 |cpu| {
            const ROWS: [(&str, u32); 4] = [("1234567890-=", 2), ("qwertyuiop[]", 16), ("asdfghjkl;'`", 30), ("\\zxcvbnm,./", 43)];
            let character = (cpu.arg(0) as u8 as char).to_ascii_lowercase();
            let shift = if (cpu.arg(0) as u8).is_ascii_uppercase() { 2 << 16 } else { 0 };
            // Space, and the control characters keys type: Backspace, Tab,
            // Enter and Escape. Z learns its Backspace from this.
            match character {
                ' ' => return 0x39,
                '\x08' => return 0x0e,
                '\t' => return 0x0f,
                '\r' => return 0x1c,
                '\x1b' => return 0x01,
                _ => {}
            }
            ROWS.iter()
                .find_map(|(keys, first)| keys.find(character).map(|index| first + index as u32 | shift))
                .unwrap_or(0xffff_ffff)
        }
        // Dialogs are shown as a panel of rows: see `dialog_box`.
        "DialogBoxParamA" 5 |cpu| {
            let (id, procedure, parameter) = (cpu.arg(1), cpu.arg(3), cpu.arg(4));
            dialog_box::run(cpu, id, procedure, parameter)
        }
        "EndDialog" 2 |cpu| { let (window, result) = (cpu.arg(0), cpu.arg(1)); dialog_box::end(cpu, window, result) }
        "GetDlgItem" 2 |cpu| { let (window, id) = (cpu.arg(0), cpu.arg(1)); dialog_box::control(cpu, window, id) }
        "SendDlgItemMessageA" 5 |cpu| {
            let (window, id, message, wparam, lparam) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3), cpu.arg(4));
            let control = dialog_box::control(cpu, window, id);
            dialog_box::message(cpu, control, message, wparam, lparam)
        }
        "CheckRadioButton" 4 |cpu| {
            let (window, first, last, ticked) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3));
            dialog_box::check_radio(cpu, window, first, last, ticked)
        }
        "SetDlgItemTextA" 3 |cpu| {
            let (window, id, text) = (cpu.arg(0), cpu.arg(1), cpu.string(cpu.arg(2)));
            let control = dialog_box::control(cpu, window, id);
            dialog_box::set_text(cpu, control, text)
        }
        "GetDlgItemTextA" 4 |cpu| {
            let (window, id, at, room) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3) as usize);
            let control = dialog_box::control(cpu, window, id);
            let Some(mut text) = dialog_box::text(cpu, control).filter(|_| room > 0) else { return_false!() };
            text.retain(|character| character.is_ascii());
            text.truncate(room - 1);
            cpu.write_string(at, &text);
            text.len() as u32
        }
        "SetTimer" 4 |cpu| {
            let (window, id, period, callback) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3));
            dialog_box::set_timer(cpu, window, id, period, callback)
        }
        "KillTimer" 2 |cpu| { let (window, id) = (cpu.arg(0), cpu.arg(1)); dialog_box::kill_timer(cpu, window, id) }
        // The size of the system font's average character, which dialog units are parts of.
        "GetDialogBaseUnits" 0 |_cpu| { 8 | 16 << 16 }
        "SetRect" 5 |cpu| {
            let at = cpu.arg(0);
            for index in 0..4 {
                let value = cpu.arg(1 + index);
                cpu.w32(at + index * 4, value);
            }
            1
        }
        "GetForegroundWindow" 0 |cpu| { cpu.host.window }
        "IsIconic" 1 |_cpu| { 0 }
        "ShowCursor" 1 |_cpu| { 0 }
        "InvalidateRect" 3 |_cpu| { 1 }
        "GetWindowLongA" 2 |_cpu| { 0 }
        "SetClassLongA" 3 |_cpu| { 0 }
        "RegisterWindowMessageA" 1 |_cpu| { 0xc000 }
        // The window has no menu bar: the game gets handles, and nothing is shown.
        "LoadMenuA" 2 |cpu| { let id = cpu.arg(1); load_menu(cpu, id) }
        "GetMenu" 1 |cpu| { cpu.host.menu_shown }
        // The menu the window has is the one `video::menu` shows.
        "SetMenu" 2 |cpu| {
            cpu.host.menu_shown = cpu.arg(1);
            cpu.host.menu_due = cpu.host.menu_shown != 0;
            1
        }
        "DestroyMenu" 1 |cpu| { let menu = cpu.arg(0); cpu.host.menus.remove(&menu); 1 }
        "CheckMenuRadioItem" 5 |cpu| {
            let (menu, first, last, ticked) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3));
            for command in first..=last.min(first.saturating_add(0xffff)) {
                if let Some(item) = menu_item(cpu, menu, command) {
                    item.checked = command == ticked;
                }
            }
            1
        }
        "EnableMenuItem" 3 |cpu| {
            const BY_POSITION: u32 = 0x400;
            const GREYED_OR_DISABLED: u32 = 3;
            let (menu, command, flags) = (cpu.arg(0), cpu.arg(1), cpu.arg(2));
            if flags & BY_POSITION == 0 {
                if let Some(item) = menu_item(cpu, menu, command) {
                    item.disabled = flags & GREYED_OR_DISABLED != 0;
                }
            }
            0
        }
        // The game renames entries, such as its saved games, and sets their state.
        "SetMenuItemInfoA" 4 |cpu| {
            const STATE: u32 = 1;
            const TYPE: u32 = 0x10;
            const GREYED_OR_DISABLED: u32 = 3;
            const CHECKED: u32 = 8;
            let (menu, command, by_position, info) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3));
            let (mask, state, text_at) = (cpu.r32(info + 4), cpu.r32(info + 12), cpu.r32(info + 36));
            let text = cpu.string(text_at);
            if by_position == 0 {
                if let Some(item) = menu_item(cpu, menu, command) {
                    if mask & TYPE != 0 && text_at != 0 {
                        (item.text, item.shortcut) = menu_words(&text);
                    }
                    if mask & STATE != 0 {
                        item.disabled = state & GREYED_OR_DISABLED != 0;
                        item.checked = state & CHECKED != 0;
                    }
                }
            }
            1
        }
        "LoadAcceleratorsA" 2 |cpu| { let id = cpu.arg(1); load_accelerators(cpu, id) }
        "TranslateAcceleratorA" 3 |cpu| { let message = cpu.arg(2); translate_accelerator(cpu, message) }
        "WaitMessage" 0 |_cpu| { std::thread::sleep(std::time::Duration::from_millis(1)); 1 }
        "TranslateMessage" 1 |_cpu| { 0 }
        "DispatchMessageA" 1 |cpu| {
            let at = cpu.arg(0);
            let (message, wparam, lparam) = (cpu.r32(at + 4), cpu.r32(at + 8), cpu.r32(at + 12));
            // A timer set with a function of its own calls that, not the window.
            if message == WM_TIMER && lparam != 0 {
                let (window, ticks) = (cpu.host.window, cpu.host.started.elapsed().as_millis() as u32);
                return_value!(super::call_guest(cpu, lparam, &[window, WM_TIMER, wparam, ticks]));
            }
            send(cpu, message, wparam, lparam)
        }
        "SendMessageA" 4 |cpu| {
            let (window, message, wparam, lparam) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3));
            if dialog_box::owns(cpu, window) {
                return_value!(dialog_box::message(cpu, window, message, wparam, lparam));
            }
            if window != cpu.host.window {
                return_false!(); // Other child controls aren't simulated.
            }
            send(cpu, message, wparam, lparam)
        }
        "DefWindowProcA" 4 |_cpu| { 0 }
        "PostQuitMessage" 1 |cpu| { cpu.host.messages.push_back((0x12, cpu.arg(0), 0)); 0 }
        "BeginPaint" 2 |cpu| { let at = cpu.arg(1); cpu.fill(at, 64, 0); 0x20 }
        "EndPaint" 2 |_cpu| { 1 }
        "GetDC" 1 |_cpu| { 0x20 }
        "GetWindowDC" 1 |_cpu| { 0x20 }
        "ReleaseDC" 2 |_cpu| { 1 }
        "FillRect" 3 |_cpu| { 1 }
        "MessageBoxA" 4 |cpu| {
            println!("message box: {}: {}", cpu.string(cpu.arg(2)), cpu.string(cpu.arg(1)));
            1
        }
    }
}

pub fn find_gdi(name: &str) -> Option<GuestFn> {
    table! { name;
        "CreateCompatibleDC" 1 |_cpu| { 0x24 }
        "DeleteDC" 1 |_cpu| { 1 }
        "GetDeviceCaps" 2 |cpu| {
            const BITS_PER_PIXEL: u32 = 12;
            const PLANES: u32 = 14;
            match cpu.arg(1) {
                BITS_PER_PIXEL => 16,
                PLANES => 1,
                _ => 0,
            }
        }
        "GetStockObject" 1 |_cpu| { 0x28 }
        "CreateSolidBrush" 1 |_cpu| { 0x2c }
        "CreateFontA" 14 |_cpu| { 0x30 }
        "CreatePalette" 1 |_cpu| { 0x34 }
        "SelectObject" 2 |_cpu| { 0x28 }
        "SelectPalette" 3 |_cpu| { 0x34 }
        "RealizePalette" 1 |_cpu| { 0 }
        "DeleteObject" 1 |_cpu| { 1 }
        "SetBkMode" 2 |_cpu| { 1 }
        "SetBkColor" 2 |_cpu| { 0 }
        "SetTextColor" 2 |_cpu| { 0 }
        "SetTextAlign" 2 |_cpu| { 0 }
        "TextOutA" 5 |_cpu| { 1 }
        "BitBlt" 9 |_cpu| { 1 }
        "StretchBlt" 11 |_cpu| { 1 }
        "GetSystemPaletteEntries" 4 |_cpu| { 0 }
        "SetDIBColorTable" 4 |_cpu| { 0 }
    }
}
