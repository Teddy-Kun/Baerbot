alias b := build
alias r := run
alias bw := build-win

build:
    bun tauri build --bundles deb,rpm -- -Z build-std

run:
    bun tauri dev -- -Z build-std

build-win:
    bun tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc -- -Z build-std

test:
    cd src-tauri && cargo test

clean:
    rm -r build && cd src-tauri && cargo clean

lint:
    bun lint:fix && bun check && cd src-tauri && cargo clippy
