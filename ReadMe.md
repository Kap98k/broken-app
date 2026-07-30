# broken-app

проект на Rust, демонстрирующий поиск и исправление ошибок (UB, гонки данных, утечки памяти) с помощью инструментов **Miri**, **Valgrind** и **criterion**. 

## Структура проекта

```
broken-app/
├── src/
│   ├── lib.rs              # Основные утилиты (sum_even, normalize, leak_buffer, average_positive, use_after_free)
│   ├── algo.rs             # Алгоритмы: slow_fib (Фибоначчи), slow_dedup (дедупликация)
│   ├── concurrency.rs      # Многопоточность: AtomicU64 счётчик, race_increment, read_after_sleep
│   └── bin/
│       └── demo.rs         # Демо-бинарник, вызывающий все функции
├── tests/
│   └── integration.rs      # Интеграционные тесты (включая регрессионные)
├── benches/
│   ├── baseline.rs         # Ручные бенчмарки (Instant-based)
│   └── criterion.rs        # Бенчмарки на criterion (статистический анализ)
├── artifacts/
│   ├── bench_output.txt    # Лог последнего прогона бенчмарков
│   ├── baseline/           # Эталонные CSV для сравнения
│   ├── compare/            # Результаты сравнения (CSV + графики)
│   └── plots/              # Сгенерированные графики (*.png)
├── miri_broken.html        # Отчёт Miri до исправлений
├── miri_finish.html        # Отчёт Miri после исправлений
├── valgrind_broken.html    # Отчёт Valgrind до исправлений
├── valgrind_finish.html    # Отчёт Valgrind после исправлений
├── rust-toolchain.toml     # Канал: nightly
├── Cargo.toml
└── .gitignore
```

## Зависимости

- **Rust nightly** (задано в `rust-toolchain.toml`)
- **criterion** `0.5` — статистические бенчмарки
- **Python 3** + `matplotlib` — для скриптов визуализации
- **Miri** — `rustup +nightly component add miri`
- **Valgrind** (Linux) — для анализа памяти

## Быстрый старт

```bash
# Сборка и запуск демо
cargo run --bin demo

# Запуск тестов
cargo test

# Запуск criterion-бенчмарков
cargo bench --bench criterion

# Запуск ручных бенчмарков
cargo bench --bench baseline

# Miri: проверка на undefined behavior
cargo +nightly miri test
cargo +nightly miri run --bin demo

```

## Описание модулей

### `lib.rs`

| Функция | Описание |
|---|---|
| `sum_even(&[i64]) -> i64` | Сумма чётных элементов |
| `normalize(&str) -> String` | Удаление пробельных символов + lowercasing |
| `leak_buffer(&[u8]) -> usize` | Подсчёт ненулевых байт |
| `average_positive(&[i64]) -> f64` | Среднее арифметическое положительных значений (один проход, без аллокаций) |
| `use_after_free() -> i32` | Корректная передача владения через `Box` (изначально содержала UB) |

### `algo.rs`

| Функция | Описание | Сложность |
|---|---|---|
| `slow_fib(u64) -> u64` | Числа Фибоначчи | O(n) итеративный (было O(2ⁿ) рекурсивный) |
| `slow_dedup(&[u64]) -> Vec<u64>` | Дедупликация с сохранением порядка | O(n) с HashSet (было O(n²) + сортировка) |

### `concurrency.rs`

| Функция | Описание |
|---|---|
| `race_increment(iterations, threads) -> u64` | Многопоточный инкремент через `AtomicU64` (Relaxed ordering) |
| `read_after_sleep() -> u64` | Чтение счётчика после паузы 10ms |
| `reset_counter()` | Сброс глобального счётчика |

## Инструменты анализа

| Инструмент | Назначение | Отчёты |
|---|---|---|
| **Miri** | Обнаружение UB в safe Rust (use-after-free, гонки данных) | `miri_broken.html` → `miri_finish.html` |
| **Valgrind** | Анализ памяти (утечки, неинициализированные чтения) | `valgrind_broken.html` → `valgrind_finish.html` |
| **criterion** | Статистические бенчмарки с регрессионным анализом | `target/criterion/report/index.html` |