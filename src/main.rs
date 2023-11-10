use std::thread;
use std::time::{Duration, Instant};

fn count() {
    let start_time = Instant::now();

    loop {
        thread::sleep(Duration::from_secs(1));

        let elapsed_time = start_time.elapsed();

        let seconds = elapsed_time.as_secs() % 60;
        let minutes = (elapsed_time.as_secs() / 60) % 60;
        let hours = (elapsed_time.as_secs() / 3600) % 24;
        let days = elapsed_time.as_secs() / (3600 * 24);

        println!(
            "{} dias, {}h:{}min:{}s",
            days, hours, minutes, seconds
        );
    }
}

fn main() {
    count();
}
