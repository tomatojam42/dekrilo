use dekrilo::*;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    cli::parse();
    let end = start.elapsed();
    println!("Затрачено времени: {} сек {:03} мс", end.as_secs(), end.subsec_millis())
}
