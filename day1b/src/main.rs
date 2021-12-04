use std::fs;

fn main() {
    let contents =
        fs::read_to_string("day1/day1-input.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();
    let lines_int2: Vec<i32> = lines.map(|x| x.parse::<i32>().unwrap()).collect();

    let mut result: i32 = 0;
    for x in 3..lines_int2.len() {
        if (lines_int2[x - 3] + lines_int2[x - 2] + lines_int2[x - 1])
            < (lines_int2[x - 2] + lines_int2[x - 1] + lines_int2[x - 0])
        {
            result += 1;
        }
    }

    println!("{}", result);
}
