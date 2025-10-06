# 15-348 Rust Template

## Overview

This is a `cargo generate` template for bare metal Rust on the rp2040.  This is meant to be the starter code for homework and projects in 15-348: Embedded Systems at Carnegie Mellon University in Qatar.

## Usage

- Change to a directory where you want your new project to live.  The new project will be a subdirectory of that directory.  For example, you might want to be in a `homeworks/` directory.  (And inside you'll make a `hw2` project.)
- `cargo generate --git https://github.com/CMUQ-15-348/rp2040-template`
- Choose a name for your project (like `hw2`)
- `cd hw2`
- `cargo run`

## Features

This template includes...

- A `Cargo.toml` with basic dependencies.
- A `src/main.rs` that blinks the LED
- A starter library for properly configuring the system clocks.
- A starter library for direct control register access
- A `.cargo/config.toml` that properly sets up using `probe-rs` to program and execute code
- A `.vscode/` directory that allows for single step debugging, assuming that you have a suitable `openocd` and `gdb` installed.
