/*!
# **rustool**

Personal toolbox for my Rust projects

## Using **rustool**

Simply add the following to your `Cargo.toml` file:

```.ignore
[dependencies]
rustool = "*" // replace * by the latest version of the crate
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

use na::{
    Dynamic, Matrix, Matrix3xX, MatrixSlice3xX, RowDVector, SliceStorage, UnitVector3, Vector3,
    VectorSlice3, U1,
};

/// Type alias for [`RowDVector`]. The matrix has 1 row and X columns.
pub type List<T> = RowDVector<T>;

/// The matrix does not own the data and has 1 row and X columns. See [`List`] for list of owned
/// data.
pub type ListSlice<'a, T> = Matrix<T, U1, Dynamic, SliceStorage<'a, T, U1, Dynamic, U1, Dynamic>>;

/// Type alias for [`Vector3`]. The matrix has 3 rows and 1 column.
pub type Vector<T> = Vector3<T>;

/// Type alias for [`VectorSlice3`]. The matrix does not own the data and has 3 rows and 1 column.
/// See [`Vector`] for owned vector data.
pub type VectorSlice<'a, T> = VectorSlice3<'a, T>;

/// Type alias for [`UnitVector3`]. The matrix has 3 rows and 1 column.
pub type Unit<T> = UnitVector3<T>;

/// Type alias for [`Matrix3xX`]. The matrix has 3 rows and X columns.
pub type Vectors<T> = Matrix3xX<T>;

/// Type alias for [`MatrixSlice3xX`]. The matrix does not own the data and has 3 rows and X column.
/// See [`Vectors`] for list of owned vector data.
pub type VectorsSlice<'a, T> = MatrixSlice3xX<'a, T>;
