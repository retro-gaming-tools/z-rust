//! Disc images, real CD and DVD drives and read-only game folders: the shared
//! `recomp-disc` crate (the tool's `tools/recomp/disc`), which every runtime
//! uses, so a fix there reaches every game. Z adds one step: a DOS disc keeps
//! its program and data in Z.PAC, which its installer unpacked to the hard
//! disk, so an unpacked or copied disc gets them beside its own files (no
//! name is in both).

use std::path::Path;

pub use recomp_disc::*;

/// Unpacks Z.PAC, if the disc has one at its top, into the half-done copy.
fn unpack_pac(folder: &Path) -> Result<(), String> {
    let Some(pac) = std::fs::read_dir(folder).ok().into_iter().flatten().flatten().map(|entry| entry.path()).find(|path| path.file_name().is_some_and(|name| name.eq_ignore_ascii_case("Z.PAC"))) else {
        return Ok(());
    };
    let data = std::fs::read(&pac).map_err(|error| format!("{} can't be read: {error}", pac.display()))?;
    super::zpac::unpack_into(&data, folder)
}

/// `recomp_disc::unpack`, with Z.PAC unpacked.
pub fn unpack(image: &Path, target: &Path, progress: &mut dyn FnMut(u64, u64)) -> Result<(), String> {
    unpack_with(image, target, progress, &mut unpack_pac)
}

/// `recomp_disc::copy_folder`, with Z.PAC unpacked.
pub fn copy_folder(source: &Path, target: &Path, progress: &mut dyn FnMut(u64, u64)) -> Result<(), String> {
    copy_folder_with(source, target, progress, &mut unpack_pac)
}
