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

    let list = tool::linspace(start, end, step);

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
fn test_linspace_integer() {
    let start = 0;
    let end = 10;
    let step = 2;
    let expected_list = tool::List::from_column_slice(&[0, 2, 4, 6, 8, 10]);

    let list = tool::linspace(start, end, step);

    assert_eq!(list.len(), 6);

    for (value, expected_value) in multizip((list.iter(), expected_list.iter())) {
        assert_eq!(value, expected_value);
    }
}
