use std::io::stdin;

fn main() {
    unsafe {
        let mut lns = stdin().lines();
        let first = lns.next().unwrap_unchecked().unwrap_unchecked();
        let first = first
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked());
        let sec = lns
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<usize>()
            .unwrap_unchecked()
            + 30;
        println!("{}", first.filter(|&x| x <= sec).count())
    }
}
