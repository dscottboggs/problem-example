pkgname=problem-example
pkgver=0.1.0
pkgrel=1
makedepends=(rust cargo)
depends=(xdo)
arch=(i686 x86_64 armv6h armv7h)

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir" --path "$(dirname "$srcdir")"
    rm "$pkgdir/.crates.toml" "$pkgdir/.crates2.json"
}