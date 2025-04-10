godot_dir := "./godot"

default := ""
build cargo-params=default:
	cargo build --manifest-path ./rust/Cargo.toml {{cargo-params}}

run: build
    godot --path {{godot_dir}}

editor: build
    godot --path {{godot_dir}} --editor

# Add GDScript formatter
fmt:
	cargo fmt --manifest-path ./rust/Cargo.toml --all
