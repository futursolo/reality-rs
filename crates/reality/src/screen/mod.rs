use crate::SearchTextOptions;

use crate::init;
use crate::sys;

#[derive(Debug, Clone)]
pub struct Screen {
    inner: sys::Screen,
}

impl Screen {
    pub fn search_text<S>(&self, text: S) -> SearchTextOptions
    where
        S: Into<String>,
    {
        SearchTextOptions::new(self.inner.clone().into_querier(), text.into())
    }
}

/// Retrieves the global screen instance.
pub fn screen() -> Screen {
    init();

    Screen {
        inner: sys::Screen::new(),
    }
}
