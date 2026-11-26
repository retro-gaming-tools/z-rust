Z Recompiled: Z (The Bitmap Brothers, 1996) as a native Windows and Linux program, translated statically from the original into Rust, on a runtime built on SDL3. The coding was done by AI (Claude, by Anthropic). The testing was done by a human.

**You need your own copy of Z**: the Windows 95 release (`WINZ.EXE` dated 1998-03-02) or the German or English DOS release. The Steam release isn't supported. No game data is included.

## Changes in 0.61-beta

- The README lists each supported release (system, language) with the full SHA-256 of its program, and how to check your own copy. Any dump of a supported disc works: only the program file is checked, not the disc image. `recomp.log` records the program's SHA-256.
- Fixed the detection of some disc images.
- `recomp.log` records the command line the port was started with.
- The launcher can be skipped next time, and the port's hotkeys can be remapped or turned off in its new Hotkeys row.
- The launcher's title bar has orange lamps instead of stripes.

The full history is in `CHANGELOG.md`.

## Downloads

- `z-rust-0.61-beta-windows-x64.zip`: unzip anywhere and start `z-rust.exe`. Windows may warn that the program is from an unknown publisher, because it isn't code-signed: choose "More info", then "Run anyway".
- `z-rust-0.61-beta-linux-x64.AppImage`: make it executable and start it.
- `z-rust-0.61-beta-SHA256SUMS.txt`: checksums of both.

In the launcher, pick your disc image or any file of the game, or drop it onto the window. You don't install the game: an image is unpacked once beside the program.

## Status

This is an early release. Both original programs, the Windows 95 and the DOS one, are recompiled and play as their own programs. If something doesn't work, please open an issue and attach `recomp.log` from beside the program. The README lists what's supported and the known limits.

## Checking these files

GitHub built these files from the tagged source with `./build.sh`, in the workflow run linked to this release. Each file has a provenance attestation:

    gh attestation verify <file> --repo retro-gaming-tools/z-rust

You can also build the release yourself with `./build.sh`.

## Legal

This is a fan project. It isn't affiliated with or endorsed by the rights holders of Z. The recompiler, runtime, mods and build are MIT-licensed. The translated game code isn't licensed, because it belongs to the game's rights holders.
