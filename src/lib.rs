
pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|v| **v % 2 == 0).sum()
}

pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|b| **b != 0).count()
}

/// Нормализация строки: удаляем все пробельные символы (включая пробелы, табуляции,
/// новые строки) и приводим к нижнему регистру.
pub fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
}

/// Усредняет только положительные значения.
/// Возвращает 0.0 если положительных значений нет или срез пуст.
/// Оптимизация: один проход без промежуточной аллокации Vec.
pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values
        .iter()
        .filter(|x| x.is_positive())
        .fold((0i64, 0usize), |(s, c), &x| (s + x, c + 1));
    if count == 0 {
        0.0
    } else {
        sum as f64 / count as f64
    }
}

/// Корректная передача владения через Box.
pub fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    *b + 42
}
