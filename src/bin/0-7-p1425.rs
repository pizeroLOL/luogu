use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut iter = st
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked());
        let sh = iter.next().unwrap_unchecked();
        let sm = iter.next().unwrap_unchecked();
        let eh = iter.next().unwrap_unchecked();
        let em = iter.next().unwrap_unchecked();
        let tm = (eh * 60 + em) - (sh * 60 + sm);
        println!("{} {}", tm / 60, tm % 60)
    }
}
