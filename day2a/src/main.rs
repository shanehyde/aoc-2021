use std::fs;

fn main() {
    let contents =
        fs::read_to_string("day2-input.txt").expect("Something went wrong reading the file");

    let lines = contents
        .lines()
        .map(|x| {
            let zz: Vec<_> = x.split(" ").collect();
            (zz[0], zz[1].parse::<i32>().unwrap())
        })
        .fold((0, 0), |(pos, depth), (command, value)| match command {
            "forward" => (pos + value, depth),
            "down" => (pos, depth + value),
            "up" => (pos, depth - value),
            _ => (pos, depth),
        });

    println!("Depth = {}, position = {}", lines.1, lines.0);
}
