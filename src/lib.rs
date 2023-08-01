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
pub use matrix::*;

//----------------------- MODULES
pub mod vector;
pub mod matrix;
mod traits;