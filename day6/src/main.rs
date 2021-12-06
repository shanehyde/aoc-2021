use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn add(hm: &mut HashMap<u8, u64>, k: u8, count: u64) {
    hm.insert(k, count + if hm.contains_key(&k) { hm[&k] } else { 0 });
}

fn advance2(f: &mut HashMap<u8, u64>, n: u32) {
    for i in 1..=n {
        let ff = f.drain().sorted().collect::<Vec<_>>();

        print!("Generation = {} ", i);

        for (age, count) in ff {
            print!("{} -> {}, ", age, count);
            if age == 0 {
                add(f, 6, count);
                add(f, 8, count);
            } else {
                add(f, age - 1, count);
            }
        }
        println!()
    }
}

fn main() {
    let contents =
        fs::read_to_string("day6-input.txt").expect("Something went wrong reading the file");

    let mut lines = contents.lines();
    let l = lines.next().unwrap();

    let s = l
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .sorted()
        .dedup_with_count()
        .collect::<Vec<_>>();

    println!("{:?}", s);

    let mut hm: HashMap<u8, u64> = HashMap::new();
    for (count, k) in s.iter() {
        hm.insert(*k, *count as u64);
    }

    {
        let mut hm2 = hm.clone();
        advance2(&mut hm2, 80);
        println!("Fish count = {}", hm2.values().sum::<u64>())
    }
    {
        let mut hm2 = hm.clone();

        advance2(&mut hm2, 256);
        println!("Fish count = {}", hm2.values().sum::<u64>())
    }
}
