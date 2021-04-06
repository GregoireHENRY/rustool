use itertools::multizip;
use na::base::storage::Storage;
use na::{
    convert, Dim, Matrix, Matrix1xX, Matrix3x1, Matrix3xX, MatrixSlice3x1, RealField, Scalar,
};

/// Slice whole matrix
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
pub fn dot_products<T>(vector_list_1: &Matrix3xX<T>, vector_list_2: &Matrix3xX<T>) -> Matrix1xX<T>
where
    T: RealField,
{
    let mut dot_products = Matrix1xX::<T>::zeros(vector_list_1.ncols());
    for (res, vector_1, vector_2) in multizip((
        dot_products.iter_mut(),
        vector_list_1.column_iter(),
        vector_list_2.column_iter(),
    )) {
        *res = dot_product_slice(vector_1, vector_2);
    }
    dot_products
}

/// Dot product between two vectors.
pub fn dot_product<T>(vector_1: &Matrix3x1<T>, vector_2: &Matrix3x1<T>) -> T
where
    T: RealField,
{
    dot_product_slice(vector_1.column(0), vector_2.column(0))
}

/// Dot product between two slices.
pub fn dot_product_slice<T>(vector_1: MatrixSlice3x1<T>, vector_2: MatrixSlice3x1<T>) -> T
where
    T: RealField,
{
    slice(&vector_1).dot(&slice(&vector_2))
}
