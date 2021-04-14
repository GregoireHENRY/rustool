use itertools::multizip;
use tool::{direct_angle, List, Vector, Vectors};

#[test]
fn compute_distances() {
    let vectors = Vectors::from_column_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 1.0]);
    let expected_distances = List::from_column_slice(&[1.0, 2.0f64.sqrt()]);

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
    let vectors = Vectors::from_column_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 1.0]);
    let expected_directions =
        Vectors::from_column_slice(&[1.0, 0.0, 0.0, 0.0, 1.0 / 2.0f64.sqrt(), 1.0 / 2.0f64.sqrt()]);

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

#[test]
fn dotproduct() {
    let vectors_1 = Vectors::from_column_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0]);
    let vectors_2 = Vectors::from_column_slice(&[1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0]);
    let expected_dot_products = List::from_column_slice(&[1.0, 1.0, 0.0]);
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

#[test]
fn projection_vector() {
    let vector_1 = Vector::new(2.0, 2.0, 0.0);
    let vector_2 = Vector::new(0.0, 1.0, 0.0);
    let expected_vector = Vector::new(0.0, 2.0, 0.0);
    let vector = tool::projection_vector(&vector_1, &vector_2);

    for (component, expected_component) in multizip((vector.iter(), expected_vector.iter())) {
        assert!(relative_eq!(
            component,
            expected_component,
            epsilon = f64::EPSILON
        ));
    }
}

#[test]
fn projection_plane() {
    let vector_1 = Vector::new(1.0, 1.0, 1.0);
    let vector_2 = Vector::new(0.0, 0.0, 1.0);
    let expected_vector = Vector::new(1.0, 1.0, 0.0);
    let vector = tool::projection_plane(&vector_1, &vector_2);

    for (component, expected_component) in multizip((vector.iter(), expected_vector.iter())) {
        assert!(relative_eq!(
            component,
            expected_component,
            epsilon = f64::EPSILON
        ));
    }
}

#[test]
fn test_direct_angle_positive() {
    let v1 = Vector::new(1.0, 0.0, 0.0);
    let v2 = Vector::new(1.0, 1.0, 0.0);
    let up = Vector::new(0.0, 0.0, 1.0);
    let ang = direct_angle(&v1, &v2, &up);
    assert!(relative_eq!(
        ang,
        0.7853981633974484,
        epsilon = f64::EPSILON
    ));
}

#[test]
fn test_direct_angle_negative() {
    let v1 = Vector::new(1.0, 0.0, 0.0);
    let v2 = Vector::new(1.0, -1.0, 0.0);
    let up = Vector::new(0.0, 0.0, 1.0);
    let ang = direct_angle(&v1, &v2, &up);
    assert!(relative_eq!(ang, 5.497787143782138, epsilon = f64::EPSILON));
}
