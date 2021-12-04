use std::fs;

struct Board {
    values: [u32; 25],
    matched: [bool; 25],
}

impl Board {
    fn new(v: [u32; 25]) -> Board {
        Board {
            values: v.clone(),
            matched: [false; 25],
        }
    }

    fn match_number(&mut self, n: u32) {
        for i in 0..25 {
            if self.values[i] == n {
                self.matched[i] = true;
            }
        }
    }

    fn get_unmarked(&self) -> u32 {
        let mut v: u32 = 0;
        for i in 0..25 {
            if !self.matched[i] {
                v += self.values[i];
            }
        }
        v
    }

    fn has_bingo(&self) -> bool {
        for x in 0..5 {
            if self.matched[x]
                && self.matched[x + 5]
                && self.matched[x + 10]
                && self.matched[x + 15]
                && self.matched[x + 20]
            {
                return true;
            }
            if self.matched[x * 5]
                && self.matched[x * 5 + 1]
                && self.matched[x * 5 + 2]
                && self.matched[x * 5 + 3]
                && self.matched[x * 5 + 4]
            {
                return true;
            }
        }

        false
    }
}

fn main() {
    let contents =
        fs::read_to_string("day4-input.txt").expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().into_iter().collect();
    let number_list: Vec<&str> = lines[0].split(',').collect();

    let numbers: Vec<u32> = number_list
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("Numbers = {}", numbers.len());

    let mut board_lines: Vec<&str> = lines.into_iter().skip(1).collect();

    let mut boards: Vec<Board> = vec![];

    let mut board_numbers: Vec<u32> = vec![];

    for nbv in board_lines {
        if nbv.len() > 2 {
            let c = nbv.split_whitespace();
            let mut nc: Vec<u32> = c.map(|x| x.parse::<u32>().unwrap()).collect();

            board_numbers.append(&mut nc);
            if board_numbers.len() == 25 {
                boards.push(Board::new(board_numbers.clone().try_into().unwrap()));
                board_numbers.clear()
            }
        }
    }

    for n in numbers {
        for b in &mut boards {
            b.match_number(n);
            if b.has_bingo() {
                println!("Answer = {}", b.get_unmarked() * n);
                return;
            }
        }
    }
}
