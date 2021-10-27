#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]

use std::cmp::max;
use std::io;
use std::str::{FromStr};
use std::io::{BufWriter, Write, BufRead, BufReader, Read};

struct Scanner<R> {
    reader: BufReader<R>,
    buff: Vec<String>,
}

impl<R: Read> Scanner<R> {
    fn new(reader: R) -> Self {
        Self { reader: BufReader::new(reader), buff: Vec::new() }
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
    let input = "5
3 7 4 6 5";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

#[test]
fn test_2() {
    let input = "5
2 1 5 8 4";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

#[test]
fn test_3() {
    let input = "5
3 5 -7 8 10";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}



static MIN: i64 = i64::min_value();


pub fn run<R: io::Read, O: io::Write>(input: R, output: &mut O) {
    // println!("{:#0130b}", 1_u128 << 127);
    let mut sc = Scanner::new(input);
    let mut out = BufWriter::new(output);
    let n: usize = sc.next();
    let mut a = Vec::<i64>::with_capacity(n);
    let mut ans = MIN;
    let mut dp = Vec::with_capacity(n);
    for i in 0..n {
        a.push(sc.next());
        dp.push(a[i]);
        if i > 1 {
            dp[i] = *[dp[i - 2] + a[i], dp[i - 1], dp[i - 2], dp[i]].iter().max().unwrap();
        }
        ans = max(ans, dp[i]);
    }
    writeln!(out, "{}", ans);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    run(stdin.lock(), &mut stdout.lock());
}