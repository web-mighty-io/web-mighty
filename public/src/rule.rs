use crate::prelude::*;
use mighty::prelude::Rule;

#[wasm_bindgen]
pub fn new_mighty_rule() -> JsValue {
    let rule = Rule::new();
    JsValue::from_serde(&rule).unwrap()
}
