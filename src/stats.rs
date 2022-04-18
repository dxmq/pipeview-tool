use std::io::Result;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub fn stats_loop(
    silent: bool,
    stats_rx: Receiver<Vec<u8>>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        let buffer = stats_rx.recv().unwrap();
        let nums_bytes = buffer.len();
        total_bytes += nums_bytes;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if write_tx.send(buffer).is_err() {
            break;
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
