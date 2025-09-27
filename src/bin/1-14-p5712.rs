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
        match st < 2 {
            true => println!("Today, I ate {st} apple."),
            false => println!("Today, I ate {st} apples."),
        }
    }
}
