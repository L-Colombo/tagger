use crate::{
    config::Userconfig,
    orgtree::{get_tags, has_tags, is_headline},
};
use minus::{MinusError, Pager, page_all};
use rayon::prelude::*;
use std::fmt::Write;
use std::{
    fs::File,
    io::{self, BufRead},
    process::exit,
};

pub fn get_all_tags(cfg: &Userconfig) -> Option<Vec<String>> {
    let files_to_search: Vec<String> = cfg.get_files_to_search();

    let mut tmp: Vec<Option<Vec<String>>> = vec![];

    for file in files_to_search {
        tmp.push(get_tags_from_file(cfg, file));
    }

    let mut tags: Vec<_> = tmp.into_par_iter().flatten().flatten().collect();

    tags.sort_by_key(|a| a.to_lowercase());
    tags.dedup();

    match tags.is_empty() {
        true => None,
        false => Some(tags),
    }
}

pub fn get_tags_from_file(cfg: &Userconfig, file_name: String) -> Option<Vec<String>> {
    let file = match File::open(format!("{}/{}", cfg.org_directory, file_name)) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error {e}: file named `{file_name}` does not exist in your org directory");
            exit(1)
        }
    };

    let file_lines = io::BufReader::new(file).lines();

    let mut tags: Vec<_> = file_lines
        .map_while(Result::ok)
        .filter(is_headline)
        .filter(has_tags)
        .filter_map(|taglist| get_tags(&taglist))
        .flatten()
        .collect();

    tags.sort_by_key(|a| a.to_lowercase());
    tags.dedup();

    match tags.is_empty() {
        true => None,
        false => Some(tags),
    }
}

pub fn print_tags_to_stdout_or_pager(
    taglist: Vec<String>,
    force_pager: bool,
) -> Result<(), MinusError> {
    let mut output = Pager::new();

    if force_pager {
        let _ = Pager::set_run_no_overflow(&output, true);
    }

    for tag in taglist {
        writeln!(output, "{tag}")?;
    }

    page_all(output)
}
