use std::io::stdin;

fn main() {
    unsafe {
        let x = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<i32>()
            .unwrap_unchecked();
        let pow_5 = 5.0f64.sqrt();
        println!(
            "{:.2}",
            (((1.0 + pow_5) / 2.0).powi(x) - ((1.0 - pow_5) / 2.0).powi(x)) / pow_5
        )
    }
}
