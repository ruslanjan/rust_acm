#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]

use std::io;
use std::str::{FromStr};
use core::convert::From;
use std::io::{BufWriter, Write, BufRead, BufReader};
use std::cmp::{min};
use std::ops::{Add, AddAssign};
use std::cell::Cell;

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
fn test_1() {
    let input = "\
    4
abacada
abracadabra
rxacadd
dzzzaca
";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

#[test]
fn test_2() {
    let input = "\
2
abacaba
acabaab

";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

const Z: Cell<Vec<usize>> = Cell::new(Vec::new());

fn check(prefix: &str, s: &str) -> bool {
    let n = s.len() + prefix.len() + 1;
    let a = prefix.to_string()  + "$" + s;
    for _ in Z.get_mut().len()..n {
        z.push(0);
    }
    let mut r = 0;
    let mut l = 0;
    for i in 1..n {
        if i <= r {
            z[i] = min(r - i + 1, z[i - l]);
        }
        while i + z[i] < n && a.as_bytes()[z[i]] == a.as_bytes()[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
        if z[i] == prefix.len() {
            return true;
        }
    }
    return false;
}

pub fn run<R: io::Read, O: io::Write>(input: R, output: &mut O) {
    // println!("{:#0130b}", 1_u128 << 127);
    let mut sc = Scanner::new(BufReader::new(input));
    let mut out = BufWriter::new(output);
    let n: usize = sc.next();

    let mut a: Vec<String> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(sc.next::<String>());
    }

    if n == 1 {
        writeln!(out, "{}", a[0].len());
        return;
    }

    a.sort_by(|a, b| {
        a.len().cmp(&b.len()).reverse()
    });

    let max_len = a.last().unwrap().len();
    let w = a.last().unwrap().clone() + a.last().unwrap();
    a.pop();
    for i in &mut a {
        i.push_str(i.clone().as_str());
    }

    let mut l = 0;
    let mut r = max_len + 1;
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut good = false;
        for i in 0..w.len() - m {
            let prefix = &w[i..i + m];
            good |= &a.iter().fold(true, |acc, j| {
                acc && check(prefix, j.as_str())
            });
        }
        if good {
            l = m;
        } else {
            r = m;
        }
    }
    writeln!(out, "{}", l);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    run(stdin.lock(), &mut stdout.lock());
}