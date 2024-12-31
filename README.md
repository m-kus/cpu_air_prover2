#  CPU AIR Prover II

Drop-in replacement for `cpu_air_prover` that allows to do simple (rough) benchmarks of Stwo vs Stone for existing proving pipelines.

## Install

Make sure you have the recent Rust nightly toolchain installed.

```sh
cargo install --git https://github.com/m-kus/cpu_air_prover2 cpu_air_prover2
```

## Use

See https://stone-packaging.pages.dev/usage/proving

> [!NOTE]  
> Stwo currently does not support builtins

## Test

Clone this repo and run `make test`
