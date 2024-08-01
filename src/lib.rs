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