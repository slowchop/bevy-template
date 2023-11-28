
deploy-web: build-web assets-web netlify-deploy

deploy-web-debug: build-web-debug assets-web netlify-deploy-debug

assets-web:
    cp web/* out/
    cp -r assets out

wasm-tools-and-fixes:
    cargo update --package async-executor --precise 1.6.0
    cargo install wasm-bindgen-cli
    rm -fr out/*

build-web: wasm-tools-and-fixes
    cargo build --profile wasm-release --target wasm32-unknown-unknown
    wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/wasm-release/{{ project-name }}.wasm
    wasm-opt -Oz -s 100 -o out/new-{{ project-name }}.wasm out/{{ project-name }}_bg.wasm
    mv out/new-{{ project-name }}.wasm out/{{ project-name }}_bg.wasm

build-web-debug: wasm-tools-and-fixes
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/debug/{{ project-name }}.wasm

netlify-deploy:
    netlify deploy --prod -d out

netlify-deploy-debug:
    netlify deploy -d out
