use crate::{
    cli::{CountArgs, SearchArgs},
    config::Userconfig,
    io::{get_all_tags, get_tags_from_file},
    search::search_tags,
};

pub fn count(args: CountArgs, mut cfg: Userconfig) -> usize {
    // When the user does not supply a pattern
    // we return a count of all the tags or all the tags in a given file
    match args.pattern {
        None => match args.file {
            None => match get_all_tags(&mut cfg) {
                None => 0,
                Some(tags) => tags.len(),
            },
            Some(file) => match get_tags_from_file(&cfg, file) {
                None => 0,
                Some(tags) => tags.len(),
            },
        },
        // When the user does supply a pattern, we count the tags that match it,
        Some(pattern) => {
            let search_args: SearchArgs = SearchArgs {
                exclude: args.exclude,
                file: args.file,
                include: args.include,
                // The pager option is never necessary in this context
                pager: false,
                pattern,
            };

            match search_tags(search_args, cfg) {
                None => return 0,
                Some(tags) => tags.len(),
            }
        }
    }
}
