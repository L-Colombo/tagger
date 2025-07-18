use crate::{
    config::Userconfig,
    io::{get_all_tags, get_tags_from_file},
};
use grep::{matcher::Matcher, regex::RegexMatcher};
use rayon::prelude::*;
use std::process::exit;

pub fn search_tags(pattern: String, cfg: &Userconfig, file: Option<String>) -> Option<Vec<String>> {
    let tags = match file {
        None => get_all_tags(cfg),
        Some(file) => get_tags_from_file(cfg, file),
    };

    let regex: RegexMatcher = match RegexMatcher::new(&pattern) {
        Ok(matcher) => matcher,
        Err(_) => {
            eprintln!("The provided pattern is possibly malformed");
            exit(1)
        }
    };

    tags.map(|taglist| {
        taglist
            .par_iter()
            .filter(|elem| match regex.is_match(elem.as_bytes()) {
                Ok(match_result) => match_result,
                Err(e) => {
                    eprintln!("{e}");
                    exit(1)
                }
            })
            .map(|elem| elem.to_owned())
            .collect()
    })
}
