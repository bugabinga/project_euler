///! [Problem 3](https://projecteuler.net/problem=3)
///! The prime factors of 13195 are 5, 7, 13 and 29.
///! What is the largest prime factor of the number 600851475143 ?

pub fn prime_factors(number: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    if number < 2 {
        return factors;
    }

    for factor in 2..sqrt(number) {
        if is_divisible_by(number, factor) {
            if is_prime(factor) {
                factors.push(factor);
            }

            let other_factor = number / factor;
            if is_prime(other_factor) {
                factors.push(other_factor);
            }
        }
    }

    factors.sort();
    factors
}

/// Implementing this in SIMD should be possible in principle. But the amount
/// of work to re-implement simple operations (division, modulo) with
/// bit-shift/addition/multiplication only is too much.

pub fn prime_factors_parallel(number: u64) -> Vec<u64> {
    if number < 2 {
        return Vec::with_capacity(0);
    }

    let mut factors: Vec<u64> = (2..sqrt(number))
        .into_par_iter()
        .filter(|factor| is_divisible_by(number, *factor))
        .flat_map(|factor| {
            let mut prime_factors = Vec::new();
            if is_prime_parallel(factor) {
                prime_factors.push(factor);
            }
            let other_factor = number / factor;
            if is_prime_parallel(other_factor) {
                prime_factors.push(other_factor);
            }
            prime_factors
        })
        .collect();

    factors.sort();
    factors
}

fn sqrt(n: u64) -> u64 {
    // if the value of n is too large for f64 to hold, it will be clamped to 0
    let square_root = ((n as f64).sqrt() + 1.0) as u64;

    if square_root != 0 {
        return square_root;
    }

    panic!(
        "missing function for square root for values larger than f64::MAX ({}).",
        n
    );
}

fn is_divisible_by(n: u64, d: u64) -> bool {
    n % d == 0
}

use rayon::prelude::*;

fn is_prime_parallel(number: u64) -> bool {
    if number < 2 {
        return false;
    }

    (2..sqrt(number))
        .into_par_iter()
        .all(|factor| !(is_divisible_by(number, factor)))
}

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }

    (2..sqrt(number)).all(|factor| !(is_divisible_by(number, factor)))
}

#[cfg(test)]
mod tests {
    use problem_3::*;

    #[test]
    fn check_simple_cases() {
        assert_eq!(prime_factors(1), vec![]);
        assert_eq!(prime_factors(3), vec![]);
        assert_eq!(prime_factors(5), vec![]);
        assert_eq!(prime_factors(7), vec![]);
        assert_eq!(prime_factors(13), vec![]);
        assert_eq!(prime_factors(29), vec![]);

        assert_eq!(prime_factors(35), vec![5, 7]);
        assert_eq!(prime_factors(158), vec![2, 79]);
        assert_eq!(prime_factors(1027), vec![13, 79]);
        assert_eq!(prime_factors(23567), vec![]);
        assert_eq!(prime_factors(76543), vec![]);
        assert_eq!(prime_factors(196504), vec![2, 7, 11, 29]);

        assert_eq!(prime_factors(0), vec![]);

        assert_eq!(prime_factors(13195), vec![5, 7, 13, 29]);
    }

    #[test]
    fn rust_ranges() {
        assert_eq!((2..10 / 2), (2..5));
        assert_eq!((1..5 + 1), (1..6));
    }

    #[test]
    fn rust_casting_numbers() {
        let non_constant = vec![u64::max_value()];
        assert_ne!(((non_constant[0]) as f64) as u64, non_constant[0]);
    }
}
