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

    fn shift_left_once(&mut self) {
        if self.data.len() == 0 || self.len == 0 {
            return;
        }
        for i in 0..(self.data.len() - 1) {
            self.data[i] <<= 1;
            self.data[i] |= (self.data[i + 1] & (1 << 7)) >> 7;
        }
        if let Some(v) = self.data.last_mut() {
            *v <<= 1;
        }
        self.len -= 1;
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

    pub fn start_with(&self, other: &BitSet) -> bool {
        if other.len == 0 {
            return true
        }
        if other.len > self.len {
            return false;
        }
        for i in 0..(other.data.len() - 1) {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        let mask = if other.len % 8 == 0 {
            0xff
        } else {
            (0xff >> (8 - other.len % 8)) << (8 - other.len % 8)
        };
        let x = self.data[other.data.len() - 1] & mask;
        let y = other.data[other.data.len() - 1] & mask;
        x == y
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

impl ops::ShlAssign<usize> for BitSet {
    fn shl_assign(&mut self, shift_size: usize) {
        for _ in 0..shift_size {
            self.shift_left_once();
        }
    }
}

impl PartialEq for BitSet {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        for (a, b) in self.data.iter().zip(other.data.iter()) {
            if a != b {
                return false
            }
        }
        true
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

#[cfg(test)]
mod tests {
    use super::*;

    fn bitset_from_str(s: &str) -> BitSet {
        let mut bitset = BitSet::new();
        for c in s.chars().rev() {
            match c {
                '0' => bitset.push_front_bit(0),
                '1' => bitset.push_front_bit(1),
                _   => {},
            }
        }
        bitset
    }

    #[test]
    fn new() {
        let b = BitSet::new();
        assert_eq!(b.len, 0);
        assert_eq!(b.data.len(), 0);
    }

    #[test]
    fn start_with_one_chunk() {
        let a = bitset_from_str("1001001");
        let b = bitset_from_str("1001");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_origin_multiple_chunk() {
        let a = bitset_from_str("10010010000010001101010101111111000000000001011111");
        let b = bitset_from_str("1001");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_multiple_chunk() {
        let a = bitset_from_str("10010010000010001101010101111111000000000001011111");
        let b = bitset_from_str("100100100000100011010101");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_origin_empty() {
        let a = bitset_from_str("");
        let b = bitset_from_str("1001");
        assert!(!a.start_with(&b), "{:?} start with {:?}", a, b);
    }

    #[test]
    fn start_with_compared_empty() {
        let a = bitset_from_str("1001");
        let b = bitset_from_str("");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_both_empty() {
        let a = bitset_from_str("");
        let b = bitset_from_str("");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_zeroes() {
        let a = bitset_from_str("000000000000000000000000000000000000000000000000000000110");
        let b = bitset_from_str("000000000000000000000000000000000000000000000000000000");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_ones() {
        let a = bitset_from_str("11111111111111111111111111111111111111111111111111111010");
        let b = bitset_from_str("11111111111111111111111111111111111111111111111111111");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_long() {
        let a = bitset_from_str("0101010101111111111111110100101010101010101011111111111111\
0000000000000000000000000000000000000000000000000000000000000000111111111111100000011010100\
1010101001010101001010101001010101001010101010101001010010101010010010101010101010010");
        let b = bitset_from_str("0101010101111111111111110100101010101010101011111111111111\
0000000000000000000000000000000000000000000000000000000000000000111111111111100000011010100\
1010101001010101001010101001010101001010101010101001010010101010010010101010101010");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_chunk_equal() {
        let a = bitset_from_str("10101010");
        let b = bitset_from_str("1");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_chunk_one_more() {
        let a = bitset_from_str("101010100");
        let b = bitset_from_str("1");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_chunk_one_less() {
        let a = bitset_from_str("1010101");
        let b = bitset_from_str("1");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_one_bit() {
        let a = bitset_from_str("11011101111100001010111100000000");
        let b = bitset_from_str("1");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }

    #[test]
    fn start_with_two_bit() {
        let a = bitset_from_str("11011101111100001010111100000000");
        let b = bitset_from_str("11");
        assert!(a.start_with(&b), "{:?} doesn't start with {:?}", a, b);
    }


    #[test]
    fn shift_left_once_one_chunk() {
        let mut a = bitset_from_str("101");
        a.shift_left_once();
        assert_eq!(a, bitset_from_str("01"));
    }

    #[test]
    fn shift_left_once_multiple_chunk() {
        let mut a = bitset_from_str("1010101010100101010100101100101010011111111111111110");
        a.shift_left_once();
        assert_eq!(a, bitset_from_str("010101010100101010100101100101010011111111111111110"));
    }

    #[test]
    fn shift_left_once_long() {
        let mut a = bitset_from_str("0101010101111111111111110100101010101010101011111111111111\
0000000000000000000000000000000000000000000000000000000000000000111111111111100000011010100\
1010101001010101001010101001010101001010101010101001010010101010010010101010101010010");
        a.shift_left_once();
        assert_eq!(a, bitset_from_str("101010101111111111111110100101010101010101011111111111111\
0000000000000000000000000000000000000000000000000000000000000000111111111111100000011010100\
1010101001010101001010101001010101001010101010101001010010101010010010101010101010010"));
    }

    #[test]
    fn shift_left_once_one_bit() {
        let mut a = bitset_from_str("0");
        a.shift_left_once();
        assert_eq!(a, bitset_from_str(""));
    }

    #[test]
    fn shift_left_once_empty() {
        let mut a = bitset_from_str("");
        a.shift_left_once();
        assert_eq!(a, bitset_from_str(""));
    }

    #[test]
    fn shift_left_chunk_border_equal() {
        let mut a = bitset_from_str("01010101");
        a.shift_left_once();
        assert_eq!(a, bitset_from_str("1010101"));
    }

    #[test]
    fn shift_left_chunk_border_one_more() {
        let mut a = bitset_from_str("010101010");
        a.shift_left_once();
        assert_eq!(a, bitset_from_str("10101010"));
    }

    #[test]
    fn shift_right_once_one_chunk() {
        let mut a = bitset_from_str("101");
        a.shift_right_once();
        assert_eq!(a, bitset_from_str("0101"));
    }

    #[test]
    fn shift_right_once_multiple_chunk() {
        let mut a = bitset_from_str("1010101010100101010100101100101010011111111111111110");
        a.shift_right_once();
        assert_eq!(a, bitset_from_str("01010101010100101010100101100101010011111111111111110"));
    }

    #[test]
    fn shift_right_once_long() {
        let mut a = bitset_from_str("0101010101111111111111110100101010101010101011111111111111\
0000000000000000000000000000000000000000000000000000000000000000111111111111100000011010100\
1010101001010101001010101001010101001010101010101001010010101010010010101010101010010");
        a.shift_right_once();
        assert_eq!(a, bitset_from_str("00101010101111111111111110100101010101010101011111111111111\
0000000000000000000000000000000000000000000000000000000000000000111111111111100000011010100\
1010101001010101001010101001010101001010101010101001010010101010010010101010101010010"));
    }

    #[test]
    fn shift_right_once_one_bit() {
        let mut a = bitset_from_str("0");
        a.shift_right_once();
        assert_eq!(a, bitset_from_str("00"));
    }

    #[test]
    fn shift_right_once_empty() {
        let mut a = bitset_from_str("");
        a.shift_right_once();
        assert_eq!(a, bitset_from_str("0"));
    }

    #[test]
    fn shift_right_chunk_border_equal() {
        let mut a = bitset_from_str("01010101");
        a.shift_right_once();
        assert_eq!(a, bitset_from_str("001010101"));
    }

    #[test]
    fn shift_right_chunk_border_one_more() {
        let mut a = bitset_from_str("010101010");
        a.shift_right_once();
        assert_eq!(a, bitset_from_str("0010101010"));
    }

    #[test]
    fn shift_right_chunk_border_one_less() {
        let mut a = bitset_from_str("0101010");
        a.shift_right_once();
        assert_eq!(a, bitset_from_str("00101010"));
    }

    #[test]
    fn concat_one_chunk() {
        let mut a = bitset_from_str("101");
        a.concat(&bitset_from_str("1011"));
        assert_eq!(a, bitset_from_str("1011011"));
    }

    #[test]
    fn concat_origin_multiple_chunk() {
        let mut a = bitset_from_str("101010100101010010101111010110");
        a.concat(&bitset_from_str("1011"));
        assert_eq!(a, bitset_from_str("1010101001010100101011110101101011"));
    }

    #[test]
    fn concat_concatenated_multiple_chunk() {
        let mut a = bitset_from_str("1011");
        a.concat(&bitset_from_str("101010100101010010101111010110"));
        assert_eq!(a, bitset_from_str("1011101010100101010010101111010110"));
    }

    #[test]
    fn concat_both_multiple_chunk() {
        let mut a = bitset_from_str("1010100101010110101010011010101");
        a.concat(&bitset_from_str("101010100101010010101111010110"));
        assert_eq!(a,
            bitset_from_str("1010100101010110101010011010101101010100101010010101111010110"));
    }

    #[test]
    fn concat_origin_empty() {
        let mut a = bitset_from_str("");
        a.concat(&bitset_from_str("101010100101010010101111010110"));
        assert_eq!(a, bitset_from_str("101010100101010010101111010110"));
    }

    #[test]
    fn concat_concatenated_empty() {
        let mut a = bitset_from_str("101010100101010010101111010110");
        a.concat(&bitset_from_str(""));
        assert_eq!(a, bitset_from_str("101010100101010010101111010110"));
    }

    #[test]
    fn concat_both_empty() {
        let mut a = bitset_from_str("");
        a.concat(&bitset_from_str(""));
        assert_eq!(a, bitset_from_str(""));
    }
}
