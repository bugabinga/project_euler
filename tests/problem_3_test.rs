#[cfg(test)]
extern crate project_euler;

mod tests {
    use project_euler;

    #[test]
    fn prime_factors_of_600851475143() {
        assert_eq!(
            project_euler::problem_3::prime_factors(600_851_475_143)
                .last()
                .unwrap(),
            &6857
        );

        assert_eq!(
            project_euler::problem_3::prime_factors_parallel(600_851_475_143)
                .last()
                .unwrap(),
            &6857
        );
    }
}
