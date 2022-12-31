use std::result;

use thiserror::Error;
use wasm_bindgen::JsValue;

/// The error type returned by queries.
#[derive(Error, Debug)]
pub enum Error {
    #[error("JavaScript Error.\n{:?}", .0)]
    JavaScript(JsValue),
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JavaScript(value)
    }
}

/// Type result type returned by queries.
pub type Result<T> = result::Result<T, Error>;
