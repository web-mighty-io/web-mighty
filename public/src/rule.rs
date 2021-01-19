use crate::prelude::*;
use mighty::prelude::*;

#[wasm_bindgen]
pub fn new_rule() {
    RULE.with(|rule| rule.replace(Rule::new()));
}
