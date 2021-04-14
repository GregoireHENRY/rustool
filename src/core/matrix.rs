use crate::{List, Vectors};
use na::base::storage::Storage;
use na::{convert, Dim, Matrix, Matrix1xX, RealField};

/// Get the slice of the whole matrix.
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

/// Get the number of [`Vector`][crate::Vector]s.
pub fn number_vectors<T>(vectors: &Vectors<T>) -> usize
where
    T: RealField,
{
    vectors.ncols()
}

/// Compute the required size to go from `start` to `end` with `step`, including the end point (last
/// time step can be smaller).
pub fn size_range_with_step(start: f64, end: f64, step: f64) -> usize {
    let mut size = ((end - start) / step) as usize;
    if start + size as f64 * step < end {
        size += 1;
    }
    size + 1
}

/// Create a [`List`] from `start` to `end` with `step`. The last step can be smaller to include
/// `end`.
pub fn linspace<T>(start: f64, end: f64, step: f64) -> List<T>
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

/// Clip all elements of a [`List`] between `min` and `max`. A `None` value indicates no limit.
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

/// Compute the element-wise power of a [`List`].
pub fn pows<T>(list: &List<T>, power: i32) -> List<T>
where
    T: RealField,
{
    List::from_iterator(list.len(), list.iter().map(|x| x.powi(power)))
}
