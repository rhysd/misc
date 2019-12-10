mod utils;

use std::result::Result;
use js_sys::{Promise, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/mod.js")]
extern "C" {
    fn hello(name: &str) -> String;
    #[wasm_bindgen(js_name = fetchBytes)]
    fn fetch_bytes(url: &str) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &JsValue);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(value: &str);
}

#[wasm_bindgen]
pub fn greet() -> String {
    hello("Rust")
}

// Use fetch indirectly
#[wasm_bindgen]
pub async fn async_fetch_index_html() -> Result<JsValue, JsValue> {
    let promise = fetch_bytes("/index.html");
    let bytes = JsFuture::from(promise).await?;
    log(&bytes);
    Ok(bytes)
}

// Use fetch directly
#[wasm_bindgen]
pub async fn async_fetch_index_html_2() -> Result<(), JsValue> {
    let win = web_sys::window().unwrap();
    let promise = win.fetch_with_str("/index.html");
    let response = JsFuture::from(promise).await?;
    let response: web_sys::Response = response.dyn_into()?;
    if !response.ok() {
        return Err(response.status_text().into());
    }
    let buffer = JsFuture::from(response.array_buffer()?).await?;
    let bytes = Uint8Array::new(&buffer).to_vec();
    log_str(&format!("Bytes!: {:?}", bytes));
    Ok(())
}
