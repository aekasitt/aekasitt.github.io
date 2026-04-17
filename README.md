# Sitt's personal blog

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/aekasitt/aekasitt.github.io/blob/master/LICENSE)
[![Top](https://img.shields.io/github/languages/top/aekasitt/aekasitt.github.io)](https://github.com/aekasitt/libri)
[![Languages](https://img.shields.io/github/languages/count/aekasitt/aekasitt.github.io)](https://github.com/aekasitt/libri)
[![Size](https://img.shields.io/github/repo-size/aekasitt/aekasitt.github.io)](https://github.com/aekasitt/libri)
[![Last commit](https://img.shields.io/github/last-commit/aekasitt/aekasitt.github.io/master)](https://github.com/aekasitt/libri)
[![Fork](https://img.shields.io/badge/fork-filipesabella/speed--reader-beige?logo=github)](https://github.com/filipsabella/speed-reader)
[![Fork](https://img.shields.io/badge/fork-lizidev/leptos--devtools-beige?logo=github)](https://github.com/lizidev/leptos-devtools)

### Technical stack

- [Rust](https://rust-lang.org)
- [Leptos](https://leptos.dev)
- [WebAssembly](https://webassembly.org)

### Rendering model

- Client-side rendering only (no server-side rendering).
- No Axum backend.
- Blog posts are generated at build time from `posts/*.md` and embedded into the WASM bundle.

### Build

- Build static site artifacts with `cargo leptos build --release`.
- Output is written to `target/site` and can be deployed as a static site.

### Contributions

Not accepting contributions

## Acknowledgments

TBD;

## License

This project is licensed under the terms of the MIT license.
