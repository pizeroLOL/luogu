fn main() {}

// use std::io::stdin;

// fn main() {
//     let t = unsafe {
//         stdin()
//             .lines()
//             .next()
//             .unwrap_unchecked()
//             .unwrap_unchecked()
//             .parse::<u8>()
//             .unwrap_unchecked()
//     };
//     println!(
//         "{}",
//         match t {
//             1 => "I love Luogu!",
//             2 => "6 2",
//             3 => "3 12 2",
//             4 => "166.666666",
//             5 => "15",
//             6 => "10",
//             7 => "110\n90\n0\n",
//             8 => {
//                 let pi = 3.141593;
//                 format!(
//                     "{}\n{}\n{}",
//                     2.0 * pi * 5.0,
//                     25.0 * pi,
//                     (((4.0 / 3.0) * pi * 5.0) as f64).powi(3)
//                 )
//                 .leak()
//             }
//             9 => "15",
//             10 => "",
//             _ => panic!(),
//         }
//     )
// }
