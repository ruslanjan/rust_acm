#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
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
    let input = "123";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

pub fn run<R: io::Read, O: io::Write>(input: R, output: &mut O) {
    // println!("{:#0130b}", 1_u128 << 127);
    let mut sc = Scanner::new(input);
    let mut out = BufWriter::new(output);
    let n: i32 = sc.next();
    writeln!(out, "{}", n);

}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    run(stdin.lock(), &mut stdout.lock());
}