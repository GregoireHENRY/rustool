extern crate nalgebra as na;

fn main() {
    let v = tool::linspace::<f64>(0.0, 2.0, 1.0);

    println!("{}", v.transpose());
}
