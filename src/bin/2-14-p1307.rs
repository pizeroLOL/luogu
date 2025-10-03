use std::io::stdin;

fn main() {
    unsafe {
        let x = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let is_neg = x.starts_with('-');
        let var_name = String::from_utf8(
            x.as_bytes()
                .into_iter()
                .skip(is_neg as usize)
                .rev()
                .copied()
                .collect::<Vec<_>>(),
        )
        .unwrap_unchecked();
        let o = var_name.trim_start_matches('0');
        println!(
            "{}{}",
            match is_neg {
                true => "-",
                false => "",
            },
            match o.is_empty() {
                true => "0",
                false => o,
            }
        )
    }
}
