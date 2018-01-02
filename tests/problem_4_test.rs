#[cfg(test)]
extern crate project_euler;

mod tests {
    use project_euler;

    #[test]
    fn largest_palindrome_from_three_digit_factors_is_xxxx() {
        assert_eq!(
            project_euler::problem_4::largest_palindrome_from_three_digit_factors(),
            906609
        );
    }
}
