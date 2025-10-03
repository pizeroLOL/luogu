use std::io::stdin;

fn main() {
    unsafe {
        let (mut v, _, c) = stdin()
            .lines()
            .skip(1)
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .split_ascii_whitespace()
            .map(|x| x.parse::<isize>().unwrap_unchecked())
            .enumerate()
            .fold(
                (Vec::new(), isize::MIN, 0),
                |(mut v, acc, ct), (id, now)| {
                    if id == 0 {
                        return (v, now, 0);
                    }
                    match now - acc == 1 {
                        true => (v, now, ct + 1),
                        false => {
                            v.push(ct + 1);
                            (v, now, 0)
                        }
                    }
                },
            );
        v.push(c + 1);
        println!("{}", v.iter().max().unwrap_unchecked())
    }
}
