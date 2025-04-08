# Visualising-Sigma16
# Readme

`src/main.rs` starts the application from the GUI

`src/assembler` contains all the assembler code
 - `assembler.rs` contains the assembler
 - `code.rs` contains a wrapper for the assembler
 - `error.rs` contains the assembler error struct
 - `tokens.rs` contains tokens for the parser

`src/gui` contains all the GUI code
 - `app.rs` contains the user interface controller
 - `code_editor.rs` contains the code editor
 - `code_runner.rs` contains the code runner
 - `data_flow.rs` contains the data flow
 - `exercises.rs` contains exercises for testing and evaluation
 - `gui.rs` interfaces with JavaScript and HTML
 - `monitor.rs` contains a work-in-progress breakpoint controller
 - `syntax_highlighting.rs` contains Sigma16 syntax highlighting
 - `syntax_highlighting_runner.rs` contains syntax highlighting for the line numbers in the code editor
 - `util.rs` contains some utility functions

`src/interpreter` contains all the interpreter code
 - `interpreter.rs` contains the interpreter
 - `memory.rs` contains a struct to represent memory
 - `opcodes.rs` contains a struct and method to disassemble byte code into instructions and arguments
 - `register.rs` contains a struct to represent registers
 - `state.rs` contains a struct to represent the interpreters state
Put a brief description of your code here. This should at least describe the file structure.

## Build instructions

### Requirements

This program was developed using the following tool chain, to build the program these tool chains should be installed at their listed version:
 - [Rustup](https://www.rust-lang.org/tools/install) | Version 1.28.1
   - Rustup should install Rustc and Cargo by itself.
   - Rustc | Version 1.86.0
   - Cargo | Version 1.86.0
 - Trunk | Version 0.21.7
   - Installed via cargo by running `cargo install trunk` in a terminal

### Build steps

If these steps fail, the program is currently hosted [here](sig16.lukeweb.net).

1. Install the toolchain listed in Requirements
2. Navigate to the directory containing this file
3. Run `rustup target add wasm32-unknown-unknown`
3. Run `trunk build`
   - `trunk build` compiles the program into the `dist` directory at which point it requires a web server to serve the files.
   

### Test steps

Start the software by:

1. Install the toolchain listed in Requirements
2. Navigate to the directory containing this file
3. Run `rustup target add wasm32-unknown-unknown`
3. Run `trunk serve`
- `trunk serve` compiles the program and starts a local web server for testing.
     - Using this method you may need to hold the Ctrl key and refresh the tab to get the program to start.

