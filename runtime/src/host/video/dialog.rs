//! A yes or no question drawn over the game, for mods. It runs its own loop:
//! the last frame shown, dimmed, under a panel with the question and two
//! buttons, until the player answers. The game is stopped meanwhile, since the
//! mod that asks is waiting for the answer.
//!
//! The no button, on the right, is selected when it opens, so a stray press
//! keeps the game as it was. Left and right, Tab, the d-pad or the left stick
//! choose; Enter, Space or A answers with the selected button; Escape, B or
//! Start means no; the mouse selects by hovering and answers with a click.
//!
//! Everything answers on release, and only for a press made in the dialog, so
//! no half of a click or key reaches the game. Releases of keys and buttons
//! held when it opened, such as the Escape that asked, go to the game after
//! it closes, and so do the sticks' and triggers' last positions, so nothing
//! stays held there.

use super::*;

const PANEL_WIDTH: f32 = 460.0;
const PADDING: f32 = 20.0;
/// SDL's debug font is 8 by 8 pixels.
const GLYPH: f32 = 8.0;
const LINE: f32 = 12.0;
const BUTTON_WIDTH: f32 = 160.0;
const BUTTON_HEIGHT: f32 = 32.0;
const BUTTON_GAP: f32 = 40.0;
const STICK_PRESS: f32 = 0.5;
const STICK_RELEASE: f32 = 0.3;
const YES: usize = 0;
const NO: usize = 1;

const KEY_RETURN: u32 = 13;
const KEY_ESCAPE: u32 = 27;
const KEY_TAB: u32 = 9;
const KEY_SPACE: u32 = 32;
const KEY_RIGHT: u32 = 0x4000_004f;
const KEY_LEFT: u32 = 0x4000_0050;
const KEY_DOWN: u32 = 0x4000_0051;
const KEY_UP: u32 = 0x4000_0052;
const KEY_KEYPAD_ENTER: u32 = 0x4000_0058;
const MOUSE_LEFT: u8 = 1;
const BUTTON_SOUTH: u8 = gamepad::BUTTON_SOUTH;
const BUTTON_EAST: u8 = gamepad::BUTTON_EAST;
const BUTTON_START: u8 = gamepad::BUTTON_START;
const BUTTON_DPAD_UP: u8 = 11;
const BUTTON_DPAD_DOWN: u8 = 12;
const BUTTON_DPAD_LEFT: u8 = 13;
const BUTTON_DPAD_RIGHT: u8 = 14;
const AXES: usize = 6;

/// Where the panel's parts go, in game pixels.
struct Layout {
    panel: [f32; 4],
    lines: Vec<(f32, f32, String)>,
    buttons: [[f32; 4]; 2],
}

/// Breaks the text into lines at most `width` wide, as `measure` measures
/// them, at spaces. A word longer than a line keeps a line of its own.
pub(super) fn wrap(text: &str, width: f32, measure: impl Fn(&str) -> f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && measure(&format!("{line} {word}")) > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

fn layout(title: &str, text: &str) -> Layout {
    let _ = title;
    let wrapped = wrap(text, PANEL_WIDTH - 2.0 * PADDING, super::font::width);
    let height = menu::WINDOW_TITLE + PADDING + wrapped.len() as f32 * LINE + PADDING + BUTTON_HEIGHT + PADDING;
    let left = (WIDTH as f32 - PANEL_WIDTH) / 2.0;
    let top = ((HEIGHT as f32 - height) / 2.0).floor();
    let centre = WIDTH as f32 / 2.0;
    let text_top = top + menu::WINDOW_TITLE + PADDING;
    let lines = wrapped
        .iter()
        .enumerate()
        .map(|(row, line)| (left + PADDING, text_top + row as f32 * LINE, line.clone()))
        .collect();
    let button_top = text_top + wrapped.len() as f32 * LINE + PADDING;
    let first = centre - BUTTON_GAP / 2.0 - BUTTON_WIDTH;
    Layout {
        panel: [left, top, PANEL_WIDTH, height],
        lines,
        buttons: [
            [first, button_top, BUTTON_WIDTH, BUTTON_HEIGHT],
            [centre + BUTTON_GAP / 2.0, button_top, BUTTON_WIDTH, BUTTON_HEIGHT],
        ],
    }
}

fn inside(rect: &[f32; 4], x: f32, y: f32) -> bool {
    x >= rect[0] && x < rect[0] + rect[2] && y >= rect[1] && y < rect[1] + rect[3]
}

fn button_at(layout: &Layout, x: f32, y: f32) -> Option<usize> {
    layout.buttons.iter().position(|rect| inside(rect, x, y))
}

impl Video {
    fn draw_dialog(&self, layout: &Layout, title: &str, labels: &[String; 2], selected: usize) {
        unsafe {
            let r = self.renderer;
            if self.gl.is_some() {
                // With OpenGL this is drawn to the surface that goes over
                // the last frame.
                self.clear_overlay();
            } else {
                (self.set_color)(r, 0, 0, 0, 255);
                (self.render_clear)(r);
                (self.render_texture)(r, self.texture, std::ptr::null(), std::ptr::null());
            }
            (self.set_blend)(r, BLEND_NONE);
            // A Windows 95 message box: grey, with a blue title bar.
            self.window_frame(layout.panel, title);
            self.color(menu::DARK);
            for (x, y, line) in &layout.lines {
                self.text(*x, *y, line);
            }
            for (index, rect) in layout.buttons.iter().enumerate() {
                self.color(menu::FACE);
                self.fill(*rect);
                self.raised(*rect);
                if index == selected {
                    // The focused button has a black frame round it.
                    self.color(menu::DARK);
                    let [x, y, w, h] = *rect;
                    for (x1, y1, x2, y2) in [(x, y, x + w - 1.0, y), (x, y + h - 1.0, x + w - 1.0, y + h - 1.0), (x, y, x, y + h - 1.0), (x + w - 1.0, y, x + w - 1.0, y + h - 1.0)] {
                        (self.draw_line)(r, x1, y1, x2, y2);
                    }
                }
                let label = &labels[index];
                let width = super::font::width(label);
                let (text_x, text_y) = (rect[0] + (rect[2] - width) / 2.0, rect[1] + (rect[3] - GLYPH) / 2.0);
                self.color(menu::DARK);
                self.text(text_x, text_y, label);
            }
            if self.gl.is_none() {
                (self.render_present)(r);
            }
        }
    }
}

/// What the dialog saw pressed, so it answers only to its own presses and
/// hands the rest to the game.
#[derive(Default)]
struct Held {
    keys: Vec<u32>,
    mouse: u32,
    pad: u32,
    /// The left stick past the threshold, left or right, for one step a push.
    stick: bool,
}

/// Draws the question over the last frame and leaves it there, for scripted runs.
pub(super) fn show_once(cpu: &mut Cpu, title: &str, text: &str, yes: &str, no: &str) {
    let Some(video) = cpu.host.video.as_mut() else { return };
    video.draw_dialog(&layout(title, text), title, &[yes.to_string(), no.to_string()], NO);
    video.present_gl(Picture::Last, true);
    video.shown_frame = None;
    // The game draws over it at once, so the picture is taken here.
    if let Some((width, height, pixels)) = video.window_picture() {
        let path = format!("{}/question.png", cpu.host.script.output);
        crate::host::script::write_png(&path, width, height, &pixels);
        println!("question saved as question.png");
    }
}

pub(super) fn run(cpu: &mut Cpu, title: &str, text: &str, yes: &str, no: &str) -> bool {
    let mut video = cpu.host.video.take().unwrap();
    let layout = layout(title, text);
    let labels = [yes.to_string(), no.to_string()];
    unsafe {
        // The game draws its own pointer and hides the system's.
        (video.set_relative)(video.window, false);
        (video.show_cursor)();
    }
    video.relative = false;
    let mut selected = NO;
    let mut held = Held::default();
    // Events for the game once the dialog closes, and the axes' last values.
    let mut later: Vec<Event> = Vec::new();
    let mut axes: [Option<Event>; AXES] = Default::default();
    let mut mouse_on: Option<usize> = None;
    let answer = 'dialog: loop {
        let mut event = Event([0; 128]);
        while unsafe { (video.poll_event)(&mut event) } {
            video.convert(&mut event);
            match event.u32_at(0) {
                EVENT_QUIT => std::process::exit(0),
                EVENT_GAMEPAD_ADDED => unsafe {
                    (video.open_gamepad)(event.u32_at(16));
                },
                EVENT_MOUSE_MOTION => {
                    if let Some(index) = button_at(&layout, event.f32_at(28), event.f32_at(32)) {
                        selected = index;
                    }
                }
                EVENT_MOUSE_BUTTON_DOWN => {
                    let button = event.0[24];
                    held.mouse |= 1 << (button & 31);
                    if button == MOUSE_LEFT {
                        mouse_on = button_at(&layout, event.f32_at(28), event.f32_at(32));
                        if let Some(index) = mouse_on {
                            selected = index;
                        }
                    }
                }
                EVENT_MOUSE_BUTTON_UP => {
                    let button = event.0[24];
                    let bit = 1 << (button & 31);
                    if held.mouse & bit == 0 {
                        later.push(Event(event.0));
                        continue;
                    }
                    held.mouse &= !bit;
                    if button == MOUSE_LEFT {
                        let on = button_at(&layout, event.f32_at(28), event.f32_at(32));
                        if on.is_some() && on == mouse_on {
                            break 'dialog on == Some(YES);
                        }
                        mouse_on = None;
                    }
                }
                EVENT_KEY_DOWN => {
                    let key = event.u32_at(28);
                    if event.0[37] != 0 {
                        continue; // A repeat.
                    }
                    if crate::host::hotkeys::is_window_key(key) {
                        cpu.host.video = Some(video);
                        handle_event(cpu, &event);
                        video = cpu.host.video.take().unwrap();
                        continue;
                    }
                    if !held.keys.contains(&key) {
                        held.keys.push(key);
                    }
                    match key {
                        KEY_LEFT => selected = YES,
                        KEY_RIGHT => selected = NO,
                        KEY_TAB | KEY_UP | KEY_DOWN => selected = 1 - selected,
                        _ => {}
                    }
                }
                EVENT_KEY_UP => {
                    let key = event.u32_at(28);
                    let Some(at) = held.keys.iter().position(|k| *k == key) else {
                        later.push(Event(event.0));
                        continue;
                    };
                    held.keys.swap_remove(at);
                    match key {
                        KEY_RETURN | KEY_KEYPAD_ENTER | KEY_SPACE => break 'dialog selected == YES,
                        KEY_ESCAPE => break 'dialog false,
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_BUTTON_DOWN => {
                    let button = event.0[20];
                    held.pad |= 1 << (button & 31);
                    match button {
                        BUTTON_DPAD_LEFT => selected = YES,
                        BUTTON_DPAD_RIGHT => selected = NO,
                        BUTTON_DPAD_UP | BUTTON_DPAD_DOWN => selected = 1 - selected,
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_BUTTON_UP => {
                    let button = event.0[20];
                    let bit = 1 << (button & 31);
                    if held.pad & bit == 0 {
                        later.push(Event(event.0));
                        continue;
                    }
                    held.pad &= !bit;
                    match button {
                        BUTTON_SOUTH => break 'dialog selected == YES,
                        BUTTON_EAST | BUTTON_START => break 'dialog false,
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_AXIS => {
                    let axis = event.0[20] as usize;
                    let value = i16::from_le_bytes([event.0[24], event.0[25]]) as f32 / i16::MAX as f32;
                    if axis == 0 {
                        if !held.stick && value.abs() >= STICK_PRESS {
                            held.stick = true;
                            selected = if value < 0.0 { YES } else { NO };
                        } else if held.stick && value.abs() < STICK_RELEASE {
                            held.stick = false;
                        }
                    }
                    if axis < AXES {
                        axes[axis] = Some(Event(event.0));
                    }
                }
                EVENT_GAMEPAD_REMOVED => later.push(Event(event.0)),
                _ => {}
            }
        }
        video.draw_dialog(&layout, &title, &labels, selected);
        video.present_gl(Picture::Last, true);
        if !video.vsync {
            std::thread::sleep(REFRESH);
        }
    };
    unsafe { (video.hide_cursor)() };
    // The game's frame goes back on screen at its next swap or flip.
    video.shown_frame = None;
    cpu.host.video = Some(video);
    for event in later.iter().chain(axes.iter().flatten()) {
        handle_event(cpu, event);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_at_spaces_within_the_width() {
        let characters = |text: &str| text.chars().count() as f32;
        assert_eq!(wrap("one two three four", 9.0, characters), ["one two", "three", "four"]);
        assert_eq!(wrap("  spaced   out  ", 20.0, characters), ["spaced out"]);
        assert!(wrap("", 10.0, characters).is_empty());
    }

    #[test]
    fn keeps_a_word_longer_than_a_line_whole() {
        let characters = |text: &str| text.chars().count() as f32;
        assert_eq!(wrap("a verylongword b", 5.0, characters), ["a", "verylongword", "b"]);
    }

    #[test]
    fn buttons_fit_inside_the_panel_and_the_screen() {
        let text = "Do you really want to give up? All your units will be lost, and so will the mission.";
        let layout = layout("Give up the mission?", text);
        let [x, y, w, h] = layout.panel;
        assert!(x >= 0.0 && y >= 0.0 && x + w <= WIDTH as f32 && y + h <= HEIGHT as f32);
        for button in &layout.buttons {
            assert!(button[0] >= x && button[0] + button[2] <= x + w);
            assert!(button[1] + button[3] <= y + h);
        }
        assert_eq!(button_at(&layout, layout.buttons[NO][0] + 1.0, layout.buttons[NO][1] + 1.0), Some(NO));
        assert_eq!(button_at(&layout, x + 1.0, y + 1.0), None);
        assert!(layout.lines.iter().all(|(_, _, line)| super::super::font::width(line) <= w - 2.0 * PADDING));
    }

    #[test]
    fn the_font_has_german_letters_and_measures_proportionally() {
        use super::super::font::width;
        // Umlauts are in the font, not replaced: they measure as letters do.
        assert!(width("Größe") > width("Gre"));
        // Proportional: an i is narrower than an M.
        assert!(width("i") < width("M"));
        assert_eq!(width(""), 0.0);
    }
}
