use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn note_size(files: &[String]) -> Result<()> {
    let mut sizes = Vec::new();
    for filename in files {
        let nbytes = std::fs::metadata(filename)?.len();
        sizes.push((nbytes as f64 / 1024_f64, filename));
    }
    sizes.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .expect("Failed to compare size. Should be impossible.")
    });
    for (size, filename) in sizes {
        println!("{:.3}kb {}", size, filename);
    }
    Ok(())
}

pub fn note_complexity(files: &[String]) -> Result<()> {
    let mut complexities = Vec::new();
    for filename in files {
        let headers = get_headers(filename)?;
        let sum: usize = headers.iter().map(|(_h, d)| d).sum();
        let num = (headers.len() as f32) + 0.000000001;
        complexities.push(((sum as f32 / num), filename));
    }
    complexities.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .expect("Failed to compare complexities. Should be impossible.")
    });
    for (complexity, filename) in complexities {
        println!("{:.3} {}", complexity, filename);
    }
    Ok(())
}

fn header_char(filename: &Path) -> &str {
    match filename.extension().and_then(std::ffi::OsStr::to_str) {
        Some("org") => "*",
        Some("md") => "#",
        _ => unreachable!("Shouldn't be possible as glob only searches for these extensions."),
    }
}

pub fn get_headers(filename: impl Into<PathBuf>) -> Result<Vec<(String, usize)>> {
    let path = filename.into();
    let headerchar = header_char(&path);
    let contents = std::fs::read_to_string(&path)?;
    let headers = contents
        .lines()
        .filter(|l| {
            let first = l.split(' ').next().unwrap_or(" ");
            !first.is_empty() && first == headerchar.repeat(first.len())
        })
        .map(|l| {
            (
                l.trim_start_matches(headerchar).trim().to_string(),
                l.split(' ').next().unwrap_or("").len(),
            )
        })
        .collect();
    Ok(headers)
}

pub fn note_header_count(files: &[String]) -> Result<()> {
    let mut counts = Vec::new();
    for filename in files {
        let num = get_headers(filename)?.len();
        counts.push((num, filename));
    }
    counts.sort_by_key(|&(n, _)| n);
    for (num, filename) in counts {
        println!("{} {}", num, filename);
    }
    Ok(())
}

pub fn note_structure(files: &[String]) -> Result<()> {
    for filename in files {
        let mut all_headers = get_headers(filename)?;
        all_headers.push((String::new(), 0)); // hack to allow zipping without skipping last header
        println!("{}", filename);
        for ((header, depth), (_, depth2)) in all_headers.iter().zip(all_headers.iter().skip(1)) {
            let marker = if depth2 == depth { "├" } else { "└" };
            let indent = " ".repeat(depth * 2);
            println!("{}{} H{}: {}", indent, marker, depth, header);
        }
    }
    Ok(())
}
