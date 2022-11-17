# A demo showing how to use component model with Wasmtime

This repository contains the code demonstrating how to use component model with wasmtime. Also, it's the source code for this [blog post]("").

## prerequisites

You need to install the following libraries:

- [Rust]()

Don't forget to install `wasm32-unknown-unknown` target for rust.

## Running this demo 

```bash 
# build wasm component in 'app'
cargo b --release --target wasm32-unknown-unknown -p app

# load the component in 'runner'
cargo r -p runner

```

## Details

For more details, please read the comments in the source code or [this blog post]().
