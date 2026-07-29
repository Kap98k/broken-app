use broken_app::{
    algo, average_positive, concurrency, leak_buffer, normalize, sum_even,
};
use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup, Criterion,
    SamplingMode, Throughput,
};
use std::time::Duration;

// ═══════════════════════════════════════════════════════════════════════════════
//  Конфигурация групп
// ═══════════════════════════════════════════════════════════════════════════════

fn configure_group(group: &mut BenchmarkGroup<'_, WallTime>, name: &str) {
    group.sampling_mode(SamplingMode::Auto);
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));
    group.noise_threshold(0.02);
    group.significance_level(0.05);
    group.confidence_level(0.95);
    println!("→ Running group: {name}");
}

// ═══════════════════════════════════════════════════════════════════════════════
//  lib.rs — sum_even
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_sum_even(c: &mut Criterion) {
    let mut group = c.benchmark_group("lib/sum_even");
    configure_group(&mut group, "lib/sum_even");

    let small: Vec<i64> = (0..100).collect();
    group.throughput(Throughput::Elements(small.len() as u64));
    group.bench_function("n=100", |b| b.iter(|| sum_even(&small)));

    let medium: Vec<i64> = (0..50_000).collect();
    group.throughput(Throughput::Elements(medium.len() as u64));
    group.bench_function("n=50_000", |b| b.iter(|| sum_even(&medium)));

    let large: Vec<i64> = (0..500_000).collect();
    group.throughput(Throughput::Elements(large.len() as u64));
    group.bench_function("n=500_000", |b| b.iter(|| sum_even(&large)));

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
//  lib.rs — normalize
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_normalize(c: &mut Criterion) {
    let mut group = c.benchmark_group("lib/normalize");
    configure_group(&mut group, "lib/normalize");

    let short = " Hello World ";
    group.throughput(Throughput::Bytes(short.len() as u64));
    group.bench_function("short", |b| b.iter(|| normalize(short)));

    let medium = "  The quick brown fox \t jumps over \n the lazy dog  ".repeat(10);
    group.throughput(Throughput::Bytes(medium.len() as u64));
    group.bench_function("medium_10x", |b| b.iter(|| normalize(&medium)));

    let large = "  The quick brown fox \t jumps over \n the lazy dog  ".repeat(100);
    group.throughput(Throughput::Bytes(large.len() as u64));
    group.bench_function("large_100x", |b| b.iter(|| normalize(&large)));

    let whitespace_only = " \t\n\r".repeat(250);
    group.throughput(Throughput::Bytes(whitespace_only.len() as u64));
    group.bench_function("whitespace_only", |b| b.iter(|| normalize(&whitespace_only)));

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
//  lib.rs — leak_buffer
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_leak_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("lib/leak_buffer");
    configure_group(&mut group, "lib/leak_buffer");

    let all_nonzero: Vec<u8> = vec![1u8; 10_000];
    group.throughput(Throughput::Bytes(all_nonzero.len() as u64));
    group.bench_function("all_nonzero_10k", |b| b.iter(|| leak_buffer(&all_nonzero)));

    let all_zero: Vec<u8> = vec![0u8; 10_000];
    group.throughput(Throughput::Bytes(all_zero.len() as u64));
    group.bench_function("all_zero_10k", |b| b.iter(|| leak_buffer(&all_zero)));

    let mixed: Vec<u8> = (0..10_000).map(|i| (i % 2) as u8).collect();
    group.throughput(Throughput::Bytes(mixed.len() as u64));
    group.bench_function("mixed_10k", |b| b.iter(|| leak_buffer(&mixed)));

    let large: Vec<u8> = vec![1u8; 1_000_000];
    group.throughput(Throughput::Bytes(large.len() as u64));
    group.bench_function("all_nonzero_1M", |b| b.iter(|| leak_buffer(&large)));

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
//  lib.rs — average_positive
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_average_positive(c: &mut Criterion) {
    let mut group = c.benchmark_group("lib/average_positive");
    configure_group(&mut group, "lib/average_positive");

    let mixed: Vec<i64> = (-5000..5000).collect();
    group.throughput(Throughput::Elements(mixed.len() as u64));
    group.bench_function("mixed_10k", |b| b.iter(|| average_positive(&mixed)));

    let all_pos: Vec<i64> = (1..=10_000).collect();
    group.throughput(Throughput::Elements(all_pos.len() as u64));
    group.bench_function("all_positive_10k", |b| b.iter(|| average_positive(&all_pos)));

    let all_neg: Vec<i64> = (-10_000..=-1).collect();
    group.throughput(Throughput::Elements(all_neg.len() as u64));
    group.bench_function("all_negative_10k", |b| b.iter(|| average_positive(&all_neg)));

    let large: Vec<i64> = (-50_000..50_000).collect();
    group.throughput(Throughput::Elements(large.len() as u64));
    group.bench_function("mixed_100k", |b| b.iter(|| average_positive(&large)));

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
//  algo.rs — slow_fib
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_slow_fib(c: &mut Criterion) {
    let mut group = c.benchmark_group("algo/slow_fib");
    configure_group(&mut group, "algo/slow_fib");

    for n in [10, 20, 30, 35, 40] {
        group.bench_function(format!("n={n}"), |b| b.iter(|| algo::slow_fib(n)));
    }

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
//  algo.rs — slow_dedup
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_slow_dedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("algo/slow_dedup");
    configure_group(&mut group, "algo/slow_dedup");

    let all_unique: Vec<u64> = (0..500).collect();
    group.throughput(Throughput::Elements(all_unique.len() as u64));
    group.bench_function("all_unique_n=500", |b| {
        b.iter_batched(
            || all_unique.clone(),
            |v| {
                let _ = algo::slow_dedup(&v);
            },
            BatchSize::SmallInput,
        )
    });

    let all_same: Vec<u64> = vec![42; 500];
    group.throughput(Throughput::Elements(all_same.len() as u64));
    group.bench_function("all_same_n=500", |b| {
        b.iter_batched(
            || all_same.clone(),
            |v| {
                let _ = algo::slow_dedup(&v);
            },
            BatchSize::SmallInput,
        )
    });

    let half_dup: Vec<u64> = (0..500).flat_map(|n| [n, n]).collect();
    group.throughput(Throughput::Elements(half_dup.len() as u64));
    group.bench_function("half_dup_n=1000", |b| {
        b.iter_batched(
            || half_dup.clone(),
            |v| {
                let _ = algo::slow_dedup(&v);
            },
            BatchSize::SmallInput,
        )
    });

    let tiny: Vec<u64> = (0..50).flat_map(|n| [n, n]).collect();
    group.throughput(Throughput::Elements(tiny.len() as u64));
    group.bench_function("half_dup_n=100", |b| {
        b.iter_batched(
            || tiny.clone(),
            |v| {
                let _ = algo::slow_dedup(&v);
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
//  concurrency.rs — race_increment
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_race_increment(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrency/race_increment");
    configure_group(&mut group, "concurrency/race_increment");

    group.bench_function("1_thread_10k_iter", |b| {
        b.iter(|| {
            concurrency::reset_counter();
            concurrency::race_increment(10_000, 1)
        })
    });

    group.bench_function("2_threads_10k_iter", |b| {
        b.iter(|| {
            concurrency::reset_counter();
            concurrency::race_increment(10_000, 2)
        })
    });

    group.bench_function("4_threads_10k_iter", |b| {
        b.iter(|| {
            concurrency::reset_counter();
            concurrency::race_increment(10_000, 4)
        })
    });

    group.bench_function("8_threads_10k_iter", |b| {
        b.iter(|| {
            concurrency::reset_counter();
            concurrency::race_increment(10_000, 8)
        })
    });

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
//  concurrency.rs — read_after_sleep
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_read_after_sleep(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrency/read_after_sleep");
    configure_group(&mut group, "concurrency/read_after_sleep");

    group.bench_function("sleep_10ms", |b| {
        b.iter(|| concurrency::read_after_sleep())
    });

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
//  Регистрация групп и main
// ═══════════════════════════════════════════════════════════════════════════════

criterion_group!(
    benches,
    bench_sum_even,
    bench_normalize,
    bench_leak_buffer,
    bench_average_positive,
    bench_slow_fib,
    bench_slow_dedup,
    bench_race_increment,
    bench_read_after_sleep,
);
criterion_main!(benches);