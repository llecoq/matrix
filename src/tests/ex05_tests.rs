use crate::{Vector, angle_cos};

fn float_are_close(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.00001
}

#[test]
fn cosine_tests() {
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![1., 0.]);
    assert_eq!(float_are_close(angle_cos(&u, &v), 1.0), true);
    // 1.0
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![0., 1.]);
    assert_eq!(float_are_close(angle_cos(&u, &v), 0.0), true);
    // 0.0
    let u = Vector::from(vec![-1., 1.]);
    let v = Vector::from(vec![ 1., -1.]);
    assert_eq!(float_are_close(angle_cos(&u, &v), -1.0), true);
    // -1.0
    let u = Vector::from(vec![2., 1.]);
    let v = Vector::from(vec![4., 2.]);
    assert_eq!(float_are_close(angle_cos(&u, &v), 1.0), true);
    // 1.0
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    assert_eq!(float_are_close(angle_cos(&u, &v), 0.974631846), true);
    // 0.974631846
}