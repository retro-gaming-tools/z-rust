## The launcher

![The launcher](release/launcher.png)

The launcher opens before the game. Your choices are saved in `z-rust.settings.ini` beside the program (beside the `.AppImage` file on Linux), and `--play` on the command line skips the launcher.

- **Game files**: where your copy of the game is. Click *Choose folder...* or press Enter to pick a folder, or drop a folder or disc image onto the window. The line below it says which version was found, or what's missing.
- **Choose a disc image or game file**: pick a disc image (`.iso`, `.bin`, `.img`, `.mdf`, `.cue`, `.ccd`) or any file of the game.
- **Display**: *Window* or *Fullscreen*.
- **Skip launcher next time**: *On* starts the game straight away from then on. `--launcher` on the command line, or Shift held while the port starts, brings the launcher back.
- **Hotkeys...**: change the port's own keys (fullscreen, frame counter): choose a row, press the new key; Delete, Backspace or X turns a hotkey off, and *Reset to defaults* brings back F11 and F10.
- **Advanced settings... > Files kept** (Linux only): *Beside program* (the default: `z-rust.settings.ini`, `z-rust.log` and unpacked discs beside the `.AppImage`) or *Home (XDG)* (`~/.config/z-rust` for the settings, `~/.local/share/z-rust` for the log and unpacked discs, for AppImage managers that keep all AppImages in one folder). `--xdg` on the command line does the same.
- **Start game** and **Quit**.

The launcher works with the mouse, the keyboard (arrow keys to choose and change, Enter to confirm, Escape to quit) and a controller (d-pad to choose and change, A to confirm, Start to start the game, B to quit).

In the game, **F11** switches between fullscreen and a window, and **F10** shows or hides a frame counter. The port starts in fullscreen unless the launcher's Display row says Window. **The hotkeys belong to the port**: the game never sees them, so they can't be used for anything in the game. Move or turn them off with the launcher's *Hotkeys...* row if you need those keys.

## Supported versions

Z came as two programs: the Windows 95 release (`WINZ.EXE`) and the DOS release (`ZED.EXE`). The port carries both, and each plays as its own program, the way it did on its own system. It checks the one it starts by SHA-256:

| Release | System | Language | Program | SHA-256 of the program |
|---|---|---|---|---|
| Windows 95 disc | Windows 95 | English (menus also in German, French, Italian, Spanish) | `Z/WINZ.EXE` (1998-03-02, 2,016,256 bytes) | `9d2f373083342e76603ad3ff155c7cd557d0d7ba5fb1b6e4b7923d7739ebb25e` |
| European DOS disc | DOS | English | `ZED.EXE` (inside `Z.PAC`, 2,316,209 bytes) | `5d61461181ae0c0b4b718c9f7dab61d027f623e4d5ce02b0b79047d80e36979b` |
| German DOS disc, also the 1997 budget disc | DOS | German | `ZED.EXE` (inside `Z.PAC`, 2,316,209 bytes) | `5d61461181ae0c0b4b718c9f7dab61d027f623e4d5ce02b0b79047d80e36979b` |

Each comes as a disc image (`.iso`, `.bin`, `.img`, `.mdf`, `.cue`, `.ccd`), a real disc, or the disc's files. The two DOS discs carry the same program; the port starts it in German or English to match the disc.

Not supported, as known so far:

| Release | System | Program | SHA-256 of the program |
|---|---|---|---|
| Steam release | Windows | its own program | — |
| DOS copies installed to a hard disk, with `Z.BAT` starting `ZED.EXE /CD:C` (found online) | DOS | `ZED.EXE` (2,320,417 bytes) | `3635cb6960637eecd2298a84e8d6bdc4fe571ce88c716d3235151b7964184676` |

Only the program is checked, not the disc image or the other game files, so any dump of a supported disc works, and patches that replace only data files don't change the check. To see your program's value, open a terminal in the folder that holds it (on the disc, `Z` for the Windows 95 release) and run:

- Linux: `sha256sum WINZ.EXE`
- Windows (PowerShell): `Get-FileHash WINZ.EXE`
- Windows (Command Prompt): `certutil -hashfile WINZ.EXE SHA256`

The DOS release's `ZED.EXE` is packed inside `Z.PAC`, so it can't be checked this way: pick the disc or its folder in the launcher, which says whether it found a supported version. Either way, `z-rust.log`, beside the port's program, lists the value of each program the port checked, `ZED.EXE` inside `Z.PAC` too.

**The Steam release doesn't work with this port**: its program is a different one. Other releases and languages aren't supported either: their programs differ, and the launcher says which version it found, or that it found none.

## Starting

You don't install the game: its installers don't run on modern systems. In the launcher, pick your disc image or any file of the game in a folder, or drop it on the window. An image is unpacked once into a `game` folder beside the program (`game-zed` for the DOS release), and every later start finds it by itself. A real CD or DVD in a drive works the same way: pick the mounted disc's folder or any file on it. A DOS copy you installed to a hard disk years ago works too, with its options, saves and high scores: pick its folder, and the launcher's CD row asks for its disc (an image, the mounted disc, or a folder of its files), which becomes the game's drive E:. The DOS release gets its CD drive and language switch (`/CD:E /ger` or `/CD:E /eng`) from the port, and needs no sound, video or mouse driver set-up: the port picks the best settings, with SVGA on.

## What works

- Both releases: the intro and logo videos, the menus, cutscenes, missions, saving and loading, sound effects and MIDI music.
- Windows: the game's menu bar and dialogs, drawn by the port in the Windows 95 style, and videos filling the screen.
- DOS: SVGA (640x400) and VGA missions, edge scrolling on every side, and scrolling by dragging with the right button.
- The game runs at its own speed: the Windows release's missions run at 18 frames a second, as the original did.
- Saves made with the original.

## Known limits

- Music plays on the built-in GeneralUser GS bank, so it sounds like General MIDI, not a Roland MT-32.
- No multiplayer.
- The game is shown at its original resolution (640x480, or 640x400 and 320x200 in DOS), scaled to the window. Larger views of the battlefield are planned.
- In the Windows release, Alt (or the controller's Back button) opens a menu with **Quit**, for fullscreen, where there's no window to close.

If something doesn't work, please open an issue and attach `z-rust.log`, which is beside the program (beside the `.AppImage` file on Linux). After starting again, it's `z-rust-previous.log`.
