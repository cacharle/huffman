#[derive(Clone)]
pub struct BitSet {
    pub data: Vec<u8>,
    pub len: usize,
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet { data: Vec::new(), len: 0 }
    }

    pub fn push_front_bit(&mut self, bit: u8) {
        if bit != 0 && bit != 1 {
            panic!("bit should be 1 or 0");
        }
        *self >>= 1;
        self.data[0] |= bit << 7;
    }

    fn shift_right_once(&mut self) {
        if self.len == self.data.len() * 8 {
            self.data.push(0);
        }
        for i in (1..self.data.len()).rev() {
            self.data[i] >>= 1;
            self.data[i] |= (self.data[i - 1] & 1) << 7;
        }
        self.data[0] >>= 1;
        self.len += 1;
    }

    pub fn concat(&mut self, other: &BitSet) {
        if self.len % 8 == 0 {
            self.data.extend(other.data.iter());
            self.len += other.len;
            return;
        }

        let mut other_mut = other.clone();
        let len_pre_shift = other_mut.len;
        other_mut >>= self.len % 8;
        *self.data.last_mut().unwrap() |= other_mut.data[0];
        self.data.extend(other_mut.data.iter().skip(1));
        self.len += len_pre_shift;
    }
}

use std::ops;

impl ops::ShrAssign<usize> for BitSet {
    fn shr_assign(&mut self, shift_size: usize) {
        for _ in 0..shift_size {
            self.shift_right_once();
        }
    }
}

use std::fmt;

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "len: {:2} data: ", self.len)?;
        for chunk in &self.data {
            write!(f, "{:08b} ", chunk)?;
        }
        Ok(())
    }
}
