use just_fmt::kebab_case;

/// Represents a command node, used to match user-input command paths.
///
/// The node consists of multiple parts, each separated by a dot (`.`), and automatically converted to kebab-case.
/// For example, the input string `"node.subnode"` will be converted to a node representation of `["node", "subnode"]`.
#[derive(Debug, Default)]
pub struct Node {
    node: Vec<String>,
}

impl Node {
    /// Append a new part to the node path.
    pub fn join(self, node: impl Into<String>) -> Node {
        let mut new_node = self.node;
        new_node.push(node.into());
        Node { node: new_node }
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        let node = s.split('.').map(|part| kebab_case!(part)).collect();
        Node { node }
    }
}

impl From<String> for Node {
    fn from(s: String) -> Self {
        let node = s.split('.').map(|part| kebab_case!(part)).collect();
        Node { node }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.node.cmp(&other.node)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.node.join("."))
    }
}
