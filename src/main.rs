use std::env;
// use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

pub mod tree;
pub mod bits;
pub mod conversion;

use tree::Tree;

fn main() {
    if let Some(s) = env::args().nth(1) {
        if s != "d" {
            return
        }
        // let data: Vec<u8> = io::stdin().bytes().map(|x| x.unwrap()).collect();
        // deserialize

    } else {
        // let f = fs::File::open(file_path).unwrap();

        let data: Vec<u8> = io::stdin().bytes().map(|x| x.unwrap()).collect();

        let tree = Tree::from_data(&data);
        // print!("{:?}", tree);

        let table = conversion::Table::from_tree(&tree);
        // print!("{:?}", table);

        let converted_data = table.convert(data);
        // println!("{:?}", converted_data);

        let header = table.serialize();

        io::stdout().write_all(&header).unwrap();
        io::stdout().write_all(&converted_data).unwrap();
    }
}
