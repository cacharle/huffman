use std::fs;
use std::io;
use std::io::Read;
use std::collections::{BinaryHeap, HashMap};

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
    pub fn from_file(file_path: &str) -> io::Result<Tree> {
        let mut symbols = Symbols::from_file(file_path)?;
        Ok(Tree::from_symbols(&mut symbols))
    }

    fn from_symbols(symbols: &mut Symbols) -> Tree {
        while symbols.0.len() >= 2 {
            let first = symbols.0.pop().unwrap();
            let second = symbols.0.pop().unwrap();
            let n = Node::join(first, second);
            symbols.0.push(n);
        }
        let ret = symbols.0.pop().unwrap();
        ret
    }

    pub fn put(&self) {
        self.put_with_level(0);
    }

    fn put_with_level(&self, level: usize) {
        match &self.content {
            Content::Leaf(b) => {
                // Tree::put_spaces(level);
                print!("{} {}\n", self.occurences, b);
            },
            Content::Parent { left, right } => {
                print!("NODE {}\n", self.occurences);
                Tree::put_spaces(level);
                print!("left: ");
                left.put_with_level(level + 1);
                Tree::put_spaces(level);
                print!("right: ");
                right.put_with_level(level + 1);
            },
        }
    }

    fn put_spaces(n: usize) {
        for _ in 0..(n * 2) {
            print!(" ");
        }
    }
}

struct Symbols(BinaryHeap<Node>);

impl Symbols {
    fn from_file(file_path: &str) -> io::Result<Symbols> {
        let f = fs::File::open(file_path)?;
        let mut counter: HashMap<u8, usize> = HashMap::new();
        for b in f.bytes() {
            let k = b?;
            let v = match counter.get(&k) {
                Some(count) => count + 1,
                None        => 1,
            };
            counter.insert(k, v);
        }
        let heap: BinaryHeap<Node> = counter
            .iter()
            .map(|(k, v)| Node { occurences: *v, content: Content::Leaf(*k) })
            .collect();
        Ok(Symbols(heap))
    }
}

use std::cmp::Ordering;

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.occurences.cmp(&other.occurences)
        other.occurences.cmp(&self.occurences) // dirty hack for min-heap
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
