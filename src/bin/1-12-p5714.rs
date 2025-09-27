use std::io::stdin;

fn main() {
    unsafe {
        let next = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut x = next
            .split_ascii_whitespace()
            .map(|x| x.parse::<f32>().unwrap_unchecked());
        let m = x.next().unwrap_unchecked();
        let h = x.next().unwrap_unchecked();
        let it = m / h.powi(2);
        if it < 18.5 {
            println!("Underweight")
        } else if it < 24.0 {
            println!("Normal")
        } else {
            println!(
                "{}\nOverweight",
                format!("{it:.4}")
                    .trim_end_matches('0')
                    .trim_end_matches('.')
            );
        }
    }
}
