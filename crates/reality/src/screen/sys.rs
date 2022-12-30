//! The low-level binding to the screen type.

use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    pub(crate) type ScreenSys;

    #[wasm_bindgen(method, catch, js_name = "findByText")]
    pub(crate) async fn find_by_text(
        this: &ScreenSys,
        text: &str,
        query_options: &Object,
        wait_for_options: &Object,
    ) -> Result<JsValue, JsValue>;
}

impl ScreenSys {
    pub(crate) fn new() -> Self {
        let global = js_sys::global();
        let exports = Reflect::get(&global, &"__realityExports".into())
            .expect("failed to get registered exports");

        let screen_val = Reflect::get(&exports, &"screen".into()).expect("failed to get screen");

        screen_val.unchecked_into()
    }
}
