set dotenv-load

install_build_tools:
  cargo install basic-http-server
  cargo install wasm-bindgen-cli


clean:
  rm -rf ./target

build:
  cargo build --release

build_wasm:
  cargo build  --profile wasm-release --target wasm32-unknown-unknown --lib --target-dir target

wasm_bindgen:
  wasm-bindgen --out-dir wasm --out-name app --target web --no-typescript target/wasm32-unknown-unknown/wasm-release/app.wasm

# inject_custom_fetch:
#   sd -s 'input = fetch(input)' 'input = tFetch(input)' ./wasm/app.js

optimize_wasm:
  wasm-opt -Oz -o ./wasm/app_bg.wasm ./wasm/app_bg.wasm
  wasm-opt -O3 -o ./wasm/app_bg.wasm ./wasm/app_bg.wasm 

gen_wasm:
  just install_build_tools
  just build_wasm
  just wasm_bindgen
  just optimize_wasm
  # just inject_custom_fetch


serve path="./":
  just gen_wasm
  basic-http-server -x {{path}}

run:
  cargo run --release --config env.SERVER_INIT_ADDRESS='"$SERVER_INIT_ADDRESS"'

styles:
  pnpm dlx tailwindcss -i input.css -o assets/output.css --watch
