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
