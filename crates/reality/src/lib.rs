use js_sys::eval;

use js_sys::{global, Reflect};

mod error;
mod screen;
mod searches;
mod sys;

pub use error::{Error, Result};
pub use screen::{screen, Screen};
pub use searches::*;

static EMBEDDED: &str = include_str!("exports.cjs");

/// Initialise the jsdom and testing-library APIs.
///
/// You only need to call this once, calling it multiple times has no effect.
///
/// This function is called automatically by [`screen()`].
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

pub mod prelude {
    //! reality preludes.

    pub use crate::searches::{MatchOptions, WaitOptions};
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo::utils::body;
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    async fn it_works() -> Result<()> {
        let screen = screen();

        body().set_inner_html("<div>test</div>");

        let el = screen.search_text("test").find().await?;

        assert_eq!(el.text_content().as_deref(), Some("test"));

        Ok(())
    }
}
