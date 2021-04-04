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

extern crate itertools;
extern crate log;
extern crate nalgebra as na;
extern crate simplelog;

/// Main functionalities.
pub mod core;

pub use crate::core::*;
