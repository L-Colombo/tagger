use crate::config::Userconfig;
use std::process::exit;

#[derive(Debug)]
pub struct Orgtree {
    pub file_name: String,
    pub line_nr: usize,
    pub lines: Vec<String>,
}

pub fn get_lines(cfg: &Userconfig, ot: &mut Orgtree) {
    let file_lines: Vec<String> =
        match std::fs::read_to_string(format!("{}{}", cfg.org_directory, ot.file_name).as_str()) {
            Ok(filestring) => filestring.lines().map(String::from).collect(),
            Err(e) => {
                eprintln!("{e}: Could not read file to string");
                exit(1)
            }
        };

    let line_idx: usize = ot.line_nr;
    // Get the headline itself
    ot.lines.push(file_lines[line_idx - 1].clone().to_owned());

    // Get the rest of the lines of the subtree
    for i in line_idx..file_lines.len() {
        let current_line = file_lines[i].clone();
        if current_line.starts_with('*') {
            break;
        } else {
            ot.lines.push(current_line.to_owned());
        }
    }
}

pub fn is_headline(input: &String) -> bool {
    input.starts_with('*')
}

pub fn has_tags(input: &String) -> bool {
    input.trim().ends_with(':')
}

pub fn get_tags(input: &String) -> Option<Vec<String>> {
    match has_tags(&input.trim().to_string()) {
        false => None,
        true => Some(
            input
                .trim()
                .rsplit_once(' ')
                .expect("Some your headlines might be malformed")
                .1
                .split(':')
                .filter(|s| !s.is_empty())
                .map(|str| str.to_string())
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::orgtree::*;
    use pretty_assertions::assert_eq;

    // TEST IS_HEADLINE
    #[test]
    fn is_headline_expect_false() {
        let input = "This is not a headline".to_string();
        assert_eq!(false, is_headline(&input))
    }

    #[test]
    fn is_headline_expect_true() {
        let input = "* This is a headline".to_string();
        assert_eq!(true, is_headline(&input))
    }

    // TEST GET_TAGS
    #[test]
    fn headline_has_no_tags() {
        let input = "* I have no tags".to_string();
        assert_eq!(None, get_tags(&input))
    }

    #[test]
    fn headline_has_one_tag() {
        let input = "* I have one tag :foo:".to_string();
        assert_eq!(Some(vec!["foo".to_string()]), get_tags(&input))
    }

    #[test]
    fn headline_has_many_tags() {
        let input = "* I have many tags :foo:bar:baz:".to_string();
        assert_eq!(
            Some(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string()
            ]),
            get_tags(&input)
        )
    }

    #[test]
    fn headline_has_colons_not_tag_delim() {
        let input = "* What I have: many tags :foo:bar:baz:".to_string();
        assert_eq!(
            Some(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string()
            ]),
            get_tags(&input)
        )
    }

    #[test]
    fn headline_has_colons_not_tag_delim_and_trailing_whitespace() {
        let input = "* What I have: many tags :foo:bar:baz: ".to_string();
        assert_eq!(
            Some(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string()
            ]),
            get_tags(&input)
        )
    }
}
