use crate::sequencetree::Node;

struct BTreeNode {
    left: Option<Box<BTreeNode>>,
    right: Option<Box<BTreeNode>>,
    val: Option<Node>
}

pub struct BTree {
    root: Box<BTreeNode>
}

fn insert(n: &mut Box<BTreeNode>, c: Node) {
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

fn search( n: &BTreeNode, c: char) -> Option<&Node> {
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

fn search_mut( n: &mut BTreeNode, c: char) -> Option<&mut Node> {
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

impl BTree {
    pub fn new () -> BTree {
        BTree {
            root: Box::new(BTreeNode {
                left: None, right: None, val: None
            })
        }
    }

    pub fn insert(&mut self, c: Node) {
        insert( &mut self.root, c);
    }

    pub fn search(&self, c: char) -> Option<&Node> {
        search(&self.root, c)
    }

    pub fn search_mut(&mut self, c: char) -> Option<&mut Node> {
        search_mut(&mut self.root, c)
    }
}