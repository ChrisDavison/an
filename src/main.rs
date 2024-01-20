use anyhow::Result;
use glob::glob;
use tagsearch::filter::Filter;

use clap::{Parser, Subcommand};

mod analyse;
#[cfg(feature="links")]
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
    },
    /// ToC of each file
    Toc {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
    },
    #[cfg(feature="links")]
    /// Show broken links
    Links {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        /// Only run on local links
        #[clap(short = 'l', long = "local")]
        local: bool,
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
    },
    /// Show untagged files
    Untagged {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
    },
    /// Search notes, returning matching filenames
    Search {
        /// Words to search for
        #[clap(required = true)]
        query: Vec<String>,
    },
}

fn main() -> Result<()> {
    let opts = Cli::parse();

    match opts.command {
        Command::Complexity { files, n, reverse } => {
            analyse::note_complexity(&files_or_curdir(&files)?, n, reverse)
        }
        Command::Headercount { files, n, reverse } => {
            analyse::note_header_count(&files_or_curdir(&files)?, n, reverse)
        }
        Command::Size { files, n, reverse } => {
            analyse::note_size(&files_or_curdir(&files)?, n, reverse)
        }
        Command::Words { files, n, reverse } => {
            analyse::wordcount(&files_or_curdir(&files)?, n, reverse)
        }
        Command::Time { files, n, reverse } => {
            analyse::reading_time(&files_or_curdir(&files)?, n, reverse)
        }

        Command::Toc { files } => analyse::note_structure(&files_or_curdir(&files)?),
        #[cfg(feature="links")]
        Command::Links { files, local } => links::broken_links(&files_or_curdir(&files)?, local),
        Command::Untagged { files } => tags::display_untagged_files(&files_or_curdir(&files)?),
        Command::Search { query } => search::search(&files_or_curdir(&[])?, &query),
        Command::Tags {
            files,
            keywords,
            not,
        } => {
            let filter = Filter::new(keywords.as_slice(), not.as_slice(), false);
            tags::display_tags_for_each(filter, &files_or_curdir(&files)?)
        }
    }
}

/// If files is empty, return md files under the currend directory
fn files_or_curdir(files: &[String]) -> Result<Vec<String>> {
    let mut out = Vec::new();
    if files.is_empty() {
        for file in glob("**/*.md")? {
            if let Ok(f) = file {
                out.push(f.to_string_lossy().to_string());
            }
        }
    } else {
        for thing in files {
            if thing.ends_with("/") {
                for file in glob(&format!("{thing}**/*.md"))? {
                    if let Ok(f) = file {
                        out.push(f.to_string_lossy().to_string());
                    }
                }
            } else {
                out.push(thing.to_string());
            }
        }
    }
    Ok(out)
}
