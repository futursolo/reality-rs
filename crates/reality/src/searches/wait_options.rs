use std::time::Duration;

use js_sys::Reflect;

pub trait WaitOptions: WaitableSealed {
    /// Sets the timeout.
    ///
    /// Default: 1 second
    fn timeout(&self, dur: Duration) -> &Self {
        Reflect::set(
            self.wait_for_options(),
            &"timeout".into(),
            &dur.as_millis().into(),
        )
        .expect("failed to set timeout");

        self
    }

    /// Sets the search interval.
    ///
    /// Default: 50 milliseconds
    fn interval(&self, int: Duration) -> &Self {
        Reflect::set(
            self.wait_for_options(),
            &"interval".into(),
            &int.as_millis().into(),
        )
        .expect("failed to set interval");

        self
    }
}

mod sealed {
    use js_sys::Object;

    use crate::SearchTextOptions;

    pub trait WaitableSealed {
        fn wait_for_options(&self) -> &Object;
    }

    impl WaitableSealed for SearchTextOptions {
        fn wait_for_options(&self) -> &Object {
            self.wait_for_options()
        }
    }
}

use sealed::WaitableSealed;
