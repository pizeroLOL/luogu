use std::io::stdin;

fn main() {
    unsafe {
        let data = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked())
            .reduce(|acc, now| acc * now)
            .unwrap_unchecked()
            .to_string();
        println!("{data}")
    }
}
