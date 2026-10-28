# Android Rust Gallery

A fast photo and video gallery for Android, written in Rust.

Uses little memory, starts fast, asks for as few permissions as it can, and keeps everything on the
phone. See [PRIVACY.md](PRIVACY.md).

## Features

- Photos and videos in a date-grouped grid, and albums
- A hidden album locked with your fingerprint or screen lock
- Trash with 30 day restore, share, delete and move
- Video player with speed, loop, mute, rotate, a floating window and double-tap skipping on the screen edges
- Pinch zoom, EXIF details and a storage cleanup view

## Install

Download the APK from the releases page and follow [docs/INSTALL.md](docs/INSTALL.md).

## Build

Needs Rust with the `aarch64-linux-android` target, `cargo-ndk`, the Android SDK (build-tools 34,
platform 34), NDK r27c and a JDK 17. Then, from `apps/gallery` (Windows PowerShell):

```
cargo test
powershell -File build-apk.ps1
```

APKs are signed with your own key, see [docs/SIGNING.md](docs/SIGNING.md).

## Layout

- `apps/gallery/` the app
- `tools/build/` the shared APK build script

## More apps

- [Android Rust Files](https://github.com/HusniHeartnet/android-rust-files)
- [Android Rust Notes](https://github.com/HusniHeartnet/android-rust-notes)
- [Android Rust Home](https://github.com/HusniHeartnet/android-rust-home)

## Licence

Copyright (C) heartnetapps. Licensed under the GNU General Public License, version 3 or (at your
option) any later version. See [LICENSE](LICENSE). The names and logos are not covered by the licence.
