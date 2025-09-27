use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<usize>()
            .unwrap_unchecked();
        println!(
            "{}",
            std::mem::size_of::<usize>() * 8 - st.leading_zeros() as usize
        )
    }
}
