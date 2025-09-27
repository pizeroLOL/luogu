use std::io::stdin;

fn main() {
    unsafe {
        let mut o = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked())
            .collect::<Vec<_>>();
        o.sort();
        println!("{} {} {}", o[0], o[1], o[2])
    }
}
