use std::fs;

fn main() {
    let contents =
        fs::read_to_string("day2-input.txt").expect("Something went wrong reading the file");

    let res = contents
        .lines()
        .map(|x| {
            let zz: Vec<_> = x.split(" ").collect();
            (zz[0], zz[1].parse::<i32>().unwrap())
        })
        .fold(
            (0, 0, 0),
            |(pos, depth, aim), (command, value)| match command {
                "forward" => (pos + value, depth + value * aim, aim),
                "down" => (pos, depth, aim + value),
                "up" => (pos, depth, aim - value),
                _ => (pos, depth, aim),
            },
        );

    println!("Depth = {}, position = {}", res.1, res.0);
}
