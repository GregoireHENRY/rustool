/// Display the type of the variable.
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
