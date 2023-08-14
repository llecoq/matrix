use rand::Rng;
use core::fmt;
use std::fmt::Write as _;

use crate::{Vector, traits::{AddSubScl, FloatOrComplex}, Matrix, MatrixError};

//-------------------------------------------------------------- UTILITY FUNCTIONS
// assert vector and matrix output
fn  assert_output<K>(struct_output: &K, expected_output: &str)
where
    K: fmt::Display
{
    let mut output = String::new();
    write!(&mut output, "{}", struct_output).unwrap();
    assert_eq!(output, expected_output);
}

// format vector to string
fn  vector_to_string<K>(vector: &Vec<K>) -> String
where
    K: fmt::Display
{
    vector.iter()
        .map(|vec| format!("[{:.1}]", vec))
        .collect::<Vec<String>>()
        .join("")
}

// flatten Vec<Vec<K>> input and returns a String
fn  flatten_input<K>(input:Vec<Vec<K>>) -> String
where
    K: fmt::Display
{
    let mut flattened_input: String = String::new();

    for vector in input {
        flattened_input += &vector_to_string(&vector);
    }

    flattened_input
}

// format matrix to string
fn  matrix_to_string<K>(matrix: &Vec<Vec<K>>) -> String
where
    K: fmt::Display
{
    matrix.iter()
        .map(|vec| vector_to_string(vec))
        .collect::<Vec<String>>()
        .join("\n")
}

// match matrix output
fn match_matrix_output<K>(matrix: &Result<Matrix<K>, MatrixError>, input:Vec<Vec<K>>)
where
    K:  fmt::Display
        + FloatOrComplex
{
    match matrix {
        Ok(matrix) => {
            println!("{:?}", matrix);
            assert_output(&matrix, matrix_to_string(&input).as_str());
        }        
        Err(error) => println!("{}", error)
    };
}

// generate a vector of f32 of random size amd random content
fn  generate_random_f32_vector() -> Vec<f32> {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(0..=10);
    (0..size).map(|_| rng.gen::<f32>()).collect()
}

//---------------------------------------------------------------------- UNIT TEST
//--------------------------------------------------------------------------VECTOR
#[test]
fn  vector_utility_functions_tests() {
    let input_v = generate_random_f32_vector();
    let input_v2: Vec<f32> = generate_random_f32_vector();
    //-----------------------------------------------------------from()
    let v: Vector<f32> = Vector::from(input_v.clone());
    let v2: Vector<f32> = Vector::from(input_v2.clone());
    // let v3: Vector<i32> = Vector::from(vec![1]);
    //-------------------------------------------------------get_size()
    assert_eq!(v.get_size(), input_v.len());
    assert_eq!(v2.get_size(), input_v2.len());
    //---------------------------------------------------print_vector()
    assert_output(&v, vector_to_string(&input_v).as_str());
    //--------------------------------------------reshape_into_matrix()
    let input_v: Vector<f32> = Vector::from(vec![1., 2., 3., 4.,5.,6.,7.,8.,9.,1.,11.,12.]);
    // Not valid  
    let matrix = input_v.reshape_into_matrix(0);
    assert!(matrix.is_err());
    let matrix = input_v.reshape_into_matrix(5);
    assert!(matrix.is_err());
    let matrix = input_v.reshape_into_matrix(9);
    assert!(matrix.is_err());
    let matrix = input_v.reshape_into_matrix(12);
    assert!(matrix.is_err());
    let v: Vec<f32> = Vec::new();
    let input_v: Vector<f32> = Vector::from(v);
    let matrix = input_v.reshape_into_matrix(1);
    assert!(matrix.is_err());

    // valid
    let input_v: Vector<f32> = Vector::from(vec![1., 2., 3., 4.,5.,6.,7.,8.,9.,1.,11.,12.]);
    let matrix = input_v.reshape_into_matrix(1);
    assert!(matrix.is_ok());
    let matrix = input_v.reshape_into_matrix(2);
    assert!(matrix.is_ok());
    let matrix = input_v.reshape_into_matrix(3);
    assert!(matrix.is_ok());
    let matrix = input_v.reshape_into_matrix(4);
    assert!(matrix.is_ok());
    let matrix = input_v.reshape_into_matrix(6);
    assert!(matrix.is_ok());
}

#[test]
fn  vector_add_tests() {
    let mut v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f32> = Vector::from(vec![]);
    let v3: Vector<f32> = Vector::from(vec![1.1, 2.]);

    v.add(&v3);
    assert_output(&v, "[3.1][8.3]");

    v.add(&v2); // should not do anything. Choosed not to use Result<> to make it lighter in use
    assert_output(&v, "[3.1][8.3]");
}

#[test]
fn  vector_sub_tests() {
    let mut v: Vector<f32> = Vector::from(vec![2., 6.3]);
    let v2: Vector<f32> = Vector::from(vec![]);
    let v3: Vector<f32> = Vector::from(vec![1.1, 2.]);

    v.sub(&v3);
    assert_output(&v, "[0.9][4.3]");

    v.sub(&v2); // should not do anything. Choosed not to use Result<> to make it lighter in use
    assert_output(&v, "[0.9][4.3]");
}

#[test]
fn  vector_scl_tests() {
    let mut u = Vector::from(vec![2., 3.]);

    u.scl(2.);
    assert_output(&u, "[4.0][6.0]");
    u.scl(-1.);
    assert_output(&u, "[-4.0][-6.0]");
    u.scl(-0.);
    assert_output(&u, "[0.0][0.0]");
}

//--------------------------------------------------------------------------MATRIX
#[test]
#[allow(unused_variables)]
fn  matrix_utility_functions_tests() {
    //-------------------from(), get_shape(), reshape_into_vector(), is_a_square()
    // Invalid matrix format
    let input = vec![
        vec![1.1],
        vec![1.1, 2.]
    ];
    let matrix = Matrix::from(input.clone());
    match_matrix_output(&matrix, input);

    // Row vector
    let input = vec![
        vec![1.1],
    ];
    let matrix = Matrix::from(input.clone());
    match_matrix_output(&matrix, input); 

    // Invalid matrix format
    let input = vec![
        vec![1.1, 2., 3.6],
        vec![1.1, 2.],
        vec![1.1, 2., 4.5]
    ];
    let matrix = Matrix::from(input.clone());
    match_matrix_output(&matrix, input);

    // Empty matrix
    let v: Vec<f32> = Vec::new();
    let input: Vec<Vec<f32>> = vec![
        v,
        vec![],
        vec![],
        vec![]
    ];
    let matrix = Matrix::from(input.clone());
    match_matrix_output(&matrix, input);

    // Valid
    let input = vec![
        vec![1.1, 2.],
        vec![2.1, 2.]
    ];
    // from
    let matrix = Matrix::from(input.clone());
    match_matrix_output(&matrix, input.clone());
    // get_shape()
    let matrix: Matrix<f32> = matrix.unwrap();
    assert_eq!(matrix.get_shape(), "(2,2)");
    // is_a_square()
    assert_eq!(matrix.is_a_square(), true);
    // reshape matrix into vector
    let vector: Vector<f32> = matrix.reshape_into_vector();
    assert_output(&vector, &flatten_input(input));

    // Valid
    let input = vec![
        vec![1.1, 2., 3.6],
        vec![2.1, 3., 2.6],
        vec![3.1, 2., 4.5],
        vec![4.3, 2., 4.5]
    ];
    // from
    let matrix = Matrix::from(input.clone());
    match_matrix_output(&matrix, input.clone());
    // get_shape()
    let matrix: Matrix<f32> = matrix.unwrap();
    assert_eq!(matrix.get_shape(), "(4,3)");
    // is_a_square()
    assert_eq!(matrix.is_a_square(), false);
    // reshape matrix into vector
    let vector: Vector<f32> = matrix.reshape_into_vector();
    assert_output(&vector, &flatten_input(input));

    // Valid
    let input = vec![
        vec![1.1, 2., 3.6, 4.0, 5.3, 6.2, 7.6, 8., 9.],
        vec![2.1, 3., 2.6, 5.0, 9.3, 4.2, 2.6, 4., 6.],
        vec![3.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6.],
        vec![4.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6.],
        vec![5.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6.],
        vec![6.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6.],
        vec![7.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6.],
        vec![8.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6.],
        vec![9.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6.],
        vec![10.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6.],
        vec![11.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6.],
        vec![12.3, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6.]
    ];
    // from
    let matrix = Matrix::from(input.clone());
    match_matrix_output(&matrix, input.clone());
    // get_shape()
    let matrix: Matrix<f32> = matrix.unwrap();
    assert_eq!(matrix.get_shape(), "(12,9)");
    // is_a_square()
    assert_eq!(matrix.is_a_square(), false);
    // reshape matrix into vector
    let vector: Vector<f32> = matrix.reshape_into_vector();
    assert_output(&vector, &flatten_input(input));

}

#[test]
fn matrix_add_sub_scl_tests() {
    let input = vec![
        vec![1.1, 2., 3.6],
        vec![2.1, 3., 2.6],
        vec![3.1, 2., 4.5],
        vec![4.3, 2., 4.5]
    ];
    let input_2 = vec![
        vec![1., 1., 1.],
        vec![1., 1., 1.],
        vec![1., 1., 1.],
        vec![1., 1., 1.]
    ];
    let matrix = Matrix::from(input.clone());
    let matrix_2 = Matrix::from(input_2.clone());

    
    //-------------------------------------------------------------------- add
    let mut matrix: Matrix<f32> = matrix.unwrap();
    let matrix_2 = matrix_2.unwrap();
    let result = vec![
        vec![2.1, 3., 4.6],
        vec![3.1, 4., 3.6],
        vec![4.1, 3., 5.5],
        vec![5.3, 3., 5.5]
    ];
    matrix.add(&matrix_2);
    assert_output(&matrix, matrix_to_string(&result).as_str());

    //-------------------------------------------------------------------- sub
    matrix.sub(&matrix_2);
    let result = vec![
        vec![1.1, 2., 3.6],
        vec![2.1, 3., 2.6],
        vec![3.1, 2., 4.5],
        vec![4.3, 2., 4.5]
    ];
    assert_output(&matrix, matrix_to_string(&result).as_str());

}