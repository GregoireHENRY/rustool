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
///
/// ## Expression
///
/// $$\theta={\rm arctan2}\left(q_y, q_x\right)$$
/// $$\phi=\arcsin\left(\frac{q_z}{\left\Vert\bm{q}\right\Vert}\right)$$
/// $$\rho=\left\Vert\bm{q}\right\Vert$$
///
/// where $\theta$ is the azimuth, $\phi$ is the elevation, $\rho$ the radius, and $\bm{q}$ the
/// cartesian vector.
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

/// Dot product component-wise between two lists of [`Vector`]s.
pub fn dot_products<T>(vectors_1: &Vectors<T>, vectors_2: &Vectors<T>) -> List<T>
where
    T: RealField,
{
    let size = crate::number_vectors(vectors_1);
    let mut dot_products = List::<T>::zeros(size);
    for (res, vector_1, vector_2) in multizip((
        dot_products.iter_mut(),
        vectors_1.column_iter(),
        vectors_2.column_iter(),
    )) {
        *res = vector_1.dot(&vector_2);
    }
    dot_products
}

/// Project the first [`Vector`] onto the second one.
///
/// ## Expression
///
/// The projection of $\bm{u}$ onto $\bm{v}$ is defined as,
///
/// $${\rm proj}_{\bm{v}}\left(\bm{u}\right)=\frac{\bm{u}\cdot\bm{v}}{\left\Vert\bm{v}\right\Vert}\frac{\bm{v}}{\left\Vert\bm{v}\right\Vert}$$
pub fn projection_vector<T>(vector_1: &Vector<T>, vector_2: &Vector<T>) -> Vector<T>
where
    T: RealField,
{
    vector_2 * vector_1.dot(vector_2) / vector_2.norm().powi(2)
}

/// Project the first [`Vector`] onto the plane described the second [`Vector`] (normal of the plane).
///
/// ## Expression
///
/// The projection of $\bm{u}$ onto the plane described by its normal $\bm{v}$ is defined by
/// substracting the component of $\bm{u}$ that is orthogonal to the plane, from $\bm{u}$. The
/// projection is given by,
///
/// $$\mathrm{proj}\_{\mathrm{plane}\left(\bm{v}\right)}\left(\bm{u}\right)=\bm{u}-\mathrm{proj}_{\bm{v}}\left(\bm{u}\right)$$
///
/// where ${\rm proj}_{\bm{v}}\left(\bm{u}\right)$ is the [vector projection][projection_vector] of
/// $\bm{u}$ onto $\bm{v}$.
pub fn projection_plane<T>(vector_1: &Vector<T>, vector_2: &Vector<T>) -> Vector<T>
where
    T: RealField,
{
    vector_1 - projection_vector(vector_1, vector_2)
}

/// Compute the direct angle from 0 to 2 PI between two vectors and an upward vector.
///
/// ## Definition
///
/// The direct angle is defined as the angle between the two vectors, from 0 to
/// [$\tau$][crate::TAU]. It is opposed to the smallest angle between two vectors, from 0 to $\pi$.
/// The direct angle is built using the normal vector from the plane defined by the two vectors.
pub fn direct_angle<T>(v1: &Vector<T>, v2: &Vector<T>, up: &Vector<T>) -> T
where
    T: RealField,
{
    let mut ang = v1.angle(v2);
    let normal = v1.cross(v2);
    let test = up.cross(&normal)[0];
    if test < T::zero() {
        ang = T::two_pi() - ang;
    }
    ang
}
