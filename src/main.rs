#![windows_subsystem = "windows"]

use recomp_runtime::{loader, GuestFn};

static MODULES: &[loader::Module] = &[
    loader::Module { file: "Z/WINZ.EXE", base: 0x400000, sha256: "9d2f373083342e76603ad3ff155c7cd557d0d7ba5fb1b6e4b7923d7739ebb25e", program: true, name: "Windows 95 release" },
    loader::Module { file: "ZED.EXE", base: 0x1000000, sha256: "5d61461181ae0c0b4b718c9f7dab61d027f623e4d5ce02b0b79047d80e36979b", program: true, name: "DOS release" },
];

/// Each program's functions, from its own crate.
static TABLES: &[&[(u32, GuestFn)]] = &[
    z_code_winz::TABLE,
    z_code_zed::TABLE,
];

static GAME: loader::Game = loader::Game {
    title: "Z",
    version: include_str!("../VERSION").trim_ascii(),
    needs: "Z by the Bitmap Brothers: the Windows 95 release (WINZ.EXE dated 1998-03-02) or the German or English DOS release (the Steam release isn't supported)",
    command: "Z\\WINZ.EXE",
    switches: &[],
    renderers: &[],
    languages: &[],
    mods: &[],
    modules: MODULES,
    tables: TABLES,
    icon: include_bytes!("../icon.rgba"),
    music: &[],
    registry: &[("SOFTWARE\\Bitmap Brothers\\Z\\v1.0\\WinZPaths\\War", "C:\\GAME\\Z\\"), ("SOFTWARE\\Bitmap Brothers\\Z\\v1.0\\WinZPaths\\Cuts", "D:\\"), ("SOFTWARE\\Bitmap Brothers\\Z\\v1.0\\WinZPaths\\SeenIntro", "dword:1")],
    cd: "Z95VOLUME",
    theme: "style=metal;background=26262c;panel=46464e;selected=6e4420;border=f08c28;text=e8e4dc;value=ff7a3c;good=9be05a;bad=ff4632;dim=948e86",
};

fn main() {
    loader::run(&GAME);
}
