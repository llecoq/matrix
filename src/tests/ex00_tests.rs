use crate::{Vector, traits::AddSubScl, Matrix};
use std::fmt::Write as _;

//--------------------------------------------------------------- Utility function
fn  assert_vector_output<K>(vec: &Vector<K>, expected_output: &str)
where
    K: std::fmt::Display
{
    let mut output = String::new();
    writeln!(&mut output, "{}", vec).unwrap();
    assert_eq!(output, expected_output);
}



//---------------------------------------------------------------------- Unit Test
// Vector
#[test]
fn  vector_utility_functions_tests() {
    // from()
    let v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f64> = Vector::from(vec![]);
    // get_size()
    assert_eq!(v.get_size(), 2);
    assert_eq!(v2.get_size(), 0);
    // print_vector()
    assert_vector_output(&v, "[2.0][6.3]\n");
    // reshape_into_matrix()

}

#[test]
fn  vector_add_tests() {
    let mut v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f32> = Vector::from(vec![]);
    let v3: Vector<f32> = Vector::from(vec![1.1, 2.]);

    v.add(&v3);
    assert_vector_output(&v, "[3.1][8.3]\n");

    v.add(&v2); // should not do anything. Choosed not to use Result<> to make it lighter in use
    assert_vector_output(&v, "[3.1][8.3]\n");
}

#[test]
fn  vector_sub_tests() {
    let mut v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f32> = Vector::from(vec![]);
    let v3: Vector<f32> = Vector::from(vec![1.1, 2.]);

    v.sub(&v3);
    assert_vector_output(&v, "[0.9][4.3]\n");

    v.sub(&v2); // should not do anything. Choosed not to use Result<> to make it lighter in use
    assert_vector_output(&v, "[0.9][4.3]\n");
}

#[test]
fn  vector_scl_tests() {
    let mut u = Vector::from(vec![2., 3.]);

    u.scl(2.);
    assert_vector_output(&u, "[4.0][6.0]\n");
    u.scl(-1.);
    assert_vector_output(&u, "[-4.0][-6.0]\n");
    u.scl(-0.);
    assert_vector_output(&u, "[0.0][0.0]\n");
}

#[test]
#[allow(unused_variables)]
fn  matrix_utility_functions_tests() {
    // Valid
    let matrix = Matrix::from(vec! [
        vec![1.1, 2.],
        vec![1.1, 2.]
    ]);
    match matrix {
        Ok(matrix) => println!("{}", matrix),        
        Err(error) => println!("{}", error)
    };

    // Invalid matrix format
    let matrix = Matrix::from(vec! [
        vec![1.1],
        vec![1.1, 2.]
    ]);
    match matrix {
        Ok(matrix) => println!("{}", matrix),
        Err(error) => println!("{}", error)
    };

    // Invalid matrix format
    let matrix = Matrix::from(vec! [
        vec![1.1, 2.],
        vec![1.1, 2., 2.]
    ]);
    match matrix {
        Ok(matrix) => println!("{}", matrix),
        Err(error) => println!("{}", error)
    };

    // Valid
    let matrix = Matrix::from(vec! [
        vec![1.1, 2., 6.],
        vec![1.1, 2., 6.],
        vec![1.1, 2., 6.],
        vec![1.1, 2., 6.]
    ]);
    match matrix {
        Ok(matrix) => println!("{}", matrix),
        Err(error) => println!("{}", error)
    };

    // Empty matrix
    let v: Vec<f32> = Vec::new();
    let matrix = Matrix::from(vec! [
        v,
        vec![],
        vec![],
        vec![]
    ]);
    match matrix {
        Ok(matrix) => println!("{}", matrix),
        Err(error) => println!("{}", error)
    };

}