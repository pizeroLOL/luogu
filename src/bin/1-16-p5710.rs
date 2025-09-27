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
        let a = st % 2 == 0;
        let b = 4 <= st && st <= 12;
        println!(
            "{} {} {} {}",
            (a && b) as usize,
            (a || b) as usize,
            ((a && !b) || (!a && b)) as usize,
            !(a | b) as usize
        )
    }
}
