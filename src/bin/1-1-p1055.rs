use std::io::stdin;

fn main() {
    unsafe {
        let st = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut iter = st.split('-');
        let first = iter.next().unwrap_unchecked();
        let sec = iter.next().unwrap_unchecked();
        let third = iter.next().unwrap_unchecked();
        let e = iter.next().unwrap_unchecked();
        let v = [first, sec, third]
            .into_iter()
            .flat_map(|x| x.chars().map(|x| x.to_digit(10).unwrap_unchecked()))
            .zip(1..=9)
            .map(|(x, y)| x * y)
            .sum::<u32>()
            % 11;
        let v = if v < 10 {
            v.to_string()
        } else {
            "X".to_string()
        };
        if e == v {
            println!("Right");
        } else {
            println!("{first}-{sec}-{third}-{v}")
        }
    }
}
