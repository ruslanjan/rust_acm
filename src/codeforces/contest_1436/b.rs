#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
#![allow(unused_labels)]

use std::io;
use std::str::{FromStr};
use core::convert::From;
use std::io::{BufWriter, Write, BufRead, BufReader};
use std::cmp::{min};
use std::collections::{HashMap, BTreeMap};
use std::iter::Map;

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
fn test_sample_1() {
    let input = "2
    4
    2
    ";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

#[test]
fn test_7() {
    let input = "
    10
    39
    31
    34
    38
    35
    36
    40
    37
    33
    32
    ";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

pub fn run<R: io::Read, O: io::Write>(input: R, output: &mut O) {
    let mut sc = Scanner::new(BufReader::new(input));
    let mut out = BufWriter::new(output);

    let mut is_prime: BTreeMap<usize, bool> = BTreeMap::new();
    let mut primes = Vec::with_capacity(1e5 as usize);
    primes.push(false);
    primes.push(false);
    for _ in 2..1e5 as usize {
        primes.push(true);
    }

    let mut i = 2;
    'main: while i * i < 1e5 as usize {
        loop {
            break if primes[i] {
                is_prime.insert(i, true);
                let mut j = i * i;
                while j < 1e5 as usize {
                    primes[j] = false;
                    j += i
                }
            };
        }
        i += 1;
    }

    for i in 2..1e5 as usize {
        if primes[i] {
            is_prime.insert(i, true);
        }
    }

    let mut t: i32 = sc.next();
    while t > 0 {
        t -= 1;
        let n: usize = sc.next();
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(n);
        for i in 0..n {
            ans.push(Vec::with_capacity(n));
            for _ in 0..n {
                ans[i].push(1);
            }
        }
        for (prime, check) in is_prime.range(n + 1..1e5 as usize) {
            if !check { break; }
            let k = prime - n + 1;
            if !is_prime.contains_key(&(k)) {
                for j in 0..n {
                    ans[n - 1][j] = k as i32;
                    ans[j][n - 1] = k as i32;
                }
                let sum = k * (n - 1);
                for (prime, check) in is_prime.range(sum + 1..1e5 as usize) {
                    if !check { break; }
                    let q = prime - sum;
                    if !is_prime.contains_key(&q) {
                        ans[n - 1][n - 1] = q as i32;
                        break;
                    }
                }
                break;
            }
        }

        for i in 0..n {
            for j in 0..n {
                write!(out, "{} ", ans[i][j]);
            }
            write!(out, "\n");
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    run(stdin.lock(), &mut stdout.lock());
}