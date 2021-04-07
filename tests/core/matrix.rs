use itertools::multizip;

#[test]
fn test_size_range_with_step() {
    let start = 0.0;
    let end = 10.0;
    let step = 2.0;
    let expected_size = 6;

    let size = tool::size_range_with_step(start, end, step);

    assert_eq!(size, expected_size);
}

#[test]
fn test_size_range_with_step_odd() {
    let start = 0.0;
    let end = 11.0;
    let step = 2.0;
    let expected_size = 7;

    let size = tool::size_range_with_step(start, end, step);

    assert_eq!(size, expected_size);
}

#[test]
fn test_linspace() {
    let start = 0.0;
    let end = 10.0;
    let step = 2.0;
    let expected_list = tool::List::from_column_slice(&[0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);

    let list = tool::linspace::<f64>(start, end, step);

    assert_eq!(list.len(), 6);

    for (value, expected_value) in multizip((list.iter(), expected_list.iter())) {
        assert!(relative_eq!(value, expected_value, epsilon = f64::EPSILON));
    }
}

#[test]
fn test_linspace_odd() {
    let start = 0.0;
    let end = 11.0;
    let step = 2.0;
    let expected_list = tool::List::from_column_slice(&[0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 11.0]);

    let list = tool::linspace(start, end, step);

    assert_eq!(list.len(), 7);

    for (value, expected_value) in multizip((list.iter(), expected_list.iter())) {
        assert!(relative_eq!(value, expected_value, epsilon = f64::EPSILON));
    }
}

#[test]
fn test_dotproduct() {
    let vectors_1 =
        tool::Vectors::from_column_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0]);
    let vectors_2 =
        tool::Vectors::from_column_slice(&[1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0]);
    let expected_dot_products = tool::List::from_column_slice(&[1.0, 1.0, 0.0]);
    let dot_products = tool::dot_products(&vectors_1, &vectors_2);

    for (dot_product, expected_dot_product) in
        multizip((dot_products.iter(), expected_dot_products.iter()))
    {
        assert!(relative_eq!(
            dot_product,
            expected_dot_product,
            epsilon = f64::EPSILON
        ));
    }
}
