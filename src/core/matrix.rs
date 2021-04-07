use crate::{List, Vectors};
use itertools::multizip;
use na::base::storage::Storage;
use na::{convert, Dim, Matrix, Matrix1xX, RealField};

/// Slice whole matrix
pub fn slice<N, R, C, S1>(matrix: &Matrix<N, R, C, S1>) -> crate::Slice<'_, N, R, C, S1>
where
    N: RealField,
    R: Dim,
    C: Dim,
    S1: Storage<N, R, C>,
{
    let shape = matrix.shape();
    matrix.slice((0, 0), shape)
}

/// Number of vectors.
pub fn number_vectors<T>(vectors: &Vectors<T>) -> usize
where
    T: RealField,
{
    vectors.nrows()
}

/// Compute the required size for vector to go from start to end with step.
pub fn size_range_with_step(start: f64, end: f64, step: f64) -> usize {
    let mut size = ((end - start) / step) as usize;
    if start + size as f64 * step < end {
        size += 1;
    }
    size + 1
}

/// Create a vector from `start` to `end` with `step`. The last step might be smaller
/// than `step` just to include `end` in the vector.
pub fn linspace<T>(start: f64, end: f64, step: f64) -> Matrix1xX<T>
where
    T: RealField,
{
    let size = size_range_with_step(start, end, step);
    let mut vector = Matrix1xX::from_fn(size, |_, j| convert(start + step * j as f64));
    if vector[size - 1] > convert(end) {
        vector[size - 1] = convert(end);
    }
    vector
}

/// Dot product vectorized between two list of vectors.
pub fn dot_products<T>(vectors_1: &Vectors<T>, vectors_2: &Vectors<T>) -> List<T>
where
    T: RealField,
{
    let size = crate::number_vectors(vectors_1);
    let mut dot_products = List::<T>::zeros(size);
    for (res, vector_1, vector_2) in multizip((
        dot_products.iter_mut(),
        vectors_1.row_iter(),
        vectors_2.row_iter(),
    )) {
        *res = vector_1.dot(&vector_2);
    }
    dot_products
}

/// Clip all elements of a list between `min` and `max`. If `min` or `max` are `None`
/// there is no limit.
pub fn clip<T>(list: &List<T>, min: Option<T>, max: Option<T>) -> List<T>
where
    T: RealField,
{
    let mut work_list = list.clone();
    for element in work_list.iter_mut() {
        if let Some(mini) = min {
            if *element < mini {
                *element = mini
            };
        }
        if let Some(max) = max {
            if *element > max {
                *element = max
            };
        }
    }
    work_list
}

/// Compute the element-wise power of a list.
pub fn pows<T>(list: &List<T>, power: i32) -> List<T>
where
    T: RealField,
{
    List::from_iterator(list.len(), list.iter().map(|x| x.powi(power)))
}
