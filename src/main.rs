mod search;

use anyhow::Result;
use clap::Parser;
use search::{search_path, SearchOptions};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ripgrep-lite", about = "Fast multi-threaded recursive regex search")]
struct Cli {
    /// Regex pattern to search for
    pattern: String,

    /// Paths to search (default: current directory)
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,

    /// Case-insensitive search
    #[arg(short = 'i', long)]
    ignore_case: bool,

    /// Number of threads (0 = rayon default)
    #[arg(short = 'j', long, default_value_t = 0)]
    threads: usize,

    /// Follow symbolic links
    #[arg(long)]
    follow: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(cli.threads)
            .build_global()
            .ok();
    }

    let options = SearchOptions {
        ignore_case: cli.ignore_case,
        follow: cli.follow,
    };

    let mut total = 0usize;
    for path in &cli.paths {
        let matches = search_path(path, &cli.pattern, &options)?;
        for m in &matches {
            println!("{}:{}:{}", m.path.display(), m.line_number, m.line);
        }
        total += matches.len();
    }

    if total == 0 {
        std::process::exit(1);
    }
    Ok(())
}
