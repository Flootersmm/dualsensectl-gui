#!/bin/bash
new_version=$1
sed -i "s/pkgver=.*/pkgver=${new_version}/" PKGBUILD
sed -i "s/pkgrel=.*/pkgrel=1/" PKGBUILD
wget -O dualsensectl-gui-${new_version}.tar.gz https://github.com/Flootersmm/dualsensectl-gui/archive/v${new_version}.tar.gz
new_checksum=$(sha256sum dualsensectl-gui-${new_version}.tar.gz | awk '{print $1}')
sed -i "s/sha256sums=.*/sha256sums=('${new_checksum}')/" PKGBUILD
makepkg -si

