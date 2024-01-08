use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn rust_test() {
    assert_eq!(1, 1);
}

#[allow(clippy::eq_op)]
#[wasm_bindgen_test]
fn web_test() {
    assert_eq!(1, 1);
}

#[wasm_bindgen_test]
async fn async_test() {
    let promise = js_sys::Promise::resolve(&JsValue::from(42));
    let x = JsFuture::from(promise).await.unwrap();

    assert_eq!(x, 42);
}
