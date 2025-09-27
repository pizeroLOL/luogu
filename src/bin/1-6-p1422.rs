use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<f64>()
            .unwrap_unchecked();
        let mut x = 0.0;
        if st <= 150.0 {
            println!("{:.1}", st * 0.4463);
            return;
        }
        x += 150.0 * 0.4463;
        if st <= 400.0 {
            println!("{:.1}", x + (st - 150.0) * 0.4663);
            return;
        }
        x += (400.0 - 150.0) * 0.4663;
        println!("{:.1}", x + (st - 400.0) * 0.5663);
    }
}
