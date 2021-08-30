use anyhow::Result;
use std::collections::BTreeSet as Set;
use std::path::Path;

pub fn search(files: &[String], query: &[String]) -> Result<()> {
    let filter = NoteFilter::new(query);
    for filename in files {
        let p = std::path::Path::new(&filename);
        let matches = filter.matches(p);
        if matches.is_empty() {
            continue;
        }
        let parts = vec![
            if matches.contains("title") { "T" } else { " " },
            if matches.contains("tags") { "t" } else { " " },
            if matches.contains("contents") {
                "c"
            } else {
                " "
            },
        ]
        .join("");
        println!("{} {:60}", parts, p.to_string_lossy(),);
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
    pub fn matches(&self, path: &Path) -> Set<String> {
        [
            ("title", self.title_matches(path)),
            ("contents", self.contents_match(path)),
            ("tags", self.tags_match(path)),
        ]
        .iter()
        .filter(|(_t, matches)| *matches)
        .map(|(t, _matches)| t.to_string())
        .collect()
    }

    pub fn title_matches(&self, path: &Path) -> bool {
        let stem = path
            .file_stem()
            .expect("Failed to get file stem.")
            .to_string_lossy();
        self.all_words.iter().any(|w| stem.contains(w))
    }

    pub fn contents_match(&self, path: &Path) -> bool {
        let contents = std::fs::read_to_string(&path).expect("Failed to read file contents.");
        !self.words.is_empty() && self.words.iter().all(|word| contents.contains(word))
    }

    pub fn tags_match(&self, path: &Path) -> bool {
        let file_tags = tagsearch::utility::get_tags_for_file(&path.to_string_lossy().to_string());
        !self.tags.is_empty() && self.tags.iter().all(|t| file_tags.contains(t))
    }
}
