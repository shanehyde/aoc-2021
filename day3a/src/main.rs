use std::fs;

fn main() {
    let contents =
        fs::read_to_string("day3-input.txt").expect("Something went wrong reading the file");

    let lines: Vec<_> = contents.lines().collect();

    let bit_count = lines[0].len();

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for b in 0..bit_count {
        let mut zeroes: i32 = 0;
        let mut ones: i32 = 0;

        let mut bb: i32;

        for l in &lines {
            let ss = l.to_string();
            let s = ss.as_bytes();
            zeroes += if s[b] == '0' as u8 { 1 } else { 0 };
            ones += if s[b] == '1' as u8 { 1 } else { 0 };
        }

        if (zeroes > ones) {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }

    let gammai = isize::from_str_radix(&gamma, 2).unwrap();
    let epsiloni = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("Gamme = {}, Epsilon = {}", gamma, epsilon);
    println!(
        "Gamme = {}, Epsilon = {} power = {}",
        gammai,
        epsiloni,
        gammai * epsiloni
    );
}
