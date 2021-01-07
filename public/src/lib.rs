pub mod ws;

use crate::ws::list::List;
use crate::ws::session::SessionTrait;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

mod prelude {
    pub type Result<T, E = JsValue> = std::result::Result<T, E>;
    pub use crate::console_log;
    use wasm_bindgen::JsValue;
    use web_sys::Window;

    pub fn window() -> Result<Window> {
        web_sys::window().ok_or_else(|| JsValue::from_str("no window"))
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, public!");
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let _ = List.start();
    Ok(())
}
