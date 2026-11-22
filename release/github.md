Z Recompiled: Z (The Bitmap Brothers, 1996) as a native Windows and Linux program, translated statically from the original into Rust, on a runtime built on SDL3. The coding was done by AI (Claude, by Anthropic). The testing was done by a human.

**You need your own copy of Z**: the Windows 95 release (`WINZ.EXE` dated 1998-03-02) or the German or English DOS release. The Steam release isn't supported. No game data is included.

## Changes in 0.51-beta

- A folder that holds only a disc image can be chosen as the game.
- `recomp.log` beside the program records the launcher's steps and any crash, to attach to an issue. The run before's is kept as `recomp-previous.log`.
- Fixed a memory bug that could cut data short when the game resized a block of memory.
- The README lists the supported versions and known limits in the same form as the other ports.
- The licence and README now name `programs/` as the translated game code, which the licence excludes. They named `src/code/`, which this port doesn't use.

The full history is in `CHANGELOG.md`.

## Downloads

- `z-rust-0.51-beta-windows-x64.zip`: unzip anywhere and start `z-rust.exe`. Windows may warn that the program is from an unknown publisher, because it isn't code-signed: choose "More info", then "Run anyway".
- `z-rust-0.51-beta-linux-x64.AppImage`: make it executable and start it.
- `z-rust-0.51-beta-SHA256SUMS.txt`: checksums of both.

In the launcher, pick your disc image or any file of the game, or drop it onto the window. You don't install the game: an image is unpacked once beside the program.

## Status

This is an early release. Both original programs, the Windows 95 and the DOS one, are recompiled and play as their own programs. If something doesn't work, please open an issue and attach `recomp.log` from beside the program. The README lists what's supported and the known limits.

## Checking these files

GitHub built these files from the tagged source with `./build.sh`, in the workflow run linked to this release. Each file has a provenance attestation:

    gh attestation verify <file> --repo retro-gaming-tools/z-rust

You can also build the release yourself with `./build.sh`.

## Legal

This is a fan project. It isn't affiliated with or endorsed by the rights holders of Z. The recompiler, runtime, mods and build are MIT-licensed. The translated game code isn't licensed, because it belongs to the game's rights holders.
