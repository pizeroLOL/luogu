use std::io::stdin;

fn main() {
    unsafe {
        let nums = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .split_ascii_whitespace()
            .map(|x| x.parse::<f32>().unwrap_unchecked())
            .collect::<Vec<_>>();
        let p = nums.iter().sum::<f32>() / 2.0;
        let tmp = p * nums
            .iter()
            .map(|x| p - x)
            .reduce(|acc, now| acc * now)
            .unwrap_unchecked();
        println!("{:.1}", tmp.sqrt())
    }
}
