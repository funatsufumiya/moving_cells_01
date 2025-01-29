study "moving_cells_01" / bevy 0.15

![screenshot.png](screenshot.png)

## Interactive Demo

https://funatsufumiya.github.io/moving_cells_01/

(Wait patiently for the first time.)

## Run (locally)

```bash
$ cargo run
```

## Build WASM

```bash
$ cargo build --release --target wasm32-unknown-unknown
$ wasm-bindgen --target web --out-dir . --no-typescript target/wasm32-unknown-unknown/release/moving_cells_01.wasm
```

## License

This artwork is dual-licensed under [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/).

( Exception: Why codes here are open on GitHub, is for readability of the code of the artwork. You can cite or copy some (or more) lines of code as you like, when you create another artwork or article from my code, without any worry about license. But this exception cannot be applied for cloning or fork. )

Copyright (C) 2025 Fumiya Funatsu
