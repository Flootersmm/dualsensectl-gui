# Maintainer: Conrad Clough <conrad.clough@hotmail.com> 
pkgname=dualsensectl-gui
pkgver=v0.0.2
pkgrel=1
pkgdesc="A GTK4-based GUI for dualsensectl, built with Rust"
arch=('x86_64')
url="https://github.com/Flootersmm/dualsensectl-gui"
license=('GPL2')
depends=('gtk4' 'glib2' 'libadwaita')
makedepends=('cargo' 'rust')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855')
build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}

