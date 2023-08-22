use rand::Rng;
use core::fmt;
use std::{fmt::Write as _, f32::EPSILON};

use crate::{Matrix, MatrixError, traits::FloatOrComplex};

//-------------------------------------------------------------- UTILITY FUNCTIONS
// assert vector and matrix output
pub fn  assert_output<K>(struct_output: &K, expected_output: &str)
where
    K: fmt::Display
{
    let mut output = String::new();
    write!(&mut output, "{}", struct_output).unwrap();
    assert_eq!(output, expected_output);
}

// format vector to string
pub fn  vector_to_string<K>(vector: &Vec<K>) -> String
where
    K: fmt::Display
{
    vector.iter()
        .map(|vec| format!("[{:.1}]", vec))
        .collect::<Vec<String>>()
        .join("")
}

// flatten Vec<Vec<K>> input and returns a String
pub fn  flatten_input<K>(input:Vec<Vec<K>>) -> String
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
pub fn  matrix_to_string<K>(matrix: &Vec<Vec<K>>) -> String
where
    K: fmt::Display
{
    matrix.iter()
        .map(|vec| vector_to_string(vec))
        .collect::<Vec<String>>()
        .join("\n")
}

// match matrix result
pub fn match_matrix_result<K>(matrix: &Result<Matrix<K>, MatrixError>, input:Vec<Vec<K>>)
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
pub fn  generate_random_f32_vector() -> Vec<f32> {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(0..=10);
    (0..size).map(|_| rng.gen::<f32>()).collect()
}

// returns true if numbers are close enough
pub fn numbers_are_close<K>(a: K, b: K) -> bool
where
    K: FloatOrComplex + std::ops::Sub<Output = K>
{
    (a - b).norm_value() < EPSILON * 100.0
}

// compares 2 given matrices
pub fn compare_matrices<K>(mat_1: Matrix<K>, mat_2: Matrix<K>)
where
    K: FloatOrComplex + std::ops::Sub<Output = K>
{
    for (vec_1, vec_2) in mat_1.into_iter().zip(mat_2) {
        for (elem_1, elem_2) in vec_1.into_iter().zip(vec_2) {
            assert_eq!(numbers_are_close(elem_1, elem_2), true);
        }
    }
}