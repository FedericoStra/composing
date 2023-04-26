# List the available recipes
default:
	@just --list --justfile '{{justfile()}}'

# enable Rust backtraces by default
export RUST_BACKTRACE := env_var_or_default("RUST_BACKTRACE", "1")

# Test all feature combinations
test +FLAGS='-q':
	cargo hack --feature-powerset test {{FLAGS}}

# Document the same way Docs.rs would
docsrs:
	cargo +nightly rustdoc --all-features -- --cfg docsrs

# Run `cargo clippy` an all targets with all features enabled
clippy:
	cargo clippy --all-features --all-targets

# Run all the pre-release checks on a clean project
pre-release-check: && test clippy docsrs
	cargo clean
