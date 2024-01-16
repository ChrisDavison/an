use anyhow::Result;
use std::collections::{
BTreeSet as Set,
BTreeMap as Map,
};
use std::path::Path;

pub fn search(files: &[String], query: &[String]) -> Result<()> {
    let filter = NoteFilter::new(query);
    for filename in files {
        let p = std::path::Path::new(&filename);
        let m = filter.matches(p);
        let out = [
            if *m.get("title").unwrap_or(&false) { "T" } else {" "},
            if *m.get("tags").unwrap_or(&false) { "t" } else {" "},
            if *m.get("contents").unwrap_or(&false) { "c" } else {" "},
        ].join("");
        println!(
            "{} {:60}",
            out,
            p.to_string_lossy(),
        );
    }

    Ok(())
}

struct NoteFilter {
    all_words: Set<String>,
    words: Set<String>,
    tags: Set<String>,
}

impl NoteFilter {
    pub fn new(words: &[String]) -> NoteFilter {
        let (tag_words, content_words): (Vec<_>, Vec<_>) =
            words.iter().partition(|w| w.starts_with('@'));
        let tag_set = tag_words.iter().map(|x| x[1..].to_string()).collect();
        let content_word_set: Set<String> = content_words.iter().map(|x| x.to_string()).collect();

        NoteFilter {
            all_words: content_word_set
                .iter()
                .chain(&tag_set)
                .map(|x| x.to_string())
                .collect(),
            words: content_word_set,
            tags: tag_set,
        }
    }

    pub fn matches(&self, path: &Path) -> Map<&str, bool> {
        let mut out = Map::new();
        out.insert("title", self.title_matches(path));
        out.insert("contents", self.contents_match(path));
        out.insert("tags", self.tags_match(path));
        out
    }

    pub fn title_matches(&self, path: &Path) -> bool {
        let stem = path
            .file_stem()
            .expect("Failed to get file stem.")
            .to_string_lossy();
        self.all_words.iter().any(|w| stem.contains(w))
    }

    pub fn contents_match(&self, path: &Path) -> bool {
        let contents = std::fs::read_to_string(path).expect("Failed to read file contents.");
        !self.words.is_empty() && self.words.iter().all(|word| contents.contains(word))
    }

    pub fn tags_match(&self, path: &Path) -> bool {
        let file_tags = tagsearch::utility::get_tags_for_file(path.to_string_lossy().as_ref());
        !self.tags.is_empty() && self.tags.iter().all(|t| file_tags.contains(t))
    }
}
