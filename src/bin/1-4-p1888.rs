use std::io::stdin;

fn gcd(m: usize, n: usize) -> usize {
    // assert!(m != 0 && n != 0);
    if m > n {
        return gcd(m - n, n);
    };
    if m < n {
        return gcd(n - m, m);
    };
    m
}

fn main() {
    unsafe {
        let mut it = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked())
            .collect::<Vec<_>>();
        it.sort();
        let g = gcd(it[0], it[2]);
        println!("{}/{}", it[0] / g, it[2] / g);
    }
}
