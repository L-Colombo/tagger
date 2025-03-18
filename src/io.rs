use std::{
    fs::{File, read_dir},
    io::{self, BufRead},
    process::exit,
};

use crate::{
    config::Userconfig,
    orgtree::{get_tags, has_tags, is_headline},
};

pub fn get_all_tags<'a>(cfg: &'a Userconfig) -> Option<Vec<String>> {
    let org_dir_entries = match read_dir(&cfg.org_directory) {
        Ok(entries) => entries,
        Err(e) => {
            println!("Error {}: cannot access your org directory", e);
            exit(1)
        }
    };

    let files_to_search: Vec<_> = org_dir_entries
        .map_while(Result::ok)
        .filter(|entry| !entry.file_type().unwrap().is_dir())
        .map(|entry| entry.file_name().into_string().unwrap())
        .filter(|entry| {
            if let Some(exclude) = &cfg.exclude_files {
                !exclude.contains(&entry)
            } else {
                true
            }
        })
        .collect();

    let mut tmp: Vec<Option<Vec<String>>> = vec![];

    for file in files_to_search {
        tmp.push(get_tags_from_file(cfg, file));
    }

    let mut tags: Vec<_> = tmp
        .into_iter()
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .flatten()
        .collect();

    tags.sort();
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
        .filter(|line| is_headline(line))
        .filter(|line| has_tags(line))
        .map(|taglist| get_tags(&taglist))
        .filter(|opt| opt.is_some())
        .map(|some| some.unwrap())
        .into_iter()
        .flatten()
        .collect();

    tags.sort();
    tags.dedup();

    match tags.is_empty() {
        true => None,
        false => Some(tags),
    }
}
