use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Потокобезопасный инкремент через несколько потоков.
pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    COUNTER.store(0, Ordering::SeqCst);
    let mut handles = Vec::new();
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    COUNTER.load(Ordering::SeqCst)
}

/// Читает текущее значение счётчика после короткой паузы.
pub fn read_after_sleep() -> u64 {
    thread::sleep(Duration::from_millis(10));
    COUNTER.load(Ordering::SeqCst)
}

/// Сброс счётчика.
pub fn reset_counter() {
    COUNTER.store(0, Ordering::SeqCst);
}
