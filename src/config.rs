use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    org_directory: String,
    exclude_files: Option<Vec<String>>,
    exclude_pattern: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        let home_dir: String = std::env::var("HOME").unwrap();
        let cfg_file_name: String = format!("{home_dir}/.config/tagger/tagger.toml");
        let cfg_file = &std::fs::read_to_string(cfg_file_name);

        let data: Config = match cfg_file {
            Ok(file) => toml::from_str(file).unwrap(),
            Err(_) => Config {
                org_directory: format!("{home_dir}/Documents/Org/"),
                exclude_files: None,
                exclude_pattern: None,
            },
        };

        Config {
            org_directory: data.org_directory,
            exclude_files: data.exclude_files,
            exclude_pattern: data.exclude_pattern,
        }
    }
}
