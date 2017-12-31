#[cfg(test)]
extern crate project_euler;
extern crate num_bigint;
extern crate num_traits;

mod tests {
    use project_euler;
    use num_bigint::BigUint;
    use num_traits::Num;

    #[test]
    fn sum_even_fibonacci_numbers_4mio(){
        assert_eq!(project_euler::problem_2::sum_even_fibonacci_numbers(BigUint::from_str_radix("4_000_000",10).unwrap()), BigUint::from_str_radix("4_613_732",10).unwrap());
    }
}
