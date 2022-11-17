from wasmtime import Engine, Store, Config
from markdown import Component

if __name__ == "__main__":
    path = "/home/tric/codebase/component-model-demo/target/wasm32-unknown-unknown/release/app.wasm"
    text = "# head"

    config = Config()
    engine = Engine(config)
    store = Store(engine)
    component = Component(store)
    result = component.render(store, text)
    assert result == "<h1>head</h1>\n"
    print(result)
