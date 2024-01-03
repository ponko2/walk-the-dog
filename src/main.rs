use wasm_bindgen::JsValue;
use web_sys::console;

pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Hello, world!"));

    Ok(())
}
