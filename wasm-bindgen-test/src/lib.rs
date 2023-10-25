use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str); // binds to js function

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn table(v: Vec<i32>);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn pass_params(v: Vec<i32>) {
    log("console.table(v: Vec<i32>)");
    table(v);
}

#[wasm_bindgen]
pub fn return_value_arraytype(mut v: Vec<i32>) -> Array {
    v[2] = 101;
    v.into_iter().map(JsValue::from).collect()
}

