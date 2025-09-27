use std::io::stdin;

fn main() {
    unsafe {
        let a = stdin().lines().next().unwrap_unchecked().unwrap_unchecked();
        let mut it = a
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked());
        let n = it.next().unwrap_unchecked();
        let k = it.next().unwrap_unchecked();
        let (as_, ac, bs, bc) = (1..=n).fold(
            (0usize, 0usize, 0usize, 0usize),
            |(as_, ac, bs, bc), now| match now % k == 0 {
                true => (as_ + now, ac + 1, bs, bc),
                false => (as_, ac, bs + now, bc + 1),
            },
        );
        println!("{:.1} {:.1}", as_ as f32 / ac as f32, bs as f32 / bc as f32)
    }
}
