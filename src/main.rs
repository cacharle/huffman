use std::env;
use std::fs;
use std::io::Read;

pub mod tree;
pub mod bits;

use tree::Tree;
use bits::BitSet;

fn main() {
    let file_path = env::args().nth(1).unwrap();

    let f = fs::File::open(file_path).unwrap();
    let data: Vec<u8> = f.bytes().map(|x| x.unwrap()).collect();

    let tree = Tree::from_data(&data);
    tree.put();

    let bitmap = tree.to_bit_map();
    for (k, v) in bitmap.iter() {
        println!("{:4} {:?}", format!("{:?}", *k as char), v);
    }

    let mut bitset = BitSet::new();
    for byte in data {
        bitset.concat(&bitmap[&byte]);
    }
    println!("{:?}", bitset);
}
