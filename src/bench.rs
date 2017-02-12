#[macro_use]
extern crate bencher;
extern crate sudokusolver;

use bencher::Bencher;
use sudokusolver::*;

fn solve_simple_cross(bench: &mut Bencher) {
    let mut vec = vec![0u32;81];
    bench.iter(|| solve_threads_cross(&mut vec))
}

fn solve_simple_threads(bench: &mut Bencher) {
    let vec = vec![0u32;81];
    bench.iter(|| solve_threads_std(&vec))
}

fn solve_simple(bench: &mut Bencher) {
    let mut vec = vec![0u32;81];
    bench.iter(|| solve_single_thread(&mut vec))
}

fn solve_normal_cross(bench: &mut Bencher) {
    let mut test = vec![0, 0, 7, 0, 2, 0, 0, 0, 3, 8, 0, 5, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 6, 0, 1, 5, 8, 0, 0, 0, 0, 3, 0, 0, 6, 0, 0, 0, 1, 7, 0, 0, 0, 0,
                        9, 0, 0, 0, 2, 9, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                        0, 3, 0, 8, 0, 7];
    bench.iter(|| solve_threads_cross(&mut test))
}

fn solve_normal_threads(bench: &mut Bencher) {
    let test = vec![0, 0, 7, 0, 2, 0, 0, 0, 3, 8, 0, 5, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 6, 0, 1, 5, 8, 0, 0, 0, 0, 3, 0, 0, 6, 0, 0, 0, 1, 7, 0, 0, 0, 0, 9, 0,
                    0, 0, 2, 9, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
                    8, 0, 7];
    bench.iter(|| solve_threads_std(&test))
}

fn solve_normal(bench: &mut Bencher) {
    let mut test = vec![0, 0, 7, 0, 2, 0, 0, 0, 3, 8, 0, 5, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 6, 0, 1, 5, 8, 0, 0, 0, 0, 3, 0, 0, 6, 0, 0, 0, 1, 7, 0, 0, 0, 0,
                        9, 0, 0, 0, 2, 9, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                        0, 3, 0, 8, 0, 7];
    bench.iter(|| solve_single_thread(&mut test))
}

benchmark_group!(solve_bench, solve_simple, solve_normal);
benchmark_group!(solve_bench_threads,
                 solve_simple_threads,
                 solve_normal_threads);
benchmark_group!(solve_bench_cross, solve_simple_cross, solve_normal_cross);
benchmark_main!(solve_bench, solve_bench_threads, solve_bench_cross);