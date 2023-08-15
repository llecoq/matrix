/*!
# Enter the matrix

**matrix** is a 42school project targeting to write a simple linear algebra library :

* Add, Substract and Scale
* Linear combination
* Linear interpolation
* Dot product
* Norm
* Cosine
* Cross product
* Linear map, Matrix multiplication
* Trace
* Transpose
* Solving systems of linear equations
* Reduced row-echelon form
* Determinant
* Inverse
* Rank-nullity theorem
* Rank
* Projection matrix
* Complex vector spaces
*/

//---------------------- RE-EXPORT
#[doc(inline)]
pub use vector::*;
#[doc(inline)]
pub use self::matrix::*;

//----------------------- MODULES
pub mod vector;
pub mod matrix;
pub mod traits;
pub mod operations;

//------------------------- TESTS
#[cfg(test)]
mod tests {
    mod test_utils;
    mod ex00_tests; // Add, Substract and Scale
    mod ex01_tests; // Linear combination
    mod ex02_tests; // Linear interpolation
    mod ex03_tests; // Dot product
    mod ex04_tests; // Norm
    mod ex05_tests; // Cosine
    mod ex06_tests; // Cross product
    mod ex07_tests; // Linear map, Matrix multiplication
    mod ex08_tests; // Trace
    mod ex09_tests; // Transpose
    mod in00_tests; // Solving systems of linear equations
    mod ex10_tests; // Reduced row-echelon form
    mod ex11_tests; // Determinant
    mod ex12_tests; // Inverse
    mod in01_tests; // Rank-nullity theorem
    mod ex13_tests; // Rank
    mod ex14_tests; // Projection matrix
    mod ex15_tests; // Complex vector spaces
}