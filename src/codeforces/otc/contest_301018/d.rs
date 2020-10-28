#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
use std::io;
use std::str::{FromStr};
use core::convert::From;
use std::io::{BufWriter, Write, BufRead, BufReader};
use std::cmp::{min};

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
fn first_test() {
    let input = "
    ..|....
    ....|...
    ";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

#[test]
fn test_2() {
    let input = "
    .|...
    ..|....
    ";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

#[test]
fn test_3() {
    let input = "
    |
    |
    ";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

pub fn run<R: io::Read, O: io::Write>(input: R, output: &mut O) {
    // println!("{:#0130b}", 1_u128 << 127);
    let mut sc = Scanner::new(BufReader::new(input));
    let mut out = BufWriter::new(output);
    let a: String = sc.next();
    let b: String = sc.next();
    let mut al = 0;
    let mut ar = 0;
    let mut bl = 0;
    let mut br = 0;

    let mut iter = a.chars();
    while let Some(c)  = iter.next() {
        if c == '|' {
            break;
        }
        al += 1;
    }
    while let Some(_)  = iter.next() {
        ar += 1;
    }

    let mut iter = b.chars();
    while let Some(c)  = iter.next() {
        if c == '|' {
            break;
        }
        bl += 1;
    }
    while let Some(_)  = iter.next() {
        br += 1;
    }

    if al == bl || ar == br || al == br || ar == bl {
        write!(out, "Yes");
    } else {
        write!(out, "No");
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    run(stdin.lock(), &mut stdout.lock());
}