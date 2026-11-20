//! `Z.PAC`, the archive the DOS discs' installer unpacks: the DOS program,
//! its drivers and its data. The format is in the port's `docs/formats.md`,
//! and `tools/zpac.py` is the same decoder in Python.

const DIRECTORY: usize = 0x78;
const RECORD: usize = 29;
const BLOCK_OUTPUT: usize = 65000;

/// One file in the archive: its path (`AUDIO/` for the first folder), where its
/// data starts and its unpacked size.
pub struct Entry {
    pub path: String,
    offset: usize,
    pub size: usize,
}

fn u16_at(data: &[u8], at: usize) -> Option<usize> {
    Some(u16::from_le_bytes(data.get(at..at + 2)?.try_into().ok()?) as usize)
}

fn u32_at(data: &[u8], at: usize) -> Option<usize> {
    Some(u32::from_le_bytes(data.get(at..at + 4)?.try_into().ok()?) as usize)
}

/// The archive's files, or `None` when it isn't one.
pub fn entries(data: &[u8]) -> Option<Vec<Entry>> {
    let first = u32_at(data, DIRECTORY + 1)?;
    if first > data.len() || first < DIRECTORY + RECORD {
        return None;
    }
    let mut found = Vec::new();
    let mut at = DIRECTORY;
    while at + RECORD <= first {
        let offset = u32_at(data, at + 1)?;
        let folder = data[at + 8];
        let name = &data[at + 9..at + 20];
        let part = |bytes: &[u8]| -> String { bytes.iter().map(|&b| b as char).collect::<String>().trim_end().to_string() };
        let (base, extension) = (part(&name[..8]), part(&name[8..]));
        // Names come from the file, and mustn't lead out of a folder.
        if base.is_empty() || base.contains(['/', '\\', '.']) || extension.contains(['/', '\\', '.']) {
            return None;
        }
        let file = if extension.is_empty() { base } else { format!("{base}.{extension}") };
        let path = if folder == 1 { format!("AUDIO/{file}") } else { file };
        found.push(Entry { path, offset, size: u32_at(data, at + 25)? });
        at += RECORD;
    }
    Some(found)
}

/// A file's contents.
pub fn read(data: &[u8], entry: &Entry) -> Result<Vec<u8>, String> {
    let damaged = || format!("Z.PAC is damaged at {}", entry.path);
    let mut out: Vec<u8> = Vec::with_capacity(entry.size);
    let mut at = entry.offset;
    loop {
        let flags = *data.get(at + 1).ok_or_else(damaged)?;
        let n = u16_at(data, at + 2).ok_or_else(damaged)?;
        if flags & 8 != 0 {
            let mut position = at * 8 + 35;
            let limit = out.len() + BLOCK_OUTPUT.min(entry.size.saturating_sub(out.len()));
            let mut bit = || -> Result<usize, String> {
                let byte = *data.get(position >> 3).ok_or_else(damaged)?;
                let value = (byte >> (position & 7)) as usize & 1;
                position += 1;
                Ok(value)
            };
            while out.len() < limit {
                if bit()? == 0 {
                    let mut value = 0;
                    for index in 0..8 {
                        value |= bit()? << index;
                    }
                    out.push(value as u8);
                    continue;
                }
                let mut gamma = |bit: &mut dyn FnMut() -> Result<usize, String>| -> Result<usize, String> {
                    let mut count = 0;
                    while bit()? == 0 {
                        count += 1;
                        if count > 24 {
                            return Err(damaged());
                        }
                    }
                    let mut value = 0;
                    for index in 0..count {
                        value |= bit()? << index;
                    }
                    Ok((1 << count) - 1 + value)
                };
                let code = gamma(&mut bit)?;
                let low = |bit: &mut dyn FnMut() -> Result<usize, String>| -> Result<usize, String> {
                    let mut value = 0;
                    for index in 0..8 {
                        value |= bit()? << index;
                    }
                    Ok(value)
                };
                let (length, distance) = if code == 0 { (2, low(&mut bit)?) } else { (code + 2, gamma(&mut bit)? * 256 + low(&mut bit)?) };
                if distance == 0 || distance > out.len() {
                    return Err(damaged());
                }
                for _ in 0..length {
                    out.push(out[out.len() - distance]);
                }
            }
            out.truncate(limit);
        } else {
            out.extend_from_slice(data.get(at + 4..at + 4 + n).ok_or_else(damaged)?);
        }
        at += 4 + n + 1;
        if flags & 1 != 0 || out.len() >= entry.size {
            break;
        }
    }
    if out.len() != entry.size {
        return Err(damaged());
    }
    Ok(out)
}

/// Unpacks every file of the archive into a folder.
pub fn unpack_into(data: &[u8], folder: &std::path::Path) -> Result<(), String> {
    let list = entries(data).ok_or("Z.PAC isn't an archive this can read")?;
    for entry in &list {
        let contents = read(data, entry)?;
        let path = folder.join(&entry.path);
        let failed = |error: std::io::Error| format!("{} couldn't be written: {error}", path.display());
        std::fs::create_dir_all(path.parent().unwrap_or(folder)).map_err(failed)?;
        std::fs::write(&path, contents).map_err(failed)?;
    }
    Ok(())
}
