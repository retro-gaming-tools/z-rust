//! Sets up guest memory and starts the game.
//!
//! The game's data sections are read from the player's own files at run time,
//! at the same addresses the recompiler used.

use crate::cpu::{Cpu, GuestFn, Host, IMPORT_BASE};
use crate::{host, pe};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const STACK_TOP: u32 = 0x7000_0000;
const STACK_SIZE: u32 = 0x0040_0000;
const THREAD_BLOCK: u32 = 0x7ffd_0000;
/// Rust frames stand in for the guest's calls, so the host stack has to be deep.
const HOST_STACK: usize = 512 * 1024 * 1024;

/// DLLs Windows would start before the program, dependencies first. The others
/// wait for LoadLibrary.
const STARTED_FIRST: &[&str] = &["msvcrt.dll", "msvcirt.dll", "newtoolsr.dll"];

pub struct Module {
    pub file: &'static str,
    pub base: u32,
    pub sha256: &'static str,
    /// A program, which starts a program of its own with the DLLs after it.
    /// A port can carry several, such as a game's DOS and Windows versions,
    /// and starts the one the game folder holds.
    pub program: bool,
    /// The version players know a program by, from `recompVersionNames`, such
    /// as `Windows 95 release`. The launcher names the version it finds with it.
    pub name: &'static str,
}

/// The programs a port carries: each program's module, then its DLLs.
fn programs(game: &Game) -> Vec<&'static [Module]> {
    let modules = game.modules;
    let starts: Vec<usize> = (0..modules.len()).filter(|&index| index == 0 || modules[index].program).collect();
    starts.iter().enumerate().map(|(n, &start)| &modules[start..starts.get(n + 1).copied().unwrap_or(modules.len())]).collect()
}

/// The program whose file is in the folder, or else the first.
pub(crate) fn program_in(game: &Game, folder: &str) -> &'static [Module] {
    let all = programs(game);
    all.iter().copied().find(|program| find_file(folder, program[0].file).is_some()).unwrap_or(all[0])
}

pub struct Game {
    pub title: &'static str,
    /// The release, from the generated repo's `VERSION` file, such as `0.5-beta`.
    pub version: &'static str,
    pub needs: &'static str,
    pub command: &'static str,
    /// Command-line switches, the guest address each sets to 1, and its label.
    pub switches: &'static [(&'static str, u32, &'static str)],
    /// The renderers the launcher offers, from `recompRenderers`: the game's own switch that
    /// picks each, and its label. The first is the default.
    pub renderers: &'static [(&'static str, &'static str)],
    /// The languages the launcher offers, from `recompLanguages`, the same way.
    pub languages: &'static [(&'static str, &'static str)],
    /// Mods the player can turn off in the launcher: the file in `mods/`, its
    /// label, whether it starts on, and whether its row is on the launcher's
    /// advanced page. Mods not listed are always on.
    pub mods: &'static [(&'static str, &'static str, bool, bool)],
    pub modules: &'static [Module],
    /// Each program's translated functions, by address: one table per program crate.
    pub tables: &'static [&'static [(u32, GuestFn)]],
    pub icon: &'static [u8],
    /// CD music files, from `recompMusic`: see `host::State::music`.
    pub music: &'static [(&'static str, u32, u32)],
    /// Registry strings the game expects from its installer, from
    /// `recompRegistry`: the key and name, and the text.
    pub registry: &'static [(&'static str, &'static str)],
    /// The volume label of the game's CD, from `recompCd`. With one, the game
    /// finds a CD drive D: with that label, which is the game folder.
    pub cd: &'static str,
    /// The launcher's look, from `launcherTheme` in port.json, as
    /// `name=value` pairs split by `;`: see `host::launcher::Theme`. Empty is
    /// the plain look.
    pub theme: &'static str,
}

/// A program with no console would otherwise end without a word, so a crash
/// goes into `recomp.log` (and `recomp-crash.log` here, as before) and tells the player.
fn report_crashes(title: &'static str) {
    std::panic::set_hook(Box::new(move |information| {
        let text = format!("{title} stopped with an error:\n\n{information}");
        let backtrace = std::backtrace::Backtrace::force_capture();
        crate::runlog::line(&format!("\n{text}\n\nbacktrace:\n{backtrace}"));
        let _ = std::fs::write("recomp-crash.log", &text);
        let text = match crate::runlog::path() {
            Some(path) => format!("{text}\n\nPlease send this file with your report:\n{}", path.display()),
            None => text,
        };
        if crate::runlog::automated() {
            eprintln!("{text}");
        } else {
            host::video::show_error(title, &text);
        }
    }));
}

#[cfg(test)]
mod crash_tests;

/// Without a path, the game folder is `game` here or beside the program, or else
/// this folder or the program's own, for a build copied in with the game's files.
pub(crate) fn default_folder(game: &Game) -> String {
    let program = std::env::current_exe().ok().and_then(|path| Some(path.parent()?.to_path_buf()));
    let mut places = vec![std::path::PathBuf::from("game")];
    places.extend(program.iter().map(|folder| folder.join("game")));
    // With `--xdg`, an unpacked disc is in the data folder.
    places.extend(host::launcher::files_folder().map(|folder| folder.join("game")));
    places.push(std::path::PathBuf::from("."));
    places.extend(program);
    let firsts: Vec<&str> = programs(game).iter().map(|program| program[0].file).collect();
    let found = places.iter().map(|place| place.to_string_lossy().to_string()).find(|place| firsts.iter().any(|file| find_file(place, file).is_some()));
    found.unwrap_or_else(|| "game".to_string())
}

/// Checks the files the code was recompiled from, before anything starts.
pub(crate) fn check_files(game: &Game, folder: &str) -> Result<(), String> {
    let mut missing = Vec::new();
    let mut different = Vec::new();
    for module in program_in(game, folder) {
        match find_file(folder, module.file) {
            None => missing.push(module.file),
            Some(path) => {
                let file = std::fs::read(path).unwrap_or_default();
                let found: String = Sha256::digest(&file).iter().map(|byte| format!("{byte:02x}")).collect();
                if found != module.sha256 {
                    different.push(module.file);
                }
            }
        }
    }
    if missing.is_empty() && different.is_empty() {
        return Ok(());
    }
    let absolute = std::fs::canonicalize(folder).map_or(folder.to_string(), |path| path.display().to_string());
    let mut text = format!("{} can't start, because of the files in this folder:\n{absolute}\n", game.title);
    if !missing.is_empty() {
        text.push_str(&format!("\nMissing: {}\n", missing.join(", ")));
    }
    if !different.is_empty() {
        text.push_str(&format!("\nFrom a different version: {}\n", different.join(", ")));
    }
    text.push_str(&format!(
        "\n{} needs {}.\n\nPut this program in the game's folder, or put the game's files in a folder named game beside it, or start it with the folder as its argument.",
        game.title, game.needs
    ));
    Err(text)
}

/// Checks a disc image as `check_files` checks a folder, with a reason short
/// enough for the launcher.
pub(crate) fn check_image(game: &Game, image: &std::path::Path) -> Result<(), String> {
    image_program(game, image).map(|_| ())
}

/// The name of the version in a folder or disc image that passed its check,
/// from `recompVersionNames`, when the port gives one.
pub(crate) fn version_in(game: &Game, place: &str) -> Option<&'static str> {
    let path = std::path::Path::new(place);
    let program = if host::disc::is_image(path) { image_program(game, path).ok()? } else { program_in(game, place) };
    Some(program[0].name).filter(|name| !name.is_empty())
}

/// The program on a disc image whose files all match, or why there's none.
fn image_program(game: &Game, image: &std::path::Path) -> Result<&'static [Module], String> {
    let mut disc = host::disc::Disc::open(image).map_err(|_| "Not a disc image this can read".to_string())?;
    let files = disc.files();
    // The count tells a disc of something else from an image that couldn't
    // be read: that one has no files at all.
    let mut problem = format!("Not this game's disc ({} files)", files.len());
    // A DOS disc's program is inside Z.PAC.
    let mut pac: Option<Vec<u8>> = None;
    'programs: for program in programs(game) {
        for module in program {
            let data = match files.iter().find(|entry| entry.path.eq_ignore_ascii_case(module.file)) {
                Some(entry) => disc.read(entry).map_err(|_| "The disc image is damaged".to_string())?,
                None => {
                    if pac.is_none() {
                        if let Some(entry) = files.iter().find(|entry| entry.path.eq_ignore_ascii_case("Z.PAC")) {
                            pac = Some(disc.read(entry).map_err(|_| "The disc image is damaged".to_string())?);
                        }
                    }
                    let inside = pac.as_deref().and_then(|data| {
                        let entry = host::zpac::entries(data)?.into_iter().find(|entry| entry.path.eq_ignore_ascii_case(module.file))?;
                        host::zpac::read(data, &entry).ok()
                    });
                    match inside {
                        Some(data) => data,
                        None => continue 'programs,
                    }
                }
            };
            let found: String = Sha256::digest(&data).iter().map(|byte| format!("{byte:02x}")).collect();
            if found != module.sha256 {
                problem = "This disc is a different version".to_string();
                continue 'programs;
            }
        }
        return Ok(program);
    }
    Err(problem)
}

/// Where a disc image is unpacked: a folder named `game` beside the program,
/// which is also where a start without any path looks first.
/// The first program's version goes to `game`, any other to `game-` and its
/// program's name (`game-zed` for Z's DOS discs), so a player can keep both.
/// A folder that holds another disc (Z's German and European DOS discs carry
/// the same program) is left alone, and the disc goes to `-2`, `-3` and so on.
pub(crate) fn unpack_target(game: &Game, image: &std::path::Path) -> std::path::PathBuf {
    let base = host::launcher::files_folder().unwrap_or_default();
    let first = programs(game)[0][0].file;
    let name = match image_program(game, image) {
        Ok(program) if program[0].file != first => {
            let name = program[0].file.rsplit('/').next().unwrap_or_default();
            let stem = name.split('.').next().unwrap_or_default().to_lowercase();
            format!("game-{stem}")
        }
        _ => "game".to_string(),
    };
    (1..100)
        .map(|number| if number == 1 { base.join(&name) } else { base.join(format!("{name}-{number}")) })
        .find(|folder| !folder.exists() || holds_image_version(game, &folder.display().to_string(), image))
        .unwrap_or_else(|| base.join(&name))
}

/// The drive behind a read-only folder that is a mounted CD or DVD, read like a disc image.
pub(crate) fn disc_drive(folder: &std::path::Path) -> Option<std::path::PathBuf> {
    (folder.is_dir() && !host::disc::is_writable(folder)).then(|| host::disc::drive_of(folder)).flatten()
}

/// A game folder that can't be written to, such as a mounted disc whose drive can't be read,
/// is copied into `game` beside the program, since the game writes its saves beside its
/// files. Gives the folder to use: the copy, or `folder` itself when it's writable.
pub(crate) fn copy_read_only(game: &Game, folder: &str, progress: &mut dyn FnMut(u64, u64)) -> Result<String, String> {
    let path = std::path::Path::new(folder);
    if !path.is_dir() || host::disc::is_writable(path) {
        return Ok(folder.to_string());
    }
    let target = host::launcher::files_folder().unwrap_or_default().join("game");
    let copy = target.display().to_string();
    if check_files(game, &copy).is_ok() {
        return Ok(copy);
    }
    if target.exists() {
        return Err(format!("{folder} can't be written to, and {copy} is in the way of a copy: it holds other files. Move or delete it."));
    }
    host::disc::copy_folder(path, &target, progress)?;
    Ok(copy)
}

/// Whether a folder already holds the files of the disc image: its version,
/// and every file of the disc by name, so another disc with the same program
/// (another language, Gold Games) isn't taken for it. Sizes aren't compared:
/// the game rewrites some disc files, such as Windows saves in `Z/`.
pub(crate) fn holds_image_version(game: &Game, folder: &str, image: &std::path::Path) -> bool {
    check_files(game, folder).is_ok()
        && image_program(game, image).is_ok_and(|program| program[0].file == program_in(game, folder)[0].file)
        && holds_disc_files(folder, image)
}

fn holds_disc_files(folder: &str, image: &std::path::Path) -> bool {
    let Ok(mut disc) = host::disc::Disc::open(image) else { return false };
    disc.files().iter().all(|entry| find_file(folder, &entry.path).is_some())
}

pub(crate) fn find_file(folder: &str, name: &str) -> Option<std::path::PathBuf> {
    // A name may hold folders, such as `Z/WINZ.EXE` on a copy of a disc. Each
    // part is matched without regard to case.
    let mut path = std::path::PathBuf::from(folder);
    for part in name.split('/') {
        path = std::fs::read_dir(&path).ok()?.flatten().find(|entry| entry.file_name().to_string_lossy().eq_ignore_ascii_case(part)).map(|entry| entry.path())?;
    }
    Some(path)
}

fn load_images(modules: &[Module], folder: &str) -> Vec<(String, pe::Image)> {
    modules
        .iter()
        .map(|module| {
            let path = find_file(folder, module.file).unwrap_or_else(|| panic!("{} is missing from {folder}", module.file));
            let file = std::fs::read(path).expect("a game file can't be read");
            let mut image = pe::Image::parse_any(&file).expect("a game file isn't a 32-bit Windows or DOS/4GW program");
            image.rebase(module.base);
            // Other modules import it by its name alone, whatever folder it's in.
            (module.file.rsplit('/').next().unwrap_or(module.file).to_lowercase(), image)
        })
        .collect()
}

fn resolve(image: &pe::Image, others: &[(String, pe::Image)], cpu: &mut Cpu, known: &mut HashMap<String, u32>) {
    for import in &image.imports {
        let label = match &import.name {
            Some(name) => format!("{}!{name}", import.dll),
            None => format!("{}!#{}", import.dll, import.ordinal),
        };
        let export = others.iter().find(|(file, _)| *file == import.dll).and_then(|(_, other)| {
            other.exports.iter().find(|e| match &import.name {
                Some(name) => e.name.as_deref() == Some(name),
                None => e.ordinal == import.ordinal,
            })
        });
        let address = match export {
            Some(export) => export.address,
            None => *known.entry(label.clone()).or_insert_with(|| {
                let key = import.name.clone().unwrap_or_else(|| format!("#{}", import.ordinal));
                let function = host::find(&import.dll, &key);
                cpu.imports.push(Host { name: label.clone(), function });
                IMPORT_BASE + cpu.imports.len() as u32 - 1
            }),
        };
        cpu.w32(import.slot, address);
    }
}

fn start(game: &'static Game, folder: String, switches: Vec<bool>, mods: Vec<bool>, command: &'static str) {
    let table: Vec<(u32, GuestFn)> = game.tables.iter().flat_map(|table| table.iter().copied()).collect();
    let images = load_images(program_in(game, &folder), &folder);
    let mut cpu = Cpu {
        eax: 0, ecx: 0, edx: 0, ebx: 0, esp: STACK_TOP - 0x100, ebp: 0, esi: 0, edi: 0, returned_to: 0,
        segments: [host::dos::FLAT; 6], bases: [0; 6],
        cf: false, zf: false, sf: false, of: false, df: false, parity: 1, eflags_high: 0, interrupts_off: false,
        fs_base: THREAD_BLOCK,
        st: [0.0; 8], top: 0, c0: false, c1: false, c2: false, c3: false, fpu_control: 0x27f,
        memory: crate::sys::reserve_memory(),
        functions: table.iter().copied().collect(),
        imports: Vec::new(),
        host: Box::new(host::State::new(&folder)),
        trace: std::env::var_os("RECOMP_TRACE").is_some(),
        trace_filter: std::env::var("RECOMP_TRACE").ok().filter(|value| value != "1"),
        watched: HashMap::new(),
        watch_counts: HashMap::new(),
        profiling: false,
        hits: HashMap::new(),
        error_function: None,
    };
    let errors = cpu.host.script.steps.front().and_then(|step| step.strip_prefix("errors:").map(str::to_string));
    if let Some(address) = errors {
        cpu.host.script.steps.pop_front();
        cpu.error_function = u32::from_str_radix(&address, 16).ok();
        cpu.profiling = true;
    }
    // A script can name a DLL with a `watch:` first step, to log its exports.
    let watched = cpu.host.script.steps.front().and_then(|step| step.strip_prefix("watch:").map(str::to_string));
    if let Some(watched) = watched {
        cpu.host.script.steps.pop_front();
        for (name, image) in &images {
            if *name == watched.to_lowercase() {
                for export in &image.exports {
                    cpu.watched.insert(export.address, export.name.clone().unwrap_or_default());
                }
            }
        }
    }
    for (_, image) in &images {
        unsafe { std::ptr::copy_nonoverlapping(image.memory.as_ptr(), cpu.memory.add(image.base as usize), image.memory.len()) };
    }
    let mut known = HashMap::new();
    for (_, image) in &images {
        resolve(image, &images, &mut cpu, &mut known);
    }
    for ((_, address, _), on) in game.switches.iter().zip(switches) {
        if on {
            cpu.w32(*address, 1);
        }
    }
    cpu.w32(THREAD_BLOCK, 0xffff_ffff); // No exception handlers yet.
    cpu.w32(THREAD_BLOCK + 4, STACK_TOP);
    cpu.w32(THREAD_BLOCK + 8, STACK_TOP - STACK_SIZE);
    cpu.w32(THREAD_BLOCK + 0x18, THREAD_BLOCK);

    println!("{} modules loaded, {} functions, {} imports for the runtime", images.len(), table.len(), cpu.imports.len());
    if let Ok(path) = std::env::var("RECOMP_IMPORTS") {
        let missing: Vec<&str> = cpu.imports.iter().filter(|host| host.function.is_none()).map(|host| host.name.as_str()).collect();
        let _ = std::fs::write(path, missing.join("\n"));
    }
    cpu.host.title = game.title;
    cpu.host.command = command;
    cpu.host.icon = game.icon;
    cpu.host.music = game.music;
    cpu.host.cd = game.cd;
    cpu.host.registry = host::registry::Registry::load(&cpu.host.folder, game.registry);
    cpu.host.mods_off = game.mods.iter().zip(mods).filter(|(_, on)| !on).map(|((file, ..), _)| *file).collect();
    let entry = images[0].1.entry;
    if images[0].1.is_dos() {
        // DOS's start: the PSP, the environment and the selectors, with the
        // program as `Z.BAT` and friends named it.
        let program = format!("C:\\{}", images[0].0.to_uppercase());
        let arguments = std::env::var("RECOMP_DOS_ARGUMENTS").unwrap_or_else(|_| dos_arguments(&cpu.host.folder.to_string_lossy()));
        host::dos::start(&mut cpu, &images[0].1, &program, &arguments);
    }
    for (name, image) in images {
        cpu.host.modules.push(host::LoadedModule {
            name,
            base: image.base,
            entry: image.entry,
            exports: image.exports,
            started: !image.is_dll, // The program's entry point runs last, below.
        });
    }
    for name in STARTED_FIRST {
        if let Some(index) = cpu.host.modules.iter().position(|module| module.name == *name) {
            cpu.start_module(index);
        }
    }
    cpu.push(0); // Returning from here ends the game.
    cpu.call(entry);
    // A program that ends normally exits through DOS or Windows. Returning
    // means a `ret` went somewhere unexpected and the frames unwound.
    println!("the game's entry point returned, to {:#x}, with esp {:#x} and eax {:#x}", cpu.returned_to, cpu.esp, cpu.eax);
}

/// The command line Z's `Z.BAT` would give the DOS program: the CD drive E:,
/// and the language of the disc, told by its videos (the German disc's start
/// with G_, the European disc's with E_).
fn dos_arguments(folder: &str) -> String {
    let language = if find_file(folder, "CUTS/G_LOGO.JV").is_some() { "/ger" } else { "/eng" };
    format!("/CD:E {language}")
}

/// What the player chose, on the command line or in the launcher.
pub struct Settings {
    pub folder: String,
    /// How many times 640x480 3D is drawn at, from 1 to 6.
    pub scale: u32,
    pub fullscreen: bool,
    /// Whether Glide is drawn by the processor instead of the graphics card.
    pub processor: bool,
    /// Whether settings, `recomp.log` and unpacked discs go to the XDG folders (Linux).
    pub xdg: bool,
    /// One for each of the game's switches.
    pub switches: Vec<bool>,
    /// An index into the game's renderers.
    pub renderer: usize,
    /// An index into the game's languages.
    pub language: usize,
    /// One for each of the game's optional mods: whether it's on.
    pub mods: Vec<bool>,
}

/// The renderer or language named by a label or a switch, ignoring case.
pub fn find_choice(choices: &[(&str, &str)], name: &str) -> Option<usize> {
    choices.iter().position(|(switch, label)| label.eq_ignore_ascii_case(name) || switch.eq_ignore_ascii_case(name))
}

/// The game's command line, with the chosen renderer's and language's switches in place of
/// any others from those lists.
fn command_with_choices(game: &Game, settings: &Settings) -> String {
    let lists = [(game.renderers, settings.renderer), (game.languages, settings.language)];
    let listed = |word: &str| lists.iter().any(|(list, _)| list.iter().any(|(switch, _)| switch.eq_ignore_ascii_case(word)));
    let mut words: Vec<&str> = game.command.split_whitespace().filter(|word| !listed(word)).collect();
    for (list, index) in lists {
        if let Some((chosen, _)) = list.get(index) {
            words.push(chosen);
        }
    }
    words.join(" ")
}

pub const DEFAULT_SCALE: u32 = 3;

/// Reads `[folder] [--scale=N] [--fullscreen] [--drawing=processor|card] [--renderer=NAME] [--language=NAME] [--mod-off=NAME]
/// [--mod-on=NAME] [--play] [game switches]`.
fn settings_from_arguments(game: &Game, arguments: &[String]) -> Settings {
    let has = |wanted: &str| arguments.iter().any(|argument| argument.eq_ignore_ascii_case(wanted));
    let scale = arguments.iter().find_map(|argument| argument.strip_prefix("--scale=")?.parse().ok());
    let scale = scale.or_else(|| std::env::var("RECOMP_SCALE").ok()?.parse().ok());
    Settings {
        folder: arguments.iter().find(|argument| !argument.starts_with("--")).cloned().unwrap_or_else(|| default_folder(game)),
        scale: scale.unwrap_or(DEFAULT_SCALE).clamp(1, 6),
        fullscreen: has("--fullscreen"),
        processor: has("--drawing=processor"),
        xdg: host::launcher::xdg_on(),
        switches: game.switches.iter().map(|(name, _, _)| has(name)).collect(),
        renderer: arguments.iter().find_map(|argument| find_choice(game.renderers, argument.strip_prefix("--renderer=")?)).unwrap_or(0),
        language: arguments.iter().find_map(|argument| find_choice(game.languages, argument.strip_prefix("--language=")?)).unwrap_or(0),
        mods: game.mods.iter().map(|(file, _, on, _)| {
            let named = |prefix: &str| arguments.iter().any(|argument| argument.strip_prefix(prefix).is_some_and(|name| name.eq_ignore_ascii_case(file)));
            if named("--mod-off=") { false } else if named("--mod-on=") { true } else { *on }
        }).collect(),
    }
}

/// Runs the game on a thread with a host stack deep enough for its calls.
pub fn run(game: &'static Game) {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    host::launcher::choose_folders(game, &arguments);
    crate::runlog::start(game.title, game.version, || host::launcher::settings_folder(game));
    report_crashes(game.title);
    let mut settings = settings_from_arguments(game, &arguments);
    host::launcher::snapshot(game, &settings);
    if host::launcher::wanted(&arguments) {
        // The launcher guesses no folder: it shows the one saved last time, or none to choose.
        if !arguments.iter().any(|argument| !argument.starts_with("--")) {
            settings.folder = String::new();
        }
        // A player starts in fullscreen unless they chose a window before
        // (the saved setting wins) or asked for one with `--window`.
        settings.fullscreen = !arguments.iter().any(|argument| argument == "--window");
        match host::launcher::show(game, settings) {
            host::launcher::Outcome::Start(chosen) => settings = chosen,
            host::launcher::Outcome::Quit => std::process::exit(0),
            host::launcher::Outcome::Unavailable(unchanged) => settings = unchanged,
        }
    }
    // The graphics code reads these when the game opens its window.
    std::env::set_var("RECOMP_SCALE", settings.scale.to_string());
    if settings.fullscreen {
        std::env::set_var("RECOMP_FULLSCREEN", "1");
    }
    // The player's own RECOMP_GLIDE wins, as it does for tests.
    if settings.processor && std::env::var_os("RECOMP_GLIDE").is_none() {
        std::env::set_var("RECOMP_GLIDE", "cpu");
    }
    let command: &'static str = Box::leak(command_with_choices(game, &settings).into_boxed_str());
    let Settings { mut folder, switches, mods, .. } = settings;
    // A mounted CD or DVD stands for its drive.
    if let Some(drive) = disc_drive(std::path::Path::new(&folder)) {
        folder = drive.display().to_string();
    }
    // A folder that holds only a disc image stands for that image.
    if std::path::Path::new(&folder).is_dir() && check_files(game, &folder).is_err() {
        if let Some(image) = host::disc::image_in(std::path::Path::new(&folder)) {
            folder = image.display().to_string();
        }
    }
    // A disc image given on the command line, with no launcher to unpack it.
    if host::disc::is_image(std::path::Path::new(&folder)) {
        if let Err(problem) = check_image(game, std::path::Path::new(&folder)) {
            host::video::show_error(game.title, &format!("{} can't use this disc image:\n{folder}\n\n{problem}.\n\n{} needs {}.", game.title, game.title, game.needs));
            std::process::exit(1);
        }
        let target = unpack_target(game, std::path::Path::new(&folder));
        let unpacked = target.display().to_string();
        if !holds_image_version(game, &unpacked, std::path::Path::new(&folder)) {
            println!("unpacking {folder} into {unpacked}");
            if let Err(text) = host::disc::unpack(std::path::Path::new(&folder), &target, &mut |_, _| {}) {
                host::video::show_error(game.title, &text);
                std::process::exit(1);
            }
        }
        folder = unpacked;
    }
    // A game on a disc, or anywhere else read-only, with no launcher to copy it.
    if check_files(game, &folder).is_ok() {
        match copy_read_only(game, &folder, &mut |_, _| {}) {
            Ok(copied) => folder = copied,
            Err(problem) => {
                host::video::show_error(game.title, &problem);
                std::process::exit(1);
            }
        }
    }
    crate::runlog::line(&format!("game folder: {folder}\ncommand: {command}"));
    if let Err(text) = check_files(game, &folder) {
        crate::runlog::line(&text);
        host::video::show_error(game.title, &text);
        std::process::exit(1);
    }
    let thread = std::thread::Builder::new().stack_size(HOST_STACK).spawn(move || start(game, folder, switches, mods, command));
    if thread.expect("couldn't start the game's thread").join().is_err() {
        std::process::exit(1);
    }
}
