use std::fs;

fn main() {
    let contents =
        fs::read_to_string("day1/day1-input.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();
    let lines_int = lines.map(|x| x.parse::<i32>().unwrap());

    let mut increase = 0;
    let mut current = 10000000;

    for x in lines_int {
        if x > current {
            increase += 1;
        }
        current = x;
    }
    println!("Increase count = {}", increase)
}
