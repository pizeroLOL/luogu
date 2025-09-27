use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .chars()
            .rev()
            .collect::<String>();
        println!("{st}")
    }
}
