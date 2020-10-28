#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
use std::io;
use std::str::{FromStr};
use core::convert::From;
use std::io::{BufWriter, Write, BufRead, BufReader};
use std::cmp::{min, max};
use std::collections::HashMap;

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
    3 5 4
    1 1 2 1
    2 2 2 3
    2 4 3 4
    2 4 2 5
    5
    1 1
    2 3
    1 4
    3 3
    3 5";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

#[test]
fn test_2() {
    let input = "\
    5 5 10
3 1 2 1
3 4 3 3
1 3 1 2
4 2 3 2
5 1 4 1
3 1 4 1
3 4 2 4
2 2 2 1
2 2 1 2
1 2 1 1
5
5 5
2 3
2 1
4 1
1 5";
    // let mut out: Vec<u8> = Vec::new();
    // run(input.as_bytes(), &mut out);
    run(input.as_bytes(), &mut io::stdout());
}

pub fn run<R: io::Read, O: io::Write>(input: R, output: &mut O) {
    // println!("{:#0130b}", 1_u128 << 127);
    let mut sc = Scanner::new(BufReader::new(input));
    let mut out = BufWriter::new(output);
    let n: usize = sc.next();
    let m: usize = sc.next();

    let mut mx: Vec<Vec<usize>> = Vec::with_capacity(n);
    for i in 0..n {
        mx.push(Vec::with_capacity(m));
        for _ in 0..m {
            mx[i].push(0);
        }
    }

    let k: i32 = sc.next();
    let mut wire: HashMap<((usize, usize), (usize, usize)), bool> = HashMap::new();
    for _ in 0..k {
        let (a, b) =
            ((sc.next::<usize>() - 1, sc.next::<usize>() - 1),
             (sc.next::<usize>() - 1, sc.next::<usize>() - 1));
        wire.insert((a, b), true);
        wire.insert((b, a), true);
    }

    let q: i32 = sc.next();
    for _ in 0..q {
        let pos = (sc.next::<usize>() - 1, sc.next::<usize>() - 1);
        let (x, y) = pos;

        let d = (x + y + 1)/2;

        for i in 0..=x {
            for j in 0..=y {
                mx[i][j] = 0;
            }
        }

        if x > 0 {
            for i in (0..x).rev() {
                mx[i][y] = mx[i + 1][y];
                if i + y < d && wire.contains_key(&((i, y), (i + 1, y))) {
                    mx[i][y] += 1;
                }
            }
        }
        if y > 0 {
            for j in (0..y).rev() {
                mx[0][j] = mx[0][j + 1];
                if j < d && wire.contains_key(&((0, j), (0, j + 1))) {
                    mx[0][j] += 1;
                }
            }
            for j in (0..y).rev() {
                for i in (0..=x).rev() {
                    if i + j >= d {
                        continue;
                    }
                    mx[i][j] = wire.contains_key(&((i, j), (i, j + 1))) as usize + mx[i][j + 1];
                    if i + 1 != x + 1 {
                        mx[i][j] = max(
                            mx[i][j],
                            wire.contains_key(&((i, j), (i + 1, j))) as usize + mx[i + 1][j]
                        )
                    }
                }
            }
        }
        writeln!(out, "{}", mx[0][0]);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    run(stdin.lock(), &mut stdout.lock());
}