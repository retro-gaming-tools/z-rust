//! CD and DVD access shared by every recompiled game's runtime, the shared one
//! and the ports' own copies alike, so a fix here reaches every game.
//!
//! - A disc image (`.iso`, `.bin`/`.cue`, `.img`/`.ccd`, `.mdf`) is read file
//!   by file, so a player can hand a port their disc instead of installing it.
//! - A real drive reads like an image: `/dev/sr0` on Linux, `\\.\D:` on
//!   Windows. A mounted disc's folder leads to its drive (`drive_of`).
//! - A game folder that can't be written to, such as a disc, is copied whole
//!   (`copy_folder`), since games save beside their files.
//!
//! Plain ISO 9660 images have 2048 bytes a sector. Raw ones, such as the
//! `.bin` of a `.bin` and `.cue` pair or a CloneCD `.img`, have 2352 with the
//! data inside. Only the data track is read: audio tracks aren't files. Every
//! read covers whole sectors, as a Windows drive demands.
//!
//! Game-specific steps, such as Z unpacking its `Z.PAC`, go in through the
//! `finish` hooks of `unpack_with` and `copy_folder_with`.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub const LOGICAL: u64 = 2048;
/// The volume descriptor is in sector 16, and names the root directory here.
const DESCRIPTOR_SECTOR: u64 = 16;
const ROOT_RECORD: usize = 156;
const FOLDER_FLAG: u8 = 2;
/// Deeper than any real disc, against images whose folders loop.
const DEEPEST: usize = 16;

pub struct Disc {
    file: File,
    sector: u64,
    skip: u64,
}

#[derive(Clone, Debug)]
pub struct Entry {
    /// The path on the disc, with `/` between its parts, as the disc spells it.
    pub path: String,
    sector: u64,
    pub size: u64,
}

/// Whether a file's name says it's a disc image or the sheet that goes with one,
/// or it's a disc drive itself, such as `/dev/sr0`, which reads like an image.
pub fn is_image(path: &Path) -> bool {
    let extension = path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    (path.is_file() && matches!(extension.as_str(), "iso" | "bin" | "img" | "mdf" | "cue" | "ccd")) || is_drive(path)
}

/// Whether a path is a disc drive itself rather than an image file.
#[cfg(unix)]
pub fn is_drive(path: &Path) -> bool {
    use std::os::unix::fs::FileTypeExt;
    std::fs::metadata(path).is_ok_and(|meta| meta.file_type().is_block_device())
}

/// On Windows a drive is `\\.\D:`, which opens like a file of the disc's data sectors.
#[cfg(windows)]
pub fn is_drive(path: &Path) -> bool {
    let text = path.to_string_lossy();
    let bytes = text.as_bytes();
    bytes.len() == 6 && (text.starts_with(r"\\.\") || text.starts_with("//./")) && bytes[4].is_ascii_alphabetic() && bytes[5] == b':'
}

#[cfg(not(any(unix, windows)))]
pub fn is_drive(_path: &Path) -> bool {
    false
}

/// The drive a read-only folder is the mount of, such as `/dev/sr0` for a CD
/// mounted at `/run/media/NAME/LABEL`, when the drive can be read: reading the
/// disc whole finds a program anywhere on it, or packed in an archive.
#[cfg(target_os = "linux")]
pub fn drive_of(folder: &Path) -> Option<PathBuf> {
    let folder = std::fs::canonicalize(folder).ok()?;
    let mounts = std::fs::read_to_string("/proc/self/mounts").ok()?;
    let drive = mounts.lines().find_map(|line| {
        let mut parts = line.split(' ');
        let (source, point) = (parts.next()?, parts.next()?);
        (Path::new(&unescape_mount(point)) == folder && source.starts_with("/dev/")).then(|| PathBuf::from(source))
    })?;
    (is_drive(&drive) && Disc::open(&drive).is_ok()).then_some(drive)
}

/// On Windows, the root of a CD or DVD drive letter, such as `D:\`, stands for `\\.\D:`.
#[cfg(windows)]
pub fn drive_of(folder: &Path) -> Option<PathBuf> {
    #[link(name = "kernel32")]
    extern "system" {
        fn GetDriveTypeW(root: *const u16) -> u32;
    }
    const DRIVE_CDROM: u32 = 5;
    let text = folder.to_string_lossy();
    // A plain `D:`, `D:\` or `D:/`, or the same behind `\\?\` as `canonicalize` gives it.
    let text = text.strip_prefix(r"\\?\").unwrap_or(&text);
    let bytes = text.as_bytes();
    let root = bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && text[2..].chars().all(|c| c == '\\' || c == '/');
    if !root {
        return None;
    }
    let letter = bytes[0] as char;
    let wide: Vec<u16> = format!("{letter}:\\").encode_utf16().chain(std::iter::once(0)).collect();
    if unsafe { GetDriveTypeW(wide.as_ptr()) } != DRIVE_CDROM {
        return None;
    }
    let drive = PathBuf::from(format!(r"\\.\{letter}:"));
    Disc::open(&drive).is_ok().then_some(drive)
}

#[cfg(not(any(target_os = "linux", windows)))]
pub fn drive_of(_folder: &Path) -> Option<PathBuf> {
    None
}

/// The drive behind a folder that is a mounted CD or DVD, when the folder
/// can't be written to and the drive can be read.
pub fn mounted_drive(folder: &Path) -> Option<PathBuf> {
    (folder.is_dir() && !is_writable(folder)).then(|| drive_of(folder)).flatten()
}

/// A mount point as `/proc/self/mounts` writes it, with a space as `\040`.
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn unescape_mount(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        let octal = bytes.get(at + 1..at + 4).filter(|_| bytes[at] == b'\\').and_then(|digits| u8::from_str_radix(std::str::from_utf8(digits).ok()?, 8).ok());
        match octal {
            Some(byte) => {
                out.push(byte);
                at += 4;
            }
            None => {
                out.push(bytes[at]);
                at += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Whether files can be made in a folder: not on a CD or DVD, nor in a
/// read-only mount.
pub fn is_writable(folder: &Path) -> bool {
    let probe = folder.join(".recomp-write-test");
    let made = File::create(&probe).is_ok();
    let _ = std::fs::remove_file(&probe);
    made
}

/// The disc image in a folder that holds one, such as a `.bin` beside its
/// `.cue`: the sheet first (`.cue`, then `.ccd`), since it knows the tracks,
/// else the only image file there. Only files that open as a disc count, so a
/// game's own data file with an image's ending, such as Z's `PHRASES.BIN`,
/// doesn't make its folder an image.
pub fn image_in(folder: &Path) -> Option<PathBuf> {
    let mut images: Vec<PathBuf> = std::fs::read_dir(folder).ok()?.flatten().map(|entry| entry.path()).filter(|path| is_image(path) && Disc::open(path).is_ok()).collect();
    images.sort();
    let with = |wanted: &str| images.iter().find(|path| path.extension().is_some_and(|e| e.eq_ignore_ascii_case(wanted))).cloned();
    with("cue").or_else(|| with("ccd")).or_else(|| (images.len() == 1).then(|| images[0].clone()))
}

/// The file that holds the data: a `.cue` names it in its first FILE line, and
/// a `.ccd` sits beside an `.img` of the same name.
fn data_file(path: &Path) -> PathBuf {
    let extension = path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    match extension.as_str() {
        "cue" => {
            let sheet = std::fs::read(path).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).unwrap_or_default();
            let named = sheet.lines().find_map(|line| line.trim().strip_prefix("FILE")?.split('"').nth(1).map(str::to_string));
            named.map_or_else(|| path.with_extension("bin"), |name| path.with_file_name(name))
        }
        "ccd" => path.with_extension("img"),
        _ => path.to_path_buf(),
    }
}

impl Disc {
    pub fn open(path: &Path) -> Result<Disc, String> {
        let data = data_file(path);
        let mut file = File::open(&data).map_err(|error| format!("{} can't be opened: {error}", data.display()))?;
        // The descriptor starts with a type byte and "CD001", wherever the
        // sector's data begins. Whole sectors are read: a Windows drive
        // (`\\.\D:`) refuses reads that don't start and end on one.
        for (sector, skip) in [(2048, 0), (2352, 16), (2352, 24), (2336, 8)] {
            let mut whole = vec![0u8; sector as usize];
            let found = file.seek(SeekFrom::Start(DESCRIPTOR_SECTOR * sector)).is_ok() && file.read_exact(&mut whole).is_ok();
            if found && &whole[skip as usize + 1..skip as usize + 6] == b"CD001" {
                return Ok(Disc { file, sector, skip });
            }
        }
        Err(format!("{} isn't a CD image this can read", data.display()))
    }

    /// One logical sector's data, read whole.
    fn read_sector(&mut self, sector: u64, whole: &mut [u8; LOGICAL as usize]) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(sector * self.sector + self.skip))?;
        self.file.read_exact(whole)
    }

    fn read_sectors(&mut self, mut sector: u64, size: u64) -> std::io::Result<Vec<u8>> {
        let mut data = vec![0u8; size as usize];
        let mut whole = [0u8; LOGICAL as usize];
        for part in data.chunks_mut(LOGICAL as usize) {
            // The whole sector, even for a file's short last part: see `open`.
            self.read_sector(sector, &mut whole)?;
            part.copy_from_slice(&whole[..part.len()]);
            sector += 1;
        }
        Ok(data)
    }

    pub fn read(&mut self, entry: &Entry) -> std::io::Result<Vec<u8>> {
        self.read_sectors(entry.sector, entry.size)
    }

    /// One file of the disc, to read a part at a time, such as an installer's
    /// cabinet too big to read whole.
    pub fn open_entry(self, entry: &Entry) -> DiscFile {
        DiscFile { disc: self, entry: entry.clone(), at: 0 }
    }

    /// Every file on the disc.
    pub fn files(&mut self) -> Vec<Entry> {
        let mut found = Vec::new();
        if let Ok(descriptor) = self.read_sectors(DESCRIPTOR_SECTOR, LOGICAL) {
            let (sector, size) = extent(&descriptor[ROOT_RECORD..]);
            self.walk(sector, size, String::new(), 0, &mut found);
        }
        found
    }

    fn walk(&mut self, sector: u64, size: u64, folder: String, depth: usize, found: &mut Vec<Entry>) {
        let Ok(data) = self.read_sectors(sector, size) else { return };
        let mut at = 0;
        while at + 34 <= data.len() {
            let length = data[at] as usize;
            if length == 0 {
                // Records never cross a sector: the rest of this one is padding.
                at = (at / LOGICAL as usize + 1) * LOGICAL as usize;
                continue;
            }
            let record = &data[at..(at + length).min(data.len())];
            at += length;
            if record.len() < 34 || record.len() < 33 + record[32] as usize {
                continue;
            }
            let name = &record[33..33 + record[32] as usize];
            if name == [0] || name == [1] {
                continue; // The folder itself, and the one above it.
            }
            let text: String = name.iter().map(|&byte| byte as char).collect();
            let text = text.split(';').next().unwrap_or_default().trim_end_matches('.').to_string();
            // Names come from the image, and mustn't lead out of a folder,
            // nor name a drive on Windows.
            if text.is_empty() || text == ".." || text.contains(['/', '\\', ':']) {
                continue;
            }
            let (sector, size) = extent(record);
            let path = if folder.is_empty() { text } else { format!("{folder}/{text}") };
            if record[25] & FOLDER_FLAG != 0 {
                if depth < DEEPEST {
                    self.walk(sector, size, path, depth + 1, found);
                }
            } else {
                found.push(Entry { path, sector, size });
            }
        }
    }
}

/// A file inside an image or on a drive, read a part at a time.
pub struct DiscFile {
    disc: Disc,
    entry: Entry,
    at: u64,
}

impl Read for DiscFile {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        if self.at >= self.entry.size {
            return Ok(0);
        }
        // Up to the sector's end: in a raw image more than data follows it.
        // The sector is read whole, as a drive needs, and the part taken.
        let inside = self.at % LOGICAL;
        let count = (out.len() as u64).min(LOGICAL - inside).min(self.entry.size - self.at) as usize;
        let mut whole = [0u8; LOGICAL as usize];
        self.disc.read_sector(self.entry.sector + self.at / LOGICAL, &mut whole)?;
        out[..count].copy_from_slice(&whole[inside as usize..inside as usize + count]);
        self.at += count as u64;
        Ok(count)
    }
}

impl Seek for DiscFile {
    fn seek(&mut self, to: SeekFrom) -> std::io::Result<u64> {
        let at = match to {
            SeekFrom::Start(at) => Some(at),
            SeekFrom::End(by) => self.entry.size.checked_add_signed(by),
            SeekFrom::Current(by) => self.at.checked_add_signed(by),
        };
        self.at = at.ok_or(std::io::ErrorKind::InvalidInput)?;
        Ok(self.at)
    }
}

/// Where a directory record's data starts, and its size.
fn extent(record: &[u8]) -> (u64, u64) {
    let number = |at: usize| u32::from_le_bytes([record[at], record[at + 1], record[at + 2], record[at + 3]]) as u64;
    (number(2) + record[1] as u64, number(10))
}

/// What a game adds to a copy before it's taken as whole, run on the half-done
/// folder: Z unpacks its `Z.PAC` there.
pub type Finish<'a> = &'a mut dyn FnMut(&Path) -> Result<(), String>;

/// Copies every file of an image into a folder. `progress` gets the bytes
/// written so far and the total. The files go to a folder beside the target
/// first, which takes the target's name only when all are there, so a copy
/// that was interrupted is never taken for a whole game.
pub fn unpack(image: &Path, target: &Path, progress: &mut dyn FnMut(u64, u64)) -> Result<(), String> {
    unpack_with(image, target, progress, &mut |_| Ok(()))
}

/// `unpack`, with the game's own step before the copy is taken as whole.
pub fn unpack_with(image: &Path, target: &Path, progress: &mut dyn FnMut(u64, u64), finish: Finish) -> Result<(), String> {
    let mut disc = Disc::open(image)?;
    let files = disc.files();
    let total: u64 = files.iter().map(|entry| entry.size).sum();
    let partial = target.with_extension("unpacking");
    let failed = |error: std::io::Error| format!("The disc couldn't be unpacked into {}: {error}", target.display());
    let _ = std::fs::remove_dir_all(&partial);
    let mut written = 0;
    for entry in &files {
        let path = partial.join(&entry.path);
        std::fs::create_dir_all(path.parent().unwrap_or(&partial)).map_err(failed)?;
        let data = disc.read(entry).map_err(|error| format!("The image ends early or can't be read at {}: {error}", entry.path))?;
        File::create(&path).and_then(|mut file| file.write_all(&data)).map_err(failed)?;
        written += entry.size;
        progress(written, total);
    }
    finish(&partial)?;
    std::fs::rename(&partial, target).map_err(failed)
}

/// Copies a folder and everything in it, such as a game on a mounted disc,
/// with the same progress and half-done safety as `unpack`. Files lose the
/// read-only mark a disc gives them, so the game's saves can replace them.
pub fn copy_folder(source: &Path, target: &Path, progress: &mut dyn FnMut(u64, u64)) -> Result<(), String> {
    copy_folder_with(source, target, progress, &mut |_| Ok(()))
}

/// `copy_folder`, with the game's own step before the copy is taken as whole.
pub fn copy_folder_with(source: &Path, target: &Path, progress: &mut dyn FnMut(u64, u64), finish: Finish) -> Result<(), String> {
    fn list(folder: &Path, depth: usize, found: &mut Vec<(PathBuf, u64)>) -> std::io::Result<()> {
        for entry in std::fs::read_dir(folder)? {
            let entry = entry?;
            let meta = std::fs::metadata(entry.path())?;
            if meta.is_dir() {
                if depth < DEEPEST {
                    list(&entry.path(), depth + 1, found)?;
                }
            } else {
                found.push((entry.path(), meta.len()));
            }
        }
        Ok(())
    }
    let mut files = Vec::new();
    list(source, 0, &mut files).map_err(|error| format!("{} can't be read: {error}", source.display()))?;
    let total: u64 = files.iter().map(|(_, size)| size).sum();
    let partial = target.with_extension("copying");
    let failed = |error: std::io::Error| format!("The game couldn't be copied into {}: {error}", target.display());
    let _ = std::fs::remove_dir_all(&partial);
    let mut written = 0;
    for (path, size) in &files {
        let place = partial.join(path.strip_prefix(source).unwrap_or(path));
        std::fs::create_dir_all(place.parent().unwrap_or(&partial)).map_err(failed)?;
        std::fs::copy(path, &place).map_err(|error| format!("{} can't be read: {error}", path.display()))?;
        if let Ok(meta) = std::fs::metadata(&place) {
            let mut permissions = meta.permissions();
            #[allow(clippy::permissions_set_readonly_false)]
            permissions.set_readonly(false);
            let _ = std::fs::set_permissions(&place, permissions);
        }
        written += size;
        progress(written, total);
    }
    finish(&partial)?;
    std::fs::rename(&partial, target).map_err(failed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("recomp-disc-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A small ISO 9660 image: sector 16 the descriptor, sector 18 the root
    /// folder, holding `GAME.EXE;1` (sector 20) and a folder `DATA` (sector 19)
    /// with `LEVEL.DAT;1` (sectors 21 and 22, 3000 bytes).
    fn image(sector: usize, skip: usize) -> Vec<u8> {
        fn record(name: &[u8], at: u32, size: u32, folder: bool) -> Vec<u8> {
            let mut record = vec![0u8; 33 + name.len() + (name.len() + 1) % 2];
            record[0] = record.len() as u8;
            record[2..6].copy_from_slice(&at.to_le_bytes());
            record[10..14].copy_from_slice(&size.to_le_bytes());
            record[25] = if folder { FOLDER_FLAG } else { 0 };
            record[32] = name.len() as u8;
            record[33..33 + name.len()].copy_from_slice(name);
            record
        }
        let mut logical = vec![vec![0u8; LOGICAL as usize]; 23];
        logical[16][0] = 1;
        logical[16][1..6].copy_from_slice(b"CD001");
        let root = record(&[0], 18, LOGICAL as u32, true);
        logical[16][ROOT_RECORD..ROOT_RECORD + root.len()].copy_from_slice(&root);
        let mut folder = Vec::new();
        for part in [record(&[0], 18, 2048, true), record(&[1], 18, 2048, true), record(b"GAME.EXE;1", 20, 3, false), record(b"DATA", 19, 2048, true), record(b"C:BAD;1", 20, 3, false)] {
            folder.extend(part);
        }
        logical[18][..folder.len()].copy_from_slice(&folder);
        let inner = record(b"LEVEL.DAT;1", 21, 3000, false);
        logical[19][..inner.len()].copy_from_slice(&inner);
        logical[20][..3].copy_from_slice(b"abc");
        for at in 0..3000 {
            logical[21 + at / 2048][at % 2048] = at as u8;
        }
        let mut out = vec![0xeeu8; logical.len() * sector];
        for (number, data) in logical.iter().enumerate() {
            out[number * sector + skip..number * sector + skip + LOGICAL as usize].copy_from_slice(data);
        }
        out
    }

    #[test]
    fn reads_plain_and_raw_images() {
        let dir = scratch("images");
        for (name, sector, skip) in [("plain.iso", 2048, 0), ("raw.bin", 2352, 16)] {
            let path = dir.join(name);
            std::fs::write(&path, image(sector, skip)).unwrap();
            assert!(is_image(&path));
            let mut disc = Disc::open(&path).unwrap();
            let files = disc.files();
            let names: Vec<&str> = files.iter().map(|entry| entry.path.as_str()).collect();
            // A name with a drive's colon is left out.
            assert_eq!(names, ["GAME.EXE", "DATA/LEVEL.DAT"]);
            assert_eq!(disc.read(&files[0]).unwrap(), b"abc");
            let level = disc.read(&files[1]).unwrap();
            assert_eq!(level.len(), 3000);
            assert!(level.iter().enumerate().all(|(at, &byte)| byte == at as u8));
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn reads_a_file_a_part_at_a_time() {
        let dir = scratch("entry");
        let path = dir.join("raw.bin");
        std::fs::write(&path, image(2352, 16)).unwrap();
        let mut disc = Disc::open(&path).unwrap();
        let level = disc.files().into_iter().find(|entry| entry.path == "DATA/LEVEL.DAT").unwrap();
        let mut file = disc.open_entry(&level);
        assert_eq!(file.seek(SeekFrom::End(0)).unwrap(), 3000);
        file.seek(SeekFrom::Start(2000)).unwrap();
        let mut read = Vec::new();
        file.read_to_end(&mut read).unwrap();
        assert_eq!(read.len(), 1000);
        assert!(read.iter().enumerate().all(|(at, &byte)| byte == (at + 2000) as u8));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unpacks_whole_with_the_games_step() {
        let dir = scratch("unpack");
        let path = dir.join("plain.iso");
        std::fs::write(&path, image(2048, 0)).unwrap();
        let target = dir.join("game");
        let mut last = (0, 0);
        unpack_with(&path, &target, &mut |written, total| last = (written, total), &mut |partial| {
            std::fs::write(partial.join("EXTRA"), b"x").map_err(|error| error.to_string())
        })
        .unwrap();
        assert_eq!(last, (3003, 3003));
        assert_eq!(std::fs::read(target.join("GAME.EXE")).unwrap(), b"abc");
        assert!(target.join("DATA/LEVEL.DAT").is_file() && target.join("EXTRA").is_file());
        assert!(!dir.join("game.unpacking").exists());
        // A failing step leaves no whole-looking copy.
        let other = dir.join("other");
        assert!(unpack_with(&path, &other, &mut |_, _| {}, &mut |_| Err("no".into())).is_err());
        assert!(!other.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn copies_a_folder_whole_and_writable() {
        let dir = scratch("copy");
        let source = dir.join("disc");
        std::fs::create_dir_all(source.join("DATA")).unwrap();
        std::fs::write(source.join("GAME.EXE"), b"abc").unwrap();
        std::fs::write(source.join("DATA/LEVEL.DAT"), b"defg").unwrap();
        let mut permissions = std::fs::metadata(source.join("GAME.EXE")).unwrap().permissions();
        permissions.set_readonly(true);
        std::fs::set_permissions(source.join("GAME.EXE"), permissions).unwrap();
        assert!(is_writable(&source));
        assert_eq!(mounted_drive(&source), None);
        let target = dir.join("game");
        let mut last = (0, 0);
        copy_folder(&source, &target, &mut |written, total| last = (written, total)).unwrap();
        assert_eq!(last, (7, 7));
        assert_eq!(std::fs::read(target.join("DATA/LEVEL.DAT")).unwrap(), b"defg");
        assert!(!std::fs::metadata(target.join("GAME.EXE")).unwrap().permissions().readonly());
        assert!(!dir.join("game.copying").exists());
        assert!(!is_image(&source));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn finds_the_image_in_a_folder() {
        let dir = scratch("image-in");
        assert_eq!(image_in(&dir), None);
        // A game's data file with an image's ending isn't an image (Z's PHRASES.BIN).
        std::fs::write(dir.join("PHRASES.BIN"), b"not a disc").unwrap();
        assert_eq!(image_in(&dir), None);
        std::fs::write(dir.join("Game.bin"), image(2352, 16)).unwrap();
        assert_eq!(image_in(&dir), Some(dir.join("Game.bin")));
        // The sheet wins over the data it names.
        std::fs::write(dir.join("Game.CUE"), b"").unwrap();
        std::fs::write(dir.join("readme.txt"), b"").unwrap();
        assert_eq!(image_in(&dir), Some(dir.join("Game.CUE")));
        // Two images and no sheet: no guess.
        std::fs::remove_file(dir.join("Game.CUE")).unwrap();
        std::fs::write(dir.join("Other.iso"), image(2048, 0)).unwrap();
        assert_eq!(image_in(&dir), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn reads_mount_points_with_spaces() {
        assert_eq!(unescape_mount("/run/media/rf/Z\\040GOLD"), "/run/media/rf/Z GOLD");
        assert_eq!(unescape_mount("/mnt/cd\\"), "/mnt/cd\\");
    }
}
