extern crate faster;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
///! The idea is to export all main functions, that solve a particular
///! Euler problem here, so that they can be __integration__ tested according
///! to the definition of correctness on Project Euler.
///!
///! This is of course a silly use of integration tests, but I want to use
///! this opportunity to learn about project structure in Rust.
///!
///! More specific and rigorous testing will be found in *test*-sub module in
///! each problem module.
extern crate rayon;

pub mod problem_1;
pub mod problem_2;
pub mod problem_3;
pub mod problem_4;
