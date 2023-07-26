```
> cargo build --target wasm32-wasi --release
> wasmer run target/wasm32-wasi/release/wasmer_datetime_parser.wasm --invoke parse_unix_timestamp 1690385088
2023-07-26 15:24:48 UTC
args length: 0
> wasmer run target/wasm32-wasi/release/wasmer_datetime_parser.wasm --invoke 1690385088
2023-07-26 15:24:48 UTC
args length: 0
```