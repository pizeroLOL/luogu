use std::{fmt::Write, io::stdin};

fn main() {
    unsafe {
        let w = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<usize>()
            .unwrap_unchecked();
        let mut it = 1..=usize::MAX;
        let x = (0..w)
            .map(|_| {
                (0..w).fold(String::new(), |mut acc, _| {
                    write!(acc, "{:02}", it.next().unwrap_unchecked()).unwrap_unchecked();
                    acc
                })
            })
            .fold(String::new(), |mut acc, now| {
                writeln!(acc, "{now}").unwrap_unchecked();
                acc
            });
        let mut it = 1..=usize::MAX;
        let y = (0..w)
            .rev()
            .map(|x| {
                "  ".repeat(x)
                    + &(0..w - x).map(|_| it.next().unwrap_unchecked()).fold(
                        String::new(),
                        |mut acc, n| {
                            write!(acc, "{n:02}").unwrap_unchecked();
                            acc
                        },
                    )
            })
            .fold(String::new(), |mut acc, now| {
                writeln!(acc, "{now}").unwrap_unchecked();
                acc
            });
        println!("{x}\n{y}");
    }
}
