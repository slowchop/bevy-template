test:
    cargo test

deploy-netlify: test build-web assets-web
    netlify deploy --prod -d out

deploy-netlify-debug: test build-web-debug assets-web
    netlify deploy -d out

deploy-itch: test build-web assets-web
    butler push out/ slowchop/{{ project-name }}:html5

deploy-itch-debug: test build-web-debug assets-web
    butler push out/ slowchop/{{ project-name }}:html5

assets-web:
    cp web/* out/
    cp -r assets out

wasm-tools-and-fixes:
    cargo install wasm-bindgen-cli
    rm -fr out/*
#    cargo update --package async-executor --precise 1.6.0

build-web: wasm-tools-and-fixes
    cargo build --profile wasm-release --target wasm32-unknown-unknown
    wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/wasm-release/{{ project-name }}.wasm
    wasm-opt -Oz -s 100 -o out/new-{{ project-name }}.wasm out/{{ project-name }}_bg.wasm
    mv out/new-{{ project-name }}.wasm out/{{ project-name }}_bg.wasm

build-web-debug: wasm-tools-and-fixes
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/debug/{{ project-name }}.wasm
