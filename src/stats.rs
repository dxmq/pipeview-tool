use std::io::Result;
use std::time::{Duration, Instant};

use crossbeam::channel::Receiver;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut timer = Timer::new();
    let start = Instant::now();
    let mut total_bytes = 0;
    loop {
        let nums_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = nums_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += nums_bytes;
        if !silent && timer.ready {
            timer.ready = false;
            eprint!(
                "\r{} {} [{:.0}b/s]",
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second
            );
        }
        if nums_bytes == 0 {
            break;
        }
    }
    if !silent {
        eprintln!();
    }
    Ok(())
}

trait TimeOut {
    fn as_time(&self) -> String;
}

impl TimeOut for u64 {
    fn as_time(&self) -> String {
        let (hour, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{:02}:{:02}:{:02}", hour, minutes, seconds)
    }
}

struct Timer {
    last_instant: Instant,
    delta: Duration,
    period: Duration,
    countdown: Duration,
    ready: bool,
}

impl Timer {
    fn new() -> Self {
        let now = Instant::now();
        Timer {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}
