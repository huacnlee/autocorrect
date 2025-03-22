//! Aho-Corasick algorithm for finding multiple keywords in a text.
//!
//! https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm
use std::collections::{HashMap, VecDeque};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Node {
    children: HashMap<char, Node>,
    fail: Option<Box<Node>>,
    keywords: Vec<String>,
    /// Whether the tree is case insensitive.
    case_insensitive: bool,
}

impl fmt::Display for Node {
    /// Return a nested tree structure.
    ///
    /// For example:
    /// ```ignore
    /// |-h
    /// |-|-e
    /// |-|-l
    /// |-|-l
    /// |-|-o
    /// |-w
    /// |-|-o
    /// |-|-r
    /// |-|-l
    /// |-|-d
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut items = self.children.iter().collect::<Vec<_>>();
        items.sort_by(|a, b| a.0.cmp(b.0));
        for (c, child) in items {
            let prefix = String::from("|-");

            writeln!(f, "{}{}", prefix, c)?;
            if !child.children.is_empty() && !prefix.is_empty() {
                let child_text = format!("{}", child);
                writeln!(f, "{}{}", prefix, child_text.trim())?;
            }
        }

        Ok(())
    }
}

/// A map of matched keyword and its spans.
/// The key is the matched keyword.
/// The value is a tuple of the length of the keyword chars and a list of matched chars index.
pub type MatchedResult<'a> = HashMap<&'a str, (usize, Vec<usize>)>;

impl Node {
    pub fn new(case_insensitive: bool) -> Self {
        Node {
            children: HashMap::new(),
            fail: None,
            keywords: Vec::new(),
            case_insensitive,
        }
    }

    /// Add multiple keywords to the tree.
    pub fn add_keywords(&mut self, keywords: impl IntoIterator<Item = impl AsRef<str>>) {
        for keyword in keywords {
            self.add_keyword(keyword);
        }
    }

    /// Add a single keyword to the tree.
    fn add_keyword(&mut self, keyword: impl AsRef<str>) {
        let case_insensitive = self.case_insensitive;
        let mut node = self;
        let mut keyword = String::from(keyword.as_ref());
        if case_insensitive {
            keyword = keyword.to_lowercase();
        }

        for char in keyword.chars() {
            node = node
                .children
                .entry(char)
                .or_insert(Node::new(case_insensitive));
        }
        node.keywords.push(keyword);
    }

    /// Build fail pointers for the tree.
    pub fn build(&mut self) {
        let mut queue = VecDeque::new();
        let node = self.clone();

        for child in self.children.values_mut() {
            child.fail = Some(Box::new(node.clone()));
            queue.push_back(child);
        }

        while let Some(current) = queue.pop_front() {
            for (char, child) in current.children.iter_mut() {
                let mut fail_node = current.fail.clone().unwrap();
                while !fail_node.children.contains_key(char) && fail_node.fail.is_some() {
                    fail_node = fail_node.fail.clone().unwrap();
                }

                if let Some(fail_child) = fail_node.children.get(char) {
                    child.fail = Some(Box::new(fail_child.clone()));
                    child.keywords.extend(fail_child.keywords.iter().cloned());
                } else {
                    child.fail = Some(fail_node);
                }

                queue.push_back(child);
            }
        }
    }

    /// Match keywords in a text.
    ///
    /// # Examples
    /// ```ignore
    /// let mut dict = Node::new();
    /// dict.add_keywords(&["hello", "world"]);
    /// dict.build();
    ///
    /// let result = dict.match_keywords("hello world");
    /// ```
    pub fn match_keywords(&self, text: &str) -> MatchedResult {
        let mut result = MatchedResult::new();
        let mut node = self;

        for (i, c) in text.chars().enumerate() {
            let c = if self.case_insensitive {
                c.to_ascii_lowercase()
            } else {
                c
            };

            while !node.children.contains_key(&c) {
                if node.fail.is_none() {
                    node = self;
                    break;
                }

                node = node.fail.as_ref().unwrap();
            }

            if let Some(child) = node.children.get(&c) {
                node = child;
                for keyword in &node.keywords {
                    let (len, spans) = result
                        .entry(keyword)
                        .or_insert((keyword.chars().count(), Vec::new()));
                    spans.push(i - (*len - 1));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_keywords_keywords() {
        let mut tree = Node::new(false);
        tree.add_keywords(["hello", "world", "世界", "😀"]);
        tree.build();

        let expected = indoc::indoc! {r#"
            |-h
            |-|-e
            |-|-l
            |-|-l
            |-|-o
            |-w
            |-|-o
            |-|-r
            |-|-l
            |-|-d
            |-世
            |-|-界
            |-😀
        "#};
        assert_eq!(expected.trim(), format!("{}", tree).trim());

        let result = tree.match_keywords("hello world (世界) 😀, hello rust.");
        assert_eq!(
            result,
            vec![
                ("hello", (5, vec![0, 20])),
                ("world", (5, vec![6])),
                ("世界", (2, vec![13])),
                ("😀", (1, vec![17])),
            ]
            .into_iter()
            .collect()
        );

        // test case insensitive
        let mut tree = Node::new(true);
        tree.add_keywords(["hello", "world", "世界", "😀"]);
        let result = tree.match_keywords("hello world (世界) 😀, hello rust.");
        assert_eq!(
            result,
            vec![
                ("hello", (5, vec![0, 20])),
                ("world", (5, vec![6])),
                ("世界", (2, vec![13])),
                ("😀", (1, vec![17])),
            ]
            .into_iter()
            .collect()
        );
        let result = tree.match_keywords("HELLO WORLD (世界) 😀, HELLO RUST.");
        assert_eq!(
            result,
            vec![
                ("hello", (5, vec![0, 20])),
                ("world", (5, vec![6])),
                ("世界", (2, vec![13])),
                ("😀", (1, vec![17])),
            ]
            .into_iter()
            .collect()
        );
    }
}
