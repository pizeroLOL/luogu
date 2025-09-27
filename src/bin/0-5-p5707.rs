use std::io::stdin;

fn main() {
    unsafe {
        let x = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut x = x
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap_unchecked());
        let s = x.next().unwrap_unchecked();
        let v = x.next().unwrap_unchecked();
        let day = 24 * 60;
        let time = (day + 8 * 60 - 10 - (s / v).ceil() as usize) % day;
        println!("{:02}:{:02}", time / 60, time % 60)
    }
}
