use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut it = st
            .split_ascii_whitespace()
            .map(|x| x.parse::<f32>().unwrap_unchecked());
        let m = it.next().unwrap_unchecked();
        let t = it.next().unwrap_unchecked();
        let s = it.next().unwrap_unchecked();
        let tmp = m - s / t;
        println!(
            "{}",
            if t == 0.0 {
                0
            } else if tmp < 0.0 {
                0
            } else {
                tmp as usize
            }
        );
    }
}
