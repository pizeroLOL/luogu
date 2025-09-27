use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<usize>()
            .unwrap_unchecked();
        println!(
            "{}",
            (1..=usize::MAX)
                .flat_map(|x| (1..=x).map(move |_| x))
                .take(st)
                .sum::<usize>()
        )
    }
}
