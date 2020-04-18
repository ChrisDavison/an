#![allow(dead_code, unused_variables)]
use std::path::PathBuf;

use glob::glob;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

/// AnalyseNotes. Various utilities for summarising notes.
#[derive(StructOpt, Debug)]
enum Command {
    /// Calculate complexity of the Stringucture of each file
    Complexity { files: Vec<String> },
    /// Count number of headers in each file
    Headercount { files: Vec<String> },
    /// Show the size of each file
    Size { files: Vec<String> },
    /// Show the Structure of each file
    Structure { files: Vec<String> },
}

fn main() -> Result<()> {
    let command = Command::from_args();
    match command {
        Command::Complexity { files } => note_complexity(&files[..]),
        Command::Headercount { files } => note_header_count(&files[..]),
        Command::Size { files } => note_size(&files[..]),
        Command::Structure { files } => note_structure(&files[..]),
    }
}

fn md_files_in_curdir() -> Result<Vec<PathBuf>> {
    Ok(glob("*.md")?
        .filter(|x| x.is_ok())
        .map(|x| x.expect("Already tested each glob is ok"))
        .collect())
}

fn note_size(files: &[String]) -> Result<()> {
    for filename in files {
        let nbytes = std::fs::metadata(filename)?.len();
        println!("{:.3}kb {}", (nbytes as f64 / 1024 as f64), filename);
    }
    Ok(())
}

fn note_complexity(files: &[String]) -> Result<()> {
    for filename in files {
        let mut sum = 0;
        let mut num = 0;
        for header in get_headers(filename.into())? {
            let depth = header.split(" ").nth(0).unwrap().len();
            sum += depth;
            num += 1;
        }
        println!("{} {}", sum as f64 / num as f64, filename);
    }
    Ok(())
}

fn get_headers(filename: PathBuf) -> Result<Vec<String>> {
    let contents = std::fs::read_to_string(filename)?;
    let headers = contents
        .lines()
        .filter(|l| l.starts_with('#'))
        .map(|l| l.to_string())
        .collect();
    Ok(headers)
}

fn note_header_count(files: &[String]) -> Result<()> {
    for filename in files {
        let num = get_headers(filename.into())?.len();
        println!("{} {}", num, filename);
    }
    Ok(())
}

fn note_structure(files: &[String]) -> Result<()> {
    for filename in files {
        println!("{}", filename);
        for header in get_headers(filename.into())? {
            println!("    {}", header);
        }
    }
    Ok(())
}
