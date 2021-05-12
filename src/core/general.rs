/// Display the type of the variable.
pub fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}
