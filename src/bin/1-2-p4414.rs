use std::{fmt::Write, io::stdin};

fn main() {
    unsafe {
        // let list = ["A", "B", "C"];
        let mut lns = stdin().lines();
        let mut first = lns
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked())
            .collect::<Vec<_>>();
        first.sort();

        let sec = lns
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .chars()
            .map(|x| match x {
                'A' => first[0],
                'B' => first[1],
                'C' => first[2],
                _ => panic!(),
            })
            .enumerate()
            .fold(String::new(), |mut acc, (id, now)| {
                if id == 0 {
                    now.to_string()
                } else {
                    let _ = write!(acc, " {now}");
                    acc
                }
            });
        println!("{sec}")
    }
}
