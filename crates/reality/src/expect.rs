use std::borrow::Cow;
use std::fmt;

use crate::{Error, Result};

pub struct Expectation(Cow<'static, str>);

impl fmt::Debug for Expectation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExpectKind {
    Eq,
    Ne,
}

impl fmt::Display for ExpectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            ExpectKind::Eq => "==",
            ExpectKind::Ne => "!=",
        };

        f.write_str(op)
    }
}

fn expect_failed_inner<L, R>(
    kind: ExpectKind,
    left: &L,
    right: &R,
    args: Option<fmt::Arguments<'_>>,
) -> Result<()>
where
    L: fmt::Debug + ?Sized,
    R: fmt::Debug + ?Sized,
{
    let s = match args {
        Some(m) => format!(
            r#"expectation failed: `(left {} right)`
    left: `{:?}`,
   right: `{:?}`: {}"#,
            kind, left, right, m
        ),
        None => format!(
            r#"expectation failed: `(left {} right)`
    left: `{:?}`,
   right: `{:?}`"#,
            kind, left, right,
        ),
    };

    let e = Error::ExpectFailed(Expectation(s.into()));

    Err(e)
}

pub fn expect_eq_inner<L, R>(left: &L, right: &R, args: Option<fmt::Arguments<'_>>) -> Result<()>
where
    L: fmt::Debug + ?Sized + PartialEq<R>,
    R: fmt::Debug + ?Sized,
{
    if left != right {
        expect_failed_inner(ExpectKind::Eq, left, right, args)
    } else {
        Ok(())
    }
}

pub fn expect_ne_inner<L, R>(left: &L, right: &R, args: Option<fmt::Arguments<'_>>) -> Result<()>
where
    L: fmt::Debug + ?Sized + PartialEq<R>,
    R: fmt::Debug + ?Sized,
{
    if left == right {
        expect_failed_inner(ExpectKind::Ne, left, right, args)
    } else {
        Ok(())
    }
}

pub fn expect_inner(cond: bool, args: Option<fmt::Arguments<'_>>) -> Result<()> {
    if !cond {
        let s = match args {
            Some(m) => Cow::from(format!("expectation failed: {}", m)),
            None => "expectation failed".into(),
        };

        return Err(Error::ExpectFailed(Expectation(s)));
    }

    Ok(())
}

/// Similar to [`assert_eq!`], but uses returns a result upon failure.
#[macro_export]
macro_rules! expect_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                // The reborrows below are intentional. Without them, the stack slot for the
                // borrow is initialized even before the values are compared, leading to a
                // noticeable slow down.
                $crate::expect::expect_eq_inner(&*left_val, &*right_val, ::std::option::Option::None)
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                // The reborrows below are intentional. Without them, the stack slot for the
                // borrow is initialized even before the values are compared, leading to a
                // noticeable slow down.
                $crate::expect::expect_eq_inner(&*left_val, &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+)))
            }
        }
    };
}

/// Similar to [`assert_ne!`], but uses returns a result upon failure.
#[macro_export]
macro_rules! expect_ne {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                // The reborrows below are intentional. Without them, the stack slot for the
                // borrow is initialized even before the values are compared, leading to a
                // noticeable slow down.
                $crate::expect::expect_ne_inner(&*left_val, &*right_val, ::std::option::Option::None)
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                // The reborrows below are intentional. Without them, the stack slot for the
                // borrow is initialized even before the values are compared, leading to a
                // noticeable slow down.
                $crate::expect::expect_ne_inner(&*left_val, &*right_val, ::std::option::Option::Some(::std::format_args!($($arg)+)))
            }
        }
    };
}

/// Similar to [`assert!`], but uses returns a result upon failure.
#[macro_export]
macro_rules! expect {
    ($val:expr $(,)?) => {
        $crate::expect::expect_inner($val, ::std::option::Option::None)
    };
    ($val:expr, $($arg:tt)+) => {
        $crate::expect::expect_inner($val, ::std::option::Option::Some(::std::format_args!($($arg)+)))
    };
}
