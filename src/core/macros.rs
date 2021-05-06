/// Return the name of the first None value.
#[macro_export]
macro_rules! all_some {
    ( $($value:expr),* ) => {{
        $(
            if $value.is_none() {
                Some(stringify!($value).to_string())
            } else
        )*
        { None }
    }}
}
