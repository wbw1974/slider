extern crate rand;

use rand::prelude::*;
use std::env;
use std::io;
use std::num::ParseIntError;

fn main() {
    let (seed, iterations) = process_args(&env::args().collect());

    let iter: usize;
    match iterations {
        Some(iterations) => {
            iter = iterations;
        }
        None => {
            iter = 10;
        }
    }

    let mut map = make_map();
    println!("Beginning map.");
    print_map(&map);
    println!("Shuffle {} times.", iter);
    shuffle_map(seed, iter, &mut map);
    print_map(&map);
    println!("Begin game.");
    begin_game(&mut map);
    std::process::exit(0);
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
            _ => (),
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

fn find_empty(map: &Vec<Option<usize>>) -> usize {
    let mut ctr = 0;
    for item in map {
        match item {
            None => {
                return ctr;
            }
            _ => (),
        }
        ctr += 1;
    }
    print_map(map);
    panic!("Map does not have an empty space!");
}

fn shuffle_map(seed: Option<usize>, iterations: usize, map: &mut Vec<Option<usize>>) {
    let mut rng: rand::prelude::StdRng;
    match seed {
        Some(seed) => {
            rng = StdRng::seed_from_u64(seed as u64);
        }
        None => {
            rng = StdRng::from_entropy();
        }
    }

    let mut pointer = find_empty(map);
    let mut iter = iterations;
    while iter > 0 {
        match pointer {
            0 => {
                pointer = choose_two(&mut rng, map, pointer + 1, pointer + 4);
            }
            1 | 2 => {
                pointer = choose_three(&mut rng, map, pointer - 1, pointer + 1, pointer + 4);
            }
            3 => {
                pointer = choose_two(&mut rng, map, pointer - 1, pointer + 4);
            }
            4 | 8 => {
                pointer = choose_three(&mut rng, map, pointer - 4, pointer + 1, pointer + 4);
            }
            5 | 6 | 9 | 10 => {
                pointer = choose_four(
                    &mut rng,
                    map,
                    pointer - 4,
                    pointer - 1,
                    pointer + 1,
                    pointer + 4,
                );
            }
            7 | 11 => {
                pointer = choose_three(&mut rng, map, pointer - 4, pointer - 1, pointer + 4);
            }
            12 => {
                pointer = choose_two(&mut rng, map, pointer - 4, pointer + 1);
            }
            13 | 14 => {
                pointer = choose_three(&mut rng, map, pointer - 4, pointer - 1, pointer + 1);
            }
            15 => {
                pointer = choose_two(&mut rng, map, pointer - 4, pointer - 1);
            }
            _ => (),
        }
        iter -= 1;
    }
}

fn choose_two<R: Rng>(
    rng: &mut R,
    map: &mut Vec<Option<usize>>,
    item_1: usize,
    item_2: usize,
) -> usize {
    let val = rng.gen_range(0, 2);
    match val {
        1 => {
            move_tile(map, item_1);
            return item_1;
        }
        0 => {
            move_tile(map, item_2);
            return item_2;
        }
        _ => panic!("Function choose_two random number out of range: {}.", val),
    }
}

fn choose_three<R: Rng>(
    rng: &mut R,
    map: &mut Vec<Option<usize>>,
    item_1: usize,
    item_2: usize,
    item_3: usize,
) -> usize {
    let val = rng.gen_range(0, 3);
    match val {
        0 => {
            move_tile(map, item_1);
            return item_1;
        }
        1 => {
            move_tile(map, item_2);
            return item_2;
        }
        2 => {
            move_tile(map, item_3);
            return item_3;
        }
        _ => panic!("Function choose_three random number out of range: {}.", val),
    }
}

fn choose_four<R: Rng>(
    rng: &mut R,
    map: &mut Vec<Option<usize>>,
    item_1: usize,
    item_2: usize,
    item_3: usize,
    item_4: usize,
) -> usize {
    let val = rng.gen_range(0, 4);
    match val {
        3 => {
            move_tile(map, item_1);
            return item_1;
        }
        2 => {
            move_tile(map, item_2);
            return item_2;
        }
        1 => {
            move_tile(map, item_3);
            return item_3;
        }
        0 => {
            move_tile(map, item_4);
            return item_4;
        }
        _ => panic!("Function choose_four random number out of range: {}.", val),
    }
}

fn move_tile(map: &mut Vec<Option<usize>>, tile: usize) -> bool {
    let empty = find_empty(&map);
    let mv = empty as isize - tile as isize;
    match mv {
        -4 => {
            map[empty] = map[tile];
            map[tile] = None;
            return true;
        }
        -1 => {
            map[empty] = map[tile];
            map[tile] = None;
            return true;
        }
        1 => {
            map[empty] = map[tile];
            map[tile] = None;
            return true;
        }
        4 => {
            map[empty] = map[tile];
            map[tile] = None;
            return true;
        }
        _ => {
            println!("Illegal move: {}.", tile);
            return false;
        }
    }
}

fn check_win(map: &Vec<Option<usize>>) -> bool {
    let mut last_value: usize = 0;
    for x in 0..15 {
        let tmp = map[x];
        match tmp {
            Some(tmp) => {
                if tmp == last_value + 1 {
                    last_value = tmp;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }
    return true;
}

fn begin_game(map: &mut Vec<Option<usize>>) {
    let mut moves = 0;
    loop {
        if check_win(&map) {
            println!("Game won in {} moves.", moves);
            break;
        } else {
            println!("Tile to move (type 'quit' to quit)?");
            let mut move_to_make = String::new();
            io::stdin()
                .read_line(&mut move_to_make)
                .expect("Failed to read line.");
            let tile = legalize_input(&move_to_make, map, &moves);
            match tile {
                Some(tile) => {
                    move_tile(map, tile);
                    moves += 1;
                }
                None => {
                    println!("I do not understand your move.");
                }
            }
            println!("Current game.");
            print_map(&map);
        }
    }
}

fn legalize_input(input: &str, map: &Vec<Option<usize>>, moves: &usize) -> Option<usize> {
    if input.trim() == "quit" {
        if *moves == 1 as usize {
            println!("Quit after {} move.", moves);
        } else {
            println!("Quit after {} moves.", moves);
        }
        std::process::exit(0);
    }

    let guess: Result<usize, ParseIntError> = input.trim().parse();
    match guess {
        Ok(val) => {
            return translate_to_tile(val, map);
        }
        Err(val) => {
            println!("{}.", val);
            return None;
        }
    }
}

fn translate_to_tile(val: usize, map: &Vec<Option<usize>>) -> Option<usize> {
    for x in 0..16 {
        let tmp = map[x];
        match tmp {
            Some(tmp) => {
                if tmp == val {
                    return Some(x);
                }
            }
            None => (),
        }
    }
    println!("Could not find number {}.", val);
    return None;
}
