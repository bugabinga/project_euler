#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher,black_box};

    /// Do the closes thing to "nothing", without actually being optimized away. Serves as a
    /// baseline for other benchmarks. Ideally it should show the time to fetch a value from a
    /// register, which I expect to be around 4ns on a laptop from 2009.
    /// Measurments return 0ns :(. Does this mean the optimizer hit it or assumptions are wrong?
    #[bench]
    fn no_op(bencher: &mut Bencher)
    {
        bencher.iter(|| black_box(1));
    }

}
