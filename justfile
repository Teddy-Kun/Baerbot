alias b := build
alias r := run
alias bw := build-win

build:
    export $(cat .env | xargs) && bun tauri build --bundles deb,rpm -- -Z build-std

run:
    export $(cat .env | xargs) && bun tauri dev -- -Z build-std -- -d

build-win:
    export $(cat .env | xargs) && bun tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc -- -Z build-std

test:
    export $(cat .env | xargs) && cd src-tauri && cargo test

clean:
    rm -r build && cd src-tauri && cargo clean

lint:
    export $(cat .env | xargs) && bun lint:fix && bun check && cd src-tauri && cargo clippy
