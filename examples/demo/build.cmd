@cd %~dp0
@cargo watch -w "../.." -s "wasm-pack build --target web"