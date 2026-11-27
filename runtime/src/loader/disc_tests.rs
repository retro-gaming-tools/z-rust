//! Telling an installed DOS copy from one that holds its disc, and finding a
//! disc chosen as a folder or by a file in it (no game data: empty files).

use super::{disc_for, holds_disc};
use std::path::{Path, PathBuf};

fn folder(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("recomp-disc-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn touch(path: &Path) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, b"").unwrap();
}

/// A game with no programs: what these tests look at is only the disc's files.
static GAME: super::Game = super::Game {
    title: "Test",
    version: "1",
    needs: "",
    command: "",
    switches: &[],
    renderers: &[],
    languages: &[],
    mods: &[],
    modules: &[],
    tables: &[],
    icon: &[],
    music: &[],
    registry: &[],
    cd: "",
    theme: "",
};

fn game() -> &'static super::Game {
    &GAME
}

#[test]
fn installed_files_alone_are_no_disc() {
    let installed = folder("installed");
    touch(&installed.join("ZED.EXE"));
    touch(&installed.join("GSPRITES.RSC"));
    assert!(!holds_disc(&installed));
}

#[test]
fn disc_files_are_found_in_any_case_and_beside_the_program() {
    let disc = folder("disc");
    touch(&disc.join("z").join("main.pac"));
    assert!(holds_disc(&disc));
    // A copy of the disc with the installed files one folder down.
    let below = disc.join("INSTALL");
    std::fs::create_dir_all(&below).unwrap();
    assert!(holds_disc(&below));
}

#[test]
fn a_disc_chosen_by_a_file_in_it_is_its_folder() {
    let disc = folder("chosen");
    touch(&disc.join("Z").join("MAIN.PAC"));
    touch(&disc.join("SPRITES.RSC"));
    let picked = disc.join("SPRITES.RSC").display().to_string();
    assert_eq!(disc_for(game(), &picked), Ok(disc.display().to_string()));
}

#[test]
fn a_folder_without_the_disc_is_refused() {
    let other = folder("other");
    touch(&other.join("README.TXT"));
    assert!(disc_for(game(), &other.display().to_string()).is_err());
}
