//! The low-level binding to the screen type.

use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::Querier;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub(crate) type Screen;
}

impl Screen {
    pub(crate) fn new() -> Self {
        let global = js_sys::global();
        let exports = Reflect::get(&global, &"__realityExports".into())
            .expect("failed to get registered exports");

        let screen_val = Reflect::get(&exports, &"screen".into()).expect("failed to get screen");

        screen_val.unchecked_into()
    }

    pub(crate) fn into_querier(self) -> Querier {
        Querier::from_js_value(self.into())
    }
}
