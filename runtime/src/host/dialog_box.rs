//! Dialog boxes from the program's resources.
//!
//! Games with a Windows menu bar keep their option screens as dialog
//! templates: a title and a list of controls, each with a class (button, text,
//! edit field, list, slider), an id, a position and its words. This reads a
//! template, in both the plain and the extended layout.

use super::user32::find_resource;
use crate::cpu::Cpu;

const DIALOG: u32 = 5;
const HAS_FONT: u32 = 0x40;

/// One control of a dialog.
#[derive(Clone, Debug)]
pub struct Control {
    pub id: u32,
    /// `Button`, `Edit`, `Static`, `ListBox`, `ScrollBar`, `ComboBox`, or the
    /// name the template gives, such as `msctls_trackbar32`.
    pub class: String,
    pub text: String,
    pub style: u32,
    /// Left, top, width and height, in dialog units.
    pub place: [i32; 4],
}

#[derive(Clone, Debug)]
pub struct Template {
    pub title: String,
    pub style: u32,
    pub size: (i32, i32),
    pub controls: Vec<Control>,
}

struct Reader<'a> {
    cpu: &'a Cpu,
    at: u32,
}

impl Reader<'_> {
    fn u16(&mut self) -> u32 {
        self.at += 2;
        self.cpu.r16(self.at - 2) as u32
    }

    fn i16(&mut self) -> i32 {
        self.u16() as u16 as i16 as i32
    }

    fn u32(&mut self) -> u32 {
        self.at += 4;
        self.cpu.r32(self.at - 4)
    }

    /// Words, or 0xFFFF and a number in their place.
    fn words_or_number(&mut self) -> Result<String, u32> {
        if self.cpu.r16(self.at) == 0xffff {
            self.at += 2;
            return Err(self.u16());
        }
        let text = self.cpu.wide_string(self.at);
        self.at += (text.chars().count() as u32 + 1) * 2;
        Ok(text)
    }

    fn align(&mut self) {
        self.at = (self.at + 3) & !3;
    }
}

/// The dialog template with this number, in the game's language.
pub fn template(cpu: &Cpu, id: u32) -> Option<Template> {
    let (at, size) = find_resource(cpu, DIALOG, id)?;
    let end = at + size;
    let mut r = Reader { cpu, at };
    // The extended layout starts with version 1 and 0xFFFF.
    let extended = cpu.r16(at) == 1 && cpu.r16(at + 2) == 0xffff;
    let style = if extended {
        r.at += 12;
        r.u32()
    } else {
        let style = r.u32();
        r.at += 4;
        style
    };
    let count = r.u16();
    let (_x, _y, width, height) = (r.i16(), r.i16(), r.i16(), r.i16());
    let _menu = r.words_or_number();
    let _class = r.words_or_number();
    let title = r.words_or_number().unwrap_or_default();
    if style & HAS_FONT != 0 {
        r.at += if extended { 6 } else { 2 };
        let _font = r.words_or_number();
    }
    let mut controls = Vec::new();
    for _ in 0..count {
        r.align();
        if r.at + 18 > end {
            break;
        }
        let style = if extended {
            r.at += 8;
            r.u32()
        } else {
            let style = r.u32();
            r.at += 4;
            style
        };
        let place = [r.i16(), r.i16(), r.i16(), r.i16()];
        let id = if extended { r.u32() } else { r.u16() };
        let class = match r.words_or_number() {
            Ok(name) => name,
            Err(0x80) => "Button".to_string(),
            Err(0x81) => "Edit".to_string(),
            Err(0x82) => "Static".to_string(),
            Err(0x83) => "ListBox".to_string(),
            Err(0x84) => "ScrollBar".to_string(),
            Err(0x85) => "ComboBox".to_string(),
            Err(number) => format!("#{number}"),
        };
        let text = r.words_or_number().unwrap_or_else(|number| format!("#{number}"));
        let extra = r.u16();
        r.at += extra;
        controls.push(Control { id, class, text, style, place });
    }
    Some(Template { title, style, size: (width, height), controls })
}

// ---------------------------------------------------------------------------
// An open dialog: what its controls hold, and the game's procedure for it.

const WM_INITDIALOG: u32 = 0x110;
const WM_COMMAND: u32 = 0x111;
const WM_TIMER: u32 = 0x113;
const WM_HSCROLL: u32 = 0x114;
const WM_VSCROLL: u32 = 0x115;
const CANCEL: u32 = 2;
const STARTS_GROUP: u32 = 0x2_0000;
const LIST_TELLS: u32 = 1;
const SLIDER_UPRIGHT: u32 = 2;
const LINE_UP: u32 = 0;
const LINE_DOWN: u32 = 1;
const END_TRACK: u32 = 8;
const SELECTION_CHANGED: u32 = 1;
const LIST_ERROR: u32 = 0xffff_ffff;
/// Controls whose middles are this close, in dialog units, are on one line.
const SAME_LINE: i32 = 8;
const TAB: usize = 8;

/// What a button is, from the low four bits of its style.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Button {
    Push,
    Check { own: bool },
    Radio { own: bool },
    Frame,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Class {
    Button(Button),
    Words,
    Picture,
    Edit,
    List,
    Slider,
    Other,
}

fn class_of(control: &Control) -> Class {
    match control.class.to_ascii_lowercase().as_str() {
        "button" => Class::Button(match control.style & 0xf {
            2 | 5 => Button::Check { own: false },
            3 | 6 => Button::Check { own: true },
            4 => Button::Radio { own: false },
            9 => Button::Radio { own: true },
            7 => Button::Frame,
            _ => Button::Push,
        }),
        // Icons and bitmaps name a resource instead of holding words.
        "static" if matches!(control.style & 0x1f, 3 | 0xe) || control.text.starts_with('#') => Class::Picture,
        "static" => Class::Words,
        "edit" => Class::Edit,
        "listbox" => Class::List,
        "msctls_trackbar32" => Class::Slider,
        _ => Class::Other,
    }
}

/// A control of an open dialog.
pub struct Item {
    window: u32,
    id: u32,
    class: Class,
    style: u32,
    text: String,
    place: [i32; 4],
    checked: bool,
    disabled: bool,
    hidden: bool,
    /// A slider's range and position.
    low: i32,
    high: i32,
    at: i32,
    /// A list's lines, each with the value the game attached, and the line selected.
    lines: Vec<(String, u32)>,
    current: Option<usize>,
}

pub struct Open {
    window: u32,
    procedure: u32,
    title: String,
    items: Vec<Item>,
    result: Option<u32>,
    /// What a script asked for, until the dialog's loop does it.
    waiting: Vec<Asked>,
}

/// What a script step asks of the open dialog.
pub enum Asked {
    Press(u32),
    Slide(u32, i32),
    Line(u32, usize),
}

/// A timer the game set with `SetTimer`.
pub struct Timer {
    window: u32,
    id: u32,
    period: std::time::Duration,
    next: std::time::Instant,
    callback: u32,
}

/// What a row of the panel stands for.
#[derive(Clone, Debug, PartialEq)]
enum Target {
    Nothing,
    Control(usize),
    Radios(Vec<usize>),
    Line(usize, usize),
}

fn expand_tabs(text: &str) -> String {
    let mut out = String::new();
    for character in text.chars() {
        if character == '\t' {
            out.push(' ');
            while out.chars().count() % TAB != 0 {
                out.push(' ');
            }
        } else {
            out.push(character);
        }
    }
    out
}

fn words(text: &str) -> String {
    text.replace('&', "")
}

/// The panel's rows for a dialog's controls, top to bottom, and what each
/// stands for. Controls on one line of the dialog share a row where they
/// belong together: radio buttons become one row of choices, named by the
/// words beside them.
fn rows(open: &Open) -> (View, Vec<Target>) {
    // Blocks of the dialog by height: lines of controls, long texts and lists.
    enum Block {
        Line(Vec<usize>),
        Text(usize),
        List(usize),
    }
    let mut blocks: Vec<(i32, Block)> = Vec::new();
    let mut on_lines: Vec<(i32, usize)> = Vec::new();
    for (index, item) in open.items.iter().enumerate() {
        if item.hidden {
            continue;
        }
        let [_, top, _, height] = item.place;
        match item.class {
            Class::Picture => {}
            Class::Button(Button::Frame) if item.text.is_empty() => {}
            Class::Button(Button::Frame) => blocks.push((top - 1, Block::Text(index))),
            Class::Words if item.text.contains('\n') => blocks.push((top, Block::Text(index))),
            Class::List => blocks.push((top, Block::List(index))),
            _ => on_lines.push((top + height / 2, index)),
        }
    }
    on_lines.sort();
    let mut line: Vec<usize> = Vec::new();
    let mut last = i32::MIN;
    let mut first = 0;
    for (middle, index) in on_lines {
        if !line.is_empty() && middle - last > SAME_LINE {
            blocks.push((first, Block::Line(std::mem::take(&mut line))));
        }
        if line.is_empty() {
            first = middle;
        }
        line.push(index);
        last = middle;
    }
    if !line.is_empty() {
        blocks.push((first, Block::Line(line)));
    }
    blocks.sort_by_key(|(top, _)| *top);

    let mut view = View { title: open.title.clone(), rows: Vec::new() };
    let mut targets = Vec::new();
    let mut add = |label: String, kind: Kind, active: bool, target: Target| {
        view.rows.push(Row { label, kind, active });
        targets.push(target);
    };
    for (_, block) in blocks {
        match block {
            Block::Text(index) => {
                for text in open.items[index].text.lines() {
                    add(words(text), Kind::Text, false, Target::Nothing);
                }
            }
            Block::List(index) => {
                let item = &open.items[index];
                for (line, (text, _)) in item.lines.iter().enumerate() {
                    add(expand_tabs(text), Kind::Item { current: item.current == Some(line) }, !item.disabled, Target::Line(index, line));
                }
            }
            Block::Line(mut members) => {
                members.sort_by_key(|index| open.items[*index].place[0]);
                let name = members.iter().filter(|index| open.items[**index].class == Class::Words).map(|index| words(&open.items[*index].text)).collect::<Vec<_>>().join(" ");
                let name = expand_tabs(&name);
                let radios: Vec<usize> = members.iter().copied().filter(|index| matches!(open.items[*index].class, Class::Button(Button::Radio { .. }))).collect();
                let mut made: Vec<(String, Kind, bool, Target)> = Vec::new();
                let mut named = false;
                for index in members {
                    let item = &open.items[index];
                    let own = words(&item.text);
                    let mut shared = || {
                        named = true;
                        name.clone()
                    };
                    match item.class {
                        Class::Words | Class::Picture => {}
                        Class::Button(Button::Radio { .. }) if radios.len() > 1 => {
                            if radios[0] == index {
                                let options = radios.iter().map(|radio| words(&open.items[*radio].text)).collect();
                                let chosen = radios.iter().position(|radio| open.items[*radio].checked);
                                let active = radios.iter().any(|radio| !open.items[*radio].disabled);
                                made.push((shared(), Kind::Choice { options, chosen }, active, Target::Radios(radios.clone())));
                            }
                        }
                        Class::Button(Button::Radio { .. }) | Class::Button(Button::Check { .. }) => {
                            let label = if own.is_empty() { shared() } else { own };
                            made.push((label, Kind::Check(item.checked), !item.disabled, Target::Control(index)));
                        }
                        Class::Slider => made.push((shared(), Kind::Slider { low: item.low, high: item.high, at: item.at }, !item.disabled, Target::Control(index))),
                        Class::Edit => made.push((format!("{} {}", shared(), item.text).trim().to_string(), Kind::Text, false, Target::Nothing)),
                        _ => made.push((own, Kind::Button, !item.disabled, Target::Control(index))),
                    }
                }
                if !named && !name.is_empty() {
                    add(name, Kind::Text, false, Target::Nothing);
                }
                for (label, kind, active, target) in made {
                    add(label, kind, active, target);
                }
            }
        }
    }
    (view, targets)
}

use super::video::controls::{self, Action, Kind, Row, View};

fn open_mut(cpu: &mut Cpu, window: u32) -> Option<&mut Open> {
    cpu.host.dialogs.iter_mut().find(|open| open.window == window)
}

/// The control with this window handle, in any open dialog.
fn item_mut(cpu: &mut Cpu, window: u32) -> Option<&mut Item> {
    cpu.host.dialogs.iter_mut().flat_map(|open| open.items.iter_mut()).find(|item| item.window == window)
}

/// Whether this handle is a dialog's or a control's, and not the game's window.
pub fn owns(cpu: &Cpu, window: u32) -> bool {
    cpu.host.dialogs.iter().any(|open| open.window == window || open.items.iter().any(|item| item.window == window))
}

/// Calls the dialog's procedure, if the dialog is still open.
fn tell(cpu: &mut Cpu, window: u32, message: u32, wparam: u32, lparam: u32) -> u32 {
    let Some(procedure) = cpu.host.dialogs.iter().find(|open| open.window == window && open.result.is_none()).map(|open| open.procedure) else { return 0 };
    super::call_guest(cpu, procedure, &[window, message, wparam, lparam])
}

/// Ticks one radio button and clears the others of its group: the controls
/// from the last one that starts a group up to the next that does.
fn tick_radio(open: &mut Open, index: usize) {
    let first = (0..=index).rev().find(|at| open.items[*at].style & STARTS_GROUP != 0).unwrap_or(0);
    let end = (index + 1..open.items.len()).find(|at| open.items[*at].style & STARTS_GROUP != 0).unwrap_or(open.items.len());
    for at in first..end {
        if matches!(open.items[at].class, Class::Button(Button::Radio { .. })) {
            open.items[at].checked = at == index;
        }
    }
}

/// Presses a control as a click would, and tells the game.
fn press(cpu: &mut Cpu, window: u32, index: usize) {
    let Some(open) = open_mut(cpu, window) else { return };
    let Some(item) = open.items.get_mut(index) else { return };
    if item.disabled || item.hidden {
        return;
    }
    let (id, control) = (item.id, item.window);
    match item.class {
        Class::Button(Button::Check { own: true }) => item.checked = !item.checked,
        Class::Button(Button::Radio { own: true }) => tick_radio(open, index),
        Class::Button(_) | Class::Other => {}
        _ => return,
    }
    tell(cpu, window, WM_COMMAND, id & 0xffff, control);
}

/// Moves a slider, and tells the game as the arrow keys would.
fn slide(cpu: &mut Cpu, window: u32, index: usize, to: i32) {
    let Some(item) = open_mut(cpu, window).and_then(|open| open.items.get_mut(index)) else { return };
    if item.class != Class::Slider || item.disabled {
        return;
    }
    let to = to.clamp(item.low, item.high.max(item.low));
    if to == item.at {
        return;
    }
    let direction = if to < item.at { LINE_UP } else { LINE_DOWN };
    item.at = to;
    let (control, message) = (item.window, if item.style & SLIDER_UPRIGHT != 0 { WM_VSCROLL } else { WM_HSCROLL });
    tell(cpu, window, message, direction, control);
    tell(cpu, window, message, END_TRACK, control);
}

/// Selects a line of a list, and tells the game if the list says it does.
fn select_line(cpu: &mut Cpu, window: u32, index: usize, line: usize) {
    let Some(item) = open_mut(cpu, window).and_then(|open| open.items.get_mut(index)) else { return };
    if item.class != Class::List || item.disabled || line >= item.lines.len() {
        return;
    }
    item.current = Some(line);
    let (id, control, tells) = (item.id, item.window, item.style & LIST_TELLS != 0);
    if tells {
        tell(cpu, window, WM_COMMAND, id & 0xffff | SELECTION_CHANGED << 16, control);
    }
}

/// Escape. Windows sends Cancel's number. A dialog that numbers its buttons
/// its own way doesn't answer that, so its one plain button is pressed
/// instead, or its OK.
fn back(cpu: &mut Cpu, window: u32) {
    tell(cpu, window, WM_COMMAND, CANCEL, 0);
    let Some(open) = open_mut(cpu, window).filter(|open| open.result.is_none()) else { return };
    let pushes = |default: bool| {
        open.items.iter().enumerate().filter(move |(_, item)| item.class == Class::Button(Button::Push) && !item.hidden && !item.disabled && (item.style & 0xf == 1) == default).map(|(index, _)| index).collect::<Vec<_>>()
    };
    let (plain, default) = (pushes(false), pushes(true));
    let choice = match (plain.len(), default.len()) {
        (1, _) => Some(plain[0]),
        (0, 1) => Some(default[0]),
        _ => open.items.iter().position(|item| item.class == Class::Button(Button::Push) && item.text.replace('&', "").eq_ignore_ascii_case("ok")),
    };
    if let Some(index) = choice {
        press(cpu, window, index);
    }
}

fn act(cpu: &mut Cpu, window: u32, action: Action, targets: &[Target]) {
    let target = |row: usize| targets.get(row).cloned().unwrap_or(Target::Nothing);
    match action {
        Action::Back => back(cpu, window),
        Action::Press(row) => match target(row) {
            Target::Control(index) => press(cpu, window, index),
            Target::Line(index, line) => select_line(cpu, window, index, line),
            // Pressing a row of choices moves on to the next one.
            Target::Radios(radios) => step_radios(cpu, window, &radios, 1, true),
            Target::Nothing => {}
        },
        Action::Step(row, by) => match target(row) {
            Target::Radios(radios) => step_radios(cpu, window, &radios, by, false),
            Target::Control(index) => {
                let Some(item) = open_mut(cpu, window).and_then(|open| open.items.get(index)) else { return };
                // About twenty steps across, whatever the range.
                let step = ((item.high - item.low) / 20).max(1);
                let to = item.at + by * step;
                slide(cpu, window, index, to);
            }
            _ => {}
        },
        Action::Choose(row, option) => {
            if let Target::Radios(radios) = target(row) {
                if let Some(index) = radios.get(option) {
                    press(cpu, window, *index);
                }
            }
        }
        Action::Slide(row, to) => {
            if let Target::Control(index) = target(row) {
                slide(cpu, window, index, to);
            }
        }
    }
}

/// Presses the radio button `by` places from the ticked one, skipping greyed
/// ones, and going round if `round`.
fn step_radios(cpu: &mut Cpu, window: u32, radios: &[usize], by: i32, round: bool) {
    let Some(open) = open_mut(cpu, window) else { return };
    let count = radios.len() as i32;
    let mut at = radios.iter().position(|index| open.items[*index].checked).map_or(if by > 0 { -1 } else { count }, |at| at as i32);
    for _ in 0..count {
        at += by;
        if round {
            at = at.rem_euclid(count);
        }
        if at < 0 || at >= count {
            return;
        }
        if !open.items[radios[at as usize]].disabled {
            press(cpu, window, radios[at as usize]);
            return;
        }
    }
}

/// Prints the open dialog as the panel lists it, for scripted runs.
fn print_rows(view: &View) {
    println!("dialog {:?}:", view.title);
    for row in &view.rows {
        let state = match &row.kind {
            Kind::Text => String::new(),
            Kind::Button => " (button)".to_string(),
            Kind::Check(ticked) => format!(" [{}]", if *ticked { 'x' } else { ' ' }),
            Kind::Choice { options, chosen } => format!(": {}", options.iter().enumerate().map(|(index, option)| if *chosen == Some(index) { format!("[{option}]") } else { option.clone() }).collect::<Vec<_>>().join(" ")),
            Kind::Slider { low, high, at } => format!(": {at} of {low} to {high}"),
            Kind::Item { current } => (if *current { " (selected)" } else { "" }).to_string(),
        };
        println!("  {}{state}{}", row.label, if row.active || row.kind == Kind::Text { "" } else { " greyed" });
    }
}

/// Runs the timers the game set on this window that are due.
fn run_timers(cpu: &mut Cpu, window: u32) {
    let now = std::time::Instant::now();
    let due: Vec<(u32, u32)> = cpu
        .host
        .window_timers
        .iter_mut()
        .filter(|timer| timer.window == window && timer.next <= now)
        .map(|timer| {
            timer.next = now + timer.period;
            (timer.id, timer.callback)
        })
        .collect();
    for (id, callback) in due {
        if callback != 0 {
            let ticks = cpu.host.started.elapsed().as_millis() as u32;
            super::call_guest(cpu, callback, &[window, WM_TIMER, id, ticks]);
        } else {
            tell(cpu, window, WM_TIMER, id, 0);
        }
    }
}

/// `DialogBoxParamA`: shows a dialog until the game's procedure ends it, and
/// returns the result the procedure gave.
///
/// With no window there is nothing to press, so a scripted run presses with
/// its `press:ID`, `slide:ID,VALUE` and `line:ID,LINE` steps, and a dialog
/// still open when the script runs out answers Cancel.
pub fn run(cpu: &mut Cpu, id: u32, procedure: u32, parameter: u32) -> u32 {
    // A menu command that opens a dialog returns to the title afterwards,
    // where the menu opens again as before.
    cpu.host.menu_picked = false;
    let Some(template) = template(cpu, id) else {
        println!("dialog {id:#x} isn't in the program's resources");
        return u32::MAX;
    };
    let window = cpu.host.handle();
    let items: Vec<Item> = template
        .controls
        .iter()
        .map(|control| Item {
            window: cpu.host.handle(),
            id: control.id,
            class: class_of(control),
            style: control.style,
            text: control.text.clone(),
            place: control.place,
            checked: false,
            disabled: control.style & 0x0800_0000 != 0,
            hidden: control.style & 0x1000_0000 == 0,
            low: 0,
            high: 100,
            at: 0,
            lines: Vec::new(),
            current: None,
        })
        .collect();
    let focus = items.first().map_or(0, |item| item.window);
    cpu.host.dialogs.push(Open { window, procedure, title: template.title.clone(), items, result: None, waiting: Vec::new() });
    super::call_guest(cpu, procedure, &[window, WM_INITDIALOG, focus, parameter]);

    let scripted = cpu.host.script.scripted;
    let shown = cpu.host.video.is_some() && !scripted;
    if shown {
        controls::open(cpu);
    }
    let mut panel = controls::Panel::default();
    let mut printed: Option<View> = None;
    let result = loop {
        let Some(open) = cpu.host.dialogs.iter().find(|open| open.window == window) else { break CANCEL };
        if let Some(result) = open.result {
            break result;
        }
        let (view, targets) = rows(open);
        run_timers(cpu, window);
        super::audio::pump(cpu);
        if shown {
            for action in controls::step(cpu, &view, &mut panel) {
                act(cpu, window, action, &targets);
            }
            continue;
        }
        if !scripted {
            println!("dialog {id:#x} can't be shown without a window, and answers Cancel");
            break CANCEL;
        }
        if printed.as_ref() != Some(&view) {
            print_rows(&view);
            controls::picture(cpu, &view, &format!("dialog-{id:x}"));
            printed = Some(view);
        }
        super::script::advance(cpu);
        let asked = open_mut(cpu, window).map(|open| std::mem::take(&mut open.waiting)).unwrap_or_default();
        let idle = asked.is_empty();
        for ask in asked {
            let find = |cpu: &mut Cpu, id: u32| open_mut(cpu, window).and_then(|open| open.items.iter().position(|item| item.id == id));
            match ask {
                Asked::Press(control) => match find(cpu, control) {
                    Some(index) => press(cpu, window, index),
                    None => println!("the dialog has no control {control:#x}"),
                },
                Asked::Slide(control, to) => match find(cpu, control) {
                    Some(index) => slide(cpu, window, index, to),
                    None => println!("the dialog has no control {control:#x}"),
                },
                Asked::Line(control, line) => match find(cpu, control) {
                    Some(index) => select_line(cpu, window, index, line),
                    None => println!("the dialog has no control {control:#x}"),
                },
            }
        }
        if idle && cpu.host.script.steps.is_empty() {
            println!("the script ended with dialog {id:#x} open, which answers Cancel");
            break CANCEL;
        }
        super::sleep(cpu, std::time::Duration::from_millis(5));
    };
    if shown {
        controls::close(cpu, &mut panel);
    }
    cpu.host.dialogs.retain(|open| open.window != window);
    cpu.host.window_timers.retain(|timer| timer.window != window);
    println!("dialog {id:#x} closed with {result}");
    result
}

/// A script step for the dialog on top. Returns false when none is open.
pub fn ask(cpu: &mut Cpu, asked: Asked) -> bool {
    match cpu.host.dialogs.last_mut() {
        Some(open) => {
            open.waiting.push(asked);
            true
        }
        None => false,
    }
}

/// `EndDialog`.
pub fn end(cpu: &mut Cpu, window: u32, result: u32) -> u32 {
    match open_mut(cpu, window) {
        Some(open) => {
            open.result = Some(result);
            1
        }
        None => 0,
    }
}

/// `GetDlgItem`: a control's window handle, or 0.
pub fn control(cpu: &mut Cpu, window: u32, id: u32) -> u32 {
    open_mut(cpu, window).and_then(|open| open.items.iter().find(|item| item.id == id & 0xffff)).map_or(0, |item| item.window)
}

/// `CheckRadioButton`: ticks one of the buttons numbered `first` to `last`.
pub fn check_radio(cpu: &mut Cpu, window: u32, first: u32, last: u32, ticked: u32) -> u32 {
    let Some(open) = open_mut(cpu, window) else { return 0 };
    for item in open.items.iter_mut().filter(|item| item.id >= first && item.id <= last) {
        item.checked = item.id == ticked;
    }
    1
}

pub fn set_text(cpu: &mut Cpu, control: u32, text: String) -> u32 {
    match item_mut(cpu, control) {
        Some(item) => {
            item.text = text;
            1
        }
        None => 0,
    }
}

pub fn text(cpu: &mut Cpu, control: u32) -> Option<String> {
    item_mut(cpu, control).map(|item| item.text.clone())
}

/// `EnableWindow` for a control. Returns whether it was one.
pub fn enable(cpu: &mut Cpu, control: u32, enabled: bool) -> bool {
    item_mut(cpu, control).map(|item| item.disabled = !enabled).is_some()
}

/// `ShowWindow` for a control. Returns whether it was one.
pub fn show(cpu: &mut Cpu, control: u32, shown: bool) -> bool {
    item_mut(cpu, control).map(|item| item.hidden = !shown).is_some()
}

/// A message the game sends to a control, as `SendMessage` and
/// `SendDlgItemMessage` do: the ones that buttons, lists and sliders answer.
pub fn message(cpu: &mut Cpu, control: u32, message: u32, wparam: u32, lparam: u32) -> u32 {
    let text_in = |cpu: &Cpu| cpu.string(lparam);
    // Read before the control is borrowed.
    let incoming = match message {
        0xc | 0x180 | 0x181 => text_in(cpu),
        _ => String::new(),
    };
    let Some(item) = item_mut(cpu, control) else { return 0 };
    let line = wparam as usize;
    match (item.class, message) {
        (_, 0xc) => {
            item.text = incoming; // WM_SETTEXT
            1
        }
        (_, 0x30) => 0, // WM_SETFONT
        (Class::Button(_), 0xf0) => u32::from(item.checked), // BM_GETCHECK
        (Class::Button(_), 0xf2) => u32::from(item.checked), // BM_GETSTATE: only the tick
        (Class::Button(_), 0xf3 | 0xf4) => 0,                // BM_SETSTATE, BM_SETSTYLE
        (Class::Button(_), 0xf1) => {
            item.checked = wparam & 3 == 1; // BM_SETCHECK
            0
        }
        (Class::Slider, 0x400) => item.at as u32,   // TBM_GETPOS
        (Class::Slider, 0x401) => item.low as u32,  // TBM_GETRANGEMIN
        (Class::Slider, 0x402) => item.high as u32, // TBM_GETRANGEMAX
        (Class::Slider, 0x405) => {
            item.at = (lparam as i32).clamp(item.low, item.high.max(item.low)); // TBM_SETPOS
            0
        }
        (Class::Slider, 0x406) => {
            (item.low, item.high) = ((lparam & 0xffff) as i16 as i32, (lparam >> 16) as i16 as i32); // TBM_SETRANGE
            item.at = item.at.clamp(item.low, item.high.max(item.low));
            0
        }
        (Class::Slider, 0x407) => {
            item.low = lparam as i32; // TBM_SETRANGEMIN
            0
        }
        (Class::Slider, 0x408) => {
            item.high = lparam as i32; // TBM_SETRANGEMAX
            0
        }
        // Ticks, page and line sizes, and the thumb's look, change nothing here.
        (Class::Slider, 0x403..=0x420) => 0,
        (Class::List, 0x180) => {
            item.lines.push((incoming, 0)); // LB_ADDSTRING
            item.lines.len() as u32 - 1
        }
        (Class::List, 0x181) => {
            let at = if wparam == LIST_ERROR { item.lines.len() } else { line.min(item.lines.len()) }; // LB_INSERTSTRING
            item.lines.insert(at, (incoming, 0));
            at as u32
        }
        (Class::List, 0x182) => {
            if line >= item.lines.len() {
                return LIST_ERROR; // LB_DELETESTRING
            }
            item.lines.remove(line);
            item.current = None;
            item.lines.len() as u32
        }
        (Class::List, 0x184) => {
            item.lines.clear(); // LB_RESETCONTENT
            item.current = None;
            0
        }
        (Class::List, 0x186) => {
            item.current = Some(line).filter(|line| *line < item.lines.len()); // LB_SETCURSEL
            if item.current.is_some() { wparam } else { LIST_ERROR }
        }
        (Class::List, 0x188) => item.current.map_or(LIST_ERROR, |line| line as u32), // LB_GETCURSEL
        (Class::List, 0x18b) => item.lines.len() as u32,                             // LB_GETCOUNT
        (Class::List, 0x199) => item.lines.get(line).map_or(LIST_ERROR, |line| line.1), // LB_GETITEMDATA
        (Class::List, 0x19a) => match item.lines.get_mut(line) {
            Some(entry) => {
                entry.1 = lparam; // LB_SETITEMDATA
                0
            }
            None => LIST_ERROR,
        },
        (Class::List, 0x192) => 1, // LB_SETTABSTOPS
        (Class::List, 0x189) => {
            // LB_GETTEXT
            let Some(text) = item.lines.get(line).map(|line| line.0.clone()) else { return LIST_ERROR };
            cpu.write_string(lparam, &text);
            text.len() as u32
        }
        (Class::List, 0x18a) => item.lines.get(line).map_or(LIST_ERROR, |line| line.0.len() as u32), // LB_GETTEXTLEN
        (class, message) => {
            println!("a {class:?} control was sent message {message:#x} ({wparam:#x}, {lparam:#x}), which isn't known");
            0
        }
    }
}

/// `SetTimer`: a message, or a call, every `period` milliseconds.
pub fn set_timer(cpu: &mut Cpu, window: u32, id: u32, period: u32, callback: u32) -> u32 {
    let period = std::time::Duration::from_millis(period.max(10) as u64);
    let id = if window == 0 { cpu.host.handle() } else { id };
    cpu.host.window_timers.retain(|timer| !(timer.window == window && timer.id == id));
    cpu.host.window_timers.push(Timer { window, id, period, next: std::time::Instant::now() + period, callback });
    id.max(1)
}

pub fn kill_timer(cpu: &mut Cpu, window: u32, id: u32) -> u32 {
    let before = cpu.host.window_timers.len();
    cpu.host.window_timers.retain(|timer| !(timer.window == window && timer.id == id));
    u32::from(cpu.host.window_timers.len() != before)
}

/// The timers of the game's own window that are due, as `WM_TIMER` messages
/// for its queue: the timer's number and its callback.
pub fn due_timers(cpu: &mut Cpu) -> Vec<(u32, u32)> {
    let now = std::time::Instant::now();
    let main = cpu.host.window;
    cpu.host
        .window_timers
        .iter_mut()
        .filter(|timer| (timer.window == main || timer.window == 0) && timer.next <= now)
        .map(|timer| {
            timer.next = now + timer.period;
            (timer.id, timer.callback)
        })
        .collect()
}

/// Prints a template, for the `dialog:ID` script step.
pub fn print(cpu: &Cpu, id: u32) {
    let Some(template) = template(cpu, id) else {
        println!("no dialog {id:#x}");
        return;
    };
    println!("dialog {id:#x} {:?} {}x{} style {:#x}", template.title, template.size.0, template.size.1, template.style);
    for control in &template.controls {
        let [x, y, w, h] = control.place;
        println!("  {:#x} {} {:?} at {x},{y} {w}x{h} style {:#x}", control.id, control.class, control.text, control.style);
    }
}
