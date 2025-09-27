use std::io::stdin;

fn main() {
    unsafe {
        let collect = stdin()
            .lines()
            .map(|x| {
                x.unwrap_unchecked()
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<usize>().unwrap_unchecked())
                    .sum::<usize>()
            })
            .enumerate()
            .filter(|(_, x)| *x > 8)
            .collect::<Vec<_>>();
        let x = collect
            .into_iter()
            .rev()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .map(|(x, _)| x + 1)
            .unwrap_or(0);
        println!("{x}")
    }
}
