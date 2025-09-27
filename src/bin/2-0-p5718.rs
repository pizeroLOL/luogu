use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin()
            .lines()
            .nth(1)
            .unwrap_unchecked()
            .unwrap_unchecked()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked())
            .min()
            .unwrap_unchecked();
        println!("{st}")
    }
}
