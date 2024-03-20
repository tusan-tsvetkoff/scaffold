use std::
    fmt::{Display, Formatter}
;

pub fn is_directory(path: &str) -> bool {
    path.ends_with('/')
}

pub fn is_multiple_directories(path: &str) -> bool {
    path.contains('/')
}

pub fn is_directory_with_file(path: &str) -> bool {
    path.contains('/') && !path.ends_with('/')
}

pub fn parse_tree_structure(input: &str) -> Vec<String> {
    input.split_inclusive('/').map(|s| s.to_string()).collect()
}

pub fn parse_file_name(input: &str) -> String {
    let file_name = input.split_inclusive('/').last();

    match file_name {
        Some(name) => name.to_string(),
        None => "".to_string(),
    }
}

pub fn parse_multiple_directories(input: &str) -> Vec<String> {
    assert!(is_multiple_directories(input));
    input.split_inclusive('/').map(|s| s.to_string()).collect()
}

pub fn create_tree_structure(input: Vec<String>) -> Tree {
    let mut root = Node::new("".to_string());
    let mut current_node = &mut root;

    for path in input {
        if is_directory(&path) {
            if is_multiple_directories(&path) {
                let directories = parse_multiple_directories(&path);
                for directory in directories {
                    let new_node = Node::new(directory);
                    current_node.add_child(new_node);
                    current_node = current_node.children.last_mut().unwrap();
                }
            }
        } else if is_directory_with_file(&path) {
            let file = parse_file_name(&path);
            let mut directories: Vec<String> = parse_multiple_directories(&path);
            if directories.last().is_some() {
                directories.pop();
            }
            for directory in directories {
                let new_node = Node::new(directory);
                current_node.add_child(new_node);
                current_node = current_node.children.last_mut().unwrap();
            }
            let last_file_node = Node::new(file);
            current_node.add_child(last_file_node);
        } else {
            let new_node = Node::new(parse_file_name(&path));
            current_node.add_child(new_node);
            current_node = current_node.children.last_mut().unwrap();
        }
        current_node = &mut root;
    }

    Tree::new(root)
}

pub struct Tree {
    pub root: Node,
}

impl Tree {
    pub fn new(root: Node) -> Tree {
        Tree { root }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.root)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)?;
        for child in &self.children {
            // Need to add padding to the children
            write!(f, "\n{}", child)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub length: usize,
}

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name,
            children: Vec::new(),
            length: 0,
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn length(&self) -> usize {
        self.children.len()
    }
}
