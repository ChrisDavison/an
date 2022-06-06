use anyhow::Result;
use mdlc::links;
use rayon::prelude::*;

pub fn broken_links(files: &[String], local_only: bool) -> Result<()> {
    files.par_iter().for_each(|filename| {
        let mut broken = Vec::new();
        for link in links::from_file(filename) {
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
                println!("> {}", link.text);
                // if local_only {
                //     println!("> {}", link.text);
                // } else {
                // }
            }
            println!();
        }
    });
    Ok(())
}
