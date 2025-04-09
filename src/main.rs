use dekrilo::*;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    if let Err(e) = cli::parse() {
        println!("Во время выполнения произошла ошибка:\n  {e}")
    };
    let end = start.elapsed();
    println!("Затрачено времени: {} сек {:03} мс",
        end.as_secs(), end.subsec_millis())
}
