use anyhow::Result;
use std::path::PathBuf;

pub fn note_size(files: &[String]) -> Result<()> {
    let mut sizes = Vec::new();
    for filename in files {
        let nbytes = std::fs::metadata(filename)?.len();
        sizes.push((nbytes as f64 / 1024_f64, filename));
    }
    sizes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (size, filename) in sizes {
        println!("{:.3}kb {}", size, filename);
    }
    Ok(())
}

pub fn note_complexity(files: &[String]) -> Result<()> {
    let mut complexities = Vec::new();
    for filename in files {
        let mut sum = 0;
        let mut num = 0.000001; // Prevent divide-by-zero
        for header in get_headers(filename.into())? {
            let depth = header.split(' ').next().unwrap().len();
            sum += depth;
            num += 1.0;
        }
        complexities.push((sum as f64 / num as f64, filename));
    }
    complexities.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (complexity, filename) in complexities {
        println!("{:.3} {}", complexity, filename);
    }
    Ok(())
}

pub fn get_headers(filename: PathBuf) -> Result<Vec<String>> {
    let headerchar = match filename.extension().and_then(std::ffi::OsStr::to_str) {
        Some("org") => "*",
        Some("md") => "#",
        _ => unreachable!("Shouldn't be possible as glob only searches for these extensions."),
    };
    let contents = std::fs::read_to_string(filename)?;
    let headers = contents
        .lines()
        .filter(|l| {
            let first = l.split(' ').nth(0).unwrap_or(" ");
            !first.is_empty() && first == headerchar.repeat(first.len())
        })
        .map(|l| l.to_string())
        .collect();
    Ok(headers)
}

pub fn note_header_count(files: &[String]) -> Result<()> {
    let mut counts = Vec::new();
    for filename in files {
        let num = get_headers(filename.into())?.len();
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
        println!("{}", filename);
        for header in get_headers(filename.into())? {
            println!("    {}", header);
        }
    }
    Ok(())
}
