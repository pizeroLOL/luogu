use std::io::stdin;

fn main() {
    unsafe {
        let next = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut st = next
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked());
        let y = st.next().unwrap_unchecked();
        let m = st.next().unwrap_unchecked() - 1;
        let d = [
            31usize,
            if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
                29
            } else {
                28
            },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ][m];
        println!("{d}")
    }
}
