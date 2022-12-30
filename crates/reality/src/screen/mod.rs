use crate::SearchTextOptions;

mod sys;

pub(crate) use sys::ScreenSys;

pub struct Screen {
    inner: sys::ScreenSys,
}

impl Screen {
    pub fn search_text<S>(&self, text: S) -> SearchTextOptions
    where
        S: Into<String>,
    {
        SearchTextOptions {
            subject: self.inner.clone(),
            text: text.into(),
        }
    }
}

pub fn screen() -> Screen {
    Screen {
        inner: sys::ScreenSys::new(),
    }
}
