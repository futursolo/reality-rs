use js_sys::{global, Reflect};

/// Initialise the jsdom and testing-library APIs.
///
/// You only need to call this once, calling it multiple times has no effect.
pub fn init() {
    let global = global();

    Reflect::set(&global, &"__help".into(), &2.into()).expect("failed to register global");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    async fn it_works() {
        init();

        let global = global();
        let val = Reflect::get(&global, &"__help".into()).expect("failed to get registered global");
        assert_eq!(val, JsValue::from(2));
    }
}
