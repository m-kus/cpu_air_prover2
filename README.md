#  CPU AIR Prover II

Stone compatible CLI for Stwo.  
Just replace `cpu_air_prover` with `cpu_air_prover2` to do quick proving time benchmarks Stwo vs Stone.

## Install

Make sure you have the recent Rust nightly toolchain installed.

```sh
cargo install --git https://github.com/m-kus/cpu_air_prover2 cpu_air_prover2
```

If you face compilation issues, clone the repo and install locally instead:

```sh
cargo install --path .
```

## Use

See https://stone-packaging.pages.dev/usage/proving

> [!NOTE]  
> Stwo might not yet support all the builtins your program uses.  
> The outputs are not compatible.

## Test

Clone this repo and run `make test`
