use js_sys::eval;

use js_sys::{global, Reflect};

mod screen;
mod searches;

pub use searches::*;

pub use screen::{screen, Screen};

static EMBEDDED: &str = include_str!("exports.cjs");

/// Initialise the jsdom and testing-library APIs.
///
/// You only need to call this once, calling it multiple times has no effect.
pub fn init() {
    let global = global();
    let exports =
        Reflect::get(&global, &"__realityExports".into()).expect("failed to get registered global");

    // Don't initial twice.
    if !exports.is_undefined() {
        return;
    }

    eval(EMBEDDED).expect("failed to initialise environment");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo::utils::body;
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    async fn it_works() {
        init();
        body().set_inner_html("<div>test</div>");

        let screen = screen();

        let el = screen
            .search_text("test")
            .find()
            .await
            .expect("failed to find element!");

        assert_eq!(el.text_content().as_deref(), Some("test"));
    }
}
