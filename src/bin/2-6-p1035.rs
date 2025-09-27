use std::io::stdin;

fn main() {
    unsafe {
        let k = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<f64>()
            .unwrap_unchecked();
        let mut x = 0.0;
        let mut it = 0.0;
        while it <= k {
            x += 1.0;
            it += 1.0 / x;
        }
        println!("{x:.0}")
    }
}
