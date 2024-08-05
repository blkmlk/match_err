//! # match_err
//!
//! Macro for quick matching and asserting errors against enum-like error types
//!
//! Helps to avoid writing long and tedious structures like:
//! ```rust
//! if let Err(e) = err {
//!     if let Some(e) = e.downcast_ref::<Error>() {
//!         match e {
//!             ...
//!         }
//!     }
//! }
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use match_err::*;
//! use anyhow::anyhow;
//!
//! #[derive(thiserror::Error, Debug)]
//! enum Error {
//!     #[error("not found")]
//!     NotFound,
//!     #[error("custom: {0}")]
//!     Custom(String),
//! }
//!
//! let err: Result<(), _> = Err(anyhow!(Error::NotFound));
//!
//! match_if_err!(err, Error, {
//!     NotFound => println!("not found"),
//!     Custom(msg) => println!("custom message: {}", msg),
//!     _ => println!("unknown")
//! })
//! ```


/// Matches the error against an enum-like error type by hiding the usage of downcast_ref method
///
/// # Examples
/// ```
///  use match_err::*;
///  use anyhow::anyhow;
///
///  #[derive(thiserror::Error, Debug)]
///  enum Error {
///     #[error("not found")]
///     NotFound,
///     #[error("custom: {0}")]
///     Custom(String),
///  }
///
///  let err = anyhow!(Error::NotFound);
///
///  match_err!(err, Error, {
///     NotFound => assert!(true),
///     Custom(msg) => assert!(false),
///     _ => assert!(false)
///  })
/// ```
#[macro_export]
macro_rules! match_err {
    ( $any:expr, $ty:ident, { $( $variant:ident $( ( $($inner:ident),* ) )? => $arm:expr ),*, _ => $default:expr } ) => (
        if let Some(e) = $any.downcast_ref::<$ty>() {
            match e {
                $(
                    $ty::$variant $( ( $(ref $inner),* ) )? => $arm,
                )*
                _ => $default
            }
        } else {
            $default
        }
    );

    ( $any:expr, $ty:ident, { $( $variant:ident $( ( $($inner:ident),* ) )? => $arm:expr ),* $(,)? }) => (
        match_err!($any, $ty, { $( $variant $( ( $($inner),* ) )? => $arm ),*, _ => {} })
    );
}

/// Checks if it's an error and matches the error against an enum-like error type by hiding the usage of downcast_ref method
///
/// # Examples
/// ```
///  use match_err::*;
///  use anyhow::anyhow;
///
///  #[derive(thiserror::Error, Debug)]
///  enum Error {
///     #[error("not found")]
///     NotFound,
///     #[error("custom: {0}")]
///     Custom(String),
///  }
///
///  let err: Result<(), _> = Err(anyhow!(Error::NotFound));
///
///  match_if_err!(err, Error, {
///     NotFound => assert!(true),
///     Custom(msg) => assert!(false),
///     _ => assert!(false)
///  })
/// ```
#[macro_export]
macro_rules! match_if_err {
    ( $any:expr, $ty:ident, { $( $variant:ident $( ( $($inner:ident),* ) )? => $arm:expr ),*, _ => $default:expr } ) => (
        if let Err(e) = $any {
            match_err!(e, $ty, { $( $variant $( ( $($inner),* ) )? => $arm ),*, _ => $default })
        } else {
            $default
        }
    );

    ( $any:expr, $ty:ident, { $( $variant:ident $( ( $($inner:ident),* ) )? => $arm:expr ),* $(,)? }) => (
        match_if_err!($any, $ty, { $( $variant $( ( $($inner),* ) )? => $arm ),*, _ => {} })
    );
}

/// Asserts the variable is an error and then asserts it against an enum-like error type by hiding the usage of downcast_ref method
/// The error is required to implement PartialEq
///
/// # Examples
/// ```
///  use match_err::*;
///  use anyhow::anyhow;
///
///  #[derive(thiserror::Error, Debug, PartialEq)]
///  enum Error {
///     #[error("not found")]
///     NotFound,
///     #[error("custom: {0}")]
///     Custom(String),
///  }
///
///  let err: Result<(), _> = Err(anyhow!(Error::Custom(String::from("internal"))));
///
///  assert_if_error!(err, Error, Custom(String::from("internal")), "invalid");
/// ```
#[macro_export]
macro_rules! assert_if_error {
    ($var:expr, $ty:ty, $variant:ident $( ( $inner:expr ) )?  $(, $($arg:tt)+)? ) => (
        if let Err(err) = $var {
            assert_error!(err, $ty, $variant $( ( $inner ) )?  $(, $($arg)+)? );
        } else {
            assert!(false, "not an error")
        }
    )
}

/// Asserts the error against an enum-like error type by hiding the usage of downcast_ref method
/// The error is required to implement PartialEq
///
/// # Examples
/// ```
///  use match_err::*;
///  use anyhow::anyhow;
///
///  #[derive(thiserror::Error, Debug, PartialEq)]
///  enum Error {
///     #[error("not found")]
///     NotFound,
///     #[error("custom: {0}")]
///     Custom(String),
///  }
///
///  let err = anyhow!(Error::Custom(String::from("internal")));
///
///  assert_error!(err, Error, Custom(String::from("internal")));
/// ```
#[macro_export]
macro_rules! assert_error {
    ($var:expr, $ty:ty, $variant:ident $( ( $inner:expr ) )? $(, $($arg:tt)+)? ) => (
        match $var.downcast_ref::<$ty>() {
            Some(e) if e == &<$ty>::$variant $( ( $inner ) )? => assert!(true),
            _ => assert!(false $(, $($arg)+)? ),
        }
    )
}
