/*!
# **rustool**

Personal toolbox for my Rust projects

## Using **rustool**

Simply add the following to your `Cargo.toml` file:

```.ignore
[dependencies]
rustool = "0.3.5"
```
*/

/// Main functionalities.
pub mod core;

pub use crate::core::*;

extern crate alga;
extern crate itertools;
extern crate log;
extern crate nalgebra as na;
extern crate num_traits;
extern crate simplelog;

use alga::general::RingCommutative;
use na::base::storage::Storage;
use na::{Dynamic, Matrix, Matrix3xX, RowDVector, Scalar, SliceStorage, Vector3};
use num_traits::{NumCast, ToPrimitive};
use std::cmp::PartialOrd;

/// Type alias for [`DVector`]. The matrix has 1 row and X columns.
pub type List<T> = RowDVector<T>;

/// Type alias for [`Vector3`]. The matrix has 3 rows and 1 column.
pub type Vector<T> = Vector3<T>;

/// Type alias for [`MatrixXx3`]. The matrix has 3 rows and X columns.
pub type Vectors<T> = Matrix3xX<T>;

/// Type alias to define a slice of the size of its matrix.
pub type Slice<'a, N, R, C, S1> = Matrix<
    N,
    Dynamic,
    Dynamic,
    SliceStorage<
        'a,
        N,
        Dynamic,
        Dynamic,
        <S1 as Storage<N, R, C>>::RStride,
        <S1 as Storage<N, R, C>>::CStride,
    >,
>;

/// Trait that extends [`Scalar`] to create integer matrices like float matrices. It aims to be the
/// equivalent of [`RealField`][nalgebra::RealField] but for integer.
pub trait SuperScalar:
    Scalar
    + RingCommutative
    + PartialOrd
    + std::ops::Div
    + ToPrimitive
    + NumCast
    + Copy
    + serde::ser::Serialize
{
}

impl<T> SuperScalar for T where
    T: Scalar
        + RingCommutative
        + PartialOrd
        + std::ops::Div
        + ToPrimitive
        + NumCast
        + Copy
        + serde::ser::Serialize
{
}
