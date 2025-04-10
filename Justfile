godot_dir := "./godot"
rust_manifest := "./rust/Cargo.toml"

default := ""
build cargo-params=default:
	cargo build --manifest-path {{rust_manifest}} {{cargo-params}}

clippy:
	cargo clippy --manifest-path {{rust_manifest}}

run: build
    godot --path {{godot_dir}}

editor: build
    godot --path {{godot_dir}} --editor

# Add GDScript formatter
fmt:
	cargo fmt --manifest-path {{rust_manifest}} --all
