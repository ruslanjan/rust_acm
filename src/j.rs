#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
use std::io;
use std::str::{FromStr};
use core::convert::From;
use std::io::{BufWriter, Write, BufRead, BufReader};
use std::cmp::{min};

struct BitSet {
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

struct Scanner<R> {
    reader: R,
    buff: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self { reader, buff: Vec::new() }
    }

    fn next<T: FromStr>(&mut self) -> T {
        loop {
            if let Some(n) = self.buff.pop() {
                break n.parse().ok().expect("Error Failed to Parse Type");
            }
            let mut line: String = String::new();
            self.reader.read_line(&mut line).expect("Failed to read line");
            self.buff = line.split_ascii_whitespace().rev().map(String::from).collect()
        }
    }
}

#[test]
fn test_bit_set() {
    let mut a = BitSet::new(500);
    let mut b = BitSet::new(500);
    a.set(245, 1);
    a.set(431, 1);
    b.or(&a);
    assert_eq!(b.count_ones(), 2)
}

#[test]
fn first_test() {
    let input = "1
    3 3
    100
    011
    111";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

pub fn run<R: io::Read, O: io::Write>(input: R, output: &mut O) {
    // println!("{:#0130b}", 1_u128 << 127);
    let mut sc = Scanner::new(BufReader::new(input));
    let mut out = BufWriter::new(output);
    let t: i32 = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let m: usize = sc.next();

        let mut a: Vec<BitSet> = Vec::with_capacity(m);
        let mut d = BitSet::new(n);

        for i in 0..m {
            let s: String = sc.next();
            a.push(BitSet::new(n));
            for (j, q) in s.chars().enumerate() {
                a[i].set(j, if q == '1' {1} else {0});
            }
            d.or(&a[i])
        }

        if d.count_ones() != n as u32 {
            writeln!(out, "-1");
            continue;
        }

        let mut ans: usize = m;

        for mask in 1..(1<<m) as i32 {
            // d.print();
            d.clear();
            // d.print();

            for i in 0..m {
                if (mask>>i)&1 == 1 {
                    d.or(&a[i])
                }
            }

            if d.count_ones() == n as u32 {
                ans = min(ans, mask.count_ones() as usize)
            }
        }

        writeln!(out, "{}", ans);
    }

}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    run(stdin.lock(), &mut stdout.lock());
}

/*
1
3 3
100
011
111
*/