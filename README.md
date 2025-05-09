study artwork "moving_cells_01" / bevy 0.16

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

## Bevy Versions

| bevy | this code |
| --- | --- |
| 0.16 | 0.2 |
| 0.15 | 0.1 |

## License

This artwork is dual-licensed under [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/).

( Exception: The reason I am publishing the code here on GitHub is to make it easier to read the code of the artwork and to help other creators. It's welcome to cite or copy some (or more) lines of code as you like, when you create another new artwork or article with citing my code, without any worry about license. But this exception cannot be applied for cloning or fork. )

Copyright (C) 2025 Fumiya Funatsu
