use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;

pub fn search(files: &[String], query: &[String]) -> Result<()> {
    let mut matches: Vec<(String, usize)> = Vec::new();
    for filename in files {
        let words = word_counter(&filename)?;
        let scores: Vec<_> = query
            .iter()
            .map(|q| *words.get(q).unwrap_or(&0))
            .filter(|v| v != &0)
            .collect();
        if scores.len() == query.len() {
            let score = scores.iter().sum();
            matches.push((filename.to_string(), score));
        }
    }
    // Sort high to low
    matches.sort_by(|a, b| b.1.cmp(&a.1));
    for (name, score) in matches {
        println!("{:>3} {}", score, name);
    }
    Ok(())
}

fn word_counter(filename: &str) -> Result<HashMap<String, usize>> {
    let mut wordcount = HashMap::new();
    let f = std::fs::File::open(&filename)?;
    let buf = std::io::BufReader::new(f);
    for word in buf.split(b' ') {
        if let Ok(word) = word {
            let s = String::from_utf8_lossy(&word).to_string();
            let e = wordcount.entry(s).or_insert(0);
            *e += 1;
        }
    }
    Ok(wordcount)
}
