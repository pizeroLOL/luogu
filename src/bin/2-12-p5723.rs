use std::{fmt::Write, io::stdin};

fn main() {
    unsafe {
        let mut st = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<usize>()
            .unwrap_unchecked();

        let it = (2..st).filter(|&x| (2..x).all(|y| x % y != 0));
        let mut ct = 0usize;
        let mut tmp = String::new();
        for i in it {
            st = match st.checked_sub(i) {
                Some(x) => {
                    writeln!(tmp, "{i}").unwrap_unchecked();
                    ct += 1;
                    x
                }
                None => break,
            }
        }
        println!("{}", tmp + ct.to_string().as_str())
    }
}
