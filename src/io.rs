use std::{
    fs::File,
    io::{self, BufRead},
    process::exit,
};

use crate::{
    config::Userconfig,
    orgtree::{get_tags, has_tags, is_headline},
};

pub fn get_all_tags(_cfg: Userconfig) -> Option<Vec<String>> {
    todo!()
}

pub fn get_tags_from_file(cfg: Userconfig, file_name: String) -> Option<Vec<String>> {
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
        .flat_map(|inner_vec| inner_vec)
        .collect();

    tags.sort();
    tags.dedup();

    match tags.is_empty() {
        true => None,
        false => Some(tags),
    }
}
