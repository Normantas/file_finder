use std::io::Write;

use bstr::ByteVec;
use clap::{Parser, ValueHint};
use ignore::{DirEntry, WalkBuilder};
use wildflower::Pattern;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// The file or directory name which should be searched for.
    /// Supports glob patterns (e.g. "file*.txt", "cat.???").
    #[arg(index = 1)]
    file_name: String,

    /// The directory in which to search for the specified file or directory name.
    #[arg(value_hint(ValueHint::DirPath), default_value_t = String::from("./"), index = 2)]
    directory: String,

    /// How many threads should be used for searching. Setting it to 0 will automatically choose it for you.
    #[arg(short, long, default_value_t = 0)]
    threads: usize,

    /// How deep should the search go.
    #[arg(short = 'd', long)]
    max_depth: Option<usize>,

    /// Should hidden files also be searched?
    #[arg(short = 'i', long, default_value_t = false)]
    include_hidden: bool,
}

fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Make a channel in which entries which match will be sent into for writing to stdout
    let (tx, rx) = crossbeam_channel::unbounded::<DirEntry>();

    let mut stdout = std::io::BufWriter::new(std::io::stdout());
    let stdout_thread = std::thread::spawn(move || {
        for dent in rx {
            stdout
                .write_all(&Vec::from_path_lossy(dent.path()))
                .unwrap();
            stdout.write_all(b"\n").unwrap();
        }
    });

    // Search the files
    let walker = WalkBuilder::new(args.directory)
        .threads(args.threads)
        .ignore(false)
        .hidden(!args.include_hidden)
        .max_depth(args.max_depth)
        .build_parallel();
    let matcher = &Pattern::new(&args.file_name);
    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            use ignore::WalkState::Continue;

            if let Ok(entry) = result {
                if let Some(file_name) = entry.file_name().to_str() {
                    if matcher.matches(file_name) {
                        tx.send(entry).unwrap();
                    }
                }
            }
            Continue
        })
    });

    drop(tx);
    stdout_thread.join().unwrap();
}
