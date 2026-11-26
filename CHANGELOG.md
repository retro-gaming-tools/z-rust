# Changelog

Newest first. Each version has a tag `v<version>`, and `./changes.sh <old> <new>` shows the full diff between two of them.

## 0.61-beta (2026-11-26)

- The README lists each supported release (system, language) with the full SHA-256 of its program, and how to check your own copy. Any dump of a supported disc works: only the program file is checked, not the disc image. `recomp.log` records the program's SHA-256.
- Fixed the detection of some disc images.
- `recomp.log` records the command line the port was started with.
- The launcher can be skipped next time, and the port's hotkeys can be remapped or turned off in its new Hotkeys row.
- The launcher's title bar has orange lamps instead of stripes.

## 0.6-beta (2026-11-24)

- Linux: the settings, `recomp.log` and unpacked discs can be kept in the XDG folders (`~/.config/z-rust`, `~/.local/share/z-rust`) instead of beside the program, with the launcher's Advanced row "Files kept" or `--xdg` ([issue #1](https://github.com/retro-gaming-tools/z-rust/issues/1)). Beside the program stays the default.
- A real CD or DVD in a drive can be chosen as the game: a mounted disc is read like a disc image. A read-only game folder is copied beside the program first.
- Disc images, drives and read-only folders are read by one shared part of the runtime.

## 0.51-beta (2026-11-22)

- A folder that holds only a disc image can be chosen as the game.
- `recomp.log` beside the program records the launcher's steps and any crash, to attach to an issue. The run before's is kept as `recomp-previous.log`.
- Fixed a memory bug that could cut data short when the game resized a block of memory.
- The README lists the supported versions and known limits in the same form as the other ports.
- The licence and README now name `programs/` as the translated game code, which the licence excludes. They named `src/code/`, which this port doesn't use.

## 0.5-beta (2026-11-20)

The first public release.
