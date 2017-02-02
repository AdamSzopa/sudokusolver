#[macro_use]
extern crate bencher;
extern crate sudokusolver;

use bencher::Bencher;
use sudokusolver::*;

fn solve_simple(bench: &mut Bencher) {
    let mut vec = vec![0u32;81];
    bench.iter(|| {
        solve(&mut vec)
    })
}

fn solve_normal(bench: &mut Bencher) {
        let mut test = vec![0,0,7, 0,2,0, 0,0,3,
                        8,0,5, 0,0,0, 0,6,0,
                        0,0,0, 0,0,0, 0,0,0,
                        
                        0,6,0, 1,5,8, 0,0,0,
                        0,3,0, 0,6,0, 0,0,1,
                        7,0,0, 0,0,9, 0,0,0,
                        
                        2,9,0, 0,0,6, 0,0,0,
                        3,0,0, 0,1,0, 0,0,0,
                        0,0,0, 0,3,0, 8,0,7];
        bench.iter(|| {
            solve(&mut test)
        })
}

benchmark_group!(solve_bench, solve_simple, solve_normal);
benchmark_main!(solve_bench);