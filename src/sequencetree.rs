use crate::binarysearchtree::BTree;

pub struct Node {
    pub c: char,
    pub next: BTree,
    pub value: Option<String>
}

impl Node {
    fn new(c: char) -> Node {
        Node {
            c,
            next: BTree::new(),
            value: None
        }
    }

    fn link(&mut self, n: Node) {
        self.next.insert(n);
    }

    fn set_value(&mut self, val: String) {
        self.value = Some(val);
    }
}

fn create_node(str: String, i: usize, val: String) -> Node {
    let mut n = Node::new(str.chars().nth(i).unwrap());

    if i == str.len() - 1 {
        n.value = Some(val);
    }else {
        let child = create_node(str, i + 1, val);
        n.link(child);
    }

    n
}

fn branch(parent: &mut Node, str: String, i: usize, val: String) {
    match parent.next.search_mut (str.chars().nth(i).unwrap()) {
        Some(n) => {
            if i == str.len() - 1 {
                n.value = Some(val);

            }else {
                branch(n, str, i + 1, val);
            }

            return;
        },
        None => {}
    }

    let mut n = create_node(str, i, val);

    parent.next.insert( n);
}

fn fetch(node: &Node, str: String, i: usize) -> Option<String> {
    match node.next.search( str.chars().nth(i).unwrap() ) {
        Some(n) => {
            if i == str.len() - 1 {
                return n.value.to_owned();

            } else {
                return fetch(n, str, i + 1);
            }
        },
        None => None
    }
}

pub struct SequenceTree {
    root: Node
}

impl SequenceTree {
    pub fn new () -> SequenceTree {
        SequenceTree {
            root: Node::new('A')
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        branch(&mut self.root, key, 0, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        fetch(&self.root, key, 0)
    }
}