prepare:
	cargo fetch --locked

build: prepare
	cargo build --release

pkg-archlinux:
	PKGDEST=${PWD}/target/archlinux makepkg -fs
