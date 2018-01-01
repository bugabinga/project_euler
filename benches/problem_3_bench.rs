#![feature(test)]

extern crate project_euler;
extern crate test;

///! Test results on a Thinkpad x230 (BAT) Fedora 27:
///! test tests::prime_factors_of_100                      ... bench:         379 ns/iter (+/- 20)
///! test tests::prime_factors_of_100_000                  ... bench:       4,905 ns/iter (+/- 1,335)
///! test tests::prime_factors_of_100_000_000              ... bench:     113,818 ns/iter (+/- 33,288)
///! test tests::prime_factors_of_100_000_000_000          ... bench:   3,970,585 ns/iter (+/- 121,908)
///! test tests::prime_factors_of_600_851_475_143          ... bench:   9,768,421 ns/iter (+/- 122,950)
///! test tests::prime_factors_parallel_of_100             ... bench:      11,628 ns/iter (+/- 788)
///! test tests::prime_factors_parallel_of_100_000         ... bench:      37,345 ns/iter (+/- 10,049)
///! test tests::prime_factors_parallel_of_100_000_000     ... bench:     135,592 ns/iter (+/- 24,668)
///! test tests::prime_factors_parallel_of_100_000_000_000 ... bench:   2,290,985 ns/iter (+/- 42,828)
///! test tests::prime_factors_parallel_of_600_851_475_143 ... bench:   5,548,919 ns/iter (+/- 271,959)
///! It is yet again evident, that for CPU-bound tasks, parallelization has a high break even point. In these measurements, only values 100_000_000_000 and larger show improvements in execution time.

#[cfg(test)]
mod tests {
    use project_euler;
    use test::{black_box, Bencher};

    #[bench]
    fn prime_factors_of_600_851_475_143(bencher: &mut Bencher) {
        bencher.iter(|| black_box(project_euler::problem_3::prime_factors(600_851_475_143)));
    }

    #[bench]
    fn prime_factors_parallel_of_600_851_475_143(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_3::prime_factors_parallel(
                600_851_475_143,
            ))
        });
    }

    #[bench]
    fn prime_factors_of_100_000(bencher: &mut Bencher) {
        bencher.iter(|| black_box(project_euler::problem_3::prime_factors(100_000)));
    }

    #[bench]
    fn prime_factors_parallel_of_100_000(bencher: &mut Bencher) {
        bencher.iter(|| black_box(project_euler::problem_3::prime_factors_parallel(100_000)));
    }

    #[bench]
    fn prime_factors_of_100(bencher: &mut Bencher) {
        bencher.iter(|| black_box(project_euler::problem_3::prime_factors(100)));
    }

    #[bench]
    fn prime_factors_parallel_of_100(bencher: &mut Bencher) {
        bencher.iter(|| black_box(project_euler::problem_3::prime_factors_parallel(100)));
    }

    #[bench]
    fn prime_factors_of_100_000_000(bencher: &mut Bencher) {
        bencher.iter(|| black_box(project_euler::problem_3::prime_factors(100_000_000)));
    }

    #[bench]
    fn prime_factors_parallel_of_100_000_000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_3::prime_factors_parallel(
                100_000_000,
            ))
        });
    }

    #[bench]
    fn prime_factors_of_100_000_000_000(bencher: &mut Bencher) {
        bencher.iter(|| black_box(project_euler::problem_3::prime_factors(100_000_000_000)));
    }

    #[bench]
    fn prime_factors_parallel_of_100_000_000_000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_3::prime_factors_parallel(
                100_000_000_000,
            ))
        });
    }
}
