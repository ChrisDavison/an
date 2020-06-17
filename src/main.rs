#![allow(dead_code, unused_variables)]
use std::env;
use std::path::PathBuf;

use glob::glob;

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

const VERSION: &str = "0.2.0";

fn print_version() -> Result<()> {
    println!("an v{}", VERSION);
    Ok(())
}

fn print_usage() -> Result<()> {
    println!("Usage: an <command> <files>...

Analyse Notes {}

Commands:
    Complexity    Complexity of the structure
    Headercount   Number of headers
    Size          Filesize in bytes
    Structure     Show ToC of each file
    Help          Display this message", VERSION);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print_usage()?;
        std::process::exit(1); 
    }
    let files = if args[1..].is_empty() { md_files_in_curdir()? } else { args[1..].to_vec() };
    match args[0].to_lowercase().as_str() {
        "complexity" => note_complexity(&files),
        "headercount" => note_header_count(&files),
        "size"|"bytes" => note_size(&files),
        "structure"|"toc" => note_structure(&files),
        "version"|"-v" => print_version(),
        "help"|"-h" => print_usage(),
        _ => {
            println!("Unrecognised command: {}", args[0]); 
            Ok(())
        },
    }
}

fn md_files_in_curdir() -> Result<Vec<String>> {
    Ok(glob("*.md")?
        .filter(|x| x.is_ok())
        .map(|x| x.expect("Already tested each glob is ok").to_string_lossy().to_string())
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
