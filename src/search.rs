use grep::{matcher::Matcher, regex::RegexMatcher};

use crate::{
    config::Userconfig,
    io::{get_all_tags, get_tags_from_file},
};

// PERF: when searching the whole directory, ideally the file(s) in which the tags are found should
// also be displayed
pub fn search_tags(pattern: String, cfg: &Userconfig, file: Option<String>) -> Option<Vec<String>> {
    let tags = match file {
        None => get_all_tags(cfg),
        Some(file) => get_tags_from_file(cfg, file),
    };

    let regex: RegexMatcher = RegexMatcher::new(&pattern).unwrap();

    tags.map(|taglist| taglist
                .iter()
                .filter(|elem| regex.is_match(elem.as_bytes()).unwrap())
                .map(|elem| elem.to_owned())
                .collect())
}
