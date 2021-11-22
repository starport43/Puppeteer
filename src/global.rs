use wasm_bindgen::JsValue;

pub(crate) const PERCENTAGE_SYMBOL: &str = "%";
pub(crate) const PIXELS_SYMBOL: &str = "px";
pub type JsValueResult<T> = core::result::Result<T, JsValue>;
