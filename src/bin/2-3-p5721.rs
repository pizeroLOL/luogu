use std::{fmt::Write, io::stdin};

fn main() {
    unsafe {
        let n = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<usize>()
            .unwrap_unchecked();
        let (_, s) = (1..=n).rev().fold((0, String::new()), |(c, mut s), now| {
            (1..now + 1)
                .try_for_each(|x| write!(s, "{:02}", x + c))
                .unwrap_unchecked();
            write!(s, "\n").unwrap_unchecked();
            (c + now, s)
        });
        println!("{s}");
    }
}
