//! A settings screen before the game starts: the game folder, how sharp 3D is,
//! window or fullscreen, the game's own switches, and the mods that can be
//! turned off. It uses SDL's built-in
//! 8x8 font, works with the mouse, the keyboard and a controller, and keeps the
//! choices for next time.

use crate::host::{disc, hotkeys};
use crate::host::video::{symbol, Event};
use crate::loader::{self, Game, Settings};
use crate::sys;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::sync::Mutex;

type Pointer = *mut c_void;

const INIT_VIDEO: u32 = 0x20;
const INIT_GAMEPAD: u32 = 0x2000;
const WINDOW_RESIZABLE: u64 = 0x20;
const LOGICAL_LETTERBOX: c_int = 2;
const PIXELFORMAT_RGBA32: u32 = 0x1676_2004;
const ICON_SIZE: c_int = 64;
const EVENT_QUIT: u32 = 0x100;
const EVENT_KEY_DOWN: u32 = 0x300;
const EVENT_MOUSE_MOTION: u32 = 0x400;
const EVENT_MOUSE_BUTTON_DOWN: u32 = 0x401;
const EVENT_GAMEPAD_BUTTON_DOWN: u32 = 0x651;
const EVENT_GAMEPAD_ADDED: u32 = 0x653;
const EVENT_DROP_FILE: u32 = 0x1000;
const KEY_RETURN: u32 = 13;
const KEY_ESCAPE: u32 = 27;
const KEY_SPACE: u32 = 32;
const KEY_RIGHT: u32 = 0x4000_004f;
const KEY_LEFT: u32 = 0x4000_0050;
const KEY_DOWN: u32 = 0x4000_0051;
const KEY_UP: u32 = 0x4000_0052;
const BUTTON_SOUTH: u8 = 0;
const BUTTON_EAST: u8 = 1;
const BUTTON_WEST: u8 = 2;
const KEY_BACKSPACE: u32 = 8;
const KEY_DELETE: u32 = 0x7f;
const BUTTON_START: u8 = 6;
const BUTTON_UP: u8 = 11;
const BUTTON_DOWN: u8 = 12;
const BUTTON_LEFT: u8 = 13;
const BUTTON_RIGHT: u8 = 14;

/// The screen is laid out at 320x240, and scaled up to the window.
const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 240.0;
const CHARACTER: f32 = 8.0;
const MARGIN: f32 = 12.0;
const LONGEST_TEXT: usize = ((WIDTH - 2.0 * MARGIN - 8.0) / CHARACTER) as usize;
/// The window opens at the largest whole scale of 320x240 that fits the screen's usable area
/// with room for a title bar, up to this.
const WINDOW_SCALE: c_int = 4;
/// When the screen's size isn't known: 640x480, which fits a 1024x768 screen.
const FALLBACK_WINDOW_SCALE: c_int = 2;
/// Room for the window's title bar and frame, which the usable area doesn't take off.
const WINDOW_FRAME: c_int = 48;

/// The window's scale for a usable screen area of `usable` (width, height), if known.
fn window_scale(usable: Option<(c_int, c_int)>) -> c_int {
    match usable {
        Some((width, height)) if width > 0 && height > 0 => {
            let fits = (width / WIDTH as c_int).min((height - WINDOW_FRAME) / HEIGHT as c_int);
            fits.clamp(1, WINDOW_SCALE)
        }
        _ => FALLBACK_WINDOW_SCALE,
    }
}
/// The game's path is written at 3/4 size, so a long one fits on two lines. At the
/// window's scale of 4, that's still whole pixels.
const SMALL: f32 = 0.75;
const SMALL_LONGEST: usize = ((WIDTH - 2.0 * MARGIN - 8.0) / (CHARACTER * SMALL)) as usize;
const FRAME_MILLISECONDS: u32 = 16;
const FIRST_SCALE: u32 = 1;
const LAST_SCALE: u32 = 6;
/// An organisation name for the settings folder, shared by every recompiled game.
const SETTINGS_OWNER: &str = "Recompiled Games";
const USER_SETTINGS_FILE: &str = "launcher.ini";
/// The shared name earlier builds kept beside the program; read only when the program's own
/// file isn't there yet, so its settings carry over.
const OLD_SETTINGS_FILE: &str = "settings.ini";

/// The started program's name without its extension (`z-rust`), so that two ports' programs
/// in one folder keep their own settings and logs. Inside an AppImage this is the program in
/// the mount, whose name stays the same from one release to the next.
pub(crate) fn program_name() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|program| program.file_stem().map(|stem| stem.to_string_lossy().into_owned()))
        .filter(|stem| !stem.is_empty())
        .unwrap_or_else(|| "recomp".to_string())
}

/// The settings file beside the program: `<program>.settings.ini`.
fn settings_file() -> String {
    format!("{}.settings.ini", program_name())
}
/// Marks a settings file as the launcher's, so a game's own file of that name is never overwritten.
const SETTINGS_HEADER: &str = "; Launcher settings for this recompiled game\n";

pub enum Outcome {
    Start(Settings),
    Quit,
    /// SDL couldn't open a window, so the game goes on with what it had.
    Unavailable(Settings),
}

/// The launcher shows unless `--play` is given, or a run is scripted or headless.
pub fn wanted(arguments: &[String]) -> bool {
    let scripted = std::env::var("RECOMP_SCRIPT").is_ok_and(|script| !script.trim().is_empty());
    let play = arguments.iter().any(|argument| argument.eq_ignore_ascii_case("--play"));
    !play && !scripted && std::env::var_os("RECOMP_HEADLESS").is_none()
}

/// Whether this start skips the launcher, as its "Skip launcher next time" row chose, and if so
/// fills `settings` with what was saved. The launcher still opens with `--launcher`, with Shift
/// held as the program starts, and when the saved game folder no longer checks out.
pub fn skipped(game: &Game, arguments: &[String], settings: &mut Settings) -> bool {
    if arguments.iter().any(|argument| argument.eq_ignore_ascii_case("--launcher")) {
        return false;
    }
    let gave_folder = arguments.iter().any(|argument| !argument.starts_with("--"));
    load(game, settings, gave_folder);
    if !settings.skip_launcher {
        return false;
    }
    if shift_held() {
        note("Shift held: showing the launcher");
        return false;
    }
    let (status, ok) = folder_status(game, &settings.folder);
    if !ok {
        note(&format!("launcher skipping is on, but the game folder no longer checks out ({status}): showing it"));
        return false;
    }
    // A disc image or a read-only folder needs the launcher to unpack or copy it.
    let path = std::path::Path::new(&settings.folder);
    if disc::is_image(path) || !disc::is_writable(path) {
        return false;
    }
    note("skipped, as chosen; --launcher or Shift at start brings it back");
    note_settings(game, settings);
    true
}

/// Whether a Shift key is held now, before any window of ours is open.
fn shift_held() -> bool {
    #[cfg(windows)]
    {
        extern "system" {
            fn GetAsyncKeyState(key: i32) -> i16;
        }
        const VK_SHIFT: i32 = 0x10;
        unsafe { GetAsyncKeyState(VK_SHIFT) < 0 }
    }
    #[cfg(not(windows))]
    unsafe {
        // SDL knows the keyboard only for a window that has the focus, so on Linux this
        // finds Shift only where the system tells SDL at once; `--launcher` always works.
        let library = sys::open_sdl();
        if library.is_null() {
            return false;
        }
        let init: unsafe extern "C" fn(u32) -> bool = symbol(library, "SDL_Init");
        if !init(INIT_VIDEO) {
            return false;
        }
        let pump: unsafe extern "C" fn() = symbol(library, "SDL_PumpEvents");
        let modifiers: unsafe extern "C" fn() -> u16 = symbol(library, "SDL_GetModState");
        pump();
        const SHIFT: u16 = 0x0003;
        modifiers() & SHIFT != 0
    }
}

/// Reads the hotkeys from the saved settings, for every start, the launcher's or not.
pub fn load_hotkeys(game: &Game) {
    let beside = read_beside_program();
    let Some(text) = beside.or_else(|| settings_path(game).and_then(|path| std::fs::read_to_string(path).ok())) else { return };
    for line in text.lines() {
        if let Some((key, value)) = line.split_once('=') {
            hotkeys::read_setting(key.trim(), value);
        }
    }
}

/// A step of the launcher in `recomp.log`, written at once in case the next one crashes.
pub(crate) fn note(text: &str) {
    crate::runlog::line(&format!("launcher: {text}"));
}

fn note_settings(game: &Game, settings: &Settings) {
    let renderer = game.renderers.get(settings.renderer).map_or("the game's only one", |(_, label)| *label);
    note(&format!("game {:?}, renderer {renderer}, scale {}, fullscreen {}, 3D on the processor {}", settings.folder, settings.scale, settings.fullscreen, settings.processor));
    if !settings.disc.is_empty() {
        note(&format!("disc {:?}", settings.disc));
    }
}

/// Why SDL couldn't do what was asked.
fn sdl_error() -> String {
    unsafe {
        let library = sys::open_sdl();
        if library.is_null() {
            return "SDL3 couldn't be loaded".to_string();
        }
        let error: unsafe extern "C" fn() -> *const c_char = symbol(library, "SDL_GetError");
        CStr::from_ptr(error()).to_string_lossy().into_owned()
    }
}

/// The folder `settings_path` is in, for `recomp.log` when the program's folder is read-only.
pub(crate) fn settings_folder(game: &Game) -> Option<std::path::PathBuf> {
    Some(settings_path(game)?.parent()?.to_path_buf())
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Row {
    /// Shows the game's folder or disc image, and picks a folder.
    Folder,
    /// Picks a disc image, or a folder by a file in it.
    File,
    /// Picks the disc of an installed DOS copy, which the game reads as its CD drive.
    Disc,
    Scale,
    /// Whether the graphics card or the processor draws Glide.
    Drawing,
    Display,
    Renderer,
    Language,
    Switch(usize),
    Mod(usize),
    /// Where settings, `recomp.log` and unpacked discs go: beside the program or in the XDG
    /// folders (Linux only).
    Files,
    /// Whether later starts go straight into the game. Offered once the game folder checks out.
    SkipLauncher,
    /// Opens the page where the port's own keys are chosen.
    Hotkeys,
    /// Opens the page of settings most players should leave alone.
    Advanced,
    /// Leaves that page.
    Back,
    Start,
    Quit,
}

/// What the player picked or dropped, as the game's place: a disc image stays
/// as it is, any other file stands for the folder it's in, and a folder inside
/// the game, such as the one holding the program on a copy of a disc, stands
/// for the folder above it when only that one has the game's files.
fn game_place(game: &Game, picked: &str) -> String {
    let path = std::path::Path::new(picked);
    if disc::is_image(path) && disc::Disc::open(path).is_ok() {
        return picked.to_string();
    }
    let folder = if path.is_dir() { path.to_path_buf() } else { path.parent().map(|p| p.to_path_buf()).unwrap_or_default() };
    // A mounted CD or DVD stands for its drive, read like an image.
    if let Some(drive) = loader::disc_drive(&folder) {
        return drive.display().to_string();
    }
    let passes = |folder: &std::path::Path| loader::check_files(game, &folder.to_string_lossy()).is_ok();
    // A folder that holds only the disc image, such as a `.bin` and its `.cue`.
    if !passes(&folder) {
        if let Some(image) = disc::image_in(&folder).filter(|image| disc::Disc::open(image).is_ok()) {
            return image.display().to_string();
        }
    }
    match folder.parent() {
        Some(above) if !passes(&folder) && passes(above) => above.display().to_string(),
        _ => folder.display().to_string(),
    }
}

/// Unpacks a chosen disc image into `game` beside the program, showing how far
/// it is, and returns that folder. A `game` folder that already holds the
/// right files is used as it is.
fn unpack_image(sdl: &Sdl, game: &Game, image: &str) -> Result<String, String> {
    let target = loader::unpack_target(game, std::path::Path::new(image));
    let folder = target.display().to_string();
    if loader::holds_image_version(game, &folder, std::path::Path::new(image)) {
        return Ok(folder);
    }
    if target.exists() {
        return Err(format!("{folder} is in the way: it holds other files. Move or delete it."));
    }
    disc::unpack(std::path::Path::new(image), &target, &mut progress_screen(sdl, game, "Unpacking the disc"))?;
    Ok(folder)
}

/// Copies a game folder that can't be written to, such as a mounted disc whose drive can't
/// be read, into `game` beside the program, showing how far it is.
fn copy_read_only(sdl: &Sdl, game: &Game, folder: &str) -> Result<String, String> {
    loader::copy_read_only(game, folder, &mut progress_screen(sdl, game, "Copying the game from the disc"))
}

fn progress_screen<'a>(sdl: &'a Sdl, game: &'a Game, label: &'a str) -> impl FnMut(u64, u64) + 'a {
    let mut shown = u64::MAX;
    let mut event = Event([0; 128]);
    move |written, total| {
        let percent = written * 100 / total.max(1);
        if percent == shown {
            return;
        }
        shown = percent;
        // Reading the events keeps the system from calling the window frozen.
        while unsafe { (sdl.poll)(&mut event) } {}
        sdl.background();
        sdl.header(&game.title.to_uppercase(), game.version);
        let text = format!("{label}: {percent}%");
        sdl.write((WIDTH - text.len() as f32 * CHARACTER) / 2.0, HEIGHT / 2.0 - 12.0, &text, theme().text);
        let bar = [MARGIN, HEIGHT / 2.0 + 4.0, WIDTH - 2.0 * MARGIN, 10.0];
        sdl.rect(bar, theme().panel, true);
        sdl.rect([bar[0], bar[1], bar[2] * percent as f32 / 100.0, bar[3]], theme().border, true);
        unsafe { (sdl.present)(sdl.renderer) };
    }
}

/// Where a dialog opens: the chosen folder, or the folder of the chosen image, if it's there,
/// else the program's folder. A folder that doesn't exist, such as the default `game`, makes
/// some dialogs fail to open.
fn dialog_start(chosen: &str) -> CString {
    let path = std::path::Path::new(chosen);
    let folder = if path.is_dir() { Some(path.to_path_buf()) } else if path.is_file() { path.parent().map(|p| p.to_path_buf()) } else { None };
    let start = match folder {
        Some(folder) => absolute(&folder.display().to_string()),
        None => program_folder().map(|p| p.display().to_string()).unwrap_or_default(),
    };
    let start = if start.ends_with(std::path::MAIN_SEPARATOR) { start } else { format!("{start}{}", std::path::MAIN_SEPARATOR) };
    CString::new(start).unwrap_or_default()
}

/// A path in two lines of the small font, broken after a separator. A path too long even for
/// that loses its start, which is the least telling part.
fn path_lines(path: &str) -> [String; 2] {
    let chars: Vec<char> = path.chars().collect();
    if chars.len() <= SMALL_LONGEST {
        return [path.to_string(), String::new()];
    }
    let chars = if chars.len() > 2 * SMALL_LONGEST {
        let mut short: Vec<char> = "...".chars().collect();
        short.extend(&chars[chars.len() - (2 * SMALL_LONGEST - 3)..]);
        short
    } else {
        chars
    };
    let earliest = chars.len().saturating_sub(SMALL_LONGEST);
    let split = (earliest..=SMALL_LONGEST.min(chars.len())).rev().find(|&at| at > 0 && matches!(chars[at - 1], '/' | '\\')).unwrap_or(SMALL_LONGEST.min(chars.len()));
    [chars[..split].iter().collect(), chars[split..].iter().collect()]
}

/// Where a folder chosen in SDL's dialog arrives, from its callback.
static PICKED: Mutex<Option<String>> = Mutex::new(None);

unsafe extern "C" fn folder_picked(_: *mut c_void, list: *const *const c_char, _: c_int) {
    if list.is_null() || (*list).is_null() {
        return; // Cancelled, or the system has no folder dialog.
    }
    let folder = CStr::from_ptr(*list).to_string_lossy().into_owned();
    *PICKED.lock().unwrap() = Some(folder);
}

/// Where the disc of an installed copy arrives from the file dialog.
static PICKED_DISC: Mutex<Option<String>> = Mutex::new(None);

unsafe extern "C" fn disc_picked(_: *mut c_void, list: *const *const c_char, _: c_int) {
    if list.is_null() || (*list).is_null() {
        return;
    }
    *PICKED_DISC.lock().unwrap() = Some(CStr::from_ptr(*list).to_string_lossy().into_owned());
}

struct Sdl {
    window: Pointer,
    renderer: Pointer,
    set_color: unsafe extern "C" fn(Pointer, u8, u8, u8, u8) -> bool,
    clear: unsafe extern "C" fn(Pointer) -> bool,
    fill_rect: unsafe extern "C" fn(Pointer, *const f32) -> bool,
    outline: unsafe extern "C" fn(Pointer, *const f32) -> bool,
    text: unsafe extern "C" fn(Pointer, f32, f32, *const c_char) -> bool,
    present: unsafe extern "C" fn(Pointer) -> bool,
    poll: unsafe extern "C" fn(*mut Event) -> bool,
    convert: unsafe extern "C" fn(Pointer, *mut Event) -> bool,
    /// The file dialog: callback, its data, the window, the file kinds and
    /// their count, where to start, and whether several files may be picked.
    pick_file: unsafe extern "C" fn(unsafe extern "C" fn(*mut c_void, *const *const c_char, c_int), *mut c_void, Pointer, *const c_void, c_int, *const c_char, bool),
    /// The folder dialog: callback, its data, the window, where to start, and
    /// whether several folders may be picked.
    pick_folder: unsafe extern "C" fn(unsafe extern "C" fn(*mut c_void, *const *const c_char, c_int), *mut c_void, Pointer, *const c_char, bool),
    set_scale: unsafe extern "C" fn(Pointer, f32, f32) -> bool,
    open_gamepad: unsafe extern "C" fn(u32) -> Pointer,
    delay: unsafe extern "C" fn(u32),
    destroy_renderer: unsafe extern "C" fn(Pointer),
    destroy_window: unsafe extern "C" fn(Pointer),
    /// For the `launcher:NAME` script step: the drawn screen as a surface,
    /// turned into RGB.
    read_pixels: unsafe extern "C" fn(Pointer, *const c_void) -> Pointer,
    convert_surface: unsafe extern "C" fn(Pointer, u32) -> Pointer,
    destroy_surface: unsafe extern "C" fn(Pointer),
}

impl Sdl {
    fn open(game: &Game) -> Option<Sdl> {
        unsafe {
            let library = sys::open_sdl();
            if library.is_null() {
                return None;
            }
            let init: unsafe extern "C" fn(u32) -> bool = symbol(library, "SDL_Init");
            if !init(INIT_VIDEO) {
                return None;
            }
            init(INIT_GAMEPAD);
            let create_window: unsafe extern "C" fn(*const c_char, c_int, c_int, u64) -> Pointer = symbol(library, "SDL_CreateWindow");
            let title = CString::new(format!("{} - Rust - {} - Launcher", game.title, game.version)).unwrap();
            let primary: unsafe extern "C" fn() -> u32 = symbol(library, "SDL_GetPrimaryDisplay");
            let usable_bounds: unsafe extern "C" fn(u32, *mut [c_int; 4]) -> bool = symbol(library, "SDL_GetDisplayUsableBounds");
            let mut bounds = [0; 4];
            let usable = usable_bounds(primary(), &mut bounds).then_some((bounds[2], bounds[3]));
            let scale = window_scale(usable);
            let window = create_window(title.as_ptr(), WIDTH as c_int * scale, HEIGHT as c_int * scale, WINDOW_RESIZABLE);
            if window.is_null() {
                return None;
            }
            if game.icon.len() == (ICON_SIZE * ICON_SIZE * 4) as usize {
                let create_surface: unsafe extern "C" fn(c_int, c_int, u32, *const u8, c_int) -> Pointer = symbol(library, "SDL_CreateSurfaceFrom");
                let set_icon: unsafe extern "C" fn(Pointer, Pointer) -> bool = symbol(library, "SDL_SetWindowIcon");
                let surface = create_surface(ICON_SIZE, ICON_SIZE, PIXELFORMAT_RGBA32, game.icon.as_ptr(), ICON_SIZE * 4);
                if !surface.is_null() {
                    set_icon(window, surface);
                }
            }
            let create_renderer: unsafe extern "C" fn(Pointer, *const c_char) -> Pointer = symbol(library, "SDL_CreateRenderer");
            let renderer = create_renderer(window, std::ptr::null());
            if renderer.is_null() {
                return None;
            }
            let set_logical: unsafe extern "C" fn(Pointer, c_int, c_int, c_int) -> bool = symbol(library, "SDL_SetRenderLogicalPresentation");
            set_logical(renderer, WIDTH as c_int, HEIGHT as c_int, LOGICAL_LETTERBOX);
            Some(Sdl {
                window,
                renderer,
                set_color: symbol(library, "SDL_SetRenderDrawColor"),
                clear: symbol(library, "SDL_RenderClear"),
                fill_rect: symbol(library, "SDL_RenderFillRect"),
                outline: symbol(library, "SDL_RenderRect"),
                text: symbol(library, "SDL_RenderDebugText"),
                present: symbol(library, "SDL_RenderPresent"),
                poll: symbol(library, "SDL_PollEvent"),
                convert: symbol(library, "SDL_ConvertEventToRenderCoordinates"),
                pick_file: symbol(library, "SDL_ShowOpenFileDialog"),
                set_scale: symbol(library, "SDL_SetRenderScale"),
                pick_folder: symbol(library, "SDL_ShowOpenFolderDialog"),
                open_gamepad: symbol(library, "SDL_OpenGamepad"),
                delay: symbol(library, "SDL_Delay"),
                destroy_renderer: symbol(library, "SDL_DestroyRenderer"),
                destroy_window: symbol(library, "SDL_DestroyWindow"),
                read_pixels: symbol(library, "SDL_RenderReadPixels"),
                convert_surface: symbol(library, "SDL_ConvertSurface"),
                destroy_surface: symbol(library, "SDL_DestroySurface"),
            })
        }
    }

    fn color(&self, (red, green, blue): (u8, u8, u8)) {
        unsafe { (self.set_color)(self.renderer, red, green, blue, 255) };
    }

    fn rect(&self, area: [f32; 4], colour: (u8, u8, u8), filled: bool) {
        self.color(colour);
        unsafe {
            if filled {
                (self.fill_rect)(self.renderer, area.as_ptr());
            } else {
                (self.outline)(self.renderer, area.as_ptr());
            }
        }
    }

    fn write(&self, x: f32, y: f32, text: &str, colour: (u8, u8, u8)) {
        self.color(colour);
        // The built-in font has only ASCII.
        let plain: String = text.chars().map(|c| if c.is_ascii() && !c.is_ascii_control() { c } else { '?' }).collect();
        let text = CString::new(plain).unwrap();
        unsafe { (self.text)(self.renderer, x, y, text.as_ptr()) };
    }

    /// Writes at 3/4 size, at a place on the usual 320x240 screen.
    fn write_small(&self, x: f32, y: f32, text: &str, colour: (u8, u8, u8)) {
        unsafe { (self.set_scale)(self.renderer, SMALL, SMALL) };
        self.write(x / SMALL, y / SMALL, text, colour);
        unsafe { (self.set_scale)(self.renderer, 1.0, 1.0) };
    }

    fn write_right(&self, y: f32, text: &str, colour: (u8, u8, u8)) {
        self.write(WIDTH - MARGIN - 4.0 - text.len() as f32 * CHARACTER, y, text, colour);
    }

    fn close(self) {
        unsafe {
            (self.destroy_renderer)(self.renderer);
            (self.destroy_window)(self.window);
        }
    }
}


/// How the launcher looks, from the game's `launcherTheme` (port.json): every
/// port gets one in the look of its game, drawn here in code, with no art of
/// the game's. `style=plain` is flat panels; `style=metal` is riveted metal
/// plates with bevelled edges and a striped header, for games with metal
/// menus; `style=iron` is Z's own: a starfield in deep space (the DOS menu)
/// behind heavy slabs of scratched dark iron (the Z logo and the menu's
/// blocks), the title cut into an iron slab with hazard stripes at its ends,
/// and the chosen row's slab lit along its edge in the accent, Z's red.
/// Colours are six hex digits: `background`, `panel`,
/// `selected`, `border` (also the accent: the header stripes and the chosen
/// row's frame), `text`, `value`, `good`, `bad` and `dim`.
#[derive(Clone, Copy)]
pub struct Theme {
    pub metal: bool,
    /// `style=iron`; also sets `metal`, so the plain metal parts are the fallback.
    pub iron: bool,
    pub background: (u8, u8, u8),
    pub panel: (u8, u8, u8),
    pub selected: (u8, u8, u8),
    pub border: (u8, u8, u8),
    pub text: (u8, u8, u8),
    pub value: (u8, u8, u8),
    pub good: (u8, u8, u8),
    pub bad: (u8, u8, u8),
    pub dim: (u8, u8, u8),
}

impl Theme {
    const PLAIN: Theme = Theme {
        metal: false,
        iron: false,
        background: (14, 18, 16),
        panel: (26, 34, 30),
        selected: (46, 92, 64),
        border: (90, 150, 110),
        text: (225, 230, 220),
        value: (235, 205, 110),
        good: (110, 225, 120),
        bad: (245, 105, 90),
        dim: (120, 130, 125),
    };

    /// `name=value` pairs split by `;`; what's missing keeps the plain look.
    pub fn parse(text: &str) -> Theme {
        let mut theme = Theme::PLAIN;
        for pair in text.split(';') {
            let Some((name, value)) = pair.split_once('=') else { continue };
            let value = value.trim().trim_start_matches('#');
            let colour = (value.len() == 6)
                .then(|| u32::from_str_radix(value, 16).ok())
                .flatten()
                .map(|rgb| ((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8));
            match (name.trim(), colour) {
                ("style", _) => {
                    theme.iron = value.eq_ignore_ascii_case("iron");
                    theme.metal = theme.iron || value.eq_ignore_ascii_case("metal");
                }
                ("background", Some(c)) => theme.background = c,
                ("panel", Some(c)) => theme.panel = c,
                ("selected", Some(c)) => theme.selected = c,
                ("border", Some(c)) => theme.border = c,
                ("text", Some(c)) => theme.text = c,
                ("value", Some(c)) => theme.value = c,
                ("good", Some(c)) => theme.good = c,
                ("bad", Some(c)) => theme.bad = c,
                ("dim", Some(c)) => theme.dim = c,
                _ => {}
            }
        }
        theme
    }
}

static THEME: std::sync::OnceLock<Theme> = std::sync::OnceLock::new();

/// The game's theme, set when the launcher opens.
fn theme() -> Theme {
    *THEME.get().unwrap_or(&Theme::PLAIN)
}

/// A colour made lighter (`by` > 0) or darker, for bevelled edges.
fn shade((red, green, blue): (u8, u8, u8), by: i32) -> (u8, u8, u8) {
    let step = |c: u8| (c as i32 + by).clamp(0, 255) as u8;
    (step(red), step(green), step(blue))
}

impl Sdl {
    fn line(&self, x1: f32, y1: f32, x2: f32, y2: f32, colour: (u8, u8, u8)) {
        self.rect([x1.min(x2), y1.min(y2), (x2 - x1).abs() + 1.0, (y2 - y1).abs() + 1.0], colour, true);
    }

    /// A rivet: a bright dot with a shadow, at its top left.
    fn rivet(&self, x: f32, y: f32) {
        let base = theme().panel;
        self.rect([x + 1.0, y + 1.0, 2.0, 2.0], shade(base, -40), true);
        self.rect([x, y, 2.0, 2.0], shade(base, 70), true);
    }

    /// The screen behind everything: flat, or metal plates with seams and rivets.
    fn background(&self) {
        let theme = theme();
        self.rect([0.0, 0.0, WIDTH, HEIGHT], theme.background, true);
        if theme.iron {
            self.space();
            return;
        }
        if !theme.metal {
            return;
        }
        // Plates 80 pixels square, their seams a dark line over a light one.
        let mut y = 0.0;
        while y < HEIGHT {
            self.line(0.0, y, WIDTH, y, shade(theme.background, -12));
            self.line(0.0, y + 1.0, WIDTH, y + 1.0, shade(theme.background, 14));
            y += 80.0;
        }
        let mut x = 0.0;
        while x < WIDTH {
            self.line(x, 0.0, x, HEIGHT, shade(theme.background, -12));
            self.line(x + 1.0, 0.0, x + 1.0, HEIGHT, shade(theme.background, 14));
            x += 80.0;
        }
    }

    /// Deep space as behind Z's DOS menu: a few soft nebula patches in the
    /// accent's dull purple-red, then stars of three brightnesses, all from a
    /// fixed sequence so every frame looks the same.
    fn space(&self) {
        let theme = theme();
        let mut seed: u32 = 0x5a5a_1996;
        let mut next = |range: u32| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) % range.max(1)
        };
        let nebula = (
            ((theme.background.0 as u32 * 7 + theme.border.0 as u32) / 8) as u8,
            theme.background.1,
            ((theme.background.2 as u32 * 7 + theme.border.0 as u32 / 2) / 8) as u8,
        );
        // Small, dim specks clustered round two centres, so they read as haze.
        for (cx, cy, spread) in [(70.0f32, 330.0f32, 70u32), (450.0, 80.0, 50)] {
            for _ in 0..500 {
                let dx = next(spread) as f32 - next(spread) as f32;
                let dy = (next(spread) as f32 - next(spread) as f32) * 0.6;
                self.rect([cx + dx, cy + dy, 2.0, 1.0], shade(nebula, next(5) as i32 - 2), true);
            }
        }
        for _ in 0..220 {
            let (x, y) = (next(WIDTH as u32) as f32, next(HEIGHT as u32) as f32);
            let star = match next(10) {
                0 => (235, 235, 245),
                1..=3 => (150, 150, 165),
                _ => (80, 80, 92),
            };
            self.rect([x, y, 1.0, 1.0], star, true);
        }
    }

    /// A slab of dark iron, as the Z logo: bevelled, lit at the top and left,
    /// with fine scratches across it from a sequence seeded by its place.
    fn slab(&self, [x, y, w, h]: [f32; 4], colour: (u8, u8, u8)) {
        self.rect([x, y, w, h], colour, true);
        let mut seed: u32 = (x as u32).wrapping_mul(7919) ^ (y as u32).wrapping_mul(104_729) ^ 0x2e;
        let mut next = |range: u32| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) % range.max(1)
        };
        let count = ((w * h) / 90.0) as u32;
        for _ in 0..count {
            let (mut sx, mut sy) = (x + 2.0 + next((w - 4.0).max(1.0) as u32) as f32, y + 2.0 + next((h - 4.0).max(1.0) as u32) as f32);
            let light = next(3) == 0;
            for _ in 0..(2 + next(6)) {
                if sx >= x + w - 2.0 || sy >= y + h - 2.0 {
                    break;
                }
                self.rect([sx, sy, 1.0, 1.0], shade(colour, if light { 16 } else { -12 }), true);
                sx += 1.0;
                sy += if next(3) == 0 { 1.0 } else { 0.0 };
            }
        }
        self.line(x, y, x + w - 1.0, y, shade(colour, 50));
        self.line(x, y + 1.0, x + w - 1.0, y + 1.0, shade(colour, 22));
        self.line(x, y, x, y + h - 1.0, shade(colour, 35));
        self.line(x, y + h - 1.0, x + w - 1.0, y + h - 1.0, shade(colour, -40));
        self.line(x + w - 1.0, y, x + w - 1.0, y + h - 1.0, shade(colour, -40));
        self.rect([x - 1.0, y - 1.0, w + 2.0, h + 2.0], (6, 6, 8), false);
    }

    /// The title's band at the top: plain text, or a metal bar with hazard
    /// stripes at both ends and the accent along its bottom.
    /// The line reads `TITLE - RUST - version` in every port.
    fn header(&self, title: &str, version: &str) {
        let theme = theme();
        let rest = format!(" - RUST - {version}");
        let width = (title.len() + rest.len()) as f32 * CHARACTER;
        let left = (WIDTH - width) / 2.0;
        if theme.metal {
            let bar = [0.0, 4.0, WIDTH, 22.0];
            if theme.iron {
                self.slab([1.0, 4.0, WIDTH - 2.0, 22.0], theme.panel);
            } else {
                self.plate(bar, theme.panel);
            }
            self.line(0.0, 25.0, WIDTH, 25.0, theme.border);
            if theme.iron {
                // A lit orange lamp in a dark iron socket at each end, as the
                // lights on Z's machines, with a soft glow round it.
                for x in [16.0, WIDTH - 17.0] {
                    let y = 15.0;
                    let lamp = theme.value;
                    self.disc(x, y, 7.0, shade(theme.panel, -14));
                    self.disc(x, y, 6.0, (12, 12, 14));
                    self.disc(x, y, 4.0, shade(lamp, -90));
                    self.disc(x, y, 3.0, lamp);
                    self.rect([x - 1.0, y - 2.0, 2.0, 1.0], shade(lamp, 110), true);
                    self.line(x - 6.0, y + 7.0, x + 6.0, y + 7.0, shade(theme.panel, 30));
                }
                return self.title_text(left, title, &rest);
            }
            // Diagonal stripes in the accent colour, up to 30 pixels at each
            // end, shorter when a long title needs the room.
            let reach = (left - 6.0).clamp(0.0, 30.0);
            // The right end is the left one mirrored, so they lean towards the title.
            let mut x = 2.0f32;
            while x < 2.0 + reach {
                for step in 0..18 {
                    let (px, py) = (x + step as f32 * 0.5, 23.0 - step as f32);
                    if px < 2.0 + reach {
                        self.rect([px, py, 3.0, 1.0], theme.border, true);
                        self.rect([WIDTH - px - 3.0, py, 3.0, 1.0], theme.border, true);
                    }
                }
                x += 8.0;
            }
        }
        self.title_text(left, title, &rest);
    }

    /// `TITLE - RUST - version`, the title in the value colour.
    fn title_text(&self, left: f32, title: &str, rest: &str) {
        let theme = theme();
        self.write(left, 12.0, title, theme.value);
        self.write(left + title.len() as f32 * CHARACTER, 12.0, rest, theme.dim);
    }

    /// A filled disc, drawn as one line per pixel row.
    fn disc(&self, cx: f32, cy: f32, radius: f32, colour: (u8, u8, u8)) {
        let mut dy = -radius;
        while dy <= radius {
            let half = (radius * radius - dy * dy).max(0.0).sqrt().round();
            self.rect([cx - half, cy + dy, 2.0 * half + 1.0, 1.0], colour, true);
            dy += 1.0;
        }
    }

    /// A raised metal plate: lit at the top and left, shadowed at the other
    /// sides, with a rivet in each corner when it's tall enough.
    fn plate(&self, [x, y, w, h]: [f32; 4], colour: (u8, u8, u8)) {
        self.rect([x, y, w, h], colour, true);
        self.line(x, y, x + w - 1.0, y, shade(colour, 45));
        self.line(x, y, x, y + h - 1.0, shade(colour, 30));
        self.line(x, y + h - 1.0, x + w - 1.0, y + h - 1.0, shade(colour, -35));
        self.line(x + w - 1.0, y, x + w - 1.0, y + h - 1.0, shade(colour, -35));
        // Rows are too low for rivets beside their text.
        if h >= 20.0 {
            for (rx, ry) in [(x + 3.0, y + 3.0), (x + w - 5.0, y + 3.0), (x + 3.0, y + h - 5.0), (x + w - 5.0, y + h - 5.0)] {
                self.rivet(rx, ry);
            }
        }
    }

    /// A row's panel, the chosen one in the chosen colour and framed.
    fn panel(&self, area: [f32; 4], chosen: bool) {
        let theme = theme();
        let colour = if chosen { theme.selected } else { theme.panel };
        if theme.iron {
            // An iron slab; the chosen one lit red along its edge, with a
            // red marker at its left, as Z's red team colours a selection.
            self.slab(area, colour);
            if chosen {
                let [x, y, w, h] = area;
                self.rect([x - 1.0, y - 1.0, w + 2.0, h + 2.0], theme.border, false);
                self.rect([x - 2.0, y - 2.0, w + 4.0, h + 4.0], shade(theme.border, -110), false);
                for step in 0..4 {
                    let s = step as f32;
                    self.rect([x - 8.0 + s, y + h / 2.0 - 3.0 + s * 0.75, 1.0, 6.0 - s * 1.5], theme.border, true);
                }
            }
            return;
        }
        if theme.metal {
            self.plate(area, colour);
            if chosen {
                self.rect([area[0] - 1.0, area[1] - 1.0, area[2] + 2.0, area[3] + 2.0], theme.border, false);
            }
        } else {
            self.rect(area, colour, true);
            if chosen {
                self.rect(area, theme.border, false);
            }
        }
    }
}

/// Which version of Z a folder or disc image holds, told from its file names alone, so a
/// version the port can't start yet is still named.
fn recognised_version(place: &str) -> Option<String> {
    let path = std::path::Path::new(place);
    let has: Box<dyn Fn(&str) -> bool> = if disc::is_image(path) {
        let names: Vec<String> = disc::Disc::open(path).ok()?.files().into_iter().map(|entry| entry.path.to_lowercase()).collect();
        Box::new(move |name: &str| {
            let name = name.to_lowercase();
            names.iter().any(|path| *path == name || path.starts_with(&format!("{name}/")))
        })
    } else {
        let folder = place.to_string();
        Box::new(move |name: &str| loader::find_file(&folder, name).is_some())
    };
    if has("Z/WINZ.EXE") {
        return Some("Windows 95 version".to_string());
    }
    if !has("ZED.EXE") && !has("Z.PAC") {
        return None;
    }
    let language = if has("CUTS/G_LOGO.JV") {
        " (German)"
    } else if has("CUTS/E_LOGO.JV") {
        " (English)"
    } else {
        ""
    };
    let gold = if has("HANDBUCH") { ", Gold Games" } else { "" };
    Some(format!("DOS version{language}{gold}"))
}

/// The game's state: the folder's, and for an installed copy also its disc's.
fn game_status(game: &Game, settings: &Settings) -> (String, bool) {
    let status = folder_status(game, &settings.folder);
    if !status.1 || !installed(game, &settings.folder) {
        return status;
    }
    match found_disc(game, &settings.disc) {
        Ok(_) => (format!("{}, installed", status.0), true),
        Err(_) if settings.disc.is_empty() => ("Installed copy: choose its CD".to_string(), false),
        Err(problem) => (problem, false),
    }
}

/// The folder's state, in a line short enough for the screen.
fn folder_status(game: &Game, folder: &str) -> (String, bool) {
    let recognised = recognised_version(folder);
    let named = |problem: String| match &recognised {
        Some(version) => format!("{version}: {problem}"),
        None => problem,
    };
    if disc::is_image(std::path::Path::new(folder)) {
        return match loader::check_image(game, std::path::Path::new(folder)) {
            Ok(()) => match loader::version_in(game, folder).map(str::to_string).or(recognised.clone()) {
                Some(name) => (format!("Found: {name}, disc image"), true),
                None => ("Disc image found, unpacked at start".to_string(), true),
            },
            Err(problem) => (named(problem), false),
        };
    }
    match loader::check_files(game, folder) {
        Ok(()) => {
            let disc = if disc::is_writable(std::path::Path::new(folder)) { "" } else { ", on disc" };
            match loader::version_in(game, folder).map(str::to_string).or(recognised.clone()) {
                Some(name) => (format!("Found: {name}{disc}"), true),
                None if disc.is_empty() => ("Game files found".to_string(), true),
                None => ("Game found on disc, copied at start".to_string(), true),
            }
        }
        Err(text) => {
            let problem = text.lines().find(|line| line.starts_with("Missing") || line.starts_with("From a different")).unwrap_or("Game files not found");
            // A long list doesn't fit, and names a wrong folder better than any file does.
            let files = problem.split_once(": ").map_or(0, |(_, list)| list.split(", ").count());
            let short = match problem {
                _ if files <= 2 => problem,
                _ if problem.starts_with("Missing") => "Game files are missing",
                _ => "Game files are from a different version",
            };
            (named(short.to_string()), false)
        }
    }
}

/// Counts names in the game folder that another name in the same folder matches when case is
/// ignored. Windows can't hold both, so copying a patch over the game there replaces the old file,
/// but on Linux both stay, such as a translation's speech beside the original's, and the game
/// gets either one.
fn case_duplicates(folder: &str) -> usize {
    let mut count = 0;
    let mut folders = vec![std::path::PathBuf::from(folder)];
    while let Some(path) = folders.pop() {
        let Ok(entries) = std::fs::read_dir(&path) else { continue };
        let mut names = Vec::new();
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                folders.push(entry.path());
            }
            names.push(entry.file_name().to_string_lossy().to_lowercase());
        }
        names.sort();
        let all = names.len();
        names.dedup();
        if names.len() < all {
            eprintln!("launcher: {} has names that differ only in case", path.display());
        }
        count += all - names.len();
    }
    count
}

/// A warning about the folder, in two short lines, when files of a patch sit beside the ones it
/// was meant to replace.
fn folder_warning(folder: &str) -> Option<[String; 2]> {
    match case_duplicates(folder) {
        0 => None,
        1 => Some(["Warning: 1 file is there twice".to_string(), "Patch not copied over the old one?".to_string()]),
        count => Some([format!("Warning: {count} files are there twice"), "Patch not copied over the old ones?".to_string()]),
    }
}

/// The last characters of a long path, which are the telling part.
fn shorten(text: &str) -> String {
    let count = text.chars().count();
    if count <= LONGEST_TEXT {
        return text.to_string();
    }
    let tail: String = text.chars().skip(count - (LONGEST_TEXT - 3)).collect();
    format!("...{tail}")
}

/// The disc row's value: the last part of the disc's path, which fits beside the label.
fn disc_name(place: &str) -> String {
    let name = std::path::Path::new(place).file_name().map_or(place.to_string(), |name| name.to_string_lossy().into_owned());
    let count = name.chars().count();
    if count <= DISC_NAME_LONGEST {
        return name;
    }
    let start: String = name.chars().take(DISC_NAME_LONGEST - 3).collect();
    format!("{start}...")
}

/// Characters of a disc's name on the disc row, right of its label.
const DISC_NAME_LONGEST: usize = 28;

fn absolute(folder: &str) -> String {
    let full = std::fs::canonicalize(folder).map_or(folder.to_string(), |path| path.display().to_string());
    // Windows gives the long-path form, which nobody types.
    full.strip_prefix(r"\\?\").map(str::to_string).unwrap_or(full)
}

/// The folder of the program the player started. An AppImage's program sits in a read-only
/// mount, so for one it's the folder holding the AppImage file.
pub(crate) fn program_folder() -> Option<std::path::PathBuf> {
    let program = std::env::var_os("APPIMAGE").map(std::path::PathBuf::from).or_else(|| std::env::current_exe().ok())?;
    program.parent().map(|folder| folder.to_path_buf())
}

/// The XDG folders for settings and for data (`recomp.log`, unpacked discs), when the player
/// chose them; `None` keeps everything beside the program (the portable default).
static XDG: Mutex<Option<(std::path::PathBuf, std::path::PathBuf)>> = Mutex::new(None);

fn xdg() -> Option<(std::path::PathBuf, std::path::PathBuf)> {
    XDG.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
}

/// Whether the launcher offers the choice of where files go: only Linux has XDG folders.
const XDG_OFFERED: bool = cfg!(target_os = "linux");

/// The XDG base folder from `variable`, or `fallback` under the home folder, as the spec says:
/// a relative path in the variable is ignored.
fn xdg_base(variable: &str, fallback: &str) -> Option<std::path::PathBuf> {
    let set = std::env::var_os(variable).map(std::path::PathBuf::from).filter(|path| path.is_absolute());
    set.or_else(|| Some(std::path::PathBuf::from(std::env::var_os("HOME")?).join(fallback)))
}

/// `$XDG_CONFIG_HOME/<game>-rust` and `$XDG_DATA_HOME/<game>-rust`, on Linux.
fn xdg_paths(game: &Game) -> Option<(std::path::PathBuf, std::path::PathBuf)> {
    if !XDG_OFFERED {
        return None;
    }
    let name = format!("{}-rust", game.title.to_lowercase().replace(' ', "-"));
    Some((xdg_base("XDG_CONFIG_HOME", ".config")?.join(&name), xdg_base("XDG_DATA_HOME", ".local/share")?.join(&name)))
}

/// Linux only: the launcher's "Files kept" row, or `--xdg`, keeps the settings in
/// `$XDG_CONFIG_HOME/<game>-rust` and the data in `$XDG_DATA_HOME/<game>-rust`, for an
/// AppImage kept in a shared folder such as an AppImage manager's. Our `settings.ini` in
/// that settings folder is what keeps the choice for later starts. Called once at the
/// start, before anything is written.
pub(crate) fn choose_folders(game: &Game, arguments: &[String]) {
    let asked = arguments.iter().any(|argument| argument.eq_ignore_ascii_case("--xdg"));
    let Some((config, _)) = xdg_paths(game) else { return };
    let chosen = [settings_file(), OLD_SETTINGS_FILE.to_string()]
        .iter()
        .any(|name| std::fs::read_to_string(config.join(name)).is_ok_and(|text| text.starts_with(SETTINGS_HEADER)));
    if asked || chosen {
        set_xdg(game, true);
    }
}

/// Switches between the XDG folders and the program's. The XDG ones are made, and a
/// `settings.ini` with only the header marks the choice until the settings are saved there.
/// Switching back removes that `settings.ini` and the folder if it's then empty; the data
/// folder, with any unpacked discs, is left alone.
fn set_xdg(game: &Game, on: bool) {
    let Some((config, data)) = xdg_paths(game) else { return };
    let marker = config.join(settings_file());
    if on {
        let made = std::fs::create_dir_all(&config).is_ok() && std::fs::create_dir_all(&data).is_ok();
        if made && (marker.exists() || std::fs::write(&marker, SETTINGS_HEADER).is_ok()) {
            *XDG.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some((config, data));
        } else {
            note(&format!("couldn't make {} and {}: files stay beside the program", config.display(), data.display()));
        }
    } else {
        *XDG.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
        for marker in [marker, config.join(OLD_SETTINGS_FILE)] {
            if std::fs::read_to_string(&marker).is_ok_and(|text| text.starts_with(SETTINGS_HEADER)) {
                let _ = std::fs::remove_file(&marker);
            }
        }
        let _ = std::fs::remove_dir(&config);
    }
}

/// Whether the XDG folders are in use.
pub(crate) fn xdg_on() -> bool {
    xdg().is_some()
}

/// Where the port's own files go: `recomp.log` and unpacked or copied discs. Beside the
/// program, or the XDG data folder.
pub(crate) fn files_folder() -> Option<std::path::PathBuf> {
    match xdg() {
        Some((_, data)) => Some(data),
        None => program_folder(),
    }
}

/// Where the settings go: beside the program (or in the XDG settings folder),
/// unless a game's own file of that name is there.
fn settings_beside_program() -> Option<std::path::PathBuf> {
    let folder = match xdg() {
        Some((config, _)) => config,
        None => program_folder()?,
    };
    let path = folder.join(settings_file());
    let ours = std::fs::read_to_string(&path).map_or(!path.exists(), |text| text.starts_with(SETTINGS_HEADER));
    ours.then_some(path)
}

/// The saved settings beside the program (or in the XDG settings folder): the program's own
/// file, else the shared `settings.ini` of earlier builds when it's the launcher's.
fn read_beside_program() -> Option<String> {
    let path = settings_beside_program()?;
    std::fs::read_to_string(&path).ok().or_else(|| {
        let old = std::fs::read_to_string(path.with_file_name(OLD_SETTINGS_FILE)).ok()?;
        old.starts_with(SETTINGS_HEADER).then_some(old)
    })
}

/// SDL's settings folder for the game, used when the program's folder can't be written, and
/// where earlier builds kept their settings.
fn settings_path(game: &Game) -> Option<std::path::PathBuf> {
    unsafe {
        let library = sys::open_sdl();
        if library.is_null() {
            return None;
        }
        let pref_path: unsafe extern "C" fn(*const c_char, *const c_char) -> *mut c_char = symbol(library, "SDL_GetPrefPath");
        let (owner, title) = (CString::new(SETTINGS_OWNER).unwrap(), CString::new(game.title).unwrap());
        let folder = pref_path(owner.as_ptr(), title.as_ptr());
        if folder.is_null() {
            return None;
        }
        Some(std::path::PathBuf::from(CStr::from_ptr(folder).to_string_lossy().into_owned()).join(USER_SETTINGS_FILE))
    }
}

/// Fills in what the last run chose, where the command line didn't say.
fn load(game: &Game, settings: &mut Settings, arguments_gave_folder: bool) {
    let arguments_gave_renderer = std::env::args().any(|argument| argument.starts_with("--renderer="));
    let arguments_gave_language = std::env::args().any(|argument| argument.starts_with("--language="));
    let beside = read_beside_program();
    let Some(text) = beside.or_else(|| settings_path(game).and_then(|path| std::fs::read_to_string(path).ok())) else { return };
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else { continue };
        match key.trim() {
            "folder" if !arguments_gave_folder && !value.trim().is_empty() => settings.folder = value.trim().to_string(),
            "disc" if settings.disc.is_empty() => settings.disc = value.trim().to_string(),
            "scale" => settings.scale = value.trim().parse().unwrap_or(settings.scale).clamp(FIRST_SCALE, LAST_SCALE),
            "fullscreen" => settings.fullscreen = value.trim() == "1",
            "skip_launcher" => settings.skip_launcher = value.trim() == "1",
            name if hotkeys::read_setting(name, value) => {}
            "drawing" if !std::env::args().any(|argument| argument.starts_with("--drawing=")) => settings.processor = value.trim() == "processor",
            "renderer" if !arguments_gave_renderer => settings.renderer = loader::find_choice(game.renderers, value.trim()).unwrap_or(settings.renderer),
            "language" if !arguments_gave_language => settings.language = loader::find_choice(game.languages, value.trim()).unwrap_or(settings.language),
            name => {
                if let Some(index) = game.switches.iter().position(|(switch, _, _)| *switch == name) {
                    settings.switches[index] = value.trim() == "1";
                }
                let file = name.strip_prefix("mod.").unwrap_or_default();
                let named_on_command_line = std::env::args().any(|argument| {
                    argument.strip_prefix("--mod-off=").or_else(|| argument.strip_prefix("--mod-on=")).is_some_and(|named| named.eq_ignore_ascii_case(file))
                });
                if let Some(index) = game.mods.iter().position(|(mod_file, ..)| *mod_file == file) {
                    if !named_on_command_line {
                        settings.mods[index] = value.trim() == "1";
                    }
                }
            }
        }
    }
}

fn save(game: &Game, settings: &Settings) {
    let mut text = format!("{SETTINGS_HEADER}folder={}\nscale={}\nfullscreen={}\n", absolute(&settings.folder), settings.scale, settings.fullscreen as u8);
    if !settings.disc.is_empty() {
        text.push_str(&format!("disc={}\n", absolute(&settings.disc)));
    }
    text.push_str(&format!("drawing={}\n", if settings.processor { "processor" } else { "card" }));
    if let Some((_, label)) = game.renderers.get(settings.renderer) {
        text.push_str(&format!("renderer={label}\n"));
    }
    if let Some((_, label)) = game.languages.get(settings.language) {
        text.push_str(&format!("language={label}\n"));
    }
    for ((name, _, _), on) in game.switches.iter().zip(&settings.switches) {
        text.push_str(&format!("{name}={}\n", *on as u8));
    }
    for ((file, ..), on) in game.mods.iter().zip(&settings.mods) {
        text.push_str(&format!("mod.{file}={}\n", *on as u8));
    }
    text.push_str(&format!("skip_launcher={}\n", settings.skip_launcher as u8));
    text.push_str(&hotkeys::setting_lines());
    // The "Files kept" row moves the settings, and later logs and unpacked discs; this run's
    // log stays where it was opened.
    if settings.xdg != xdg_on() {
        note(&format!("files kept {}", if settings.xdg { "in the XDG folders" } else { "beside the program" }));
        set_xdg(game, settings.xdg);
    }
    // A folder such as Program Files can refuse the write.
    let written = settings_beside_program().is_some_and(|path| std::fs::write(path, &text).is_ok());
    if !written {
        if let Some(path) = settings_path(game) {
            let _ = std::fs::write(path, text);
        }
    }
}

const FOLDER_TOP: f32 = 34.0;
const FOLDER_HEIGHT: f32 = 46.0;
const ROWS_TOP: f32 = 86.0;
const ROW_HEIGHT: f32 = 18.0;
const ROW_GAP: f32 = 2.0;
/// Where the rows must end, above the two hint lines.
const ROWS_BOTTOM: f32 = HEIGHT - 30.0;

/// Whether the folder is an installed DOS copy that needs its disc chosen too, remembered for
/// the last folder asked about, since the rows are drawn every frame and the check reads the
/// whole program.
fn installed(game: &Game, folder: &str) -> bool {
    static LAST: Mutex<Option<(String, bool)>> = Mutex::new(None);
    let mut last = LAST.lock().unwrap();
    match &*last {
        Some((known, answer)) if known == folder => *answer,
        _ => {
            let answer = loader::installed_without_disc(game, folder);
            *last = Some((folder.to_string(), answer));
            answer
        }
    }
}

/// The disc found for an installed copy, as `loader::disc_for` finds it, remembered as
/// `installed` is: it may look through every unpacked disc beside the program.
fn found_disc(game: &Game, chosen: &str) -> Result<String, String> {
    static LAST: Mutex<Option<(String, Result<String, String>)>> = Mutex::new(None);
    let mut last = LAST.lock().unwrap();
    if let Some((known, answer)) = &*last {
        if known == chosen {
            return answer.clone();
        }
    }
    let answer = loader::disc_for(game, chosen);
    *last = Some((chosen.to_string(), answer.clone()));
    answer
}

/// The rows of the first page, or of the advanced one. That page holds what
/// the defaults get right for nearly everyone: who draws the 3D, the game's
/// renderer, and the mods marked `launcher-advanced`. The first page offers
/// it only if it has rows.
fn rows(game: &Game, settings: &Settings, advanced: bool) -> Vec<Row> {
    let mut more = Vec::new();
    // Only Glide is drawn by the runtime, so only it can go to the graphics card.
    if game.renderers.iter().any(|(_, label)| label.eq_ignore_ascii_case("Glide")) {
        more.push(Row::Drawing);
    }
    if game.renderers.len() > 1 {
        more.push(Row::Renderer);
    }
    let mods = |on_advanced: bool| (0..game.mods.len()).filter(move |index| game.mods[*index].3 == on_advanced).map(Row::Mod);
    more.extend(mods(true));
    if XDG_OFFERED {
        more.push(Row::Files);
    }
    if advanced {
        more.push(Row::Back);
        return more;
    }
    // The 3D resolution is Glide's, so only a game that draws with it gets the row.
    let glide = game.renderers.iter().any(|(_, label)| label.eq_ignore_ascii_case("Glide"));
    let mut rows = vec![Row::Folder, Row::File];
    // Only an installed DOS copy needs its disc chosen too.
    if installed(game, &settings.folder) {
        rows.push(Row::Disc);
    }
    if glide {
        rows.push(Row::Scale);
    }
    rows.push(Row::Display);
    if game.languages.len() > 1 {
        rows.push(Row::Language);
    }
    rows.extend((0..game.switches.len()).map(Row::Switch));
    rows.extend(mods(false));
    // Hotkeys sit on the first page, just above Advanced (user, 2026-11-26).
    rows.extend([Row::SkipLauncher, Row::Hotkeys]);
    if !more.is_empty() {
        rows.push(Row::Advanced);
    }
    rows.extend([Row::Start, Row::Quit]);
    rows
}

/// A row's top and height on the screen. Rows get shorter when there are too
/// many to fit at the usual height, down to a little more than the font. The
/// first page's first row is the folder's, which is taller and set apart.
fn place(index: usize, count: usize, advanced: bool) -> (f32, f32) {
    let folder = usize::from(!advanced);
    if index < folder {
        return (FOLDER_TOP, FOLDER_HEIGHT);
    }
    let pitch = ((ROWS_BOTTOM - ROWS_TOP) / (count - folder).max(1) as f32).min(ROW_HEIGHT + ROW_GAP);
    let gap = (pitch * ROW_GAP / (ROW_HEIGHT + ROW_GAP)).max(1.0);
    (ROWS_TOP + (index - folder) as f32 * pitch, pitch - gap)
}

fn draw(sdl: &Sdl, game: &Game, settings: &Settings, status: &(String, bool), warning: &Option<[String; 2]>, selected: usize, advanced: bool) {
    sdl.background();
    sdl.header(&game.title.to_uppercase(), game.version);
    let all = rows(game, settings, advanced);
    if advanced {
        let left = MARGIN + 4.0;
        sdl.write(left, FOLDER_TOP + 4.0, "Advanced settings", theme().text);
        sdl.write(left, FOLDER_TOP + 17.0, "The defaults suit most machines.", theme().dim);
        sdl.write(left, FOLDER_TOP + 30.0, "Change them if the game runs badly.", theme().dim);
    }
    for (index, row) in all.iter().enumerate() {
        let (top, height) = place(index, all.len(), advanced);
        let area = [MARGIN, top, WIDTH - 2.0 * MARGIN, height];
        sdl.panel(area, index == selected);
        let line = top + (height - CHARACTER) / 2.0;
        let left = MARGIN + 4.0;
        match row {
            Row::Folder => {
                sdl.write(left, top + 4.0, "Game files", theme().text);
                sdl.write_right(top + 4.0, "Choose folder...", theme().dim);
                let [first, second] = path_lines(&absolute(&settings.folder));
                sdl.write_small(left, top + 15.0, &first, theme().value);
                sdl.write_small(left, top + 23.0, &second, theme().value);
                sdl.write(left, top + 34.0, &shorten(&status.0), if status.1 { theme().good } else { theme().bad });
            }
            Row::File => sdl.write(left, line, "Choose a disc image or game file", theme().text),
            Row::Disc => {
                sdl.write(left, line, "CD", theme().text);
                let (value, colour) = match found_disc(game, &settings.disc) {
                    Ok(place) => (disc_name(&place), theme().value),
                    Err(_) => ("Choose disc image...".to_string(), theme().bad),
                };
                sdl.write_right(line, &value, colour);
            }
            Row::SkipLauncher if !status.1 => {
                // Offered only once the game folder checks out.
                sdl.write(left, line, "Skip launcher next time", theme().dim);
                sdl.write_right(line, "Off", theme().dim);
            }
            Row::Scale | Row::Drawing | Row::Display | Row::Renderer | Row::Language | Row::Switch(_) | Row::Mod(_) | Row::Files | Row::SkipLauncher => {
                let label = match row {
                    Row::Files => "Files kept",
                    Row::SkipLauncher => "Skip launcher next time",
                    Row::Scale => "3D resolution",
                    Row::Drawing => "3D drawn by",
                    Row::Display => "Display",
                    Row::Renderer => "Renderer",
                    Row::Language => "Text language",
                    Row::Switch(switch) => game.switches[*switch].2,
                    Row::Mod(index) => game.mods[*index].1,
                    _ => "",
                };
                if *row == Row::Scale && fixed_resolution(game, settings) {
                    // Greyed out, with no arrows: there's nothing to choose.
                    sdl.write(left, line, label, theme().dim);
                    sdl.write_right(line, "640 x 480", theme().dim);
                } else if *row == Row::Drawing && fixed_resolution(game, settings) {
                    // The game's own software renderer runs on the processor.
                    sdl.write(left, line, label, theme().dim);
                    sdl.write_right(line, "CPU", theme().dim);
                } else {
                    sdl.write(left, line, label, theme().text);
                    sdl.write_right(line, &value_text(game, *row, settings).unwrap_or_default(), theme().value);
                }
            }
            Row::Start => {
                let label = "Start game";
                let colour = if status.1 { theme().good } else { theme().dim };
                sdl.write((WIDTH - label.len() as f32 * CHARACTER) / 2.0, line, label, colour);
            }
            Row::Advanced | Row::Hotkeys | Row::Back | Row::Quit => {
                let label = match row {
                    Row::Advanced => "Advanced settings...",
                    Row::Hotkeys => "Hotkeys...",
                    Row::Back => "Back",
                    _ => "Quit",
                };
                sdl.write((WIDTH - label.len() as f32 * CHARACTER) / 2.0, line, label, theme().text);
            }
        }
    }
    // A warning takes the place of the two hint lines, which are the least needed.
    let lines = match warning {
        Some([first, second]) => [(first.as_str(), theme().value), (second.as_str(), theme().value)],
        None => {
            let hint = if fixed_resolution(game, settings) {
                "Software draws 3D at 640 x 480"
            } else if settings.processor && settings.scale >= 4 {
                "Higher 3D resolutions need a fast CPU"
            } else {
                "Drop a disc image or folder here"
            };
            [(hint, theme().dim), (if advanced { "Arrows/d-pad, Enter/A, Esc to go back" } else { "Arrows/d-pad, Enter/A, Esc to quit" }, theme().dim)]
        }
    };
    for (index, (text, colour)) in lines.iter().enumerate() {
        sdl.write((WIDTH - text.len() as f32 * CHARACTER) / 2.0, HEIGHT - 26.0 + index as f32 * 12.0, text, *colour);
    }
    if !TAKING_PICTURE.load(std::sync::atomic::Ordering::Relaxed) {
        unsafe { (sdl.present)(sdl.renderer) };
    }
}

/// Set while `snapshot` draws, so the drawing stays in the renderer to be read.
static TAKING_PICTURE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The `launcher:NAME` script step, for the look of the launcher: with a
/// display (`./port recomp z "display launcher:NAME ..."`), the launcher is
/// drawn once and saved as NAME.png beside the run's other pictures, and the
/// scripted run goes on without it.
pub fn snapshot(game: &Game, settings: &Settings) {
    const PIXELFORMAT_RGB24: u32 = 0x1710_1803;
    let Ok(script) = std::env::var("RECOMP_SCRIPT") else { return };
    let Some(name) = script.split_whitespace().find_map(|step| step.strip_prefix("launcher:")) else { return };
    let _ = THEME.set(Theme::parse(game.theme));
    let Some(sdl) = Sdl::open(game) else {
        println!("no window for the launcher's picture");
        return;
    };
    let all = rows(game, settings, false);
    let start = all.iter().position(|row| *row == Row::Start).unwrap_or(0);
    TAKING_PICTURE.store(true, std::sync::atomic::Ordering::Relaxed);
    draw(&sdl, game, settings, &game_status(game, settings), &None, start, false);
    TAKING_PICTURE.store(false, std::sync::atomic::Ordering::Relaxed);
    unsafe {
        let read = (sdl.read_pixels)(sdl.renderer, std::ptr::null());
        let rgb = if read.is_null() { read } else { (sdl.convert_surface)(read, PIXELFORMAT_RGB24) };
        if !rgb.is_null() {
            let base = rgb as *const u8;
            let (width, height, pitch) = (*(base.add(8) as *const i32), *(base.add(12) as *const i32), *(base.add(16) as *const i32));
            let pixels = *(base.add(24) as *const *const u8);
            let mut rows = Vec::with_capacity((width * height * 3) as usize);
            for y in 0..height {
                rows.extend_from_slice(std::slice::from_raw_parts(pixels.add((y * pitch) as usize), (width * 3) as usize));
            }
            let out = std::env::var("RECOMP_OUT").unwrap_or_else(|_| ".".to_string());
            crate::host::script::write_png(&format!("{out}/{name}.png"), width as u32, height as u32, &rows);
            println!("launcher saved as {name}.png");
            (sdl.destroy_surface)(rgb);
        }
        if !read.is_null() {
            (sdl.destroy_surface)(read);
        }
    }
    sdl.close();
}

/// Whether the chosen renderer draws 3D at 640x480 whatever the resolution row says. Only Glide
/// is drawn by the runtime, at any size. The chosen resolution is kept for when Glide is chosen
/// again.
fn fixed_resolution(game: &Game, settings: &Settings) -> bool {
    game.renderers.get(settings.renderer).is_some_and(|(_, label)| label.eq_ignore_ascii_case("Software"))
}

/// The value a setting row shows on its right, between its arrows.
fn value_text(game: &Game, row: Row, settings: &Settings) -> Option<String> {
    let value = match row {
        Row::Scale if fixed_resolution(game, settings) => return None,
        Row::Scale => format!("{} x {}", 640 * settings.scale, 480 * settings.scale),
        Row::Drawing if fixed_resolution(game, settings) => return None,
        // "Graphics card (OpenGL)" doesn't fit beside the label.
        Row::Drawing => (if settings.processor { "CPU" } else { "GPU (OpenGL)" }).to_string(),
        Row::Display => (if settings.fullscreen { "Fullscreen" } else { "Window" }).to_string(),
        Row::Renderer => game.renderers.get(settings.renderer)?.1.to_string(),
        Row::Language => game.languages.get(settings.language)?.1.to_string(),
        Row::Switch(switch) => (if settings.switches[switch] { "On" } else { "Off" }).to_string(),
        Row::Mod(index) => (if settings.mods[index] { "On" } else { "Off" }).to_string(),
        Row::Files => (if settings.xdg { "Home (XDG)" } else { "Beside program" }).to_string(),
        Row::SkipLauncher => (if settings.skip_launcher { "On" } else { "Off" }).to_string(),
        _ => return None,
    };
    Some(format!("< {value} >"))
}

/// The step a click at `x` on a row means: down on its `<`, up on its `>`, and
/// otherwise what Enter does.
fn click_step(game: &Game, row: Row, settings: &Settings, x: f32) -> i32 {
    let right = WIDTH - MARGIN - 4.0;
    let Some(text) = value_text(game, row, settings) else { return 0 };
    let left = right - text.len() as f32 * CHARACTER;
    // A little wider than the arrow itself, which is one character.
    if (left - CHARACTER..left + 1.5 * CHARACTER).contains(&x) {
        -1
    } else if (right - 1.5 * CHARACTER..right + CHARACTER).contains(&x) {
        1
    } else {
        0
    }
}

/// Changes a setting by one step, or does what a button row does. Returns the
/// outcome when the launcher is done.
fn change(sdl: &Sdl, game: &Game, row: Row, step: i32, settings: &mut Settings, ready: bool) -> Option<bool> {
    note(&format!("pressed {row:?}, step {step}"));
    match row {
        Row::Folder => {
            let start = dialog_start(&settings.folder);
            unsafe { (sdl.pick_folder)(folder_picked, std::ptr::null_mut(), sdl.window, start.as_ptr(), false) };
        }
        // A file: the disc image, or any file of a game that's in a folder.
        Row::File => {
            let start = dialog_start(&settings.folder);
            unsafe { (sdl.pick_file)(folder_picked, std::ptr::null_mut(), sdl.window, std::ptr::null(), 0, start.as_ptr(), false) };
        }
        // The disc image, or any file of a disc mounted or copied to a folder.
        Row::Disc => {
            let start = dialog_start(if settings.disc.is_empty() { &settings.folder } else { &settings.disc });
            unsafe { (sdl.pick_file)(disc_picked, std::ptr::null_mut(), sdl.window, std::ptr::null(), 0, start.as_ptr(), false) };
        }
        Row::Scale if fixed_resolution(game, settings) => {}
        Row::Scale => {
            let next = settings.scale as i32 + if step == 0 { 1 } else { step };
            settings.scale = if next > LAST_SCALE as i32 { FIRST_SCALE } else if next < FIRST_SCALE as i32 { LAST_SCALE } else { next as u32 };
        }
        Row::Drawing if fixed_resolution(game, settings) => {}
        Row::Drawing => settings.processor = !settings.processor,
        Row::Display => settings.fullscreen = !settings.fullscreen,
        Row::Renderer => {
            let count = game.renderers.len() as i32;
            settings.renderer = (settings.renderer as i32 + if step == 0 { 1 } else { step }).rem_euclid(count) as usize;
        }
        Row::Language => {
            let count = game.languages.len() as i32;
            settings.language = (settings.language as i32 + if step == 0 { 1 } else { step }).rem_euclid(count) as usize;
        }
        Row::Switch(index) => settings.switches[index] = !settings.switches[index],
        Row::Mod(index) => settings.mods[index] = !settings.mods[index],
        Row::Files => settings.xdg = !settings.xdg,
        Row::SkipLauncher if ready => settings.skip_launcher = !settings.skip_launcher,
        Row::Hotkeys if step == 0 => hotkey_page(sdl, game),
        Row::Start if step == 0 && ready => return Some(true),
        Row::Quit if step == 0 => return Some(false),
        _ => {}
    }
    None
}

/// The "Hotkeys..." page: one row per key the port adds, then "Reset to defaults" and
/// "Back". Enter or a click on a row waits for the next key, which becomes that hotkey's;
/// Escape cancels the wait, or leaves the page. The keys are saved with the other settings
/// when the game starts.
fn hotkey_page(sdl: &Sdl, game: &Game) {
    let count = hotkeys::HOTKEYS.len() + 2;
    let (reset, back) = (count - 2, count - 1);
    let mut selected = 0;
    let mut waiting = false;
    let mut event = Event([0; 128]);
    loop {
        while unsafe { (sdl.poll)(&mut event) } {
            unsafe { (sdl.convert)(sdl.renderer, &mut event) };
            let kind = event.u32_at(0);
            if waiting {
                if kind == EVENT_KEY_DOWN {
                    let key = event.u32_at(28);
                    if key != KEY_ESCAPE {
                        hotkeys::set(selected, key);
                        note(&format!("hotkey {} set to {}", hotkeys::HOTKEYS[selected].name, hotkeys::display_name(key)));
                    }
                    waiting = false;
                } else if kind == EVENT_GAMEPAD_BUTTON_DOWN && event.0[20] == BUTTON_EAST {
                    waiting = false;
                } else if kind == EVENT_QUIT {
                    return;
                }
                continue;
            }
            // Delete, Backspace or X turns the chosen hotkey off: no key triggers it.
            let clear = match kind {
                EVENT_KEY_DOWN => matches!(event.u32_at(28), KEY_DELETE | KEY_BACKSPACE),
                EVENT_GAMEPAD_BUTTON_DOWN => event.0[20] == BUTTON_WEST,
                _ => false,
            };
            if clear && selected < reset {
                hotkeys::set(selected, hotkeys::NONE);
                note(&format!("hotkey {} turned off", hotkeys::HOTKEYS[selected].name));
                continue;
            }
            // (movement, press, leave)
            let action = match kind {
                EVENT_QUIT => return,
                EVENT_KEY_DOWN => match event.u32_at(28) {
                    KEY_UP => Some((-1, false, false)),
                    KEY_DOWN => Some((1, false, false)),
                    KEY_RETURN | KEY_SPACE => Some((0, true, false)),
                    KEY_ESCAPE => Some((0, false, true)),
                    _ => None,
                },
                EVENT_GAMEPAD_BUTTON_DOWN => match event.0[20] {
                    BUTTON_UP => Some((-1, false, false)),
                    BUTTON_DOWN => Some((1, false, false)),
                    BUTTON_SOUTH => Some((0, true, false)),
                    BUTTON_EAST | BUTTON_START => Some((0, false, true)),
                    _ => None,
                },
                EVENT_MOUSE_MOTION | EVENT_MOUSE_BUTTON_DOWN => {
                    let (x, y) = (event.f32_at(28), event.f32_at(32));
                    let hit = (0..count).find(|index| {
                        let (top, height) = place(*index, count, true);
                        (MARGIN..WIDTH - MARGIN).contains(&x) && (top..top + height).contains(&y)
                    });
                    hit.and_then(|index| {
                        selected = index;
                        (kind == EVENT_MOUSE_BUTTON_DOWN).then_some((0, true, false))
                    })
                }
                _ => None,
            };
            let Some((movement, press, leave)) = action else { continue };
            if leave || (press && selected == back) {
                return;
            }
            if movement != 0 {
                selected = (selected as i32 + movement).rem_euclid(count as i32) as usize;
            } else if press && selected == reset {
                hotkeys::reset();
                note("hotkeys reset");
            } else if press {
                waiting = true;
            }
        }
        sdl.background();
        sdl.header(&game.title.to_uppercase(), game.version);
        let left = MARGIN + 4.0;
        sdl.write(left, FOLDER_TOP + 4.0, "Hotkeys", theme().text);
        sdl.write(left, FOLDER_TOP + 17.0, "The keys this port adds.", theme().dim);
        sdl.write(left, FOLDER_TOP + 30.0, "Move one that clashes with the game.", theme().dim);
        let keys = hotkeys::all();
        for index in 0..count {
            let (top, height) = place(index, count, true);
            sdl.panel([MARGIN, top, WIDTH - 2.0 * MARGIN, height], index == selected);
            let line = top + (height - CHARACTER) / 2.0;
            if index < reset {
                sdl.write(left, line, hotkeys::HOTKEYS[index].label, theme().text);
                let value = if waiting && index == selected { "Press a key".to_string() } else { hotkeys::display_name(keys[index]) };
                sdl.write_right(line, &value, theme().value);
            } else {
                let label = if index == reset { "Reset to defaults" } else { "Back" };
                sdl.write((WIDTH - label.len() as f32 * CHARACTER) / 2.0, line, label, theme().text);
            }
        }
        let hint = if waiting { "Press the new key, Esc to cancel" } else { "Enter: change  Del/X: none  Esc: back" };
        sdl.write((WIDTH - hint.len() as f32 * CHARACTER) / 2.0, HEIGHT - 20.0, hint, theme().dim);
        unsafe { (sdl.present)(sdl.renderer) };
        unsafe { (sdl.delay)(FRAME_MILLISECONDS) };
    }
}

/// The row to select: Start when the game is ready, else the disc's row when the folder
/// is an installed copy that only lacks its disc, else the folder's.
fn waiting_row(all: &[Row], status: &(String, bool)) -> usize {
    if status.1 {
        return all.iter().position(|row| *row == Row::Start).unwrap_or(0);
    }
    all.iter().position(|row| *row == Row::Disc).unwrap_or(0)
}

pub fn show(game: &Game, mut settings: Settings) -> Outcome {
    let gave_folder = std::env::args().skip(1).any(|argument| !argument.starts_with("--"));
    load(game, &mut settings, gave_folder);
    let _ = THEME.set(Theme::parse(game.theme));
    note_settings(game, &settings);
    note("opening the window");
    let Some(sdl) = Sdl::open(game) else {
        note(&format!("no window: {}", sdl_error()));
        return Outcome::Unavailable(settings);
    };
    note("window open, checking the game");
    let mut advanced = false;
    let mut all = rows(game, &settings, advanced);
    let mut status = game_status(game, &settings);
    note(&status.0);
    let mut warning = folder_warning(&settings.folder);
    // Start is ready to press, unless the folder or the disc needs choosing first.
    let mut selected = waiting_row(&all, &status);
    let mut event = Event([0; 128]);
    let finished = loop {
        let mut result = None;
        while unsafe { (sdl.poll)(&mut event) } {
            unsafe { (sdl.convert)(sdl.renderer, &mut event) };
            let action = match event.u32_at(0) {
                EVENT_QUIT => Some((None, 0, true)),
                EVENT_KEY_DOWN => match event.u32_at(28) {
                    KEY_UP => Some((Some(-1), 0, false)),
                    KEY_DOWN => Some((Some(1), 0, false)),
                    KEY_LEFT => Some((None, -1, false)),
                    KEY_RIGHT => Some((None, 1, false)),
                    KEY_RETURN | KEY_SPACE => Some((None, 0, false)),
                    KEY_ESCAPE => Some((None, 0, true)),
                    _ => None,
                },
                EVENT_GAMEPAD_ADDED => {
                    unsafe { (sdl.open_gamepad)(event.u32_at(16)) };
                    None
                }
                EVENT_GAMEPAD_BUTTON_DOWN => match event.0[20] {
                    BUTTON_UP => Some((Some(-1), 0, false)),
                    BUTTON_DOWN => Some((Some(1), 0, false)),
                    BUTTON_LEFT => Some((None, -1, false)),
                    BUTTON_RIGHT => Some((None, 1, false)),
                    BUTTON_SOUTH => Some((None, 0, false)),
                    BUTTON_START => {
                        advanced = false;
                        all = rows(game, &settings, advanced);
                        selected = all.iter().position(|row| *row == Row::Start).unwrap_or(0);
                        Some((None, 0, false))
                    }
                    BUTTON_EAST => Some((None, 0, true)),
                    _ => None,
                },
                kind @ (EVENT_MOUSE_MOTION | EVENT_MOUSE_BUTTON_DOWN) => {
                    let (x, y) = (event.f32_at(28), event.f32_at(32));
                    let hit = (0..all.len()).find(|index| {
                        let (top, height) = place(*index, all.len(), advanced);
                        (MARGIN..WIDTH - MARGIN).contains(&x) && (top..top + height).contains(&y)
                    });
                    match hit {
                        Some(index) => {
                            selected = index;
                            (kind == EVENT_MOUSE_BUTTON_DOWN).then(|| (None, click_step(game, all[index], &settings, x), false))
                        }
                        None => None,
                    }
                }
                EVENT_DROP_FILE => {
                    let pointer = usize::from_le_bytes(event.0[40..48].try_into().unwrap()) as *const c_char;
                    if !pointer.is_null() {
                        let dropped = unsafe { CStr::from_ptr(pointer) }.to_string_lossy().into_owned();
                        *PICKED.lock().unwrap() = Some(dropped);
                    }
                    None
                }
                _ => None,
            };
            let Some((movement, step, quit)) = action else { continue };
            // Escape and B leave the advanced page before they leave the
            // launcher, and closing the window always quits.
            let closing = event.u32_at(0) == EVENT_QUIT;
            let leave = advanced && ((quit && !closing) || (step == 0 && movement.is_none() && all[selected] == Row::Back));
            let enter = !advanced && !quit && step == 0 && movement.is_none() && all[selected] == Row::Advanced;
            if leave || enter {
                advanced = enter;
                all = rows(game, &settings, advanced);
                selected = if enter { 0 } else { all.iter().position(|row| *row == Row::Advanced).unwrap_or(0) };
                continue;
            }
            if quit {
                result = Some(false);
                break;
            }
            if let Some(movement) = movement {
                selected = (selected as i32 + movement).rem_euclid(all.len() as i32) as usize;
                continue;
            }
            if let Some(done) = change(&sdl, game, all[selected], step, &mut settings, status.1) {
                result = Some(done);
                break;
            }
        }
        if let Some(picked) = PICKED.lock().unwrap().take() {
            note(&format!("picked {picked:?}"));
            settings.folder = game_place(game, &picked);
            status = game_status(game, &settings);
            note(&status.0);
            warning = folder_warning(&settings.folder);
            // The folder's row is on the first page.
            advanced = false;
            all = rows(game, &settings, advanced);
            selected = waiting_row(&all, &status);
        }
        if let Some(picked) = PICKED_DISC.lock().unwrap().take() {
            note(&format!("picked the disc {picked:?}"));
            settings.disc = picked;
            status = game_status(game, &settings);
            note(&status.0);
            selected = waiting_row(&all, &status);
        }
        // An installed copy's disc image is unpacked before the game starts, as a
        // chosen disc image is, and the folder it went to is what's remembered.
        if result == Some(true) && installed(game, &settings.folder) {
            let unpacked = found_disc(game, &settings.disc).and_then(|place| {
                if disc::is_image(std::path::Path::new(&place)) {
                    note("unpacking the installed copy's disc");
                }
                loader::disc_folder(game, &place, &mut progress_screen(&sdl, game, "Unpacking the disc"))
            });
            match unpacked {
                Ok(folder) => settings.disc = folder.display().to_string(),
                Err(problem) => {
                    note(&problem);
                    status = (problem, false);
                    selected = waiting_row(&all, &status);
                    result = None;
                }
            }
        }
        // A disc image is unpacked before the game starts, and the folder it
        // went to is what's remembered. If that fails the launcher stays, with
        // the reason where the folder's state is shown.
        if result == Some(true) && disc::is_image(std::path::Path::new(&settings.folder)) {
            note("unpacking the disc image");
            match unpack_image(&sdl, game, &settings.folder) {
                Ok(folder) => settings.folder = folder,
                Err(problem) => {
                    note(&problem);
                    status = (problem, false);
                    selected = 0;
                    result = None;
                }
            }
        }
        // A game folder that can't be written to, such as a mounted disc, is copied first,
        // since the game saves beside its files.
        if result == Some(true) && !disc::is_writable(std::path::Path::new(&settings.folder)) {
            note("copying the read-only game folder");
            match copy_read_only(&sdl, game, &settings.folder) {
                Ok(folder) => settings.folder = folder,
                Err(problem) => {
                    note(&problem);
                    status = (problem, false);
                    selected = 0;
                    result = None;
                }
            }
        }
        if let Some(done) = result {
            break done;
        }
        draw(&sdl, game, &settings, &status, &warning, selected, advanced);
        unsafe { (sdl.delay)(FRAME_MILLISECONDS) };
    };
    note(if finished { "closing to start the game" } else { "closing, the player quit" });
    note_settings(game, &settings);
    sdl.close();
    if finished {
        save(game, &settings);
        Outcome::Start(settings)
    } else {
        Outcome::Quit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_fits_the_screen() {
        // 1024x768 with a taskbar: 640x480.
        assert_eq!(window_scale(Some((1024, 728))), 2);
        // With no taskbar, 960x720 and its title bar just fit.
        assert_eq!(window_scale(Some((1024, 768))), 3);
        assert_eq!(window_scale(Some((1920, 1040))), 4);
        assert_eq!(window_scale(Some((3840, 2100))), 4);
        assert_eq!(window_scale(Some((800, 560))), 2);
        assert_eq!(window_scale(Some((640, 400))), 1);
        assert_eq!(window_scale(None), FALLBACK_WINDOW_SCALE);
        assert_eq!(window_scale(Some((0, 0))), FALLBACK_WINDOW_SCALE);
    }
}
