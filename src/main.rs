use std::env;
// use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

pub mod tree;
pub mod bits;
pub mod conversion;

use tree::Tree;
use bits::BitSet;

fn main() {
    if let Some(s) = env::args().nth(1) {
        if s != "d" {
            return
        }
        let data: Vec<u8> = io::stdin().bytes().map(|x| x.unwrap()).collect();

        let mut iter = data.iter();

        let mut size: u32 = 0;
        for i in 0..4 {
            size |= (*iter.next().unwrap() as u32) << ((3 - i) * 8);
        }
        // println!("{}", size);

        let mut header_iter = iter.clone().take(size as usize).cloned();
        let mut table = conversion::Table::new();
        loop {
            let test = header_iter.next();
            if test == None {
                break;
            }
            // FIXME not working with lorem
            let key             = test.unwrap();
            let bits_len: usize = header_iter.next().unwrap().into();
            let value: Vec<u8>  = header_iter.clone().take(bits_len / 8 +
                if bits_len % 8 != 0 {1} else {0}).collect();
            for _ in 0..(bits_len / 8 + 1) {
                header_iter.next();
            }

            table.0.insert(key, BitSet { data: value, len: bits_len });
        }
        // println!("{:?}", table);

        let data: Vec<u8> = iter.skip(size as usize).cloned().collect();
        let mut bitset = BitSet { len: data.len() * 8, data: data };

        // TODO: store bit size in serialized, cause wrong matches due to padding
        let mut found = true;
        let mut content = Vec::new();
        while found {
            println!("{:?}", bitset);
            found = false;
            for (k, v) in &table.0 {
                if bitset.start_with(&v) {
                    content.push(k);
                    bitset <<= v.len;
                    found = true;
                }
            }
        }
        for x in content {
            print!("{}", *x as char);
        }

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
