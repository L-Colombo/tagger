pub fn is_headline(input: &str) -> bool {
    input.starts_with('*')
}

pub fn has_tags(input: &str) -> bool {
    input.ends_with(':')
}

pub fn get_tags(input: &str) -> Option<Vec<&str>> {
    let taglist_opt = match has_tags(input.trim()) {
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
        let input = "This is not a headline";
        assert_eq!(false, is_headline(input))
    }

    #[test]
    fn is_headline_expect_true() {
        let input = "* This is a headline";
        assert_eq!(true, is_headline(input))
    }

    // TEST GET_TAGS
    #[test]
    fn headline_has_no_tags() {
        let input = "* I have no tags";
        assert_eq!(None, get_tags(input))
    }

    #[test]
    fn headline_has_one_tag() {
        let input = "* I have one tag :foo:";
        assert_eq!(Some(vec!["foo"]), get_tags(input))
    }

    #[test]
    fn headline_has_many_tags() {
        let input = "* I have many tags :foo:bar:baz:";
        assert_eq!(Some(vec!["foo", "bar", "baz"]), get_tags(input))
    }

    #[test]
    fn headline_has_colons_not_tag_delim() {
        let input = "* What I have: many tags :foo:bar:baz:";
        assert_eq!(Some(vec!["foo", "bar", "baz"]), get_tags(input))
    }

    #[test]
    fn headline_has_colons_not_tag_delim_and_trailing_whitespace() {
        let input = "* What I have: many tags :foo:bar:baz: ";
        assert_eq!(Some(vec!["foo", "bar", "baz"]), get_tags(input))
    }
}
