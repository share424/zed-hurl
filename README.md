# zed-hurl

A Zed extension for [Hurl](https://hurl.dev) files, providing syntax highlighting and the ability to run individual requests directly from the editor.

This is a fork of [tommy/zed-hurl](https://github.com/tommy/zed-hurl) with added runnable support.

Many thanks to @pfeiferj, the author of the [Tree Sitter grammar for
Hurl](https://github.com/pfeiferj/tree-sitter-hurl) which this extension uses.

## Features

- Syntax highlighting for `.hurl` files
- Run the request under your cursor via Zed's task runner
- Run all requests in the current file

## Requirements

- [Hurl](https://hurl.dev/docs/installation.html) must be installed and available on your `PATH`
- [Rust](https://www.rust-lang.org/tools/install) (to build the `hurl-runner` helper binary)

## Installation

1. Clone this repository (with submodules):
   ```sh
   git clone --recurse-submodules https://github.com/share424/zed-hurl
   ```
2. Build the `hurl-runner` binary:
   ```sh
   cd hurl-runner
   cargo build --release
   ```
   or
   ```sh
   cargo install --path .
   ```
3. Add the `hurl-runner` binary to your `PATH`:
   ```sh
   # e.g. copy to a directory already on your PATH
   cp target/release/hurl-runner ~/.local/bin/
   ```
4. In Zed, open the Extensions page, click "Install Dev Extension", and select the repository directory.

## Usage

Open any `.hurl` file. Use Zed's task runner (`Ctrl+Shift+R` / `Cmd+Shift+R`) to:

- **Run hurl request** — runs the single entry at your cursor position
- **Run all hurl requests** — runs all entries in the file
