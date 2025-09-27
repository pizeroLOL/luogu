use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut iter = st
            .split_ascii_whitespace()
            .map(|x| x.parse::<f32>().unwrap_unchecked());
        let x = iter.next().unwrap_unchecked();
        let y = iter.next().unwrap_unchecked();
        let z = iter.next().unwrap_unchecked();
        println!("{}", x * 0.2 + y * 0.3 + z * 0.5)
    }
}
