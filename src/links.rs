use regex::Regex;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum LinkType {
    Web,
    Local,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub text: String,
    source: String,
    pub linktype: LinkType,
}

pub fn from_file(filename: &str) -> Vec<Link> {
    lazy_static! {
        static ref RE_WEB: Regex = Regex::new(
            r#"(https+://)*[-a-zA-Z0-9@:%._/\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b[-a-zA-Z0-9@:%_\+.~#?&//=]*"#
        )
            .unwrap();
        static ref RE_LOCAL: Regex = Regex::new(r#"\((?:\.+/)*([a-zA-Z0-9\-_ ]*?\.md)"#).unwrap();
    }
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut links = Vec::new();
    let mut seen = HashSet::new();

    for cap in RE_LOCAL.captures_iter(&contents) {
        let linktext: String = if cap[0].starts_with("(") {
            cap[0][1..].into()
        } else {
            cap[0].into()
        };
        if seen.contains(&linktext) {
            continue;
        } else {
            seen.insert(linktext.clone());
        }
        links.push(Link {
            text: linktext.clone(),
            linktype: LinkType::Local,
            source: filename.to_string(),
        });
    }

    for cap in RE_WEB.captures_iter(&contents) {
        let linktext: String = cap[0].into();
        if seen.contains(&linktext) {
            continue;
        } else {
            seen.insert(linktext.clone());
        }
        links.push(Link {
            text: linktext.clone(),
            linktype: LinkType::Web,
            source: filename.to_string(),
        });
    }
    links
}

impl Link {
    pub fn is_alive(&self) -> bool {
        if self.linktype == LinkType::Local {
            self.is_valid_local_link()
        } else {
            self.is_valid_web_link()
        }
    }

    fn is_valid_local_link(&self) -> bool {
        let source = std::path::PathBuf::from(&self.source);
        let parent = source.parent().unwrap();
        let p = parent.join(&self.text).canonicalize();
        if let Err(_e) = p {
            false
        } else {
            p.unwrap().exists()
        }
    }

    fn is_valid_web_link(&self) -> bool {
        match reqwest::blocking::get(&self.text) {
            Ok(resp) => resp.status() == 200,
            Err(_e) => false,
        }
    }
}
