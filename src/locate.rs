use crate::config::Userconfig;
use grep::{
    regex::RegexMatcher,
    searcher::{Searcher, sinks::UTF8},
};
use std::{fs::File, process::exit};

pub fn locate(pattern: String, cfg: Userconfig, strict: bool) -> Vec<String> {
    let raw_pattern = match strict {
        false => format!(r":\w*{pattern}\w*:"),
        true => format!(r":{pattern}:"),
    };

    let files_to_search: Vec<String> = cfg.get_files_to_search();

    let mut files: Vec<String> = Vec::new();

    for file in files_to_search {
        let fname = match File::open(format!("{}{}", cfg.org_directory, &file)) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{}: Could not open file named {}", e, file);
                exit(1)
            }
        };

        let matcher = match RegexMatcher::new(&raw_pattern) {
            Ok(matcher) => matcher,
            Err(_) => {
                eprintln!("Your regex pattern is possibly malformed");
                exit(1)
            }
        };

        let mut matches: Vec<usize> = vec![];

        let _ = Searcher::new().search_reader(
            &matcher,
            fname,
            UTF8(|linenr, _| {
                matches.push(linenr as usize);
                Ok(true)
            }),
        );

        if !matches.is_empty() {
            files.push(file);
        }
    }

    files.sort();
    files
}
