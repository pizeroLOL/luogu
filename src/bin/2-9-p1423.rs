use std::io::stdin;

fn main() {
    unsafe {
        let mut st = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<f64>()
            .unwrap_unchecked();
        let mut i = 0usize;
        let mut status = 2.0;
        while st > 0.0 {
            st -= status;
            i += 1;
            status *= 0.98
        }
        println!("{i}")
    }
}
