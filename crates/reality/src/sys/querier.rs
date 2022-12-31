use js_sys::Object;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    /// An interface type that implements queries.
    #[derive(Debug, Clone)]
    pub(crate) type Querier;

    #[wasm_bindgen(method, catch, js_name = "findByText")]
    pub(crate) async fn find_by_text(
        this: &Querier,
        text: &str,
        query_options: &Object,
        wait_for_options: &Object,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "findAllByText")]
    pub(crate) async fn find_all_by_text(
        this: &Querier,
        text: &str,
        query_options: &Object,
        wait_for_options: &Object,
    ) -> Result<JsValue, JsValue>;
}

impl Querier {
    // Assume a value implements the querier interface.
    pub fn from_js_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}
