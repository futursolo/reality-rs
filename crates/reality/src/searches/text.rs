use js_sys::Object;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Element;

use crate::screen;

pub struct SearchTextOptions {
    pub(crate) subject: screen::ScreenSys,
    pub(crate) text: String,
}

impl SearchTextOptions {
    pub async fn find(&self) -> Result<Element, JsValue> {
        let val = self
            .subject
            .find_by_text(&self.text, &Object::default(), &Object::default())
            .await?;

        Ok(val.unchecked_into())
    }
}
