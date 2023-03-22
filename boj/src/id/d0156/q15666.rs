// BOJ 15666 [N and M (12)]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn track(so: &mut io::BufWriter<io::StdoutLock>, n: usize, x: &[u16], v: &mut [u16], k: usize) {
    if k == 0 {
        v.iter().rev().for_each(|x| write!(so, "{} ", x).unwrap());
        writeln!(so).unwrap();
    } else {
        let mut mi = 0;
        for i in 0..n {
            if k >= v.len() {break;}
            if v[k] >= x[i] {
                mi = i;
            }
        }

        for i in mi..n {
            v[k - 1] = x[i];
            track(so, n, x, v, k - 1);
        }
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (_n, m): (usize, usize) = (next(&mut it), next(&mut it));
    let mut x = it.map(|x| x.parse().unwrap()).collect::<Vec<u16>>();
    x.sort();
    x.dedup();
    track(&mut so, x.len(), &x, &mut vec![0; m], m);
}