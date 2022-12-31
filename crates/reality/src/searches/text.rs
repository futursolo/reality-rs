use js_sys::{Array, Object};
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::{sys, Result};

#[derive(Debug, Clone)]
pub struct SearchTextOptions {
    querier: sys::Querier,
    text: String,

    wait_for_options: Object,
    match_options: Object,
}

impl SearchTextOptions {
    pub(crate) fn new(querier: sys::Querier, text: String) -> Self {
        SearchTextOptions {
            querier,
            text,
            wait_for_options: Object::default(),
            match_options: Object::default(),
        }
    }

    pub(crate) fn wait_for_options(&self) -> &Object {
        &self.wait_for_options
    }

    pub(crate) fn match_options(&self) -> &Object {
        &self.wait_for_options
    }

    /// Find one element that matches the condition.
    pub async fn find(&self) -> Result<Element> {
        let val = self
            .querier
            .find_by_text(&self.text, &self.match_options, &self.wait_for_options)
            .await?;

        Ok(val.unchecked_into())
    }

    /// Find all elements that matches the condition.
    pub async fn find_all(&self) -> Result<Vec<Element>> {
        let val = self
            .querier
            .find_all_by_text(&self.text, &self.match_options, &self.wait_for_options)
            .await?;

        let vals: Array = val.unchecked_into();

        Ok(vals.iter().map(|m| m.unchecked_into()).collect())
    }
}
