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

fn load_boards<'a, I>(lines: I) -> Vec<Board>
where
    I: Iterator<Item = &'a str>,
{
    let mut boards: Vec<Board> = Vec::new();
    let mut board_numbers: Vec<u32> = Vec::new();

    for line in lines {
        if line.len() > 2 {
            let mut nc = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>(); //.clone();

            board_numbers.append(&mut nc);
            if board_numbers.len() == 25 {
                boards.push(Board::new(board_numbers.clone().try_into().unwrap()));
                board_numbers.clear()
            }
        }
    }
    boards
}

fn main() {
    let contents =
        fs::read_to_string("day4-input.txt").expect("Something went wrong reading the file");

    let mut lines = contents.lines();
    let number_list = lines
        .nth(0)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap());

    let mut boards = load_boards(lines);

    for n in number_list.into_iter() {
        for b in &mut boards {
            b.match_number(n);
            if b.has_bingo() {
                println!("Answer = {}", b.get_unmarked() * n);
                return;
            }
        }
    }
}
