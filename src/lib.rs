/// Matches an error against an enum-like error type by hiding the usage of downcast_ref method
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
