use std::io::stdin;

fn main() {
    unsafe {
        let iter = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut iter = iter
            .split_whitespace()
            .map(|x| x.parse::<f32>().unwrap_unchecked());
        let h = iter.next().unwrap_unchecked();
        let r = iter.next().unwrap_unchecked();
        let pi_like = 3.14;
        let tmp = r * r * pi_like * h / 1000.0;
        println!("{:.0}", (20.0 / tmp).ceil())
    }
}
