use std::env;

pub mod symbols;
// pub mod tree;
//
use symbols::Tree;

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let tree = Tree::from_file(&file_path).unwrap();
    tree.put();
}
