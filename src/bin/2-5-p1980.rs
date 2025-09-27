use std::io::stdin;

fn ta(n: usize, c: usize, t: usize) -> usize {
    if n == 0 {
        return c;
    }
    ta(n / 10, (n % 10 == t) as usize + c, t)
}

fn main() {
    unsafe {
        let next = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut st = next
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked());
        let a = st.next().unwrap_unchecked();
        let b = st.next().unwrap_unchecked();
        println!("{}", (1..=a).map(|x| ta(x, 0, b)).sum::<usize>())
    }
}
