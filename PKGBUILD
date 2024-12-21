# Maintainer: Conrad Clough <conrad.clough@hotmail.com>

pkgname=dualsensectl-gui
pkgver=0.0.2
pkgrel=2
pkgdesc="A GTK4-based GUI for dualsensectl, built with Rust"
arch=('x86_64')
url="https://github.com/Flootersmm/dualsensectl-gui"
license=('GPL2')
depends=('gtk4' 'glib2' 'libadwaita')
makedepends=('cargo' 'rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Flootersmm/dualsensectl-gui/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('8e061612e1ab512aa0ac0070398c36f775d0620fc359643c815d131e3f0e854c')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo update
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname-$pkgver"

  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

  # TODO: manpage
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"

  install -Dm644 "resources/dualsensectl-gui.desktop" \
    "$pkgdir/usr/share/applications/$pkgname.desktop"

  install -Dm644 "resources/dualsensectl-gui.png" \
    "$pkgdir/usr/share/icons/hicolor/48x48/apps/$pkgname.png"
}
