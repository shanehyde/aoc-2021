use std::fs;

fn main() {
    let contents =
        fs::read_to_string("day1/day1-input.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();
    let lines_int = lines.map(|x| x.parse::<i32>().unwrap());

    let res = lines_int.fold((100000000, 0), |acc, x| {
        (x, acc.1 + if x > acc.0 { 1 } else { 0 })
    });
    println!("Increase count = {}", res.1);
}
