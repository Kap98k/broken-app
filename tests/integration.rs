use broken_app::{algo, average_positive, leak_buffer, normalize, sum_even, use_after_free};

// ─── sum_even ──────────────────────────────────────────────────────────────

#[test]
fn sum_even_basic() {
    assert_eq!(sum_even(&[1, 2, 3, 4]), 6);
}

#[test]
fn sum_even_empty_slice() {
    assert_eq!(sum_even(&[]), 0);
}

#[test]
fn sum_even_all_odd() {
    assert_eq!(sum_even(&[1, 3, 5, 7]), 0);
}

#[test]
fn sum_even_all_negative_even() {
    assert_eq!(sum_even(&[-2, -4, -6]), -12);
}

// ─── normalize ─────────────────────────────────────────────────────────────

#[test]
fn normalize_trims_spaces() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

/// Регрессионный тест: табуляции и переводы строк должны удаляться
#[test]
fn normalize_removes_tabs_and_newlines() {
    assert_eq!(normalize("a\tb\nc\r\nd"), "abcd");
}

#[test]
fn normalize_empty_string() {
    assert_eq!(normalize(""), "");
}

#[test]
fn normalize_only_whitespace() {
    assert_eq!(normalize(" \t \n "), "");
}

// ─── leak_buffer ───────────────────────────────────────────────────────────

#[test]
fn leak_buffer_basic() {
    assert_eq!(leak_buffer(&[0, 1, 0, 2, 3]), 3);
}

#[test]
fn leak_buffer_all_zeros() {
    assert_eq!(leak_buffer(&[0, 0, 0]), 0);
}

#[test]
fn leak_buffer_empty() {
    assert_eq!(leak_buffer(&[]), 0);
}

// ─── average_positive ──────────────────────────────────────────────────────

#[test]
fn average_positive_mixed() {
    assert!((average_positive(&[-5, 5, 15]) - 10.0).abs() < f64::EPSILON);
}

/// Регрессионный тест: нет положительных — не должно быть паники / деления на 0
#[test]
fn average_positive_no_positives() {
    let result = average_positive(&[-1, -2, -3]);
    assert_eq!(result, 0.0);
}

/// Регрессионный тест: пустой срез
#[test]
fn average_positive_empty_slice() {
    assert_eq!(average_positive(&[]), 0.0);
}

#[test]
fn average_positive_single_value() {
    assert!((average_positive(&[7]) - 7.0).abs() < f64::EPSILON);
}

// ─── use_after_free ────────────────────────────────────────────────────────

/// Регрессионный тест: use_after_free больше не содержит UB
#[test]
fn use_after_free_returns_84() {
    assert_eq!(use_after_free(), 84);
}

// ─── algo ──────────────────────────────────────────────────────────────────

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![5, 1, 2, 3]);
}

#[test]
fn dedup_empty_slice() {
    assert_eq!(algo::slow_dedup(&[]), Vec::<u64>::new());
}

#[test]
fn dedup_all_same() {
    assert_eq!(algo::slow_dedup(&[7, 7, 7, 7]), vec![7]);
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn fib_zero_and_one() {
    assert_eq!(algo::slow_fib(0), 0);
    assert_eq!(algo::slow_fib(1), 1);
}
