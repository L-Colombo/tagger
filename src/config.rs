use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Userconfig {
    pub org_directory: String,
    pub exclude_files: Option<Vec<String>>,
    pub exclude_pattern: Option<String>,
}

impl Userconfig {
    pub fn new() -> Userconfig {
        let home_dir: String = std::env::var("HOME").unwrap();
        let cfg_file_name: String = format!("{home_dir}/.config/tagger/tagger.toml");
        let cfg_file = &std::fs::read_to_string(cfg_file_name);

        let data: Userconfig = match cfg_file {
            Ok(file) => toml::from_str(file).unwrap(),
            Err(_) => Userconfig {
                org_directory: format!("{home_dir}/Documents/Org/"),
                exclude_files: None,
                exclude_pattern: None,
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
        }
    }
}
