use std::fs::File;

use grep::{
    regex::RegexMatcher,
    searcher::{Searcher, sinks::UTF8},
};

use crate::{config::Userconfig, orgtree::Orgtree};

/// This function returns the refiled files in form of a string.
/// The `write_refiled_file` in the `io` module will then take care of writing it to a file.
pub fn refile(pattern: String, cfg: Userconfig) -> String {
    let raw_pattern = format!(r":\w*{pattern}\w*:");
    let files_to_search: Vec<String> = cfg.get_files_to_search();

    let mut all_matches: Vec<(String, Vec<usize>)> = vec![];

    for file in files_to_search {
        let fname = &File::open(format!("{}{}", cfg.org_directory, file)).unwrap();
        let matcher = RegexMatcher::new(&raw_pattern).unwrap();

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
            all_matches.push((file, matches));
        }
    }

    let mut org_trees: Vec<Orgtree> = vec![];
    for entry in all_matches {
        for line_nr in entry.1 {
            let mut ot = Orgtree {
                file_name: entry.0.clone(),
                line_nr,
                lines: vec![],
            };
            crate::orgtree::get_lines(&cfg, &mut ot);
            org_trees.push(ot);
        }
    }

    let mut str_buf = string_builder::Builder::default();
    let mut current_headline = org_trees[0].file_name.clone();
    str_buf.append("* ");
    str_buf.append(current_headline.clone());
    str_buf.append("\n");
    for org_tree in org_trees {
        if org_tree.file_name != current_headline {
            current_headline = org_tree.file_name.clone();
            str_buf.append("* ");
            str_buf.append(current_headline.clone());
            str_buf.append("\n");
        }
        for line in org_tree.lines {
            str_buf.append(line);
            str_buf.append("\n");
        }
    }

    str_buf.string().unwrap()
}
