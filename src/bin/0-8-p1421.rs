use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut it = st.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap_unchecked());
        let y = it.next().unwrap_unchecked();
        let j = it.next().unwrap_unchecked();
        println!("{}", (y * 10 + j) / 19)
    }
}
