pub fn is_headline(input: &String) -> bool {
    input.starts_with('*')
}

pub fn has_tags(input: &String) -> bool {
    input.trim().ends_with(':')
}

pub fn get_tags(input: &String) -> Option<Vec<String>> {
    let taglist_opt = match has_tags(&input.trim().to_string()) {
        false => return None,
        true => Some(
            input
                .trim()
                .rsplit_once(' ')
                .unwrap()
                .1
                .split(':')
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|str| str.to_string())
                .collect(),
        ),
    };
    taglist_opt
}

#[cfg(test)]
mod tests {
    use crate::orgtree::*;

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
