dev-native:
    cargo run --features bevy/dynamic_linking

dev-web:
    trunk serve

release-web:
    trunk build --release

release-build-windows:
    cargo build --release --target x86_64-pc-windows-msvc

