use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn advance2(f: &mut HashMap<u8, u64>, n: u32) {
    for i in 0..n {
        let ff = f.drain().collect::<Vec<_>>();

        print!("Generation = {} ", i);

        for (age, count) in ff {
            print!("{} -> {}, ", age, count);
            if age == 0 {
                f.insert(6, count + if f.contains_key(&6) { f[&6] } else { 0 });
                f.insert(8, count);
            } else {
                f.insert(
                    age - 1,
                    count
                        + if f.contains_key(&(age - 1)) {
                            f[&(age - 1)]
                        } else {
                            0
                        },
                );
            }
        }
        println!("")
    }
}

fn advance(f: &mut Vec<u32>, n: u32) {
    for _i in 0..n {
        let mut new_count = 0;

        for j in 0..f.len() {
            if f[j] == 0 {
                f[j] = 6;
                new_count += 1;
            } else {
                f[j] -= 1;
            }
        }

        let mut z = vec![8; new_count];

        f.append(&mut z);
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
    //s.into_iter().dedup_with_count()

    //.collect::<Vec<_>>();
    {
        let mut hm: HashMap<u8, u64> = HashMap::new();
        for (count, k) in s.iter() {
            hm.insert(*k, *count as u64);
        }

        //let mut n = s.clone();

        advance2(&mut hm, 80);
        println!("Fish count = {}", hm.values().sum::<u64>())
    }
    {
        let mut hm: HashMap<u8, u64> = HashMap::new();
        for (count, k) in s.iter() {
            hm.insert(*k, *count as u64);
        }

        //let mut n = s.clone();

        advance2(&mut hm, 256);
        println!("Fish count = {}", hm.values().sum::<u64>())
    }
}
