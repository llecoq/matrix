use crate::{Vector, traits::AddSubScl};
use std::fmt::Write as _;

#[test]
fn  vector_utility_functions_tests() {
    // from()
    let v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f64> = Vector::from(vec![]);
    // get_size()
    assert_eq!(v.get_size(), 2);
    assert_eq!(v2.get_size(), 0);
    // print_vector()
    let mut output = String::new();
    writeln!(&mut output, "{}", v).unwrap();
    assert_eq!(output, "[2.0]\n[6.3]\n");
    output.clear();
    // reshape_into_matrix()

}

#[test]
fn  vector_add_tests() {
    let mut v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f32> = Vector::from(vec![]);
    let v3: Vector<f32> = Vector::from(vec![1.1, 2.]);
    let mut output = String::new();

    v.add(&v3);
    writeln!(&mut output, "{}", v).unwrap();
    assert_eq!(output, "[3.1]\n[8.3]\n");

    v.add(&v2); // should not do anything. Later one, maybe handle error !
    output.clear();
    writeln!(&mut output, "{}", v).unwrap();
    assert_eq!(output, "[3.1]\n[8.3]\n");
}

#[test]
fn  vector_sub_tests() {
    let mut v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f32> = Vector::from(vec![]);
    let v3: Vector<f32> = Vector::from(vec![1.1, 2.]);
    let mut output = String::new();

    v.sub(&v3);
    writeln!(&mut output, "{}", v).unwrap();
    assert_eq!(output, "[0.9]\n[4.3]\n");

    v.sub(&v2); // should not do anything. Later one, maybe handle error !
    output.clear();
    writeln!(&mut output, "{}", v).unwrap();
    assert_eq!(output, "[0.9]\n[4.3]\n");
}