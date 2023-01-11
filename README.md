# Problem example

some trouble I'm having with the [tray-icon](https://lib.rs/crates/tray-icon) Crate.

## What happens:

~~~console
$ makepkg -s
==> Making package: problem-example 0.1.0-1 (Wed 11 Jan 2023 08:17:52 AM EST)
==> Checking runtime dependencies...
==> Checking buildtime dependencies...
==> Retrieving sources...
==> Extracting sources...
==> Removing existing $pkgdir/ directory...
==> Starting build()...
==> Entering fakeroot environment...
==> Starting package()...
  Installing problem-example v0.1.0 (/home/scott/src/problem-example)
    Updating crates.io index
    Finished release [optimized] target(s) in 0.24s
  Installing /home/scott/src/problem-example/pkg/problem-example/bin/problem-example
   Installed package `problem-example v0.1.0 (/home/scott/src/problem-example)` (executable `problem-example`)
warning: be sure to add `/home/scott/src/problem-example/pkg/problem-example/bin` to your PATH to be able to run the installed binaries
==> Tidying install...
  -> Removing libtool files...
  -> Purging unwanted files...
  -> Removing static library files...
  -> Stripping unneeded symbols from binaries and libraries...
  -> Compressing man and info pages...
==> Checking for packaging issues...
==> Creating package "problem-example"...
  -> Generating .PKGINFO file...
  -> Generating .BUILDINFO file...
  -> Generating .MTREE file...
  -> Compressing package...
==> Leaving fakeroot environment.
==> Finished making: problem-example 0.1.0-1 (Wed 11 Jan 2023 08:17:55 AM EST)

$ pkg/problem-example/bin/problem-example 

(process:70403): Gtk-CRITICAL **: 08:18:12.185: gtk_icon_theme_get_for_screen: assertion 'GDK_IS_SCREEN (screen)' failed

(process:70403): GLib-GObject-WARNING **: 08:18:12.187: invalid (NULL) pointer instance

(process:70403): GLib-GObject-CRITICAL **: 08:18:12.187: g_signal_connect_data: assertion 'G_TYPE_CHECK_INSTANCE (instance)' failed

$
~~~
