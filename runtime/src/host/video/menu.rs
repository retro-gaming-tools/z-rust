//! A game's Windows menu bar, drawn over the top of the game.
//!
//! Some games of the time had no menu screen of their own: the window had an
//! ordinary menu bar, with entries such as New Game and Options. The window
//! here has no menu bar of its own, so the menu the game loaded is drawn as
//! one, in the grey of Windows 95, along the top of the picture, with its
//! lists dropping down below it. The game greys entries out, ticks them and
//! renames them as it would on a real menu bar.
//!
//! It opens by itself where the game waits for a command, such as Z's title
//! picture, and anywhere with a tap of Alt or the controller's Back button.
//! Left and right go along the bar, up and down along a list, as do the d-pad
//! and the mouse; Enter, Space, A or a click picks; Escape, B or the right
//! mouse button closes a list, and the menu at the top. Like the question in
//! `dialog`, it runs its own loop, answers on release, and only to presses
//! made while it was open.

use super::*;

/// One entry of a menu: a command, a list of further entries, or a line.
#[derive(Clone, Default)]
pub struct Item {
    pub text: String,
    /// The key the menu names beside it, such as F2.
    pub shortcut: String,
    /// What the game is sent when it's picked. Lists and lines have none.
    pub command: u32,
    pub children: Vec<Item>,
    pub separator: bool,
    pub disabled: bool,
    pub checked: bool,
}

impl Item {
    /// The entry with this command, here or further down.
    pub fn find(&mut self, command: u32) -> Option<&mut Item> {
        if self.command == command && self.children.is_empty() && !self.separator {
            return Some(self);
        }
        self.children.iter_mut().find_map(|child| child.find(command))
    }

    /// A command with no text shows as nothing on a menu bar, until the game
    /// gives it a name.
    fn visible(&self) -> bool {
        self.separator || !self.text.is_empty() || !self.children.is_empty()
    }

    fn pickable(&self) -> bool {
        self.visible() && !self.separator && !self.disabled
    }
}

/// The bar's height, a list row's and a line's, in the 640x480 frame.
const BAR: f32 = 18.0;
const ROW: f32 = 16.0;
const LINE: f32 = 8.0;
const GLYPH: f32 = 8.0;
/// Room left of a list's words for a tick, and right of them for an arrow.
const GUTTER: f32 = 18.0;
const KEY_RETURN: u32 = 13;
const KEY_ESCAPE: u32 = 27;
const KEY_SPACE: u32 = 32;
const KEY_RIGHT: u32 = 0x4000_004f;
const KEY_LEFT: u32 = 0x4000_0050;
const KEY_DOWN: u32 = 0x4000_0051;
const KEY_UP: u32 = 0x4000_0052;
const KEY_KEYPAD_ENTER: u32 = 0x4000_0058;
const MOUSE_LEFT: u8 = 1;
const MOUSE_RIGHT: u8 = 3;
const BUTTON_DPAD_UP: u8 = 11;
const BUTTON_DPAD_DOWN: u8 = 12;
const BUTTON_DPAD_LEFT: u8 = 13;
const BUTTON_DPAD_RIGHT: u8 = 14;

/// The height of a window's title bar, with the frame above it.
pub(super) const WINDOW_TITLE: f32 = 22.0;

/// Windows 95's menu colours.
pub(super) const FACE: (u8, u8, u8) = (192, 192, 192);
pub(super) const LIGHT: (u8, u8, u8) = (255, 255, 255);
pub(super) const SHADOW: (u8, u8, u8) = (128, 128, 128);
pub(super) const DARK: (u8, u8, u8) = (0, 0, 0);
pub(super) const HIGHLIGHT: (u8, u8, u8) = (0, 0, 128);


/// What's open: the entry chosen on the bar, then in each list dropped down
/// from it. The last one is where the keys work; with only the bar's, no list
/// is open.
type Open = Vec<usize>;

/// The list that drops down at `depth`: the bar's entries at 0, and below
/// that the entries of what's chosen one level up.
fn list_at<'a>(items: &'a [Item], open: &[usize], depth: usize) -> &'a [Item] {
    open[..depth].iter().fold(items, |list, entry| &list[*entry].children)
}

/// The next entry of a list that can be picked, `step` on, going round.
fn step(list: &[Item], from: usize, step: i32) -> usize {
    let count = list.len() as i32;
    let mut at = from as i32;
    for _ in 0..count {
        at = (at + step).rem_euclid(count.max(1));
        if list.get(at as usize).is_some_and(Item::pickable) {
            return at as usize;
        }
    }
    from
}

fn first(list: &[Item]) -> Option<usize> {
    list.iter().position(Item::pickable)
}

/// Opens the list under what's chosen last, if it has one.
fn open_list(items: &[Item], open: &mut Open) -> bool {
    let depth = open.len() - 1;
    let item = &list_at(items, open, depth)[open[depth]];
    if !item.pickable() {
        return false;
    }
    match first(&item.children) {
        Some(entry) => {
            open.push(entry);
            true
        }
        None => false,
    }
}

/// Chooses a bar entry and drops its list down.
fn choose_on_bar(items: &[Item], open: &mut Open, entry: usize) {
    open.clear();
    open.push(entry);
    open_list(items, open);
}

struct Drop {
    rect: [f32; 4],
    /// Each shown entry, by its place in the list, and where it is.
    rows: Vec<(usize, [f32; 4])>,
}

struct Layout {
    bar: Vec<(usize, [f32; 4])>,
    drops: Vec<Drop>,
}

fn text_width(text: &str) -> f32 {
    super::font::width(text)
}

fn layout(items: &[Item], open: &[usize]) -> Layout {
    let mut bar = Vec::new();
    let mut x = 4.0;
    for (index, item) in items.iter().enumerate() {
        if !item.visible() || item.separator {
            continue;
        }
        let width = text_width(&item.text) + 2.0 * GLYPH;
        bar.push((index, [x, 1.0, width, BAR - 2.0]));
        x += width;
    }
    let mut drops: Vec<Drop> = Vec::new();
    for depth in 1..open.len() {
        let list = list_at(items, open, depth);
        let (mut left, mut top) = match drops.last() {
            None => {
                let at = bar.iter().find(|(entry, _)| *entry == open[0]).map_or(4.0, |(_, rect)| rect[0]);
                (at, BAR)
            }
            Some(above) => {
                let row = above.rows.iter().find(|(entry, _)| *entry == open[depth - 1]).map_or(above.rect[1], |(_, rect)| rect[1]);
                (above.rect[0] + above.rect[2] - 3.0, row - 3.0)
            }
        };
        let widest = list
            .iter()
            .filter(|item| item.visible() && !item.separator)
            .map(|item| {
                let right = if item.children.is_empty() { text_width(&item.shortcut) } else { GLYPH };
                text_width(&item.text) + if right > 0.0 { 3.0 * GLYPH + right } else { 0.0 }
            })
            .fold(0.0, f32::max);
        let width = (GUTTER + widest + GUTTER).max(120.0);
        let height: f32 = 6.0 + list.iter().filter(|item| item.visible()).map(|item| if item.separator { LINE } else { ROW }).sum::<f32>();
        left = left.min(WIDTH as f32 - width).max(0.0);
        top = top.min(HEIGHT as f32 - height).max(0.0);
        let mut rows = Vec::new();
        let mut y = top + 3.0;
        for (index, item) in list.iter().enumerate() {
            if !item.visible() {
                continue;
            }
            let height = if item.separator { LINE } else { ROW };
            rows.push((index, [left + 3.0, y, width - 6.0, height]));
            y += height;
        }
        drops.push(Drop { rect: [left, top, width, height], rows });
    }
    Layout { bar, drops }
}

fn inside(rect: &[f32; 4], x: f32, y: f32) -> bool {
    x >= rect[0] && x < rect[0] + rect[2] && y >= rect[1] && y < rect[1] + rect[3]
}

/// Where the pointer is: on a bar entry, on a list's entry at a depth, or on
/// some other part of the menu.
#[derive(Clone, Copy, PartialEq)]
enum Spot {
    Bar(usize),
    Row(usize, usize),
    Menu,
    Outside,
}

impl Layout {
    fn spot(&self, x: f32, y: f32) -> Spot {
        for (depth, drop) in self.drops.iter().enumerate().rev() {
            if inside(&drop.rect, x, y) {
                return drop.rows.iter().find(|(_, rect)| inside(rect, x, y)).map_or(Spot::Menu, |(entry, _)| Spot::Row(depth + 1, *entry));
            }
        }
        if let Some((entry, _)) = self.bar.iter().find(|(_, rect)| inside(rect, x, y)) {
            return Spot::Bar(*entry);
        }
        if y < BAR { Spot::Menu } else { Spot::Outside }
    }
}

impl Video {
    pub(super) fn color(&self, (red, green, blue): (u8, u8, u8)) {
        unsafe { (self.set_color)(self.renderer, red, green, blue, 255) };
    }

    pub(super) fn fill(&self, rect: [f32; 4]) {
        unsafe { (self.fill_rect)(self.renderer, rect.as_ptr()) };
    }

    /// A sunken edge, as round a text box or a tick box: dark at the top and
    /// left, light at the other sides.
    pub(super) fn sunken(&self, [x, y, w, h]: [f32; 4]) {
        let r = self.renderer;
        unsafe {
            self.color(SHADOW);
            (self.draw_line)(r, x, y, x + w - 1.0, y);
            (self.draw_line)(r, x, y, x, y + h - 1.0);
            self.color(DARK);
            (self.draw_line)(r, x + 1.0, y + 1.0, x + w - 2.0, y + 1.0);
            (self.draw_line)(r, x + 1.0, y + 1.0, x + 1.0, y + h - 2.0);
            self.color(LIGHT);
            (self.draw_line)(r, x, y + h - 1.0, x + w - 1.0, y + h - 1.0);
            (self.draw_line)(r, x + w - 1.0, y, x + w - 1.0, y + h - 1.0);
        }
    }

    /// A Windows 95 window's frame and title bar, with the title in it.
    pub(super) fn window_frame(&self, rect: [f32; 4], title: &str) {
        self.color(FACE);
        self.fill(rect);
        self.raised(rect);
        let [x, y, w, _] = rect;
        self.color(HIGHLIGHT);
        self.fill([x + 3.0, y + 3.0, w - 6.0, WINDOW_TITLE - 4.0]);
        self.color(LIGHT);
        self.text(x + 8.0, y + 3.0 + (WINDOW_TITLE - 4.0 - GLYPH) / 2.0, title);
    }

    /// A tick, from its top-left corner.
    pub(super) fn tick(&self, x: f32, y: f32) {
        for offset in 0..2 {
            let o = offset as f32;
            unsafe {
                (self.draw_line)(self.renderer, x, y + 2.0 + o, x + 2.0, y + 4.0 + o);
                (self.draw_line)(self.renderer, x + 2.0, y + 4.0 + o, x + 6.0, y + o);
            }
        }
    }

    /// A raised edge, light at the top and left and dark at the other sides.
    pub(super) fn raised(&self, [x, y, w, h]: [f32; 4]) {
        let r = self.renderer;
        unsafe {
            self.color(LIGHT);
            (self.draw_line)(r, x + 1.0, y + 1.0, x + w - 2.0, y + 1.0);
            (self.draw_line)(r, x + 1.0, y + 1.0, x + 1.0, y + h - 2.0);
            self.color(SHADOW);
            (self.draw_line)(r, x + 1.0, y + h - 2.0, x + w - 2.0, y + h - 2.0);
            (self.draw_line)(r, x + w - 2.0, y + 1.0, x + w - 2.0, y + h - 2.0);
            self.color(DARK);
            (self.draw_line)(r, x, y + h - 1.0, x + w - 1.0, y + h - 1.0);
            (self.draw_line)(r, x + w - 1.0, y, x + w - 1.0, y + h - 1.0);
        }
    }

    fn draw_menu(&self, items: &[Item], open: &[usize]) {
        let shown = layout(items, open);
        let r = self.renderer;
        unsafe {
            if self.gl.is_some() {
                self.clear_overlay();
            } else {
                (self.set_color)(r, 0, 0, 0, 255);
                (self.render_clear)(r);
                (self.render_texture)(r, self.texture, std::ptr::null(), std::ptr::null());
            }
            (self.set_blend)(r, BLEND_NONE);
            self.color(FACE);
            self.fill([0.0, 0.0, WIDTH as f32, BAR]);
            self.color(SHADOW);
            (self.draw_line)(r, 0.0, BAR - 1.0, WIDTH as f32, BAR - 1.0);
            let focus = open.len() - 1;
            for (entry, rect) in &shown.bar {
                let item = &items[*entry];
                let chosen = *entry == open[0];
                if chosen {
                    self.color(HIGHLIGHT);
                    self.fill(*rect);
                }
                self.color(if chosen { LIGHT } else if item.disabled { SHADOW } else { DARK });
                self.text(rect[0] + GLYPH, rect[1] + (rect[3] - GLYPH) / 2.0, &item.text);
            }
            for (index, drop) in shown.drops.iter().enumerate() {
                let depth = index + 1;
                let list = list_at(items, open, depth);
                self.color(FACE);
                self.fill(drop.rect);
                self.raised(drop.rect);
                for (entry, rect) in &drop.rows {
                    let item = &list[*entry];
                    if item.separator {
                        let y = rect[1] + LINE / 2.0 - 1.0;
                        self.color(SHADOW);
                        (self.draw_line)(r, rect[0] + 1.0, y, rect[0] + rect[2] - 2.0, y);
                        self.color(LIGHT);
                        (self.draw_line)(r, rect[0] + 1.0, y + 1.0, rect[0] + rect[2] - 2.0, y + 1.0);
                        continue;
                    }
                    // The entry chosen in this list, which is lit while the
                    // keys work here or a list below it is open.
                    let chosen = open.get(depth) == Some(entry) && depth <= focus;
                    if chosen {
                        self.color(HIGHLIGHT);
                        self.fill(*rect);
                    }
                    let ink = if item.disabled { SHADOW } else if chosen { LIGHT } else { DARK };
                    self.color(ink);
                    let text_y = rect[1] + (ROW - GLYPH) / 2.0;
                    self.text(rect[0] + GUTTER, text_y, &item.text);
                    if item.checked {
                        // A tick.
                        let (x, y) = (rect[0] + 5.0, text_y + 4.0);
                        for offset in 0..2 {
                            let o = offset as f32;
                            (self.draw_line)(r, x, y + o, x + 2.0, y + 2.0 + o);
                            (self.draw_line)(r, x + 2.0, y + 2.0 + o, x + 7.0, y - 3.0 + o);
                        }
                    }
                    let right_x = rect[0] + rect[2] - GUTTER / 2.0;
                    if !item.children.is_empty() {
                        // An arrow to the list beside it.
                        let (x, y) = (right_x - 4.0, rect[1] + ROW / 2.0);
                        for column in 0..4 {
                            let c = column as f32;
                            (self.draw_line)(r, x + c, y - 3.0 + c, x + c, y + 3.0 - c);
                        }
                    } else if !item.shortcut.is_empty() {
                        self.text(right_x - text_width(&item.shortcut), text_y, &item.shortcut);
                    }
                }
            }
            if self.gl.is_none() {
                (self.render_present)(r);
            }
        }
    }
}

/// What a press that ended in the menu meant.
enum Press {
    Pick,
    Back,
    Close,
}

/// Shows the menu until the player picks a command, which is returned, or
/// closes it.
pub(super) fn run(cpu: &mut Cpu, _title: &str, items: &[Item]) -> Option<u32> {
    let Some(start) = first(items) else { return None };
    let mut video = cpu.host.video.take().unwrap();
    unsafe {
        // The game draws its own pointer and hides the system's.
        (video.set_relative)(video.window, false);
        (video.show_cursor)();
    }
    video.relative = false;
    let mut open: Open = Vec::new();
    choose_on_bar(items, &mut open, start);
    let (mut keys, mut mouse, mut pad): (Vec<u32>, u32, u32) = (Vec::new(), 0, 0);
    let mut later: Vec<Event> = Vec::new();
    let mut pressed: Option<Spot> = None;
    let picked = 'menu: loop {
        let mut event = Event([0; 128]);
        while unsafe { (video.poll_event)(&mut event) } {
            video.convert(&mut event);
            let mut press = None;
            // Moves along the bar or a list, by key or d-pad.
            let mut go = |open: &mut Open, (across, down): (i32, i32)| {
                let focus = open.len() - 1;
                if down != 0 {
                    if focus == 0 {
                        open_list(items, open);
                    } else {
                        open[focus] = step(list_at(items, open, focus), open[focus], down);
                    }
                } else if across > 0 && focus >= 1 && open_list(items, open) {
                } else if across < 0 && focus >= 2 {
                    open.pop();
                } else if across != 0 {
                    let next = step(items, open[0], across);
                    choose_on_bar(items, open, next);
                }
            };
            match event.u32_at(0) {
                EVENT_QUIT => std::process::exit(0),
                EVENT_GAMEPAD_ADDED => unsafe {
                    (video.open_gamepad)(event.u32_at(16));
                },
                EVENT_MOUSE_MOTION => match layout(items, &open).spot(event.f32_at(28), event.f32_at(32)) {
                    Spot::Bar(entry) if entry != open[0] => choose_on_bar(items, &mut open, entry),
                    // As on Windows: the row under the pointer is chosen, and
                    // a list it has opens beside it. On the row whose list is
                    // already open, that list stays open, so the pointer can
                    // move over to it.
                    Spot::Row(depth, entry) if open.get(depth) == Some(&entry) => {}
                    Spot::Row(depth, entry) if list_at(items, &open, depth)[entry].pickable() => {
                        open.truncate(depth);
                        open.push(entry);
                        open_list(items, &mut open);
                    }
                    _ => {}
                },
                EVENT_MOUSE_BUTTON_DOWN => {
                    mouse |= 1 << (event.0[24] & 31);
                    if event.0[24] == MOUSE_LEFT {
                        pressed = Some(layout(items, &open).spot(event.f32_at(28), event.f32_at(32)));
                    }
                }
                EVENT_MOUSE_BUTTON_UP => {
                    let bit = 1 << (event.0[24] & 31);
                    if mouse & bit == 0 {
                        later.push(Event(event.0));
                        continue;
                    }
                    mouse &= !bit;
                    match event.0[24] {
                        MOUSE_LEFT => {
                            let on = layout(items, &open).spot(event.f32_at(28), event.f32_at(32));
                            if pressed == Some(on) {
                                press = match on {
                                    Spot::Bar(entry) => {
                                        if open.len() == 1 && !open_list(items, &mut open) && items[entry].pickable() {
                                            break 'menu Some(items[entry].command);
                                        }
                                        None
                                    }
                                    Spot::Row(depth, entry) => {
                                        open.truncate(depth);
                                        open.push(entry);
                                        Some(Press::Pick)
                                    }
                                    Spot::Menu => None,
                                    Spot::Outside => Some(Press::Close),
                                };
                            }
                            pressed = None;
                        }
                        MOUSE_RIGHT => press = Some(Press::Back),
                        _ => {}
                    }
                }
                EVENT_KEY_DOWN => {
                    let key = event.u32_at(28);
                    if key == KEY_F10 || key == KEY_F11 {
                        cpu.host.video = Some(video);
                        handle_event(cpu, &event);
                        video = cpu.host.video.take().unwrap();
                        continue;
                    }
                    if !keys.contains(&key) {
                        keys.push(key);
                    }
                    match key {
                        KEY_UP => go(&mut open, (0, -1)),
                        KEY_DOWN => go(&mut open, (0, 1)),
                        KEY_LEFT => go(&mut open, (-1, 0)),
                        KEY_RIGHT => go(&mut open, (1, 0)),
                        _ => {}
                    }
                }
                EVENT_KEY_UP => {
                    let key = event.u32_at(28);
                    let Some(at) = keys.iter().position(|k| *k == key) else {
                        later.push(Event(event.0));
                        continue;
                    };
                    keys.swap_remove(at);
                    match key {
                        KEY_RETURN | KEY_KEYPAD_ENTER | KEY_SPACE => press = Some(Press::Pick),
                        KEY_ESCAPE => press = Some(Press::Back),
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_BUTTON_DOWN => {
                    pad |= 1 << (event.0[20] & 31);
                    match event.0[20] {
                        BUTTON_DPAD_UP => go(&mut open, (0, -1)),
                        BUTTON_DPAD_DOWN => go(&mut open, (0, 1)),
                        BUTTON_DPAD_LEFT => go(&mut open, (-1, 0)),
                        BUTTON_DPAD_RIGHT => go(&mut open, (1, 0)),
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_BUTTON_UP => {
                    let bit = 1 << (event.0[20] & 31);
                    if pad & bit == 0 {
                        later.push(Event(event.0));
                        continue;
                    }
                    pad &= !bit;
                    match event.0[20] {
                        gamepad::BUTTON_SOUTH => press = Some(Press::Pick),
                        gamepad::BUTTON_EAST | gamepad::BUTTON_START => press = Some(Press::Back),
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_REMOVED => later.push(Event(event.0)),
                _ => {}
            }
            match press {
                Some(Press::Close) => break 'menu None,
                Some(Press::Back) => {
                    if open.len() <= 2 {
                        break 'menu None;
                    }
                    open.pop();
                }
                Some(Press::Pick) => {
                    let depth = open.len() - 1;
                    let item = &list_at(items, &open, depth)[open[depth]];
                    if !item.pickable() || open_list(items, &mut open) {
                        continue;
                    }
                    if item.children.is_empty() {
                        break 'menu Some(item.command);
                    }
                }
                None => {}
            }
        }
        video.draw_menu(items, &open);
        video.present_gl(Picture::Last, true);
        if !video.vsync {
            std::thread::sleep(REFRESH);
        }
        // Sound plays on under the menu, as it did under a Windows menu bar.
        crate::host::audio::pump(cpu);
    };
    unsafe { (video.hide_cursor)() };
    // The game's frame goes back on screen at its next swap or flip.
    video.shown_frame = None;
    cpu.host.video = Some(video);
    for event in &later {
        handle_event(cpu, event);
    }
    picked
}

/// Draws the menu bar with its first list dropped down over the last frame
/// and saves a picture of the window, for scripted runs with a display.
pub(super) fn show_once(cpu: &mut Cpu, _title: &str, items: &[Item]) {
    let Some(video) = cpu.host.video.as_mut() else { return };
    let Some(start) = first(items) else { return };
    let mut open: Open = Vec::new();
    choose_on_bar(items, &mut open, start);
    video.draw_menu(items, &open);
    video.present_gl(Picture::Last, true);
    video.shown_frame = None;
    if let Some((width, height, pixels)) = video.window_picture() {
        let path = format!("{}/menu.png", cpu.host.script.output);
        crate::host::script::write_png(&path, width, height, &pixels);
        println!("menu saved as menu.png");
    }
}
