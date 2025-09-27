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
            (st % 400 == 0 || st % 4 == 0 && st % 100 != 0) as usize
        )
    }
}
