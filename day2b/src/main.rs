use std::fs;

fn main() {
    let contents =
        fs::read_to_string("day2-input.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();

    let mut forward_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for l in lines {
        let sp: Vec<&str> = l.split(" ").collect();

        match sp[0] {
            "forward" => {
                let v = sp[1].parse::<i32>().unwrap();
                forward_position += v;
                depth += v * aim;
            }
            "down" => aim += sp[1].parse::<i32>().unwrap(),
            "up" => aim -= sp[1].parse::<i32>().unwrap(),
            _ => {}
        }
    }

    println!("Depth = {}, position = {}", depth, forward_position);
}
