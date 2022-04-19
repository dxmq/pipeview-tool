//! That stats module contains stats loop
//!
//! # Some Header

mod timer;

use std::io::{self, Result, Stderr, Write};
use std::time::{Duration, Instant};

use crate::stats::timer::Timer;
use crossbeam::channel::Receiver;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut timer = Timer::new();
    let start = Instant::now();
    let mut total_bytes = 0;
    let mut stderr = io::stderr();
    loop {
        let nums_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = nums_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += nums_bytes;
        if !silent && timer.ready {
            timer.ready = false;
            output_process(
                &mut stderr,
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second,
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

fn output_process(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{} ", bytes)).with(Color::Red);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(" [{:.0}b/s]", rate)).with(Color::Blue);

    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );

    let _ = stderr.flush();
}

/// The TimeOut add for `.as_time()` method to `u64`
/// # Example
trait TimeOut {
    fn as_time(&self) -> String;
}

impl TimeOut for u64 {
    fn as_time(&self) -> String {
        let (hour, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hour, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {

    use super::TimeOut;

    #[test]
    fn as_time_format() {
        let pair = vec![(5_u64, "0:00:05")];
        let pair = vec![(60_u64, "0:01:00")];

        for (input, output) in pair {
            assert_eq!(input.as_time().as_str(), output);
        }
    }
}
