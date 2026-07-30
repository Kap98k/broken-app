use std::vec::Vec;

/// Оптимизированная реализация: O(n) с HashSet вместо O(n²) + сортировка.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    use std::collections::HashSet;
    let mut seen = HashSet::with_capacity(values.len());
    let mut out = Vec::with_capacity(values.len());
    for v in values {
        if seen.insert(*v) {
            out.push(*v);
        }
    }
    out
}

/// Оптимизированная реализация: O(n) итеративный алгоритм вместо O(2ⁿ) рекурсивного.
pub fn slow_fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    b
}
