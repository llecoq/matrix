use crate::Vector;
use std::fmt::Write as _;

#[test]
fn vector_utility_functions_tests() {
    // from()
    let v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f64> = Vector::from(vec![]);
    // get_size()
    assert_eq!(v.get_size(), 2);
    assert_eq!(v2.get_size(), 0);
    // print_vector()
    let mut output = String::new();
    writeln!(&mut output, "{}", v).unwrap();

}