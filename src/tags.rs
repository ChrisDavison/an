use anyhow::Result;
use tagsearch::filter::Filter;
use tagsearch::utility::get_tags_for_file;

pub fn display_tags_for_each(filter: Filter, files: &[String]) -> Result<()> {
    for filename in files {
        let tags = get_tags_for_file(filename);
        if tags.is_empty() || !filter.matches(&tags) {
            continue;
        }
        println!(
            "{:40} {}",
            filename,
            tags.iter()
                .map(|x| format!("@{}", x))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
    Ok(())
}

pub fn display_untagged_files(files: &[String]) -> Result<()> {
    for filename in files {
        if !get_tags_for_file(filename).is_empty() {
            continue;
        }
        println!("{}", filename,);
    }
    Ok(())
}
