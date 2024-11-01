use anyhow::Result;
use glob::glob;
use tagsearch::filter::Filter;

use clap::{Parser, Subcommand};

mod analyse;
mod links;
mod search;
mod tags;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Complexity of the header structure
    Complexity {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        /// Show only top n items
        #[clap(short, long)]
        n: Option<usize>,
        /// Show in descending, not ascending order
        #[clap(short, long)]
        reverse: bool,
        #[clap(short, long)]
        exclude: Vec<String>,
    },
    /// How many headers
    Headercount {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        /// Show only top n items
        #[clap(short, long)]
        n: Option<usize>,
        /// Show in descending, not ascending order
        #[clap(short, long)]
        reverse: bool,
        #[clap(short, long)]
        exclude: Vec<String>,
    },
    /// Filesize in bytes
    #[clap(alias = "bytes")]
    Size {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        /// Show only top n items
        #[clap(short, long)]
        n: Option<usize>,
        /// Show in descending, not ascending order
        #[clap(short, long)]
        reverse: bool,
        #[clap(short, long)]
        exclude: Vec<String>,
    },
    /// Wordcount
    #[clap(aliases = &["w", "wc"])]
    Words {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        /// Show only top n items
        #[clap(short, long)]
        n: Option<usize>,
        /// Show in descending, not ascending order
        #[clap(short, long)]
        reverse: bool,
        #[clap(short, long)]
        exclude: Vec<String>,
    },
    /// Reading time (estimate)
    Time {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        /// Show only top n items
        #[clap(short, long)]
        n: Option<usize>,
        /// Show in descending, not ascending order
        #[clap(short, long)]
        reverse: bool,
        #[clap(short, long)]
        exclude: Vec<String>,
    },
    /// ToC of each file
    Toc {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        #[clap(short, long)]
        exclude: Vec<String>,
    },
    /// Show broken links
    Links {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        /// Only run on local links
        #[clap(short = 'l', long = "local")]
        local: bool,
        #[clap(short, long)]
        exclude: Vec<String>,
    },
    /// Show tags for each file
    Tags {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        /// Tags that the file must have
        #[clap(short = 't')]
        keywords: Vec<String>,
        /// Tags that the file must NOT have
        #[clap(short = 'n')]
        not: Vec<String>,
        #[clap(short, long)]
        exclude: Vec<String>,
    },
    /// Show untagged files
    Untagged {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        #[clap(short, long)]
        exclude: Vec<String>,
    },
    /// Search notes, returning matching filenames
    Search {
        /// Words to search for
        #[clap(required = true)]
        query: Vec<String>,
        #[clap(short, long)]
        exclude: Vec<String>,
    },
}

fn main() -> Result<()> {
    let opts = Cli::parse();

    match opts.command {
        Command::Complexity {
            files,
            n,
            reverse,
            exclude,
        } => analyse::note_complexity(&files_or_curdir(&files, &exclude)?, n, reverse),
        Command::Headercount {
            files,
            n,
            reverse,
            exclude,
        } => analyse::note_header_count(&files_or_curdir(&files, &exclude)?, n, reverse),
        Command::Size {
            files,
            n,
            reverse,
            exclude,
        } => analyse::note_size(&files_or_curdir(&files, &exclude)?, n, reverse),
        Command::Words {
            files,
            n,
            reverse,
            exclude,
        } => {
            let files_filtered: Vec<_> = files_or_curdir(&files, &exclude)?
                .iter()
                .filter(|x| !exclude.iter().any(|e| x.contains(e)))
                .cloned()
                .collect();
            analyse::wordcount(&files_filtered, n, reverse)
        }
        Command::Time {
            files,
            n,
            reverse,
            exclude,
        } => analyse::reading_time(&files_or_curdir(&files, &exclude)?, n, reverse),

        Command::Toc { files, exclude } => {
            analyse::note_structure(&files_or_curdir(&files, &exclude)?)
        }
        Command::Links {
            files,
            local,
            exclude,
        } => links::broken_links(&files_or_curdir(&files, &exclude)?, local),
        Command::Untagged { files, exclude } => {
            tags::display_untagged_files(&files_or_curdir(&files, &exclude)?)
        }
        Command::Search { query, exclude } => {
            search::search(&files_or_curdir(&[], &exclude)?, &query)
        }
        Command::Tags {
            files,
            keywords,
            not,
            exclude,
        } => {
            let filter = Filter::new(keywords.as_slice(), not.as_slice(), false);
            tags::display_tags_for_each(filter, &files_or_curdir(&files, &exclude)?)
        }
    }
}

/// If files is empty, return md files under the currend directory
fn files_or_curdir(files: &[String], exclude: &[String]) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let mut files = files.to_vec();
    if files.is_empty() {
        files.push("./".into());
    }
    for thing in files {
        if thing.ends_with('/') {
            for file in glob(&format!("{thing}**/*.md"))?.flatten() {
                out.push(file.to_string_lossy().to_string());
            }
        } else {
            out.push(thing.to_string());
        }
    }
    out = out
        .iter()
        .filter(|f| !exclude.iter().any(|e| f.contains(e)))
        .cloned()
        .collect::<Vec<_>>();
    Ok(out)
}
