use grep::{matcher::Matcher, regex::RegexMatcher};

use crate::{
    config::Userconfig,
    io::{get_all_tags, get_tags_from_file},
};

pub fn search_tags(pattern: String, cfg: &Userconfig, file: Option<String>) -> Option<Vec<String>> {
    let tags = match file {
        None => get_all_tags(&cfg),
        Some(file) => get_tags_from_file(&cfg, file),
    };

    match tags {
        None => None,
        Some(taglist) => Some(
            taglist
                .iter()
                .filter(|elem| {
                    RegexMatcher::new(&pattern)
                        .unwrap()
                        .is_match(elem.as_bytes())
                        .unwrap()
                })
                .map(|elem| elem.to_owned())
                .collect(),
        ),
    }
}
