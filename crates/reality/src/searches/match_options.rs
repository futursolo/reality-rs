use js_sys::Reflect;

pub trait MatchOptions: MatchableSealed {
    /// Sets the selector to use.
    fn selector<S>(&self, sel: S) -> &Self
    where
        S: AsRef<str>,
    {
        let sel = sel.as_ref();

        Reflect::set(self.match_options(), &"selector".into(), &sel.into())
            .expect("failed to set selector");

        self
    }

    /// Sets the elements it should ignore in a comma separated list.
    ///
    /// If you wish to disable the default value, set it to `None`.
    ///
    /// Default: `script, style`
    fn ignore<S>(&self, ign: Option<S>) -> &Self
    where
        S: AsRef<str>,
    {
        let ign = match ign {
            Some(m) => m.as_ref().into(),
            None => false.into(),
        };

        Reflect::set(self.match_options(), &"ignore".into(), &ign).expect("failed to set ignore");

        self
    }
}

mod sealed {
    use js_sys::Object;

    use crate::SearchTextOptions;

    pub trait MatchableSealed {
        fn match_options(&self) -> &Object;
    }

    impl MatchableSealed for SearchTextOptions {
        fn match_options(&self) -> &Object {
            self.match_options()
        }
    }
}

use sealed::MatchableSealed;
