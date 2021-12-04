use std::fs;

fn main() {
    let contents =
        fs::read_to_string("day1/day1-input.txt").expect("Something went wrong reading the file");

    let (_, c) = contents
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((100000000, 0), |(prev, count), x| {
            (x, count + if x > prev { 1 } else { 0 })
        });

    println!("Increase count = {}", c);
}
