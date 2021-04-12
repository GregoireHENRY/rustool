use crate::{List, Vector, Vectors};
use itertools::multizip;
use na::{convert, RealField};

/// Magnitudes of a list of [`Vector`]s.
pub fn magnitudes<T>(vectors: &Vectors<T>) -> List<T>
where
    T: RealField,
{
    let size = crate::number_vectors(vectors);
    let mut magnitudes = List::<T>::zeros(size);
    for (res, vector) in multizip((magnitudes.iter_mut(), vectors.column_iter())) {
        *res = vector.norm();
    }
    magnitudes
}

/// Distances from of a list of [`Vector`]s to another one.
pub fn distances<T>(vectors_1: &Vectors<T>, vectors_2: &Vectors<T>) -> List<T>
where
    T: RealField,
{
    magnitudes(&(vectors_2 - vectors_1))
}

/// Distance from a [`Vector`] to another one.
pub fn distance<T>(vector_1: &Vector<T>, vector_2: &Vector<T>) -> T
where
    T: RealField,
{
    (vector_2 - vector_1).norm()
}

/// Unit vectors of a list of [`Vector`]s.
pub fn units<T>(vectors: &Vectors<T>) -> Vectors<T>
where
    T: RealField,
{
    let size = crate::number_vectors(vectors);
    let mut directions = Vectors::zeros(size);
    for (mut direction, vector) in multizip((directions.column_iter_mut(), vectors.column_iter())) {
        direction.copy_from(&vector.normalize());
    }
    directions
}

/// Directions from of a list of [`Vector`]s to another one.
pub fn directions<T>(vectors_1: &Vectors<T>, vectors_2: &Vectors<T>) -> Vectors<T>
where
    T: RealField,
{
    units(&(vectors_2 - vectors_1))
}

/// Direction from a [`Vector`] to another one.
pub fn direction<T>(vector_1: &Vector<T>, vector_2: &Vector<T>) -> Vector<T>
where
    T: RealField,
{
    (vector_2 - vector_1).normalize()
}

/// Convert a list of [`Vector`]s from cartesian to spherical coordinates.
pub fn cart_to_sph<T>(vectors: &Vectors<T>) -> Vectors<T>
where
    T: RealField,
{
    let size = crate::number_vectors(vectors);
    let mut sphericals = Vectors::zeros(size);
    for (mut spherical, cartesian) in
        multizip((sphericals.column_iter_mut(), vectors.column_iter()))
    {
        if cartesian.norm() == convert(0.) {
            spherical.copy_from(&Vector::<T>::zeros());
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
