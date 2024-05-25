# sig

[![ci](https://github.com/ynqa/sig/actions/workflows/ci.yml/badge.svg)](https://github.com/ynqa/sig/actions/workflows/ci.yml)

Interactive grep (for streamings).

<img src="https://github.com/ynqa/ynqa/blob/master/demo/sig.gif">

## Features

- Filter streaming container logs based on keywords
  - (currently) Not offer search functionality at the level of regular expressions, grep or fuzzy search
  - Extracts logs that match a specific word by
    [contains](https://doc.rust-lang.org/std/string/struct.String.html#method.contains)
- Digger mode
  - Enable querying the latest N logs when switching to the mode
- Reconnect to log API
  - Allows users to control when to reconnect
- Flow control that determines how many logs are rendered within a certain period

## Installation

### Homebrew

```bash
brew install ynqa/tap/sig
```

### Cargo

```bash
cargo install sig-rs
```

## Keymap

| Key                  | Action
| :-                   | :-
| <kbd>Ctrl + C</kbd>  | Exit `sig`
| <kbd>Ctrl + F</kbd>  | Enter Archived mode
| <kbd>Ctrl + R</kbd>  | Reconnect to log API
| <kbd>←</kbd>         | Move the cursor one character to the left
| <kbd>→</kbd>         | Move the cursor one character to the right
| <kbd>Ctrl + A</kbd>  | Move the cursor to the start of the filter
| <kbd>Ctrl + E</kbd>  | Move the cursor to the end of the filter
| <kbd>Backspace</kbd> | Delete a character of filter at the cursor position
| <kbd>Ctrl + U</kbd>  | Delete all characters of filter

## Usage

```bash

```
