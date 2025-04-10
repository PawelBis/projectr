1. Make sure that `GODOT4_BIN` env var is set to the path of the Godot 4 binary.
2. Install  the [Godot 4](https://godotengine.org/download) editor.
3. Install [rustup](https://rustup.rs/)
4. Install [just](https://github.com/casey/just)

# Code guidelines

## Rust
01. `rustfmt` and `clippy` are used for formatting and linting - use `just fmt clippy` from root directory
02. Follow rust naming conventions and code style

## GDScript
01. tool
02. class_name
03. extends
04. docstring

05. signals
06. enums
07. constants
08. exported variables
09. public variables
10. private variables
11. onready variables

12. optional built-in virtual _init method
13. built-in virtual _ready method
14. remaining built-in virtual methods
15. public methods
16. private methods
