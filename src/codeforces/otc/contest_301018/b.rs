#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
use std::io;
use std::str::{FromStr};
use core::convert::From;
use std::io::{BufWriter, Write, BufRead, BufReader};
use std::cmp::{min};
use std::collections::{BTreeSet, HashSet, HashMap};
use std::hash::Hash;

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
    let input = "7
00:05 A RE
00:07 A WA
00:09 A TL
00:29 B OK
00:55 D WA
01:08 A CE
01:10 A OK
";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

#[test]
fn test_2() {
    let input = "7
00:05 A RE
00:07 A WA
00:09 A TL
00:29 B OK
00:55 D WA
01:08 A CE
04:59 A OK
";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

pub fn run<R: io::Read, O: io::Write>(input: R, output: &mut O) {
    // println!("{:#0130b}", 1_u128 << 127);
    let mut sc = Scanner::new(BufReader::new(input));
    let mut out = BufWriter::new(output);
    let n: i32 = sc.next();
    let mut cnt = 0;
    let mut penalty = 0;
    let mut s: HashSet<char> = HashSet::new();
    let mut tries: HashMap<char, i32> = HashMap::new();
    for i in 'A' as u8..='Z' as u8 {
        tries.insert(i as char, 0);
    }
    for _ in 0..n {
        let time: String = sc.next();
        let problem: char = sc.next();
        let status: String = sc.next();
        let bad = i32::from_str(&time[..2]).expect("failed to read time H")*60 +
            i32::from_str(&time[3..]).expect("failed to read time M") +
            20 * tries.get(&problem).unwrap();

        match status.as_str() {
            _ if s.contains(&problem) => {}
            "OK" => {
                penalty += bad;
                s.insert(problem);
                cnt += 1;
            }
            "CE" => {}
            _ => {
                *tries.get_mut(&problem).unwrap() += 1;
            }
        }
        // writeln!(out, "{}-{}-{}", time, problem, status);
        // writeln!(out, "{} {}", cnt, penalty);
    }
    writeln!(out, "{} {}", cnt, penalty);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    run(stdin.lock(), &mut stdout.lock());
}