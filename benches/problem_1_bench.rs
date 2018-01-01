#![feature(test)]

extern crate project_euler;
extern crate test;

/// Results on a Thinkpad x230 (AC) Fedora 27:
/// test tests::sum_multiples_of_3_and_5_1000             ... bench:       1,150 ns/iter (+/- 74)
/// test tests::sum_multiples_of_3_and_5_10000            ... bench:      11,474 ns/iter (+/- 189)
/// test tests::sum_multiples_of_3_and_5_100000           ... bench:     116,110 ns/iter (+/- 10,365)
/// test tests::sum_multiples_of_3_and_5_1000000          ... bench:   1,147,006 ns/iter (+/- 77,932)
/// test tests::sum_multiples_of_3_and_5_parallel_1000    ... bench:      12,358 ns/iter (+/- 199)
/// test tests::sum_multiples_of_3_and_5_parallel_10000   ... bench:      20,692 ns/iter (+/- 1,313)
/// test tests::sum_multiples_of_3_and_5_parallel_100000  ... bench:      73,143 ns/iter (+/- 4,428)
/// test tests::sum_multiples_of_3_and_5_parallel_1000000 ... bench:     593,214 ns/iter (+/- 20,628)
/// test tests::sum_multiples_of_3_and_5_simd_1000        ... bench:      10,977 ns/iter (+/- 712)
/// test tests::sum_multiples_of_3_and_5_simd_10000       ... bench:     110,010 ns/iter (+/- 6,285)
/// test tests::sum_multiples_of_3_and_5_simd_100000      ... bench:   1,118,675 ns/iter (+/- 36,424)
/// test tests::sum_multiples_of_3_and_5_simd_1000000     ... bench:  12,195,559 ns/iter (+/- 577,532)
///
/// Linear growth of serial function is as expected.
/// Parallel function gets faster only with very large collections. This also supports the model,
/// that synchronization adds significant overhead. ROI seems to be around 100000 elements.
/// The SIMD case is surprising. It grows linearly, which is expected, but has a high additional
/// constant time compared to the serial case.
/// This can mean various different things, which I cannot inspect yet, because I know too little
/// about LLVM/assembler. My theory: The high constant cost is probably due to th preallocation of
/// the data in the function. The other two functions work on streams. Additionally, the serial case
/// probaly got automatically SIMD'd by the optimizer. This could explain the seeming __slowness__
/// of the SIMD case.
///
/// test tests::sum_multiples_of_3_and_5_preallocate_vector_1000000 ... bench:   4,982,959 ns/iter (+/- 176,725)
/// This result is < SIMD > serial. This suggests that, a.) preallocation is very costly (4x) and
/// b.) preallocation is not the only problem since the SIMD case is still 3x slower.
/// This suggests to me, that I must've misunderstood the point of SIMD/__faster__. Maybe it is not
/// meant to be usd the way I did?
#[cfg(test)]
mod tests {
    use project_euler;
    use test::{black_box, Bencher};

    //This kind of repition can probably be fixed with Rust macros. But I do not know how yet.

    #[bench]
    fn sum_multiples_of_3_and_5_1000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5(1000));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_simd_1000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5_simd(
                1000,
            ));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_parallel_1000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5_parallel(1000));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_10000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5(10000));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_simd_10000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5_simd(
                10000,
            ));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_parallel_10000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5_parallel(10000));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_100000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5(100000));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_simd_100000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5_simd(
                100000,
            ));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_parallel_100000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5_parallel(100000));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_1000000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5(1000000));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_simd_1000000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5_simd(
                1000000,
            ));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_parallel_1000000(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(project_euler::problem_1::sum_multiples_of_3_and_5_parallel(1000000));
        });
    }

    #[bench]
    fn sum_multiples_of_3_and_5_preallocate_vector_1000000(bencher: &mut Bencher) {
        //Reimplements to sum_multiples_of_3_and_5 function with a (unnecessary) preallocation of
        //the data. This can be used to indicate why the SIMD case is seemingly so slow.:w

        bencher.iter(|| {
            let size = 1000000;
            let mut data = vec![032; size];
            let mut element = 0u32;
            for i in 0..size {
                data[i] = element;
                element = element + 1;
            }

            let sum: u32 = data.into_iter().filter(|n| n % 3 == 0 || n % 5 == 0).sum();

            black_box(sum);
        });
    }
}
