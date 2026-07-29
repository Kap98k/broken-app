use broken_app::{
    algo, average_positive, concurrency, leak_buffer, normalize, sum_even,
};
use std::time::{Duration, Instant};

/// Прогоняет замыкание `warmup` + `iters` раз, возвращает (min, mean, max) в наносекундах.
fn bench(label: &str, warmup: u32, iters: u32, mut f: impl FnMut()) -> (u64, u64, u64) {
    // Прогрев
    for _ in 0..warmup {
        f();
    }
    let mut times = Vec::with_capacity(iters as usize);
    for _ in 0..iters {
        let start = Instant::now();
        f();
        times.push(start.elapsed().as_nanos() as u64);
    }
    times.sort_unstable();
    let min = times[0];
    let max = times[times.len() - 1];
    let mean = times.iter().sum::<u64>() / times.len() as u64;
    println!("{label:<45} min={min:>10} ns  mean={mean:>10} ns  max={max:>10} ns");
    (min, mean, max)
}

fn main() {
    const WARMUP: u32 = 5;
    const ITERS: u32 = 20;

    println!("=== baseline bench  (warmup={WARMUP}, iters={ITERS}) ===\n");

    // ── sum_even ──────────────────────────────────────────────────────────
    let s100: Vec<i64> = (0..100).collect();
    let s50k: Vec<i64> = (0..50_000).collect();
    let s500k: Vec<i64> = (0..500_000).collect();

    bench("sum_even/n=100", WARMUP, ITERS, || { let _ = sum_even(&s100); });
    bench("sum_even/n=50_000", WARMUP, ITERS, || { let _ = sum_even(&s50k); });
    bench("sum_even/n=500_000", WARMUP, ITERS, || { let _ = sum_even(&s500k); });

    println!();

    // ── normalize ─────────────────────────────────────────────────────────
    let short = " Hello World ";
    let medium = "  The quick brown fox \t jumps over \n the lazy dog  ".repeat(10);
    let large = "  The quick brown fox \t jumps over \n the lazy dog  ".repeat(100);
    let ws = " \t\n\r".repeat(250);

    bench("normalize/short", WARMUP, ITERS, || { let _ = normalize(short); });
    bench("normalize/medium_10x", WARMUP, ITERS, || { let _ = normalize(&medium); });
    bench("normalize/large_100x", WARMUP, ITERS, || { let _ = normalize(&large); });
    bench("normalize/whitespace_only", WARMUP, ITERS, || { let _ = normalize(&ws); });

    println!();

    // ── leak_buffer ───────────────────────────────────────────────────────
    let lb_nonzero: Vec<u8> = vec![1u8; 10_000];
    let lb_zero: Vec<u8> = vec![0u8; 10_000];
    let lb_mixed: Vec<u8> = (0..10_000).map(|i| (i % 2) as u8).collect();
    let lb_large: Vec<u8> = vec![1u8; 1_000_000];

    bench("leak_buffer/all_nonzero_10k", WARMUP, ITERS, || { let _ = leak_buffer(&lb_nonzero); });
    bench("leak_buffer/all_zero_10k", WARMUP, ITERS, || { let _ = leak_buffer(&lb_zero); });
    bench("leak_buffer/mixed_10k", WARMUP, ITERS, || { let _ = leak_buffer(&lb_mixed); });
    bench("leak_buffer/all_nonzero_1M", WARMUP, ITERS, || { let _ = leak_buffer(&lb_large); });

    println!();

    // ── average_positive ──────────────────────────────────────────────────
    let ap_mixed: Vec<i64> = (-5000..5000).collect();
    let ap_pos: Vec<i64> = (1..=10_000).collect();
    let ap_neg: Vec<i64> = (-10_000..=-1).collect();
    let ap_large: Vec<i64> = (-50_000..50_000).collect();

    bench("average_positive/mixed_10k", WARMUP, ITERS, || { let _ = average_positive(&ap_mixed); });
    bench("average_positive/all_positive_10k", WARMUP, ITERS, || { let _ = average_positive(&ap_pos); });
    bench("average_positive/all_negative_10k", WARMUP, ITERS, || { let _ = average_positive(&ap_neg); });
    bench("average_positive/mixed_100k", WARMUP, ITERS, || { let _ = average_positive(&ap_large); });

    println!();

    // ── algo::slow_fib ────────────────────────────────────────────────────
    for n in [10u64, 20, 30, 35, 40] {
        bench(&format!("algo/slow_fib/n={n}"), WARMUP, ITERS, || { let _ = algo::slow_fib(n); });
    }

    println!();

    // ── algo::slow_dedup ──────────────────────────────────────────────────
    let sd_unique: Vec<u64> = (0..500).collect();
    let sd_same: Vec<u64> = vec![42; 500];
    let sd_half1k: Vec<u64> = (0..500).flat_map(|n| [n, n]).collect();
    let sd_half100: Vec<u64> = (0..50).flat_map(|n| [n, n]).collect();

    bench("algo/slow_dedup/all_unique_n=500", WARMUP, ITERS, || { let _ = algo::slow_dedup(&sd_unique); });
    bench("algo/slow_dedup/all_same_n=500", WARMUP, ITERS, || { let _ = algo::slow_dedup(&sd_same); });
    bench("algo/slow_dedup/half_dup_n=1000", WARMUP, ITERS, || { let _ = algo::slow_dedup(&sd_half1k); });
    bench("algo/slow_dedup/half_dup_n=100", WARMUP, ITERS, || { let _ = algo::slow_dedup(&sd_half100); });

    println!();

    // ── concurrency::race_increment ───────────────────────────────────────
    for threads in [1usize, 2, 4, 8] {
        bench(
            &format!("concurrency/race_increment/{threads}_threads_10k_iter"),
            WARMUP,
            ITERS,
            || {
                concurrency::reset_counter();
                let _ = concurrency::race_increment(10_000, threads);
            },
        );
    }

    println!();

    // ── concurrency::read_after_sleep ─────────────────────────────────────
    bench("concurrency/read_after_sleep/sleep_10ms", WARMUP, ITERS, || {
        let _ = concurrency::read_after_sleep();
    });

    println!("\n=== done ===");
}