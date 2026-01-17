alias b := build
alias r := run
alias bw := build-win

build:
    export $(cat .env | xargs) && bun tauri build --bundles deb,rpm

run:
    export $(cat .env | xargs) && bun tauri dev -- -- -d

build-win:
    export $(cat .env | xargs) && bun tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc

test:
    export $(cat .env | xargs) && cd src-tauri && cargo test

clean:
    rm -r build node_modules && cd src-tauri && cargo clean

lint:
    export $(cat .env | xargs) && bun lint:fix && bun check && cd src-tauri && cargo clippy
