// generate bindings.
wit_bindgen_host_wasmtime_rust::generate!("../wits/markdown.wit");
use wit_bindgen_host_wasmtime_rust::anyhow::{self, Result};
use wit_bindgen_host_wasmtime_rust::wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};
// Acutally, when you run `cargo b` in the last step, you will get a wasm module,
// not a wasm component, so we need to use this ComponentEncoder to transform the
// wasm module to component.
use wit_component::ComponentEncoder;

fn main() -> Result<()> {
    let mut config = Config::new();
    // Enable component here.
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, 0);
    let linker = Linker::new(&engine);

    // we first read the bytes of the wasm module.
    let module = std::fs::read("./target/wasm32-unknown-unknown/release/app.wasm")?;
    // then we transform module to compoennt.
    // remember to get wasi_snapshot_preview1.wasm first.
    let component = ComponentEncoder::default()
        .module(module.as_slice())?
        .validate(true)
        .adapter_file(std::path::Path::new("./wits/wasi_snapshot_preview1.wasm"))?
        .encode()?;
    let component = Component::from_binary(&engine, &component)?;

    // after getting the component, we can instantiate a markdown instance.
    let (markdown, _instance) = Markdown::instantiate(&mut store, &component, &linker)?;
    let res = markdown.render(&mut store, "# hello")?;
    assert_eq!(res, "<h1>hello</h1>\n");
    Ok(())
}
