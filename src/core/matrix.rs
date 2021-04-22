use crate::{List, Vectors};
use na::base::storage::Storage;
use na::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub, Dim, Matrix, Matrix1xX, RealField, Scalar};
use num_traits::{cast, NumCast};
use std::cmp::PartialOrd;

/// Get the slice of the whole matrix.
pub fn slice<N, R, C, S1>(matrix: &Matrix<N, R, C, S1>) -> crate::Slice<'_, N, R, C, S1>
where
    N: Scalar,
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
    T: Scalar,
{
    vectors.ncols()
}

/// Compute the required size to go from `start` to `end` with `step`, including the end point (last
/// time step can be smaller).
pub fn size_range_with_step<T>(start: T, end: T, step: T) -> usize
where
    T: Scalar + NumCast + Copy + ClosedAdd + ClosedSub + ClosedDiv + ClosedMul + PartialOrd,
{
    let mut size: usize = cast((end - start) / step).unwrap();
    if start + cast::<usize, T>(size).unwrap() * step < end {
        size += 1;
    }
    size + 1
}

/// Create a [`List`] from `start` to `end` with `step`. The last step can be smaller to include
/// `end`.
pub fn linspace<T>(start: T, end: T, step: T) -> List<T>
where
    T: Scalar + NumCast + ClosedSub + ClosedDiv + ClosedMul + ClosedAdd + PartialOrd + Copy,
{
    let size = size_range_with_step(start, end, step);
    let mut vector = Matrix1xX::from_fn(size, |_, j| start + step * cast(j).unwrap());
    if vector[size - 1] > end {
        vector[size - 1] = end;
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
