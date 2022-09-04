#[macro_export]
/// Print a statement if the first parameter is true.
macro_rules! vprintln {
    ($cond:expr, $str:literal) => {
        if $cond { println!("{}", $str); }
    };
    ($cond:expr, $str:literal, $($args:expr),*) => {
        if $cond { println!($str, $($args),*); }
    };
}
