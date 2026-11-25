# Maintainer: nekodajun <ALBAlinux-project@://github.com>
pkgname=aurore-at-a-glance
pkgver=0.1.0
pkgrel=1
pkgdesc="Pixel-inspired At a Glance desktop widget for Aurore (Norrsken Linux)"
arch=('x86_64')
url="https://github.com"
license=('GPL-3.0-or-later')
depends=('gtk3' 'libxkbcommon' 'wayland')
makedepends=('cargo' 'rust')
source=()
sha256sums=()

build() {
    cd "$startdir"
    cargo build --release --locked
}

package() {
    cd "$startdir"
    install -Dm755 "target/release/aurore-at-a-glance" "$pkgdir/usr/bin/aurore-at-a-glance"
    install -Dm644 "aurore-at-a-glance.desktop" "$pkgdir/usr/share/applications/aurore-at-a-glance.desktop"
}
