pub mod grid;
use std::time::Instant;

pub fn measure<T, F>(f: F) -> (T, u128)
where
    F: FnOnce() -> T,
{
    let now = Instant::now();
    let result = f();
    let elapsed = now.elapsed().as_micros();

    (result, elapsed)
}
