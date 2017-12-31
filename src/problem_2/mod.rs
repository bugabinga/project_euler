///! (Problem 2)[https://projecteuler.net/problem=2]
///!
///! Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
///!
///! 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
///!
///! By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;
use num_integer::Integer; //is_even

/// Sum even numbers in a Fibonacci sequence up to __bound__.
/// __bound__: Inclusive upper bound of Fibonacci sequence value.
/// The number space of this problem would have allowed to stay with u32 (<~16 million) but I was curious about big int support in Rust.
/// Since this is a counting algorithm, meaning it is fundamentally sequential, there will be no parallelization attempt here.:e
pub fn sum_even_fibonacci_numbers(bound:BigUint) -> BigUint
{
    let mut current_number:BigUint = One::one();
    let mut previous_number:BigUint = One::one();
    let mut sum = Zero::zero();

    loop {
        let fib  = current_number + &previous_number;
        current_number = replace(&mut previous_number, fib);
        if &current_number > &bound {
            break;
        }
        if current_number.is_even() {
            sum = sum + &current_number;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use problem_2::*;
    use num_bigint::ToBigUint;

    #[test]
    fn check_simple_cases(){

        fn assert_that_bound_gives_sum(bound:usize, sum:usize){
            assert_eq!(sum_even_fibonacci_numbers(bound.to_biguint().unwrap()),sum.to_biguint().unwrap() ,"sum of even fib numbers bound to {} should be {}.", bound,sum);
        }

        // 1, 2, 3, 5, 8, 13, 21, 34, 55, 89
        assert_that_bound_gives_sum(0,0);
        assert_that_bound_gives_sum(1,0);
        assert_that_bound_gives_sum(2,2);
        assert_that_bound_gives_sum(3,2);
        assert_that_bound_gives_sum(4,2);
        assert_that_bound_gives_sum(5,2);
        assert_that_bound_gives_sum(6,2);
        assert_that_bound_gives_sum(7,2);
        assert_that_bound_gives_sum(8, 10);
        assert_that_bound_gives_sum(9, 10);
        assert_that_bound_gives_sum(10, 10);
        assert_that_bound_gives_sum(34, 44);
        assert_that_bound_gives_sum(89,44);
    }
}