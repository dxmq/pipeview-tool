use std::io::Result;
use eminent_spoon::{args::Args, read, stats, write};


fn main() -> Result<()> {
    let mut total_bytes = 0;
    let args = Args::parse();
    loop {
        let buffer = match read::read(&args.infile) {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };
        stats::stats(args.silent, buffer.len(), &mut total_bytes, false);
        if !write::write(&args.outfile, &buffer)? {
            break;
        }
    }
    stats::stats(args.silent, 0, &mut total_bytes, true);
    Ok(())
}
