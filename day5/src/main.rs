use std::cmp::{max, min, Ordering};
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
            grid: vec![0; (maxx * maxy)],
        }
    }

    fn draw_any_line(&mut self, line: &Line) {
        let mut cx = line.x1;
        let mut cy = line.y1;

        let x_off = match line.x1.cmp(&line.x2) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let y_off = match line.y1.cmp(&line.y2) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

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
        self.grid
            .iter()
            .fold(0, |vv, x| if x >= &v { vv + 1 } else { vv })
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn main() {
    let contents =
        fs::read_to_string("day5-input.txt").expect("Something went wrong reading the file");

    //let lines: Vec<&str> = contents.lines().into_iter().collect();
    let mut lines = contents.lines();

    let lx: Vec<Line> = lines
        .map(|x| {
            let xx = x.split(" ").collect::<Vec<_>>();
            let start = xx[0].split(",").collect::<Vec<_>>();
            let end = xx[2].split(",").collect::<Vec<_>>();

            let x1 = start[0].parse::<i32>().unwrap();
            let y1 = start[1].parse::<i32>().unwrap();
            let x2 = end[0].parse::<i32>().unwrap();
            let y2 = end[1].parse::<i32>().unwrap();

            Line { x1, y1, x2, y2 }
        })
        .collect();

    let maxx = lx.iter().fold(0, |m, x: &Line| max(max(m, x.x1), x.x2));
    let maxy = lx.iter().fold(0, |m, x: &Line| max(max(m, x.y1), x.y2));

    let mut w = World::new((maxx + 1) as usize, (maxy + 1) as usize);
    let mut w2 = World::new((maxx + 1) as usize, (maxy + 1) as usize);

    for lxx in lx {
        w.draw_line_horiz_vert(&lxx);
        w2.draw_any_line(&lxx);
    }

    println!("Overlap count 1 = {}", w.get_overlap_count(2));
    println!("Overlap count 2 = {}", w2.get_overlap_count(2));
}
