mod global;
pub mod ws;

use mighty::prelude::Rule;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

mod prelude {
    pub type Result<T, E = JsValue> = std::result::Result<T, E>;
    pub use crate::console_log;
    pub use crate::global::*;
    pub use js_sys::Function;
    pub use wasm_bindgen::prelude::*;
    pub use wasm_bindgen::JsValue;
    use web_sys::Window;

    pub fn window() -> Result<Window> {
        web_sys::window().ok_or_else(|| JsValue::from_str("no window"))
    }
}

#[wasm_bindgen]
pub fn new_mighty_rule() -> JsValue {
    let rule = Rule::new();
    JsValue::from_serde(&rule).unwrap()
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    Ok(())
}
