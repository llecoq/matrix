use crate::{Vector, traits::AddSubScl, Matrix};
use std::fmt::Write as _;

//--------------------------------------------------------------- Utility function
// assert vector and matrix output
fn  assert_output<K>(struct_output: &K, expected_output: &str)
where
    K: std::fmt::Display
{
    let mut output = String::new();
    write!(&mut output, "{}", struct_output).unwrap();
    assert_eq!(output, expected_output);
}

// format vector to string
fn  vector_to_string<K>(vector: &Vec<K>) -> String
where
    K: std::fmt::Display
{
    vector.iter()
        .map(|vec| format!("[{:.1}]", vec))
        .collect::<Vec<String>>()
        .join("")
}

// format matrix to string
fn  matrix_to_string<K>(matrix: &Vec<Vec<K>>) -> String
where
    K: std::fmt::Display
{
    let flat_matrix: Vec<&K> = matrix
        .into_iter()
        .flatten()
        .collect();

    flat_matrix
        .iter()
        .map(|data| data.to_string())
        .collect::<Vec<String>>()
        .join("]")
}



//---------------------------------------------------------------------- Unit Test
// Vector
#[test]
fn  vector_utility_functions_tests() {
    let input_v = vec![2., 6.3];
    let input_v2: Vec<f64> = vec![];
    // from()
    let v: Vector<f32> = Vector::from(input_v.clone());
    let v2: Vector<f64> = Vector::from(input_v2.clone());
    // get_size()
    assert_eq!(v.get_size(), input_v.len());
    assert_eq!(v2.get_size(), input_v2.len());
    // print_vector()
    assert_output(&v, vector_to_string(&input_v).as_str());
    // reshape_into_matrix()

}

#[test]
fn  vector_add_tests() {
    let mut v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f32> = Vector::from(vec![]);
    let v3: Vector<f32> = Vector::from(vec![1.1, 2.]);

    v.add(&v3);
    assert_output(&v, "[3.1][8.3]\n");

    v.add(&v2); // should not do anything. Choosed not to use Result<> to make it lighter in use
    assert_output(&v, "[3.1][8.3]\n");
}

#[test]
fn  vector_sub_tests() {
    let mut v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f32> = Vector::from(vec![]);
    let v3: Vector<f32> = Vector::from(vec![1.1, 2.]);

    v.sub(&v3);
    assert_output(&v, "[0.9][4.3]\n");

    v.sub(&v2); // should not do anything. Choosed not to use Result<> to make it lighter in use
    assert_output(&v, "[0.9][4.3]\n");
}

#[test]
fn  vector_scl_tests() {
    let mut u = Vector::from(vec![2., 3.]);

    u.scl(2.);
    assert_output(&u, "[4.0][6.0]\n");
    u.scl(-1.);
    assert_output(&u, "[-4.0][-6.0]\n");
    u.scl(-0.);
    assert_output(&u, "[0.0][0.0]\n");
}

#[test]
#[allow(unused_variables)]
fn  matrix_utility_functions_tests() {
    // Valid
    let input = vec![
        vec![1.1, 2.],
        vec![1.1, 2.]
    ];
    let matrix = Matrix::from(input.clone());
    match matrix {
        Ok(matrix) => {
            println!("{}", matrix);
            assert_output(&matrix, matrix_to_string(&input).as_str());
        }        
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