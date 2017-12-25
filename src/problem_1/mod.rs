///! [Problem 1](https://projecteuler.net/problem=1)
///!
///! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
///! Find the sum of all the multiples of 3 or 5 below 1000.

/// Even though we are only looking for multiples below 1000, this function
/// accepts any bound. I suspect the code will be the same.
pub fn sum_multiples_of_3_and_5(bound:u32) -> u32
{
    if bound < 3 {
        return 0
    }
    // there is no point in including 0 and below 3 in the range, because all modulo filters will return true,
    // but 0 will not affect the sum.
    (3..bound).filter(|n| n%3==0 || n%5==0).sum()
}

use faster::*;

/// SIMD-version of sum_multiples_of_3_and_5.
pub fn sum_multiples_of_3_and_5_simd(bound:u32) -> u32
{
    if bound < 3 {
        return 0
    }
    //I am not sure if this is the normal way to initialize an array in Rust or if
    //this is horribly inefficient.
    //If the benchmark shows problems, it might be worth taking this part out of the
    //measurement to check if this takes a long time
    let mut data = vec![0u32; bound as usize];
    //This cast means bounds are limited to architecture pointer size. Are 8bit CPUs always limited
    //to arrays of max size u8 ?
    //I would've liked to change the signature of this function to usize -> usize, but later in the
    //simd operation there is a need to explicitly declare the size of the packed vectors.
    let mut counter = 0;
    for i in 0..(bound as usize)  {
        data[i] = counter;
        counter = counter + 1;
    }
    // this preallocation of the data also means this function behaves very differently from a
    // memory-consumption perspective.
    // TODO Is there no way to stream with __faster__?
    //TODO bench this!
    (&data[..])
        .simd_iter()
        //This reduction combines a filter for multiples of 3 or 5 and a summation.
        .simd_reduce(u32s::splat(0), u32s::splat(0), |accumulator, value| {
            // __faster__ currently has no filter operation. But since we know
            // the values will be summed, we can fake a filter by setting values to 0.
            // we need to iterate over all the scalars in the packed vector, because our vecs do
            // not have a uniform value.
            let mut vector = *value;
            for idx in 0..value.width() {
                let scalar = vector.extract(idx as u32);
                let new_scalar = if scalar % 3 == 0 || scalar % 5 == 0 {
                    scalar
                }
                else {
                    0
                };
                //FIXME this is confusing;
                //[Docs](https://docs.rs/faster/0.3.0/faster/vecs/trait.Packed.html#tymethod.replace) state a signature of usize for idx but compiler comlains that it should be u32. why?
                // is it because the method gets inlined and idx is cast internally to u32?
                vector = vector.replace(idx as u32, new_scalar);
            }

            *accumulator + vector
        })
        // This sums the packed value, not the data!
        .sum()
}

use rayon::prelude::*;

/// Parallel version of sum_multiples_of_3_and_5.
pub fn sum_multiples_of_3_and_5_parallel(bound:u32) -> u32
{
    if bound < 3 {
        return 0
    }
    // there is no point in including 0 and below 3 in the range, because all modulo filters will return true,
    // but 0 will not affect the sum.
    (3..bound)
        .into_par_iter()
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()

}

#[cfg(test)]
mod tests {
    use problem_1::*;

    #[test]
    fn check_simple_cases() {
        assert_eq!(sum_multiples_of_3_and_5(0),0);
        assert_eq!(sum_multiples_of_3_and_5(1),0);
        assert_eq!(sum_multiples_of_3_and_5(4),3);
        assert_eq!(sum_multiples_of_3_and_5(6),8);
        assert_eq!(sum_multiples_of_3_and_5(15),45);
        assert_eq!(sum_multiples_of_3_and_5(16),60);

        assert_eq!(sum_multiples_of_3_and_5_simd(0),0);
        assert_eq!(sum_multiples_of_3_and_5_simd(1),0);
        assert_eq!(sum_multiples_of_3_and_5_simd(4),3);
        assert_eq!(sum_multiples_of_3_and_5_simd(6),8);
        assert_eq!(sum_multiples_of_3_and_5_simd(15),45);
        assert_eq!(sum_multiples_of_3_and_5_simd(16),60);

        assert_eq!(sum_multiples_of_3_and_5_parallel(0),0);
        assert_eq!(sum_multiples_of_3_and_5_parallel(1),0);
        assert_eq!(sum_multiples_of_3_and_5_parallel(4),3);
        assert_eq!(sum_multiples_of_3_and_5_parallel(6),8);
        assert_eq!(sum_multiples_of_3_and_5_parallel(15),45);
        assert_eq!(sum_multiples_of_3_and_5_parallel(16),60);

    }
}
