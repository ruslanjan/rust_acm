#![allow(dead_code)]

pub struct BitSet {
    bits: Vec<u128>,
    size: usize
}

impl BitSet {
    fn new(n: usize) -> BitSet {
        let mut bits = Vec::with_capacity((n + 127)/128);

        for _ in 0..(n + 127)/128 {
            bits.push(0)
        }

        BitSet {
            bits,
            size: n
        }
    }

    fn get(&self, i: usize) -> u128 {
        return (self.bits[i/128]>>(i%128))&1
    }

    fn set(&mut self, i: usize, v: u128) {
        if self.get(i) != v {
            self.bits[i / 128] ^= v << (i % 128);
        }
    }

    fn count_ones(&self) -> u32 {
        self.bits.iter().fold(0, |a, v| {
            a + v.count_ones()
        })
    }

    // fn or(self, rhs: &BitSet) -> Self::Output {
    //     if self.size >= rhs.size {
    //         BitSet {
    //             bits: self.bits.iter().enumerate().map(|(i, v)|
    //                 if i < rhs.bits.len() {
    //                     *v | rhs.bits[i]
    //                 } else {
    //                     *v
    //                 }).collect(),
    //             size: self.size,
    //         }
    //     } else {
    //         BitSet {
    //             bits: rhs.bits.iter().enumerate().map(|(i, v)|
    //                 if i < self.bits.len() {
    //                     *v | self.bits[i]
    //                 } else {
    //                     *v
    //                 }).collect(),
    //             size: rhs.size,
    //         }
    //     }
    // }
    fn or(&mut self, rhs: &BitSet) {
        if rhs.size > self.size {
            self.size = rhs.size;
            self.bits.reserve(rhs.bits.len() - self.bits.len());
            for _ in 0..(rhs.bits.len() - self.bits.len()) {
                self.bits.push(0);
            }
        }
        for i in 0..rhs.bits.len() {
            self.bits[i] |= rhs.bits[i];
        }
    }

    fn clear(&mut self) {
        for b in &mut self.bits {
            *b = 0;
        }
    }

    fn print(&self) {
        for b in &self.bits {
            println!("{:#0130b}", *b)
        }
    }
}

impl Clone for BitSet {
    fn clone(&self) -> Self {
        BitSet {
            bits: self.bits.clone(),
            size: self.size
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.bits = source.bits.clone();
        self.size = source.size
    }
}