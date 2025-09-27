use std::io::stdin;

fn main() {
    unsafe {
        let next = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut st = next
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked());
        let start = st.next().unwrap_unchecked() - 1;
        let days = st.next().unwrap_unchecked();
        let ir = days % 7;
        let of = 5 - if start > 4 { 4 } else { start };
        let x = days / 7 * 5 + if ir <= of { ir } else { ir - of };
        println!("{}", x * 250)
    }
}
