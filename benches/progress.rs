use divan::{Bencher, black_box};
use rust_foundations::calculate_completion_percentage;

fn main() {
    divan::main();
}

/// A single valid progress computation.
#[divan::bench]
fn valid_progress() -> u32 {
    calculate_completion_percentage(black_box(5), black_box(8)).unwrap_or_default()
}

/// Error path: target sessions is zero.
#[divan::bench]
fn zero_target_error() -> bool {
    calculate_completion_percentage(black_box(5), black_box(0)).is_err()
}

/// Error path: completed sessions exceed the target.
#[divan::bench]
fn completed_exceeds_target_error() -> bool {
    calculate_completion_percentage(black_box(9), black_box(8)).is_err()
}

/// Batch of mixed inputs (valid, zero target, exceeding target), summing valid results.
#[divan::bench(args = [100, 1_000, 10_000])]
fn batch_mixed_inputs(bencher: Bencher, len: u32) {
    let inputs: Vec<(u32, u32)> = (0..len)
        .map(|i| match i % 10 {
            0 => (i, 0),
            1 => (i + 1, i),
            _ => (i % 97, 100 + i % 53),
        })
        .collect();

    bencher.bench_local(|| {
        black_box(&inputs)
            .iter()
            .filter_map(|&(completed, target)| {
                calculate_completion_percentage(completed, target).ok()
            })
            .sum::<u32>()
    });
}
