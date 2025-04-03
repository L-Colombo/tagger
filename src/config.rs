use grep::{matcher::Matcher, regex::RegexMatcher};
use serde_derive::Deserialize;
use std::{fs::read_dir, process::exit};

#[derive(Deserialize, Debug)]
pub struct Userconfig {
    pub org_directory: String,
    pub exclude_files: Option<Vec<String>>,
    pub exclude_pattern: Option<String>,
    pub exclude_patterns: Option<Vec<String>>,
}

impl Default for Userconfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Userconfig {
    pub fn new() -> Userconfig {
        let home_dir: String = match std::env::var("HOME") {
            Ok(home) => home,
            Err(e) => {
                eprintln!("{e}: Could not get the path of your $HOME");
                exit(1)
            }
        };
        let cfg_file_name: String = format!("{home_dir}/.config/tagger/tagger.toml");
        let cfg_file = &std::fs::read_to_string(cfg_file_name);

        let data: Userconfig = match cfg_file {
            Ok(file) => match toml::from_str(file) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("{e}: Configuration file is possibly malformed");
                    exit(1)
                }
            },
            Err(_) => Userconfig {
                org_directory: format!("{home_dir}/Documents/Org/"),
                exclude_files: None,
                exclude_pattern: None,
                exclude_patterns: None,
            },
        };

        Userconfig {
            org_directory: {
                if data.org_directory.starts_with('~') {
                    let expanded_orgdir_path = data.org_directory.replace('~', home_dir.as_str());
                    expanded_orgdir_path
                } else {
                    data.org_directory
                }
            },
            exclude_files: data.exclude_files,
            exclude_pattern: data.exclude_pattern,
            exclude_patterns: data.exclude_patterns,
        }
    }

    pub fn get_files_to_search(&self) -> Vec<String> {
        let org_dir_entries = match read_dir(&self.org_directory) {
            Ok(entries) => entries,
            Err(e) => {
                println!("Error {}: cannot access your org directory", e);
                exit(1)
            }
        };

        org_dir_entries
            .map_while(Result::ok)
            .filter(|entry| !entry.file_type().unwrap().is_dir())
            .map(|entry| entry.file_name().into_string().unwrap())
            .filter(|entry| {
                if let Some(exclude) = &self.exclude_files {
                    !exclude.contains(entry)
                } else {
                    true
                }
            })
            .filter(|entry| {
                if let Some(pattern) = &self.exclude_pattern {
                    let regex = RegexMatcher::new(pattern).unwrap();
                    !regex.is_match(entry.as_bytes()).unwrap()
                } else {
                    true
                }
            })
            .filter(|entry| {
                if let Some(patterns) = &self.exclude_patterns {
                    for i in patterns.iter() {
                        let regex = RegexMatcher::new(i).unwrap();
                        if regex.is_match(entry.as_bytes()).unwrap() {
                            return false;
                        }
                    }
                    true
                } else {
                    true
                }
            })
            .collect()
    }
}
