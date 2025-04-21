# Maintainer: Fabio Montefuscolo <fabio.montefuscolo@gmail.com>
pkgname=rtpl
pkgver=0.1.0
pkgrel=1
pkgdesc="A command-line tool for rendering Jinja2 templates with data from various sources"
arch=('x86_64')
url="https://github.com/fabiomontefuscolo/rtpl"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
source=()

prepare() {
  cd "${startdir}"
  cargo fetch --locked
}

build() {
  cd "${startdir}"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  cd "${startdir}"
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --all-features
}

package() {
  cd "${startdir}"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

