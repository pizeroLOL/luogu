use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let n = st.parse::<usize>().unwrap_unchecked();
        match n * 3 + 11 < n * 5 {
            true => println!("Luogu"),
            false => println!("Local"),
        }
    }
}
