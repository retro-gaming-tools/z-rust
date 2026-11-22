# Changelog

Newest first. Each version has a tag `v<version>`, and `./changes.sh <old> <new>` shows the full diff between two of them.

## 0.51-beta (2026-11-22)

- A folder that holds only a disc image can be chosen as the game.
- `recomp.log` beside the program records the launcher's steps and any crash, to attach to an issue. The run before's is kept as `recomp-previous.log`.
- Fixed a memory bug that could cut data short when the game resized a block of memory.
- The README lists the supported versions and known limits in the same form as the other ports.
- The licence and README now name `programs/` as the translated game code, which the licence excludes. They named `src/code/`, which this port doesn't use.

## 0.5-beta (2026-11-20)

The first public release.
