#[cfg(test)]
extern crate project_euler;

mod tests {
    use project_euler;

    #[test]
    fn sum_of_1000_multiples(){
        assert_eq!(project_euler::problem_1::sum_multiples_of_3_and_5(1000), 233168);
        assert_eq!(project_euler::problem_1::sum_multiples_of_3_and_5_simd(1000), 233168);
        assert_eq!(project_euler::problem_1::sum_multiples_of_3_and_5_parallel(1000), 233168);
    }
}
