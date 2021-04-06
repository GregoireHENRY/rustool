/*!
# **rustool**

Personal toolbox for my Rust projects

## Using **rustool**

Simply add the following to your `Cargo.toml` file:

```.ignore
[dependencies]
rustool = "0.1.0"
```
*/

/// Main functionalities.
pub mod core;

pub use crate::core::*;

extern crate itertools;
extern crate log;
extern crate nalgebra as na;
extern crate num_traits;
extern crate simplelog;

use na::base::storage::Storage;
use na::{Dynamic, Matrix, SliceStorage};

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
