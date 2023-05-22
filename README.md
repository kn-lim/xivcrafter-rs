# XIVCrafter (Rust)

Automatically activates multiple crafting macros while refreshing food and potion buffs.

Tested on Windows and Keyboard only.

## Packages

- [tui](https://github.com/fdehau/tui-rs)
- [serde](https://github.com/serde-rs/serde)
- [serde_json](https://github.com/serde-rs/json)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [enigo](https://github.com/enigo-rs/enigo)
- [dirs](https://github.com/dirs-dev/dirs-rs)

# Using the Tool

**Download the Windows 64-bit binary in the [Releases](https://github.com/kn-lim/xivcrafter-rs/releases) page.**

## How to Build

Run:

```
cargo build
```

Binary is located in `target/debug`

## How to Run

Run:

```
./xivcrafter
```

- Although this program is able to output a keyboard event to any window in focus, the terminal must be in focus for it to receive a user input. Therefore, if the program is running and you want to pause it, you will need to focus on the terminal and then press the pause hotkey.
