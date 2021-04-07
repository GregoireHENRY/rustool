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
        *res = vector.norm();
    }
    magnitudes
}

/// Distances from of a list of vectors 3xX to another one.
pub fn distances<T>(vectors_1: &Matrix3xX<T>, vectors_2: &Matrix3xX<T>) -> Matrix1xX<T>
where
    T: RealField,
{
    magnitudes(&(vectors_2 - vectors_1))
}

/// Distance from a vector 3x1 to another one.
pub fn distance<T>(vector_1: &Matrix3x1<T>, vector_2: &Matrix3x1<T>) -> T
where
    T: RealField,
{
    (vector_2 - vector_1).norm()
}

/// Unit vectors of a list of vectors 3xX.
pub fn units<T>(vectors: &Matrix3xX<T>) -> Matrix3xX<T>
where
    T: RealField,
{
    let number_vectors = vectors.ncols();
    let mut directions = Matrix3xX::zeros(number_vectors);
    for (mut direction, vector) in multizip((directions.column_iter_mut(), vectors.column_iter())) {
        direction.copy_from(&vector.normalize());
    }
    directions
}

/// Directions from of a list of vectors 3xX to another one.
pub fn directions<T>(vectors_1: &Matrix3xX<T>, vectors_2: &Matrix3xX<T>) -> Matrix3xX<T>
where
    T: RealField,
{
    units(&(vectors_2 - vectors_1))
}

/// Direction from a vector 3x1 to another one.
pub fn direction<T>(vector_1: &Matrix3x1<T>, vector_2: &Matrix3x1<T>) -> Matrix3x1<T>
where
    T: RealField,
{
    (vector_2 - vector_1).normalize()
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
