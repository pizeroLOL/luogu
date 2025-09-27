use std::io::stdin;

fn main() {
    unsafe {
        println!(
            "{}",
            (1..=stdin()
                .lines()
                .next()
                .unwrap_unchecked()
                .unwrap_unchecked()
                .parse::<usize>()
                .unwrap_unchecked())
                .sum::<usize>()
        )
    }
}
