use anyhow::Result;
use rayon::prelude::*;
use tagsearch::filter::Filter;
use tagsearch::utility::get_tags_for_file;

pub fn display_tags_for_each(filter: Filter, files: &[String]) -> Result<()> {
    files.par_iter().for_each(|filename| {
        let tags = get_tags_for_file(filename);
        if tags.is_empty() || !filter.matches(&tags) {
            return;
        }
        println!(
            "{:40} {}",
            filename,
            tags.iter()
                .map(|x| format!("@{}", x))
                .collect::<Vec<String>>()
                .join(", ")
        );
    });
    Ok(())
}

pub fn display_untagged_files(files: &[String]) -> Result<()> {
    files.par_iter().for_each(|filename| {
        if !get_tags_for_file(filename).is_empty() {
            return;
        }
        println!("{}", filename,);
    });
    Ok(())
}
