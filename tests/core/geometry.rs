use itertools::multizip;

#[test]
fn compute_distances() {
    let vectors = tool::Vectors::from_column_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 1.0]);
    let expected_distances = tool::List::from_column_slice(&[1.0, 2.0f64.sqrt()]);

    let distances = tool::magnitudes(&vectors);

    for (distance, expected_distance) in multizip((distances.iter(), expected_distances.iter())) {
        assert!(relative_eq!(
            distance,
            expected_distance,
            epsilon = f64::EPSILON
        ));
    }
}

#[test]
fn compute_directions() {
    let vectors = tool::Vectors::from_column_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 1.0]);
    let expected_directions = tool::Vectors::from_column_slice(&[
        1.0,
        0.0,
        0.0,
        0.0,
        1.0 / 2.0f64.sqrt(),
        1.0 / 2.0f64.sqrt(),
    ]);

    let directions = tool::units(&vectors);

    for (direction_projection, expected_direction_projection) in
        multizip((directions.iter(), expected_directions.iter()))
    {
        assert!(relative_eq!(
            direction_projection,
            expected_direction_projection,
            epsilon = f64::EPSILON
        ));
    }
}
