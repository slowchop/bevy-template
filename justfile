test:
    cargo test

deploy-local-release: build-web-release
    miniserve --index index.html out

deploy-local-debug: build-web-debug
    miniserve --index index.html out

deploy-netlify-release: build-web-release
    netlify deploy --prod -d out

deploy-netlify-debug: build-web-debug
    netlify deploy -d out

deploy-itch-release: build-web-release
    butler push out/ slowchop/{{ project-name }}:html5

deploy-itch-debug: build-web-debug
    butler push out/ slowchop/{{ project-name }}:html5

assets-web:
    cp web/* out/
    cp -r assets out

wasm-prepare:
    cargo install wasm-bindgen-cli
    rm -fr out/*

build-web-release: wasm-prepare
    cargo build --profile wasm-release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --out-dir out --target web target/wasm32-unknown-unknown/wasm-release/{{ project-name }}.wasm
    wasm-opt -Oz -s 100 -o out/new-{{ project-name }}.wasm out/{{ project-name }}_bg.wasm
    mv out/new-{{ project-name }}.wasm out/{{ project-name }}_bg.wasm
    @just assets-web

build-web-debug: wasm-prepare
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --out-dir out --target web target/wasm32-unknown-unknown/debug/{{ project-name }}.wasm
    @just assets-web
