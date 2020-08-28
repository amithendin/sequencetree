use crate::binarysearchtree::BTree;

pub struct Node<K, T> {
    pub c: K,
    pub next: BTree<K, T>,
    pub value: Option<T>
}

impl<K: PartialOrd + PartialEq + Copy + Default, T> Node<K, T> {
    fn empty() -> Node<K, T> {
        Node {
            c: Default::default(),
            next: BTree::new(),
            value: None
        }
    }

    fn new(c: K) -> Node<K, T> {
        Node {
            c,
            next: BTree::new(),
            value: None
        }
    }

    fn link(&mut self, n: Node<K, T>) {
        self.next.insert(n);
    }

    fn set_value(&mut self, val: T) {
        self.value = Some(val);
    }
}

fn create_node<K: PartialOrd + PartialEq + Copy + Default, T>(key: Vec<K>, i: usize, val: T) -> Node<K, T> {
    let mut n = Node::new(key[i].to_owned() );

    if i == key.len() - 1 {
        n.value = Some(val);
    }else {
        let child = create_node(key, i + 1, val);
        n.link(child);
    }

    n
}

fn branch<K: PartialOrd + PartialEq + Copy + Default, T>(parent: &mut Node<K, T>, key: Vec<K>, i: usize, val: T) {
    match parent.next.search_mut (key[i].to_owned() ) {
        Some(n) => {
            if i == key.len() - 1 {
                n.value = Some(val);

            }else {
                branch(n, key, i + 1, val);
            }

            return;
        },
        None => {}
    }

    let mut n = create_node(key, i, val);

    parent.next.insert( n);
}

fn fetch<K: PartialOrd + PartialEq + Copy + Default, T>(node: &Node<K, T>, key: Vec<K>, i: usize) -> &Option<T> {
    match node.next.search( key[i].to_owned() ) {
        Some(n) => {
            if i == key.len() - 1 {
                return &n.value;

            } else {
                return fetch(n, key, i + 1);
            }
        },
        None => &None
    }
}

fn remove<K: PartialOrd + PartialEq + Copy + Default, T>(node: &mut Node<K, T>, key: Vec<K>, i: usize) -> bool {
    match node.next.search_mut (key[i].to_owned() ) {
        Some(n) => {
            if i == key.len() - 1 {
                n.value = None;

                return n.next.is_empty();

            }else {
                let del_node = remove(n, key, i + 1);

                if del_node {

                }
            }
        },
        None => {}
    }

    false
}

pub struct SequenceTree<K, T> {
    root: Node<K, T>
}

impl<K: PartialOrd + PartialEq + Copy + Default, T> SequenceTree<K, T> {
    pub fn new () -> SequenceTree<K, T> {
        SequenceTree {
            root: Node::empty()
        }
    }

    pub fn del(&mut self, key: Vec<K>) {
        remove(&mut self.root, key, 0);
    }

    pub fn set(&mut self, key: Vec<K>, value: T) {
        branch(&mut self.root, key, 0, value)
    }

    pub fn get(&self, key: Vec<K>) -> &Option<T> {
        fetch(&self.root, key, 0)
    }
}