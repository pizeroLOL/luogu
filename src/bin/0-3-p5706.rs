use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut iter = st.split_whitespace();
        let t: f32 = iter.next().unwrap_unchecked().parse().unwrap_unchecked();
        let n: f32 = iter.next().unwrap_unchecked().parse().unwrap_unchecked();
        println!("{:.3}\n{}\n", t / n, n * 2.0)
    }
}
