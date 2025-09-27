use std::io::stdin;

fn main() {
    unsafe {
        let mut lns = stdin().lines();
        let ln1 = lns
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<f64>()
            .unwrap_unchecked();
        let i = lns
            .map(|x| {
                let var_name = x.unwrap_unchecked();
                let mut x = var_name
                    .split_whitespace()
                    .map(|x| x.parse::<f64>().unwrap_unchecked());
                let (a, b) = (x.next().unwrap_unchecked(), x.next().unwrap_unchecked());
                ((ln1 / a).ceil() * b) as usize
            })
            .min()
            .unwrap_unchecked();
        println!("{i}")
    }
}
