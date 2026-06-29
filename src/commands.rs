use crate::{
    cli::{CountArgs, LocateArgs, RefileArgs, SearchArgs, SedArgs, TagArgs},
    config::Userconfig,
    count::count,
    io::*,
    locate::locate,
    refile,
    search::search_tags,
};
use bat::PrettyPrinter;
use minus::{MinusError, Pager, page_all};
use std::process::Command;

pub fn count_command(args: CountArgs) -> Result<(), MinusError> {
    let mut cfg: Userconfig = Userconfig::new();
    let count = count(args.clone(), cfg.clone());
    let files_searched: usize = cfg.get_files_to_search(args.include, args.exclude).len();

    match args.pattern {
        Some(pattern) => match args.file {
            // 1) no pattern given, search in all files
            None => println!(
                "Found {count} unique tags across {files_searched} file searched that match pattern `{pattern}`"
            ),
            // 2) no pattern given, search in one file
            Some(file) => {
                println!(
                    "Found {count} unique tags that match pattern `{pattern}` in file `{file}`"
                )
            }
        },
        None => match args.file {
            // 3) pattern given, search in all files
            None => println!("Found {count} unique tags across {files_searched} files searched"),
            // 4) pattern given, search in one file
            Some(file) => println!("Found {count} unique tags in file `{file}`"),
        },
    }

    Ok(())
}

pub fn locate_command(args: LocateArgs) -> Result<(), MinusError> {
    let cfg: Userconfig = Userconfig::new();
    let files: Vec<String> = locate(args, cfg);

    for file in files {
        println!("{file}")
    }

    Ok(())
}

pub fn refile_command(args: RefileArgs) -> Result<(), MinusError> {
    let cfg: Userconfig = Userconfig::new();
    let file_contents: String = refile::refile(&args, cfg);

    match args.output_file {
        None => {
            if args.no_pager {
                println!("{file_contents}");
                Ok(())
            } else {
                let mut pager = Pager::new();

                let _ = PrettyPrinter::new()
                    .input_from_bytes(file_contents.as_bytes())
                    .language("org")
                    .print_with_writer(Some(&mut pager));

                let _ = Pager::set_run_no_overflow(&pager, true);

                page_all(pager)
            }
        }
        Some(output_file) => {
            let fname = if output_file.ends_with(".org") {
                output_file
            } else {
                format!("{output_file}.org")
            };

            print_to_file(file_contents, fname)
        }
    }
}

pub fn search_command(args: SearchArgs) -> Result<(), MinusError> {
    let cfg: Userconfig = Userconfig::new();

    match search_tags(args.clone(), cfg) {
        None => println!("No tags matching the provided pattern were found!"),
        Some(taglist) => print_tags_to_stdout_or_pager(taglist, args.pager)?,
    }

    Ok(())
}

pub fn sed_command(args: SedArgs) -> Result<(), MinusError> {
    let cfg: Userconfig = Userconfig::new();

    let files: Vec<String> = locate(
        LocateArgs {
            pattern: args.tag.clone(),
            strict: true,
            include: None,
            exclude: None,
        },
        cfg.clone(),
    );

    if files.is_empty() {
        println!(
            "There is no tag named `{}` to be substituted",
            args.tag.clone()
        );

        return Ok(());
    }

    let mut counter: usize = 0;

    for fname in files {
        let sed_str: String = format!("s/:{}:/:{}:/g", args.tag, args.replacement);

        if let Ok(_) = Command::new("sed")
            .arg("--in-place")
            .arg(sed_str)
            .arg(format!("{}/{}", cfg.org_directory, fname))
            .spawn()
        {
            if args.verbose {
                println!("Performing substitution in file `{}`", fname)
            }
            counter += 1;
        }
    }

    if args.verbose {
        print!("\n")
    }

    println!(
        "Substituted successfully `{}` with `{}` in {} files",
        args.tag, args.replacement, counter
    );

    Ok(())
}

pub fn tags_command(args: TagArgs) -> Result<(), MinusError> {
    let mut cfg: Userconfig = Userconfig::new();

    match args.file {
        None => match get_all_tags(&mut cfg) {
            Some(taglist) => match args.print {
                true => print_to_file(taglist.join("\n"), String::from("tags.txt"))?,
                false => print_tags_to_stdout_or_pager(taglist, args.pager)?,
            },
            None => println!("Could not find any tags in your org directory"),
        },
        Some(file_name) => match get_tags_from_file(&cfg, file_name.clone()) {
            Some(taglist) => match args.print {
                true => print_to_file(taglist.join("\n"), String::from("tags.txt"))?,
                false => print_tags_to_stdout_or_pager(taglist, args.pager)?,
            },
            None => println!("I have found no tags in file: {file_name}"),
        },
    }

    Ok(())
}
