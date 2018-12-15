extern crate rand;

use rand::prelude::*;
use std::env;

fn main() {
    let (seed, iterations) = process_args(&env::args().collect());

    let mut rng: rand::prelude::StdRng;
    match seed {
        Some(seed) => {
            println!("seed: {}", seed);
            rng = StdRng::seed_from_u64(seed as u64);
        }
        None => {
            rng = StdRng::from_entropy();
        }
    }

    let mut val: usize;
    match iterations {
        Some(iterations) => {
            println!("iterations: {}", iterations);
            val = iterations;
        }
        None => {
            val = 10;
        }
    }

    let mut map = make_map();
    println!("Beginning map");
    print_map(&map);
    println!("Shuffle {} times.", val);
    let mut ptr = find_empty(&mut map);
    while val > 0 {
        ptr = shuffle_map(&mut rng, &mut map, ptr);
        val -= 1;
        print_map(&map);
        println!();
    }
}

fn process_args(args: &Vec<String>) -> (Option<usize>, Option<usize>) {
    let mut seed = None;
    let mut iterations = None;

    let mut ptr = 0;
    for arg in args {
        match ptr {
            1 => {
                let res = arg.parse::<usize>();
                if res.is_ok() {
                    seed = Some(res.unwrap());
                }
            }
            2 => {
                let res = arg.parse::<usize>();
                if res.is_ok() {
                    iterations = Some(res.unwrap());
                }
            }
            _ => ()
        }
        ptr = match_key(arg.trim(), "--seed", 1, "--iterations", 2);
    }

    return (seed, iterations);
}

fn match_key(arg: &str, key1: &str, ptr1: usize, key2: &str, ptr2: usize) -> usize {
    if arg == key1 {
        return ptr1;
    } else if arg == key2 {
        return ptr2;
    } else {
        return 0;
    }
}

fn make_map() -> Vec<Option<usize>> {
    let mut map = Vec::new();
    for x in 1..16 {
        map.push(Some(x));
    }
    map.push(None);
    return map;
}

fn print_map(map: &Vec<Option<usize>>) {
    let mut out = String::new();
    let mut ctr = 0;
    for item in map {
        match item {
            Some(item) => match item {
                0..=9 => {
                    out.push_str(&format!("  {} ", item));
                }
                _ => {
                    out.push_str(&format!(" {} ", item));
                }
            },
            None => {
                out.push_str("    ");
            }
        }
        ctr += 1;
        if ctr % 4 == 0 {
            out.push_str("\n");
        }
    }
    println!("{}", &out);
}

fn shuffle_map<R: Rng>(
    rng: &mut R,
    map: &mut Vec<Option<usize>>,
    empty: Option<usize>,
) -> Option<usize> {
    match empty {
        Some(empty) => {
            return random_pick(rng, map, empty);
        }
        None => {
            return find_empty(map);
        }
    }
}

fn find_empty(map: &Vec<Option<usize>>) -> Option<usize> {
    let mut ctr = 0;
    for item in map {
        match item {
            None => {
                return Some(ctr);
            }
            _ => (),
        }
        ctr += 1;
    }
    return None;
}

fn random_pick<R: Rng>(rng: &mut R, map: &mut Vec<Option<usize>>, pointer: usize) -> Option<usize> {
    match pointer {
        0 => {
            return choose_two(rng, map, pointer, 1, 4);
        }
        1 => {
            return choose_three(rng, map, pointer, 0, 2, 5);
        }
        2 => {
            return choose_three(rng, map, pointer, 1, 3, 6);
        }
        3 => {
            return choose_two(rng, map, pointer, 2, 7);
        }
        4 => {
            return choose_three(rng, map, pointer, 0, 5, 8);
        }
        5 => {
            return choose_four(rng, map, pointer, 1, 4, 6, 9);
        }
        6 => {
            return choose_four(rng, map, pointer, 2, 5, 7, 10);
        }
        7 => {
            return choose_three(rng, map, pointer, 3, 6, 11);
        }
        8 => {
            return choose_three(rng, map, pointer, 4, 9, 12);
        }
        9 => {
            return choose_four(rng, map, pointer, 5, 8, 10, 13);
        }
        10 => {
            return choose_four(rng, map, pointer, 6, 9, 11, 14);
        }
        11 => {
            return choose_three(rng, map, pointer, 7, 10, 15);
        }
        12 => {
            return choose_two(rng, map, pointer, 8, 13);
        }
        13 => {
            return choose_three(rng, map, pointer, 9, 12, 14);
        }
        14 => {
            return choose_three(rng, map, pointer, 10, 13, 15);
        }
        15 => {
            return choose_two(rng, map, pointer, 11, 14);
        }
        _ => (),
    }
    return None;
}

fn choose_two<R: Rng>(
    rng: &mut R,
    map: &mut Vec<Option<usize>>,
    pointer: usize,
    item_1: usize,
    item_2: usize,
) -> Option<usize> {
    let val = rng.gen_range(0, 1);
    match val {
        1 => {
            map[pointer] = map[item_1];
            map[item_1] = None;
            return Some(item_1);
        }
        0 => {
            map[pointer] = map[item_2];
            map[item_2] = None;
            return Some(item_2);
        }
        _ => panic!("choose_two random number out of range: {}", val),
    }
}

fn choose_three<R: Rng>(
    rng: &mut R,
    map: &mut Vec<Option<usize>>,
    pointer: usize,
    item_1: usize,
    item_2: usize,
    item_3: usize,
) -> Option<usize> {
    let val = rng.gen_range(0, 2);
    match val {
        0 => {
            map[pointer] = map[item_1];
            map[item_1] = None;
            return Some(item_1);
        }
        1 => {
            map[pointer] = map[item_2];
            map[item_2] = None;
            return Some(item_2);
        }
        2 => {
            map[pointer] = map[item_3];
            map[item_3] = None;
            return Some(item_3);
        }
        _ => panic!("choose_three random number out of range: {}", val),
    }
}

fn choose_four<R: Rng>(
    rng: &mut R,
    map: &mut Vec<Option<usize>>,
    pointer: usize,
    item_1: usize,
    item_2: usize,
    item_3: usize,
    item_4: usize,
) -> Option<usize> {
    let val = rng.gen_range(0, 3);
    match val {
        3 => {
            map[pointer] = map[item_1];
            map[item_1] = None;
            return Some(item_1);
        }
        2 => {
            map[pointer] = map[item_2];
            map[item_2] = None;
            return Some(item_2);
        }
        1 => {
            map[pointer] = map[item_3];
            map[item_3] = None;
            return Some(item_3);
        }
        0 => {
            map[pointer] = map[item_4];
            map[item_4] = None;
            return Some(item_4);
        }
        _ => panic!("choose_four random number out of range: {}", val),
    }
}
