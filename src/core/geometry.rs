use itertools::multizip;
use na::{convert, Matrix1xX, Matrix3x1, Matrix3xX, MatrixSlice3x1, RealField};

/// Magnitudes of a list of vectors 3xX.
pub fn magnitudes<T>(vectors: &Matrix3xX<T>) -> Matrix1xX<T>
where
    T: RealField,
{
    let number_vectors = vectors.ncols();
    let mut magnitudes = Matrix1xX::<T>::zeros(number_vectors);
    for (res, vector) in multizip((magnitudes.iter_mut(), vectors.column_iter())) {
        *res = magnitude_slice(vector);
    }
    magnitudes
}

/// Magnitude of a vector 3x1.
pub fn magnitude<T>(vector: &Matrix3x1<T>) -> T
where
    T: RealField,
{
    magnitude_slice(vector.column(0))
}

/// Magnitude of a slice 3x1.
pub fn magnitude_slice<T>(vector: MatrixSlice3x1<T>) -> T
where
    T: RealField,
{
    (vector).norm()
}

/// Directions of a list of vectors 3xX.
pub fn directions<T>(vectors: &Matrix3xX<T>) -> Matrix3xX<T>
where
    T: RealField,
{
    let number_vectors = vectors.ncols();
    let mut directions = Matrix3xX::zeros(number_vectors);
    for (mut direction, vector) in multizip((directions.column_iter_mut(), vectors.column_iter())) {
        direction.copy_from(&direction_slice(vector));
    }
    directions
}

/// Direction of a vector 3x1.
pub fn direction<T>(vector: &Matrix3x1<T>) -> Matrix3x1<T>
where
    T: RealField,
{
    direction_slice(vector.column(0))
}

/// Direction of a slice 3x1.
pub fn direction_slice<T>(vector: MatrixSlice3x1<T>) -> Matrix3x1<T>
where
    T: RealField,
{
    (vector).normalize()
}

/// Component-wise cartesian to spherical conversion.
pub fn cart_to_sph<T>(matrix: &Matrix3xX<T>) -> Matrix3xX<T>
where
    T: RealField,
{
    let mut sphericals = Matrix3xX::zeros(matrix.ncols());
    for (mut spherical, cartesian) in multizip((sphericals.column_iter_mut(), matrix.column_iter()))
    {
        if cartesian.norm() == convert(0.) {
            spherical.copy_from(&Matrix3x1::<T>::zeros());
        } else {
            spherical.copy_from_slice(&[
                cartesian[1].atan2(cartesian[0]),
                (cartesian[2] / cartesian.norm()).asin(),
                cartesian.norm(),
            ]);
        }
    }
    sphericals
}
