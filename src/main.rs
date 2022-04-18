use std::io::Result;
use std::sync::mpsc;
use std::thread;

use eminent_spoon::{args::Args, read, stats, write};

fn main() -> Result<()> {
    let (stats_tx, stats_rx) = mpsc::channel();
    let (writer_tx, write_rx) = mpsc::channel();

    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx, writer_tx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    let read_io_result = read_handle.join().unwrap();
    let stats_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_result?;
    write_io_result?;
    Ok(())
}
