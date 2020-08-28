use crate::sequencetree::Node;
use std::fmt::Debug;

struct BTreeNode<K, T> {
    left: Option<Box<BTreeNode<K, T>>>,
    right: Option<Box<BTreeNode<K, T>>>,
    val: Option<Node<K, T>>
}

impl<K: PartialOrd + PartialEq + Copy + Default, T> BTreeNode<K, T> {
    fn is_empty(&self) -> bool {
        match &self.val {
            Some(_) => return true,
            None => {}
        };

        match &self.left {
            Some(l) => { if l.is_empty() { return true } },
            None => {}
        };

        match &self.right {
            Some(r) => { if r.is_empty() { return true } },
            None => {}
        };

        false
    }
}

pub struct BTree<K, T> {
    root: Box<BTreeNode<K, T>>
}

fn insert<K: PartialOrd + PartialEq + Copy + Default, T>(n: &mut Box<BTreeNode<K, T>>, c: Node<K, T>) {
    match &mut n.val {
        Some(val) => {
            if val.c < c.c {
                match &mut n.right {
                    Some(r) => insert(r, c),
                    None => n.right = Some(Box::new(BTreeNode { left: None, right: None, val: Some(c) }))
                }
            } else {
                match &mut n.left {
                    Some(l) => insert(l, c),
                    None => n.left = Some(Box::new(BTreeNode { left: None, right: None, val: Some(c) }))
                }
            }
        },
        None => {
            n.val = Some(c);
        }
    }
}

fn search<K: PartialOrd + PartialEq + Copy + Default, T>( n: &BTreeNode<K, T>, c: K) -> Option<&Node<K, T>> {

    let val = match &n.val {
        Some(val) => val,
        None => return None
    };

    if val.c == c {
        return Some(val)
    }

    if val.c < c {
        return match &n.right {
            Some(r) => search(r, c),
            None => None
        }
    }

    match &n.left {
        Some(l) => search(l, c),
        None => None
    }
}

fn search_mut<K: PartialOrd + PartialEq + Copy + Default, T>( n: &mut BTreeNode<K, T>, c: K) -> Option<&mut Node<K, T>> {
    let mut val = match &mut n.val {
        Some(val) => val,
        None => return None
    };

    if val.c == c {
        return Some(val)
    }

    if val.c < c {
        return match &mut n.right {
            Some(r) => search_mut(r, c),
            None => None
        }
    }

    match &mut n.left {
        Some(l) => search_mut(l, c),
        None => None
    }
}

fn remove<K: PartialOrd + PartialEq + Copy + Default, T>( n: &mut BTreeNode<K, T>, c: K) -> bool {
    let mut val = match &mut n.val {
        Some(val) => val,
        None => return false
    };

    if val.c == c {
        return true
    }

    if val.c < c {
        return match &mut n.right {
            Some(r) => {
                if remove(r, c) {
                    n.right = None;
                }

                match &n.left {
                    Some(_) => false,
                    None => true
                }

            },
            None => false
        }
    }

    match &mut n.left {
        Some(l) => {
            if remove(l, c) {
                n.left = None;
            }

            match &n.right {
                Some(_) => false,
                None => true
            }
        },
        None => false
    }
}

impl<K: PartialOrd + PartialEq + Copy + Default, T> BTree<K, T> {
    pub fn new () -> BTree<K, T> {
        BTree {
            root: Box::new(BTreeNode {
                left: None, right: None, val: None
            })
        }
    }

    pub fn remove(&mut self, c: K) {
        remove(&mut self.root, c);
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn insert(&mut self, c: Node<K, T>) {
        insert( &mut self.root, c);
    }

    pub fn search(&self, c: K) -> Option<&Node<K, T>> {
        search(&self.root, c)
    }

    pub fn search_mut(&mut self, c: K) -> Option<&mut Node<K, T>> {
        search_mut(&mut self.root, c)
    }
}