use std::io::Write;

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

    // Search the files
    let walker = WalkDir::new(args.directory);
    let matcher = Pattern::new(args.file_name);

    let walk_dir = walker
        .process_read_dir(move |_depth, _path, _read_dir_state, children| {
        children.retain(|dir_entry_result| {
            dir_entry_result.as_ref().map(|dir_entry| {
                dir_entry.file_name
                    .to_str()
                    .map(|s| matcher.matches(s))
                    .unwrap_or(false)
            }).unwrap_or(false)
        });
    });

    let mut stdout = std::io::BufWriter::new(std::io::stdout());
    for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
        stdout
            .write_all(&Vec::from_path_lossy(&entry.path()))
            .unwrap();
        stdout.write_all(b"\n").unwrap();
    }
}
