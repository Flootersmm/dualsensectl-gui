# Maintainer: Conrad Clough <conrad.clough@hotmail.com> 
pkgname=dualsensectl-gui
pkgver=0.0.1
pkgrel=1
pkgdesc="A GTK4-based GUI for dualsensectl, built with Rust"
arch=('x86_64')
url="https://github.com/Flootersmm/dualsensectl-gui"
license=('GPL2')
depends=('gtk4' 'glib2' 'libadwaita')
makedepends=('cargo' 'rust')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('ede2059f8c6b606b5fe2ca73cff7ffb10b08721709f550912b726ab7a05a7095')
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

