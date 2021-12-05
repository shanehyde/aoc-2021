use itertools::Itertools;
use std::fs;

struct World {
    width: i32,
    grid: Vec<u8>,
}

impl World {
    fn new(maxx: usize, maxy: usize) -> World {
        //println!("New world size = {},{}", maxx, maxy);
        World {
            width: maxx as i32,
            grid: vec![0; maxx * maxy],
        }
    }

    fn draw_any_line(&mut self, line: &Line) {
        let mut cx = line.x1;
        let mut cy = line.y1;

        let x_off = (line.x2 - line.x1).signum();
        let y_off = (line.y2 - line.y1).signum();

        loop {
            let off: usize = (cx + cy * self.width) as usize;
            //println!("xx = {}, yy = {}, off ={}", cx, cy, off);
            self.grid[off] += 1;
            cx += x_off;
            cy += y_off;
            if cx == line.x2 + x_off && cy == line.y2 + y_off {
                return;
            }
        }
    }

    fn draw_line_horiz_vert(&mut self, line: &Line) {
        //println!("Processing line - {:?}", line);
        if line.x1 == line.x2 {
            self.draw_any_line(&line);
        }
        if line.y1 == line.y2 {
            self.draw_any_line(&line);
        }
    }

    fn get_overlap_count(&self, v: u8) -> usize {
        self.grid.iter().filter(|x| x >= &&v).count()
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let mut w = s
            .split(" -> ")
            .flat_map(|x| x.split(","))
            .map(|x| x.parse::<i32>().unwrap()); //.collect::<Vec<_>>();
        Line {
            x1: w.next().unwrap(),
            y1: w.next().unwrap(),
            x2: w.next().unwrap(),
            y2: w.next().unwrap(),
        }
    }
}

fn main() {
    let contents =
        fs::read_to_string("day5-input.txt").expect("Something went wrong reading the file");

    let lx = contents.lines().map_into::<Line>().collect::<Vec<_>>();

    let maxx = lx.iter().flat_map(|x| [x.x1, x.x2]).max().unwrap();
    let maxy = lx.iter().flat_map(|x| [x.y1, x.y2]).max().unwrap();

    let mut w = World::new((maxx + 1) as usize, (maxy + 1) as usize);
    let mut w2 = World::new((maxx + 1) as usize, (maxy + 1) as usize);

    for lxx in lx {
        w.draw_line_horiz_vert(&lxx);
        w2.draw_any_line(&lxx);
    }

    println!("Overlap count 1 = {}", w.get_overlap_count(2));
    println!("Overlap count 2 = {}", w2.get_overlap_count(2));
}
