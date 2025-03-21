use pretty_assertions::assert_eq;
use relative_path::RelativePath;
use std::env::current_dir;
use tagger::{config::Userconfig, search::search_tags};

#[test]
fn search_tags_in_all_files() {
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

    let pattern1: String = String::from("foo");
    let pattern2: String = String::from("\\w+foo");

    assert_eq!(
        search_tags(pattern1, &cfg, None),
        Some(vec![
            String::from("foo_another_file"),
            String::from("foo_file_1"),
            String::from("foo_file_2"),
            String::from("trailing_foo_another_file"),
            String::from("trailing_foo_file_1"),
            String::from("trailing_foo_file_2"),
        ])
    );

    assert_eq!(
        search_tags(pattern2, &cfg, None),
        Some(vec![
            String::from("trailing_foo_another_file"),
            String::from("trailing_foo_file_1"),
            String::from("trailing_foo_file_2"),
        ])
    )
}

#[test]
fn search_tags_in_specific_file() {
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

    let pattern1: String = String::from("foo");
    let pattern2: String = String::from("\\w+foo");
    const FILE: &str = "org_file1.org";

    assert_eq!(
        search_tags(pattern1, &cfg, Some(FILE.to_string())),
        Some(vec![
            String::from("foo_file_1"),
            String::from("trailing_foo_file_1"),
        ])
    );

    assert_eq!(
        search_tags(pattern2, &cfg, Some(FILE.to_string())),
        Some(vec![
            //String::from("foo_file_1"),
            String::from("trailing_foo_file_1"),
        ])
    )
}
