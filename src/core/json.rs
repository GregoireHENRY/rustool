use na::{Dim, Matrix, Scalar};
use serde::Serialize;
use serde_json::{to_string, Value};

/// Write JSON string to file.
pub fn write_str<S: AsRef<str>>(path: S, string: String) {
    std::fs::write(path.as_ref(), string).unwrap();
}

/// Convert matrix to json.
pub fn matrix_to_json<N, R, C, S>(matrix: &Matrix<N, R, C, S>) -> Value
where
    N: Scalar,
    R: Dim,
    C: Dim,
    S: Serialize,
{
    serde_json::from_str(&to_string(matrix).unwrap()).unwrap()
}

/// Add **kwargs to json map.
#[macro_export]
macro_rules! addjs {
    ($map:expr, $(($key:expr, $value:expr)), *) => {{
        use serde_json::{from_str, to_string_pretty};
        $(
            let json_value = from_str(&to_string_pretty(&$value).unwrap()).unwrap();
            $map.insert($key.into(), json_value);
        )*
        $map
    }}
}

/// Create new json map from **kwargs.
#[macro_export]
macro_rules! newjs {
    ($(($key:expr, $value:expr)), *) => {{
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        $crate::addjs!(&mut map, $(($key, $value)), *);
        map
    }}
}

/// Write json map to file.
#[macro_export]
macro_rules! writejs {
    ($path:expr, $json:expr) => {
        use serde_json::to_string_pretty;
        use std::fs;
        let json_string = to_string_pretty(&$json).unwrap();
        fs::write($path, json_string).unwrap();
    };
}
