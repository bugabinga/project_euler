///! The idea is to export all main functions, that solve a particular
///! Euler problem here, so that they can be __integration__ tested according
///! to the definition of correctness on Project Euler.
///!
///! This is of course a silly use of integration tests, but I want to use
///! this opportunity to learn about project structure in Rust.
///!
///! More specific and rigorous testing will be found in *test*-submodule in
///! each problem module.

extern crate rayon;
extern crate faster;

pub mod problem_1;
