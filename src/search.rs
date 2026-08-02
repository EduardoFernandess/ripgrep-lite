use anyhow::{Context, Result};
use ignore::WalkBuilder;
use rayon::prelude::*;
use regex::RegexBuilder;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("invalid regex: {0}")]
    Regex(#[from] regex::Error),
}

#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub ignore_case: bool,
    pub follow: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
}

pub fn search_path(root: &Path, pattern: &str, options: &SearchOptions) -> Result<Vec<Match>> {
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(options.ignore_case)
        .build()
        .map_err(SearchError::Regex)?;

    let mut builder = WalkBuilder::new(root);
    builder.follow_links(options.follow);
    builder.hidden(false);

    let files: Vec<PathBuf> = builder
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|e| e.into_path())
        .collect();

    let matches: Vec<Match> = files
        .par_iter()
        .flat_map_iter(|path| search_file(path, &regex).unwrap_or_default())
        .collect();

    Ok(matches)
}

fn search_file(path: &Path, regex: &regex::Regex) -> Result<Vec<Match>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    let mut out = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        if regex.is_match(line) {
            out.push(Match {
                path: path.to_path_buf(),
                line_number: idx + 1,
                line: line.to_string(),
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn finds_matching_lines() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("a.txt");
        let mut f = File::create(&file).unwrap();
        writeln!(f, "hello world").unwrap();
        writeln!(f, "nope").unwrap();
        writeln!(f, "hello rust").unwrap();

        let opts = SearchOptions {
            ignore_case: false,
            follow: false,
        };
        let matches = search_path(dir.path(), "hello", &opts).unwrap();
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].line_number, 1);
        assert_eq!(matches[1].line_number, 3);
    }

    #[test]
    fn case_insensitive() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("b.txt");
        std::fs::write(&file, "HeLLo\n").unwrap();
        let opts = SearchOptions {
            ignore_case: true,
            follow: false,
        };
        let matches = search_path(dir.path(), "hello", &opts).unwrap();
        assert_eq!(matches.len(), 1);
    }
}
