# rust-foundations

[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://app.codspeed.io/bakate/rust-foundations?utm_source=badge)

Learning Rust in public, coming from 5+ years of full-stack TypeScript.

Each binary in `src/bin/` explores one core concept:

| Binary | Topic |
|---|---|
| `ownership` | Ownership, borrowing and moves |
| `collections` | `Vec`, `String` and slices |
| `hash-map` | `HashMap` and the entry API |
| `iterators` | Iterator adapters and closures |
| `progress` | A small CLI with typed errors (`Result`, error enums, `let-else`) on top of the library in `src/lib.rs` |

## Run

```sh
cargo test                          # unit tests
cargo run --bin progress -- 5 8     # Progress: 62%
cargo bench                         # local benchmarks (divan)
```

## Benchmarks

Benchmarks live in `benches/` and use [divan](https://github.com/nvzqz/divan) through the CodSpeed compatibility layer. They run in CI on every push to `main` and on pull requests via [CodSpeed](https://codspeed.io), so each pull request shows whether a change made the code faster or slower.
