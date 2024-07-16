# A demo showing how to use component model with Wasmtime

The component model is a way to build WebAssembly modules that can be composed together to form a larger application. 
This demo will show how to use the component model to build a simple application that uses multiple WebAssembly modules compiled from various languages, such as Rust, C++, Python and JavaScript.

This demo showcases a web application.

- Rust: Web Server using wasi-http
- Python: String Filtering
- JavaScript: HTML Rendering

## JS

For JavaScript, we mainly use the ComponentizeJS to build the WebAssembly module. Note that we will not use `jco` cause we want to disable all wasi features, which can not be done by `jco` for now.

We will use `marked` to render the markdown content. There are two problems we need to handle:
1. We need to bundle the `marked` module into one JS file.
2. We need to transpile the regex in `marked` module, and we need `marked(v5.0.0)`.

## Python

For Python, we just need `componentize-py`.

## Rust

It is relatively easy to build a WebAssembly module using Rust. We will use `wasi-http` to build a simple web server.

First we need to download all `wits` from Wasmtime repository. Then we need to modify the `wasmtime` crate to support a large wasm file, specifically, we need to modify the `wasmtime` and set `max_core_instances_per_component` to 50.
