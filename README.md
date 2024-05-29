# sig

[![ci](https://github.com/ynqa/sig/actions/workflows/ci.yml/badge.svg)](https://github.com/ynqa/sig/actions/workflows/ci.yml)

Interactive grep

|![sig.gif](https://github.com/ynqa/ynqa/blob/master/demo/sig.gif)|![sig_archived.gif](https://github.com/ynqa/ynqa/blob/master/demo/sig_archived.gif)|
|---|---|

## Features

- Interactive grep (for streaming)
  - *sig* allows users to interactively search through (streaming) data,
    updating results in real-time.
- Archived mode
  - In Archived mode, since there is no seeking capability
    for streaming data received through a pipe,
    it is not possible to search backwards without exiting the process.
    Therefore, in *sig*, the latest N entries of streaming data are saved,
    and it is possible to switch to a mode
    where you can grep through these N entries
    based on key inputs at any given moment.
  - Additionally, by starting in this mode,
    it is also possible to grep through static data such as files.

## Installation

### Homebrew

```bash
brew install ynqa/tap/sigrs
```

### Cargo

```bash
cargo install sigrs
```

### Nix (flakes)

Add it as an input to your flake:
```nix
inputs = {
  sig.url = 'github:ynqa/sig/<optional-ref>'
}
```

Create a shell with it:
```nix
nix shell github:ynqa/sig
```

Or run it directly:
```nix
cat README.md | nix run github:ynqa/sig -- --archived
```

### Nix (classic)

Fetch the source and use it, e.g. in your shell:

```nix
let
  # At the time of writing this, pkgs need to be unstable for the package to build properly
  # (requires Rust 1.74, stable has 1.73)
  pkgs = import <nixpkgs-unstable> {};

  sig = pkgs.callPackage (pkgs.fetchFromGitHub {
    owner = "ynqa";
    repo = "sig";
    rev = "<revision, e.g. master/v0.1.0/etc.>";
    hash = ""; # Build first, put proper hash in place
  }) {};
in
  pkgs.mkShell {
    packages = [sig];
  }
```

## Keymap

| Key                  | Action
| :-                   | :-
| <kbd>Ctrl + C</kbd>  | Exit `sig`
| <kbd>Ctrl + F</kbd>  | Enter Archived mode
| <kbd>←</kbd>         | Move the cursor one character to the left
| <kbd>→</kbd>         | Move the cursor one character to the right
| <kbd>Ctrl + A</kbd>  | Move the cursor to the start of the filter
| <kbd>Ctrl + E</kbd>  | Move the cursor to the end of the filter
| <kbd>Backspace</kbd> | Delete a character of filter at the cursor position
| <kbd>Ctrl + U</kbd>  | Delete all characters of filter

(Archived mode)

| Key                  | Action
| :-                   | :-
| <kbd>Ctrl + C</kbd>  | Exit Archived mode
| <kbd>←</kbd>         | Move the cursor one character to the left
| <kbd>→</kbd>         | Move the cursor one character to the right
| <kbd>Ctrl + A</kbd>  | Move the cursor to the start of the filter
| <kbd>Ctrl + E</kbd>  | Move the cursor to the end of the filter
| <kbd>Backspace</kbd> | Delete a character of filter at the cursor position
| <kbd>Ctrl + U</kbd>  | Delete all characters of filter

## Usage

```bash
Interactive grep (for streaming)

Usage: sig [OPTIONS]

Options:
      --retrieval-timeout <RETRIEVAL_TIMEOUT_MILLIS>
          Timeout to read a next line from the stream in milliseconds. [default: 10]
      --render-interval <RENDER_INTERVAL_MILLIS>
          Interval to render a line in milliseconds. [default: 10]
  -q, --queue-capacity <QUEUE_CAPACITY>
          Queue capacity to store lines. [default: 1000]
  -a, --archived
          Archived mode to grep through static data.
  -i, --ignore-case
          Case insensitive search.
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```
