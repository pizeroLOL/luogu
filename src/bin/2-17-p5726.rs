use std::io::stdin;

fn main() {
    unsafe {
        let mut t = stdin()
            .lines()
            .skip(1)
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked())
            .collect::<Vec<_>>();
        t.sort();
        let x = t[1..t.len() - 1].iter().map(|&x| x as f32).sum::<f32>() / (t.len() as f32 - 2.0);
        println!("{x:.2}",)
    }
}
