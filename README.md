# A demo showing how to use component model with Wasmtime

This repository contains the code demonstrating how to use component model with wasmtime. Also, it's the source code for this [blog post](https://blog.mediosz.club/2022/11/17/how-to-use-wit-bindgen/).

## prerequisites

You need to install the following libraries:

- [Rust](https://rustup.rs/)

Don't forget to install `wasm32-unknown-unknown` target for rust.

## Running this demo 

```bash 
# build wasm component in 'app'
cargo b --release --target wasm32-unknown-unknown -p app

# load the component in 'runner'
cargo r -p runner
# note that `./target/component.wasm` was saved in runner.

```

## Python host

If you want to embed wasm component with Python host, you can follow the following steps.

```bash 
# Install wasmtime for python, which contains bindgen
pip3 install wasmtime

# Generate python bindings
python -m wasmtime.bindgen ./target/component.wasm --out-dir python/markdown/

# Run! 
cd python 
python main.py
```

## Details

For more details, please read the comments in the source code or [this blog post](https://blog.mediosz.club/2022/11/17/how-to-use-wit-bindgen/).
