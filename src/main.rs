use anyhow::Result;
use glob::glob;
use structopt::clap::AppSettings;
use structopt::StructOpt;
use tagsearch::filter::Filter;

#[derive(StructOpt, Debug)]
#[structopt(name="an", setting=AppSettings::InferSubcommands)]
enum Command {
    /// Complexity of the header structure
    Complexity {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
    },
    /// How many headers
    Headercount {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
    },
    /// Filesize in bytes
    #[structopt(alias = "bytes")]
    Size {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
    },
    /// ToC of each file
    #[structopt(alias = "toc")]
    Structure {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
    },
    /// Show broken links
    Links {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        /// Only run on local links
        #[structopt(short = "l", long = "local")]
        local: bool,
    },
    /// Show tags for each file
    Tags {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
        /// Tags that the file must have
        #[structopt(short = "t")]
        keywords: Vec<String>,
        /// Tags that the file must NOT have
        #[structopt(short = "n")]
        not: Vec<String>,
    },
    /// Show untagged files
    Untagged {
        /// Which files to operate on, or all under cwd
        files: Vec<String>,
    },
}

fn main() -> Result<()> {
    let opts = Command::from_args();

    match opts {
        Command::Complexity { files } => cmd::note_complexity(&files_or_curdir(&files)?),
        Command::Headercount { files } => cmd::note_header_count(&files_or_curdir(&files)?),
        Command::Size { files } => cmd::note_size(&files_or_curdir(&files)?),
        Command::Structure { files } => cmd::note_structure(&files_or_curdir(&files)?),
        Command::Links { files, local } => cmd::broken_links(&files_or_curdir(&files)?, local),
        Command::Untagged { files } => cmd::display_untagged_files(&files_or_curdir(&files)?),
        Command::Tags {
            files,
            keywords,
            not,
        } => {
            let filter = Filter::new(keywords.as_slice(), not.as_slice(), false);
            cmd::display_tags_for_each(filter, &files_or_curdir(&files)?)
        }
    }
}

/// If files is empty, return md files under the currend directory
fn files_or_curdir(files: &[String]) -> Result<Vec<String>> {
    if files.is_empty() {
        Ok(glob("*.md")?
            .filter(|x| x.is_ok())
            .map(|x| {
                x.expect("Already tested each glob is ok")
                    .to_string_lossy()
                    .to_string()
            })
            .collect())
    } else {
        Ok(files.to_vec())
    }
}

mod cmd {
    use anyhow::Result;
    use mdlc::links;
    use std::path::PathBuf;
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
        let contents = std::fs::read_to_string(filename)?;
        let headers = contents
            .lines()
            .filter(|l| l.starts_with('#'))
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

    pub fn broken_links(files: &[String], local_only: bool) -> Result<()> {
        for filename in files {
            let mut broken = Vec::new();
            for link in links::from_file(&filename) {
                if local_only && !(link.linktype == links::LinkType::Local) {
                    continue;
                }
                if !link.is_alive() {
                    broken.push(link);
                }
            }
            if !broken.is_empty() {
                println!("{}", filename);
                for link in broken {
                    if local_only {
                        println!("> {}", link.text);
                    } else {
                        println!("> {:?} {}", link.linktype, link.text);
                    }
                }
                println!();
            }
        }
        Ok(())
    }
}
