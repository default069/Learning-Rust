Project description:
This project was created for learning the Rust programming language and for gaining personal experience with systems programming and the Rust standard library.

The goal of this project is not to create a full Bash replacement, but to understand how shells work internally by implementing their basic functionality from scratch.

Current implemented commands:

• ls — lists all files and folders in the current working directory.
• cd — changes the current working directory.
• mkdir — creates a new directory.
• touch — mention it as a planned feature if it is not implemented yet, or describe it if implemented.

The project is written only with the Rust Standard Library.
No external crates are used.

README should include:

# Project Title

A short introduction explaining why the project exists.

## About

Explain that this project is a personal educational project made to practice Rust, filesystem operations, command parsing, and terminal applications.

## Features

Use nice checkboxes.

Example:

✅ ls

✅ cd

✅ mkdir

✅ touch 
## Project Goals

Explain that the purpose is to learn:

- Rust ownership
- Borrow checker
- Pattern matching
- File system API
- PathBuf
- std::fs
- std::env
- Command parsing
- Error handling
- Modular code

## Technologies

- Rust
- Cargo
- Standard Library

## Current Project Structure

Describe the current architecture.

main()
├── input()
├── command_ls()
├── command_cd()
├── command_mkdir()
└── (touch planned)

Use a Markdown code block.

## Example

Show example terminal usage.

please writing command:
ls

Documents
Downloads
Music

please writing command:
mkdir test

Folder successfully created!

please writing command:
cd test

Successfully changed working directory.

## What I Learned

Describe what this project helped me understand:

- filesystem programming
- PathBuf
- directory traversal
- reading directories
- working directory management
- Rust error handling
- ownership
- Result and Option
- match expressions

## Getting Started

Include installation instructions.

```bash
git clone https://github.com/USERNAME/bash-prototype.rs.git
cd bash-prototype.rs
cargo run
