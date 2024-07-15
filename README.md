# A demo showing how to use component model with Wasmtime

The component model is a way to build WebAssembly modules that can be composed together to form a larger application. 
This demo will show how to use the component model to build a simple application that uses multiple WebAssembly modules compiled from various languages, such as Rust, C++, Python and JavaScript.

This demo showcases a web application.

- Rust: Web Server
- Python: String Filtering
- JavaScript: HTML Rendering

## JS

- use rollup to bundle external libs
- use babel to transpile regex 
- use marked v5.0.0
- use componentizeJS directly to disable all wasi features.

## Python

- gen bindgen 

## Rust

- Use wasi-http
- set proxy 
- download wits
- modify wasmtime crate