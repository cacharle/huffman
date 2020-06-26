use std::fmt;
use std::collections::{BinaryHeap, HashMap};

use super::bits::BitSet;

pub struct Node {
    occurences: usize,
    content: Content,
}

impl Node {
    fn join(left: Node, right: Node) -> Node {
        Node {
            occurences: left.occurences + right.occurences,
            content: Content::Parent {
                left:  Box::new(left),
                right: Box::new(right),
            }
        }
    }
}

enum Content {
    Leaf(u8),
    Parent {
        left:  Box<Node>,
        right: Box<Node>,
    },
}

pub type Tree = Node;

impl Tree {
    pub fn from_data(data: &[u8]) -> Tree {
        let mut counter: HashMap<u8, usize> = HashMap::new();
        for k in data {
            let v = match counter.get(&k) {
                Some(count) => count + 1,
                None        => 1,
            };
            counter.insert(*k, v);
        }
        let mut heap: BinaryHeap<Node> = counter
            .iter()
            .map(|(k, v)| Node { occurences: *v, content: Content::Leaf(*k) })
            .collect();
        while heap.len() >= 2 {
            let first = heap.pop().unwrap();
            let second = heap.pop().unwrap();
            let n = Node::join(first, second);
            heap.push(n);
        }
        heap.pop().unwrap()
    }

    pub fn to_hash_map(&self) -> HashMap<u8, BitSet> {
        match &self.content {
            Content::Leaf(b) => {
                let mut hm = HashMap::with_capacity(1);
                hm.insert(*b, BitSet::new());
                hm
            },
            Content::Parent { left, right } => {
                let mut left_hm = left.to_hash_map();
                for (_, bits) in left_hm.iter_mut() {
                    bits.push_front_bit(0);
                }
                let mut right_hm = right.to_hash_map();
                for (_, bits) in right_hm.iter_mut() {
                    bits.push_front_bit(1);
                }
                left_hm.extend(right_hm);
                left_hm
            },
        }
    }

    fn fmt_spaces(f: &mut fmt::Formatter, level: usize) -> fmt::Result {
        for _ in 0..level {
            write!(f, "  ")?;
        }
        Ok(())
    }

    fn fmt_with_level(&self, f: &mut fmt::Formatter, level: usize) -> fmt::Result {
        match &self.content {
            Content::Leaf(b) => {
                write!(f, "{} {}\n", self.occurences, b)?;
            },
            Content::Parent { left, right } => {
                write!(f, "NODE {}\n", self.occurences)?;
                Tree::fmt_spaces(f, level)?;
                write!(f, "left: ")?;
                left.fmt_with_level(f, level + 1)?;
                Tree::fmt_spaces(f, level)?;
                write!(f, "right: ")?;
                right.fmt_with_level(f, level + 1)?;
            },
        }
        Ok(())
    }
}

use std::cmp::Ordering;

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.occurences.cmp(&other.occurences)
        other.occurences.cmp(&self.occurences) // dirty hack to fake min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.occurences == other.occurences
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_level(f, 0)
    }
}
