use std::{
    fs::File,
    io::{self, BufRead},
    process::exit,
};

use crate::{
    config::Userconfig,
    orgtree::{get_tags, has_tags, is_headline},
};
use minus::{MinusError, Pager, page_all};
use std::fmt::Write;

pub fn get_all_tags(cfg: &Userconfig) -> Option<Vec<String>> {
    let files_to_search: Vec<String> = cfg.get_files_to_search();

    let mut tmp: Vec<Option<Vec<String>>> = vec![];

    for file in files_to_search {
        tmp.push(get_tags_from_file(cfg, file));
    }

    let mut tags: Vec<_> = tmp
        .into_iter()
        .flatten()
        .flatten()
        .collect();

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
            eprintln!(
                "Error {}: file named `{}` does not exist in your org directory",
                e, file_name
            );
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

pub fn print_tags_to_stdout_or_pager(taglist: Vec<String>) -> Result<(), MinusError> {
    match taglist.len() <= 20 {
        true => {
            for tag in taglist {
                println!("{tag}")
            }
        }
        false => {
            let mut output = Pager::new();
            for tag in taglist {
                writeln!(output, "{}", tag)?;
            }
            page_all(output)?;
        }
    }

    Ok(())
}
