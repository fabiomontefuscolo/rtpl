prepare:
	cargo fetch --locked

build: prepare
	cargo build --release

test: prepare
	cargo test --release

pkg-archlinux:
	PKGDEST=${PWD}/target/archlinux makepkg -fs
