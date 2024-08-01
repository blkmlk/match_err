/// Matches an error against an enum-like error type by hiding the usage of downcast_ref method
///
/// # Examples
/// ```
///  use match_err::match_err;
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
///  match_err!(err, Error, {
///     NotFound => println!("not found"),
///     Custom(msg) => println!("custom message: {}", msg),
///     _ => println!("unknown")
///  })
/// ```
#[macro_export]
macro_rules! match_err {
    ( $any:expr, $ty:ident, { $( $variant:ident $( ( $($inner:ident),* ) )? => $arm:expr ),*, _ => $default:expr } ) => (
        if let Err(e) = $any {
            if let Some(e) = e.downcast_ref::<$ty>() {
                match e {
                    $(
                        $ty::$variant $( ( $(ref $inner),* ) )? => $arm,
                    )*
                    _ => $default
                }
            } else {
                $default
            }
        } else {
            $default
        }
    );

    ( $any:expr, $ty:ident, { $( $variant:ident $( ( $($inner:ident),* ) )? => $arm:expr ),* $(,)? }) => (
        match_err!($any, $ty, { $( $variant $( ( $($inner),* ) )? => $arm ),*, _ => {} })
    );
}