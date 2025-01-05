#  CPU AIR Prover II

> [!WARNING]
> Use [`adapted_stwo`](https://github.com/starkware-libs/stwo-cairo/blob/main/stwo_cairo_prover/crates/adapted_prover) instead
>
> ```sh
> cargo install --git https://github.com/starkware-libs/stwo-cairo adapted_stwo
> ```

Stone compatible CLI for Stwo.  
Just replace `cpu_air_prover` with `cpu_air_prover2` to do quick proving time benchmarks Stwo vs Stone.

## Install

Make sure you have the recent Rust nightly toolchain installed.

```sh
cargo install --git https://github.com/m-kus/cpu_air_prover2 cpu_air_prover2
```

> [!NOTE]  
> This fork can work slower for small inputs because of the extra preprocessed columns.  
> If you want to check againts the untampered version, use an earlier revision:  
> `cargo install --git https://github.com/m-kus/cpu_air_prover2 --rev 1cfa34a11aacf6e23ff6b01d3bbdea170086950c`

## Use

See https://stone-packaging.pages.dev/usage/proving

> [!NOTE]  
> Stwo might not yet support all the builtins your program uses.  
> The outputs are not compatible.

## Test

Clone this repo and run `make test`
