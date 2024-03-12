use std::{io::Write, path::PathBuf};

use bstr::ByteVec;
use clap::{Parser, ValueHint};
use jwalk::WalkDir;
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

    let (tx, rx) = crossbeam_channel::unbounded::<PathBuf>();

    let mut stdout = std::io::BufWriter::new(std::io::stdout());
    let stdout_thread = std::thread::spawn(move || {
        for dent in rx {
            stdout
                .write_all(&Vec::from_path_lossy(&dent))
                .unwrap();
            stdout.write_all(b"\n").unwrap();
        }
    });

    // Walk the directories
    let walker = WalkDir::new(args.directory)
        .skip_hidden(!args.include_hidden)
        .follow_links(false)
        .max_depth(args.max_depth.unwrap_or(usize::MAX));
    let matcher = Pattern::new(args.file_name);

    for entry in walker
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| matcher.matches(&e.file_name.to_string_lossy()))
    {
        tx.send(entry.path()).unwrap();
    }

    drop(tx);
    stdout_thread.join().unwrap();
}
