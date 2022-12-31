use std::result;

use thiserror::Error;
use wasm_bindgen::JsValue;

use crate::expect::Expectation;

/// The error type returned by queries.
#[derive(Error, Debug)]
pub enum Error {
    #[error("JavaScript Error.\n{:?}", .0)]
    JavaScript(JsValue),

    #[error("{:?}", .0)]
    ExpectFailed(Expectation),
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JavaScript(value)
    }
}

/// Type result type returned by queries.
pub type Result<T> = result::Result<T, Error>;
