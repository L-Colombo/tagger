use pretty_assertions::assert_eq;
use relative_path::RelativePath;
use std::env::current_dir;
use tagger::{config::*, io::*};

#[test]
fn get_tags_from_all_files() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: None,
        exclude_pattern: None,
    };

    assert_eq!(
        get_all_tags(cfg),
        Some(vec![
            String::from("bar_file_1"),
            String::from("bar_file_2"),
            String::from("baz_file_1"),
            String::from("baz_file_2"),
            String::from("foo_file_1"),
            String::from("foo_file_2"),
            String::from("non_delimiters_file_1"),
            String::from("non_delimiters_file_2"),
            String::from("trailing_baz_file_1"),
            String::from("trailing_baz_file_2"),
            String::from("trailing_foo_file_1"),
            String::from("trailing_foo_file_2"),
        ])
    )
}

#[test]
fn get_tags_from_all_files_with_exclude_files() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: Some(vec![String::from("org_file2.org")]),
        exclude_pattern: None,
    };

    assert_eq!(
        get_all_tags(cfg),
        Some(vec![
            String::from("bar_file_1"),
            String::from("baz_file_1"),
            String::from("foo_file_1"),
            String::from("non_delimiters_file_1"),
            String::from("trailing_baz_file_1"),
            String::from("trailing_foo_file_1"),
        ])
    )
}

#[test]
fn get_tags_from_one_file_that_has_tags() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: None,
        exclude_pattern: None,
    };

    let file: String = String::from("org_file1.org");

    assert_eq!(
        get_tags_from_file(cfg, file),
        Some(vec![
            String::from("bar_file_1"),
            String::from("baz_file_1"),
            String::from("foo_file_1"),
            String::from("non_delimiters_file_1"),
            String::from("trailing_baz_file_1"),
            String::from("trailing_foo_file_1"),
        ])
    )
}

#[test]
fn get_tags_from_one_file_that_has_no_tags() {
    let project_root_directory = current_dir().unwrap();
    let org_dir_path = RelativePath::new("/tests/org_files/")
        .to_path(&project_root_directory)
        .to_str()
        .unwrap()
        .to_string();

    let cfg: Userconfig = Userconfig {
        org_directory: org_dir_path,
        exclude_files: None,
        exclude_pattern: None,
    };

    let file: String = String::from("org_file_with_no_tags.org");

    assert_eq!(get_tags_from_file(cfg, file), None)
}
