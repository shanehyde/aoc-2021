use std::arch::x86_64::__cpuid;
use std::fs;

#[derive(Debug, Clone)]
struct BitValue {
    v: Vec<char>,
}

impl BitValue {
    fn get_bit(&self, bit: usize) -> char {
        self.v[bit]
    }

    fn bit_count(&self) -> usize {
        self.v.len()
    }

    fn to_string(&self) -> String {
        String::from_iter(self.v.iter())
        //self.v.into_iter().collect()
    }

    fn len(&self) -> usize {
        self.bit_count()
    }

    fn get_dec(&self) -> isize {
        isize::from_str_radix(&*self.to_string(), 2).unwrap()
    }
}

fn count_bits(l1: &Vec<BitValue>, bit: usize) -> (usize, usize) {
    let zeroes = l1.iter().filter(|l| l.get_bit(bit) == '0').count();
    let ones = l1.iter().filter(|l| l.get_bit(bit) == '1').count();

    (zeroes, ones)
}

fn get_oxygen(l: &Vec<BitValue>) -> String {
    let mut lines = l.clone();
    let bit_count = lines[0].bit_count();

    for b in 0..bit_count {
        if lines.len() == 1 {
            break;
        }
        let (zeroes, ones) = count_bits(&lines, b);

        if zeroes > ones {
            let new_lines = lines.into_iter().filter(|x| x.get_bit(b) == '0');
            let nl: Vec<BitValue> = new_lines.collect();
            lines = nl.clone()
        } else {
            let new_lines = lines.into_iter().filter(|x| x.get_bit(b) == '1');
            let nl: Vec<BitValue> = new_lines.collect();
            lines = nl.clone()
        }
    }
    lines[0].to_string()
}

fn get_co2(l: &Vec<BitValue>) -> String {
    let mut lines = l.clone();
    let bit_count = lines[0].len();

    for b in 0..bit_count {
        if lines.len() == 1 {
            break;
        }
        let (zeroes, ones) = count_bits(&lines, b);

        if zeroes <= ones {
            let new_lines = lines.into_iter().filter(|x| x.get_bit(b) == '0');
            let nl: Vec<BitValue> = new_lines.collect();
            lines = nl.clone()
        } else {
            let new_lines = lines.into_iter().filter(|x| x.get_bit(b) == '1');
            let nl: Vec<BitValue> = new_lines.collect();
            lines = nl.clone()
        }
    }
    lines[0].to_string()
}

fn main() {
    let contents =
        fs::read_to_string("day3-input.txt").expect("Something went wrong reading the file");

    let mut lines: Vec<_> = contents
        .lines()
        .map(|x| BitValue {
            v: x.as_bytes().into_iter().map(|c| *c as char).collect(),
        })
        .collect();

    let ox = get_oxygen(&lines);
    let co2 = get_co2(&lines);

    let oxi = isize::from_str_radix(&*ox, 2).unwrap();
    let co2i = isize::from_str_radix(&*co2, 2).unwrap();

    println!("Gamme = {}, Epsilon = {}", oxi, co2i);
}
