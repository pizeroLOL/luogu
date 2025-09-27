use std::io::stdin;

fn main() {
    unsafe {
        let mut i = stdin()
            .lines()
            .next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap_unchecked())
            .collect::<Vec<_>>();
        i.sort();
        let (a, b, c) = (i[0], i[1], i[2]);
        if a + b <= c || b + c <= a || a + c <= b {
            println!("Not triangle");
            return;
        }
        use std::cmp::Ordering::*;
        println!(
            "{}",
            match (a.pow(2) + b.pow(2)).cmp(&c.pow(2)) {
                Less => "Obtuse triangle",
                Equal => "Right triangle",
                Greater => "Acute triangle",
            }
        );
        if a == b || b == c || a == c {
            println!("Isosceles triangle")
        };
        if a == b && b == c {
            println!("Equilateral triangle")
        }
    }
}
