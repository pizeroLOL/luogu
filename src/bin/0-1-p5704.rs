use std::io::stdin;

fn main() {
    unsafe {
        println!(
            "{}",
            stdin()
                .lines()
                .next()
                .unwrap_unchecked()
                .unwrap_unchecked()
                .to_uppercase()
        )
    }
}
