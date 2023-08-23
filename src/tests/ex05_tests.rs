use num_complex::Complex;

use crate::{Vector, angle_cos, tests::test_utils::numbers_are_close};

#[test]
fn cosine_tests() {
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![1., 0.]);
    assert_eq!(numbers_are_close(angle_cos(&u, &v), 1.0), true);
    // 1.0
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![0., 1.]);
    assert_eq!(numbers_are_close(angle_cos(&u, &v), 0.0), true);
    // 0.0
    let u = Vector::from(vec![-1., 1.]);
    let v = Vector::from(vec![ 1., -1.]);
    assert_eq!(numbers_are_close(angle_cos(&u, &v), -1.0), true);
    // -1.0
    let u = Vector::from(vec![2., 1.]);
    let v = Vector::from(vec![4., 2.]);
    assert_eq!(numbers_are_close(angle_cos(&u, &v), 1.0), true);
    // 1.0
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    assert_eq!(numbers_are_close(angle_cos(&u, &v), 0.974631846), true);
    // 0.974631846

    // undefined
    let u = Vector::from(vec![1., 2.]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));

    // NaN
    let u = Vector::from(vec![]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));

    let u = Vector::from(vec![1.]);
    let v = Vector::from(vec![]);
    println!("{}", angle_cos(&u, &v));

}

#[test]
fn cosine_complex_tests() {
    let u = Vector::from(vec![Complex::new(1., 0.), Complex::new(0., 0.)]);
    let v = Vector::from(vec![Complex::new(1., 0.), Complex::new(0., 0.)]);
    assert_eq!(numbers_are_close(angle_cos(&u, &v), 1.0), true);
    // 1.0

    let u = Vector::from(vec![Complex::new(1., 0.), Complex::new(0., 0.)]);
    let v = Vector::from(vec![Complex::new(0., 0.), Complex::new(1., 0.)]);
    assert_eq!(numbers_are_close(angle_cos(&u, &v), 0.0), true);
    // 0.0

    let u = Vector::from(vec![Complex::new(-1., 0.), Complex::new(1., 0.)]);
    let v = Vector::from(vec![Complex::new(1., 0.), Complex::new(-1., 0.)]);
    assert_eq!(numbers_are_close(angle_cos(&u, &v), -1.0), true);
    // -1.0

    let u = Vector::from(vec![Complex::new(2., 0.), Complex::new(1., 0.)]);
    let v = Vector::from(vec![Complex::new(4., 0.), Complex::new(2., 0.)]);
    assert_eq!(numbers_are_close(angle_cos(&u, &v), 1.0), true);
    // 1.0

    let u = Vector::from(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.)]);
    let v = Vector::from(vec![Complex::new(4., 0.), Complex::new(5., 0.), Complex::new(6., 0.)]);
    assert_eq!(numbers_are_close(angle_cos(&u, &v), 0.974631846), true);
    // 0.974631846

    // undefined
    let u = Vector::from(vec![Complex::new(1., 0.), Complex::new(2., 0.)]);
    let v = Vector::from(vec![Complex::new(4., 0.), Complex::new(5., 0.), Complex::new(6., 0.)]);
    println!("{}", angle_cos(&u, &v));

    // NaN
    let u = Vector::from(vec![]);
    let v = Vector::from(vec![Complex::new(4., 0.), Complex::new(5., 0.), Complex::new(6., 0.)]);
    println!("{}", angle_cos(&u, &v));

    let u = Vector::from(vec![Complex::new(1., 0.)]);
    let v = Vector::from(vec![]);
    println!("{}", angle_cos(&u, &v));
}
