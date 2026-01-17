mod fib;
mod perf;

use crate::{fib::bu_kib_sized, perf::funbox};
use fib::{bu_fib, bu_fib_sized, r_fib_memo};
use perf::perf;

fn main() {
    let reps = 10000;
    perf(
        [
            //funbox("r_fib", r_fib),
            funbox("r_fib_memo", r_fib_memo),
            funbox("bu_fib", bu_fib),
            funbox("bu_fib_sized", bu_fib_sized),
        ],
        reps,
    );

    let k = 2;
    println!();
    println!("# Testing k'ibbonacci of k={} {} times", k, reps);
    let start = std::time::Instant::now();
    let r = (0..=reps).fold(0, |_, n| bu_kib_sized(n, k));
    println!("- bu_kib_sized => {} in {:.2?}", r, start.elapsed());
}
