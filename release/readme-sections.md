## The launcher

![The launcher](release/launcher.png)

The launcher opens before the game. Your choices are saved in `settings.ini` beside the program (beside the `.AppImage` file on Linux), and `--play` on the command line skips the launcher.

- **Game files**: where your copy of the game is. Click *Choose folder...* or press Enter to pick a folder, or drop a folder or disc image onto the window. The line below it says which version was found, or what's missing.
- **Choose a disc image or game file**: pick a disc image (`.iso`, `.bin`, `.img`, `.mdf`, `.cue`, `.ccd`) or any file of the game.
- **Display**: *Window* or *Fullscreen*.
- **Start game** and **Quit**.

The launcher works with the mouse, the keyboard (arrow keys to choose and change, Enter to confirm, Escape to quit) and a controller (d-pad to choose and change, A to confirm, Start to start the game, B to quit).

## Supported versions

Z came as two programs: the Windows 95 release (`WINZ.EXE`) and the DOS release (`ZED.EXE`). The port carries both, and each plays as its own program, the way it did on its own system. It checks the one it starts by SHA-256:

| Version | Program | SHA-256 | Where it comes from |
|---|---|---|---|
| Windows 95 release | `Z/WINZ.EXE` (1998-03-02) | `9d2f3730…` | The Windows 95 disc, as a disc image or its files |
| DOS release, German or English (European disc) | `ZED.EXE` (inside the disc's `Z.PAC`) | `5d614611…` | The German or the European DOS disc, as a disc image (`.iso`, `.bin`, `.img`, `.mdf`, `.cue`, `.ccd`) or its files. Both discs carry the same program; the port starts it in German or English to match the disc |

Only these original DOS and Windows 95 releases are supported. **The Steam release doesn't work with this port**: its program is a different one. Other releases and languages aren't supported either: their programs differ, and the launcher says which version it found, or that it found none.

## Starting

You don't install the game: its installers don't run on modern systems. In the launcher, pick your disc image or any file of the game in a folder, or drop it on the window. An image is unpacked once into a `game` folder beside the program (`game-zed` for the DOS release), and every later start finds it by itself. The DOS release gets its CD drive and language switch (`/CD:E /ger` or `/CD:E /eng`) from the port, and needs no sound, video or mouse driver set-up: the port picks the best settings, with SVGA on.

## What works

Played on real hardware:

- Both releases: the intro and logo videos, the menus, starting a new game with its cutscene, mission 1, sound effects and MIDI music.
- Windows: the game's menu bar and dialogs, drawn by the port in the Windows 95 style, and videos filling the screen.
- DOS: SVGA (640x400) and VGA missions, edge scrolling on every side, and scrolling by dragging with the right button.

Checked by automated runs on every build: loading the original's saves in both releases, saving and loading, and winning mission 1 and going on to mission 2 (Windows).
- The game runs at its own speed: the Windows release's missions run at 18 frames a second, as the original did.

## Known limits

- Only the first missions have been played. Later missions, planets and the ending haven't been checked, though nothing in them is known to differ.
- Music plays on the built-in GeneralUser GS bank, so it sounds like General MIDI, not a Roland MT-32.
- No multiplayer.
- The game is shown at its original resolution (640x480, or 640x400 and 320x200 in DOS), scaled to the window. Larger views of the battlefield are planned.
- In the Windows release, Alt (or the controller's Back button) opens a menu with **Quit**, for fullscreen, where there's no window to close.
