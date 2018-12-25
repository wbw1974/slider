/// Slider
/// Author: W. Brent Williams
/// Since: 2018-12-15
extern crate rand;

use rand::prelude::*;
use std::env;
use std::io;
use std::num::ParseIntError;

/// Program entry point.
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

/// Attempts to parse out the optional --seed and --inerations arguments. Any other arguments
/// are ignored.
/// If --seed cannot be parsed, the system defaults to use from_entropy for the seed.
/// If --iterations cannot be parsed, the system defaults to the value 10.
/// If --iterations is less than zero, the system defaults to the value 10.
/// 
/// args: The pointer to a Vec<String> corresponding to the command line arguments.
/// 
/// Returns: A tuple containing two options (Option<usize>, Option<usize>) corresponding to parsed arguments or defaults (None, None).
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
                } else {
                    println!(
                        "Could not parse --seed argument {}. Defaulting to random.",
                        arg
                    );
                }
            }
            2 => {
                let res = arg.parse::<usize>();
                if res.is_ok() {
                    let iter = res.unwrap();
                    if iter > 0 as usize {
                        iterations = Some(iter);
                    } else {
                        println!(
                            "The --iterations argument {} is less than zero. As shuffling negative moves does not make any logical sense, I will default to 10. Defaulting to 10.",
                            arg
                        );
                    }
                } else {
                    println!(
                        "Could not parse --iterations argument {}. Defaulting to 10.",
                        arg
                    );
                }
            }
            _ => (),
        }
        ptr = match_key(arg.trim(), "--seed", 1, "--iterations", 2);
    }

    return (seed, iterations);
}

/// Helper for process_args. Checks to see if the argument matches a key and returns a pointer to drive argument parsing. If nothing matches, 0 is returned.
/// 
/// arg: The string (pointer to a str) to match.
/// 
/// key1: A candidate string (pointer to a str) with which to match arg.
/// 
/// prt1: The return value if arg and key1 match.
/// 
/// key2: A candidate string (pointer to a str) with which to match arg.
/// 
/// ptr2: The return value is arg and key2 match.
fn match_key(arg: &str, key1: &str, ptr1: usize, key2: &str, ptr2: usize) -> usize {
    if arg == key1 {
        return ptr1;
    } else if arg == key2 {
        return ptr2;
    } else {
        return 0;
    }
}

/// Function that creates a map, defined as a set of contiguous integers 1 - 15 and a None value (tiles) in a Vec.
/// 
/// Returns a map (Vec<Option<usize>>) with 16 tiles (Some(val) or None).
fn make_map() -> Vec<Option<usize>> {
    let mut map = Vec::new();
    for x in 1..16 {
        map.push(Some(x));
    }
    map.push(None);
    return map;
}

/// Function that prints out a map.
/// 
/// map: The map of tiles to print out.
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

/// Function that walks the tiles in a map to find the empty tile.
/// 
/// map: The map of tiles to walk, looking for the first (and only) empty tile (None).
/// 
/// Returns: The index in the map of the empty tile.
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

/// Function that shuffles a map, mutating the passed in map to a new permeutation.
/// 
/// seed: Seed for the random number generator. If None, the random number grnerator generates a key using the from_entropy function.
/// 
/// iterations: The number of times a random tile on the map is moved.
/// 
/// map: The map of tiles. Mutatable this time as the map is being shuffled (implicit return is this map, shuffled).
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

/// Function that moves one of two tiles at random.
/// 
/// rng: The random number generator.
///
/// map: The map that contains the tiles to move (implicit return).
/// 
/// item_1: The index on the map that might be moved.
/// 
/// item_2: The index on the map that might be moved.
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

/// Function that moves one of three tiles at random.
/// 
/// rng: The random number generator.
///
/// map: The map that contains the tiles to move (implicit return).
/// 
/// item_1: The index on the map that might be moved.
/// 
/// item_2: The index on the map that might be moved.
/// 
/// item_3: The index on the map that might be moved.
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

/// Function that moves one of four tiles at random.
/// 
/// rng: The random number generator.
///
/// map: The map that contains the tiles to move (implicit return).
/// 
/// item_1: The index on the map that might be moved.
/// 
/// item_2: The index on the map that might be moved.
/// 
/// item_3: The index on the map that might be moved.
/// 
/// item_4: The index on the map that might be moved.
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

/// Function that mutates a map by moving the spedified tile.
/// 
/// map: The map that contains the tiles to move (implicit return).
/// 
/// tile: The index of the tile in the map to move.
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

/// Function that walks a mpa, checking for a win condition.
/// 
/// map: The map that contains the tiles.
/// 
/// Returns: True if win, False if not.
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

/// Function that encapsulates the game loop.
/// 
/// map: The map that contains the tiles to move.
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

/// Function that parses the keyboard input and handles it if it is legal input. Also handles the quit command.
/// 
/// input: The string (pointer to a str) from the keyboard.
/// 
/// map: The map that contains the tiles to move.
/// 
/// moves: The number of maves made so far.
/// 
/// Returns: The tile to be moved.
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

/// Function that translates the value from the keyboard to the tile in the map to be moved.
/// 
/// val: The value from the keyboard.
/// 
/// map: The map that contains the tiles to move.
/// 
/// Returns: The tile to be moved or None.
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
