# matrix
An introduction to Linear Algebra (in Rust)

## Task list

### Prerequisites :
- [x] Rust Book: Better understanding of Rust libraries / tree structure ?
- [x] Choosing and building a clean tree structure

```
matrix
└── src
   ├── operations
   │    ├── cosine.rs
   │    ├── cross_product.rs
   │    ├── dot_product.rs
   │    └── [...]   // all the operations needed
   ├── tests
   |    ├── ex00_tests.rs 
   │    ├── ex01_tests.rs
   │    └── [...]   
   ├── lib.rs
   ├── matrix.rs  // main structures
   ├── traits.rs
   └── vector.rs
```

- [x] Test driven ? (Rust book) => choosed to tests in a dedicated dir for better clarity
- [x] Understanding Rust traits (Rust book + rustlings)
- [x] Working on doc generation
- [x] Defining Vector and Matrix structures in a generic way to be able to implement a Complex number type later on
- [x] Handling errors : subject is asking for undefined behavior in many cases, but I may choose to handle some
- [x] Row vectors: Should the library handle the case where the matrix has only one row ? -> No
- [x] Better understanding of Matrices / Maths (Essence of linear algebra - 3Blue1Brown - YT)

### Let's do it !
- [x] Exercise 00 - Add, Subtract and Scale
- [x] Exercise 01 - Linear combination
- [x] Exercise 01 - Linear interpolation
- [x] Exercise 03 - Dot product
- [x] Exercise 04 - Norm
- [x] Exercise 05 - Cosine
- [x] Exercise 06 - Cross product
- [x] Exercise 07 - Linear map, Matrix multiplication
- [x] Exercise 08 - Trace
- [x] Exercise 09 - Transpose
- [x] Exercise 10 - Reduced row-echelon form
- [x] Exercise 11 - Determinant
- [x] Exercise 12 - Inverse
- [x] Exercise 13 - Rank

### Bonus
- [ ] Exercise 14 - Bonus: Projection matrix
- [x] Exercise 15 - Bonus: Complex vector spaces
