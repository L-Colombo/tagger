use crate::{
    cli::SearchArgs,
    config::Userconfig,
    io::{get_all_tags, get_tags_from_file},
};
use grep::{matcher::Matcher, regex::RegexMatcher};
use rayon::prelude::*;
use std::process::exit;

pub fn search_tags(args: SearchArgs, mut cfg: Userconfig) -> Option<Vec<String>> {
    let tags = match args.file {
        None => get_all_tags(&mut cfg),
        Some(file) => get_tags_from_file(&cfg, file),
    };

    let regex: RegexMatcher = match RegexMatcher::new(&args.pattern) {
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
