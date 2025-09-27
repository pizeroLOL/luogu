use std::{io::stdin, usize};

fn main() {
    unsafe {
        let (mn, mx) = stdin()
            .lines()
            .skip(1)
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked())
            .fold((usize::MAX, usize::MIN), |(mn, mx), now| {
                match (now < mn, now > mx) {
                    (true, _) => (now, mx),
                    (_, true) => (mn, now),
                    _ => (mn, mx),
                }
            });
        println!("{}", mx.checked_sub(mn).unwrap_or(0))
    }
}
