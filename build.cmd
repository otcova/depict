@REM USE EXAMPLE: .\build.cmd examples\webgl_demo
@cd %~dp0
@cd %*
@cargo watch -w "../.." -s "wasm-pack build --target web"