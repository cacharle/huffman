use std::collections::HashMap;

use super::bits::BitSet;
use super::tree::Tree;

pub struct Table(pub HashMap<u8, BitSet>);

impl Table {
    pub fn from_tree(tree: &Tree) -> Table {
        Table(tree.to_hash_map())
    }

    pub fn convert(&self, data: Vec<u8>) -> Vec<u8> {
        let mut bitset = BitSet::new();
        for byte in data {
            bitset.concat(&self.0[&byte]);
        }
        bitset.data
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let size: u32 = self.0.iter()
            .fold(0, |acc, (_, v)| acc + 2 + v.data.len() as u32);
        out.push(((size & 0xff000000) >> 24) as u8);
        out.push(((size & 0x00ff0000) >> 16) as u8);
        out.push(((size & 0x0000ff00) >> 8) as u8);
        out.push(((size & 0x000000ff) >> 0) as u8);

        for (k, v) in &self.0 {
            out.push(*k);
            out.push(v.len as u8);
            out.extend(v.data.iter());
        }
        out
    }
}

use std::fmt;

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (k, v) in self.0.iter() {
            write!(f, "{:4} {:?}\n", format!("{:?}", *k as char), v)?;
        }
        Ok(())
    }
}
