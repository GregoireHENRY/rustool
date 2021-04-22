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

use na::base::storage::Storage;
use na::{Dynamic, Matrix, Matrix3xX, RowDVector, SliceStorage, UnitVector3, Vector3};

/// Type alias for [`RowDVector`]. The matrix has 1 row and X columns.
pub type List<T> = RowDVector<T>;

/// Type alias for [`Vector3`]. The matrix has 3 rows and 1 column.
pub type Vector<T> = Vector3<T>;

/// Type alias for [`UnitVector3`]. The matrix has 3 rows and 1 column.
pub type Unit<T> = UnitVector3<T>;

/// Type alias for [`Matrix3xX`]. The matrix has 3 rows and X columns.
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
