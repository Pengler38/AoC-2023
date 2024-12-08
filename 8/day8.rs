//ppengler
//day8.rs

use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    //Get arguments, find input file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./(executable) path/to/input.txt");
        return;
    }

    let binding = fs::read_to_string(args[1].as_str()).expect("File not found");
    let input: Vec<&str> = binding.lines().collect();

    let pattern = &input[0];
    
    //Populate paths hashmap
    let mut paths: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in &input[2..] {
        paths.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }

    //Part 1:
    println!("{}\n", solve(&paths, pattern, vec!["AAA"]));

    //Part 2:
    let mut start_points: Vec<&str> = vec![];
    for line in &input[2..] {
        if line.chars().nth(2).unwrap() == 'A' /*&& &line[0..3] != "AAA"*/{
            start_points.push(&line[0..3]);
        }
    }
    //for x in &start_points {
        //print!("{}, ", x);
    //}
    //print!("\n");

    println!("{}", solve(&paths, pattern, start_points));
}

fn solve(paths: &HashMap<&str, (&str, &str)>, pattern: &str, initial_letters: Vec<&str>) -> u64 {
    let mut letter_list = initial_letters.clone();
    let mut i: u64 = 0;
    let mut cycle: Vec<u64> = Vec::with_capacity(initial_letters.len());
    for _ in 0..initial_letters.len() {
        cycle.push(0);
    }

    loop {
        for letters in &mut letter_list {
            let next = paths.get(letters).unwrap();
            if pattern.chars().nth((i%pattern.len() as u64) as usize).unwrap() == 'L' {
                *letters = next.0;
            } else {
                *letters = next.1;
            }
        }
        i += 1;

        //Handle part 1
        if letter_list.len() == 1 && letter_list[0] == "ZZZ" {
            return i;
        } else{
            //Handle Part 2 end case
            let mut done = true;
            for num in 0..letter_list.len() {
                if &letter_list[num][2..3] == "Z" && cycle[num] == 0{
                    cycle[num] = i;
                } else if cycle[num] == 0 {
                    done = false;
                }
            }
            if done == true {
                return cycle.iter().fold(1, |product, x| product*x / gcd(product, *x));
            }
        }
    }
}

//calcs the greatest common divisor
//WE LOVE EUCLID HE WAS A COOL DUDE
fn gcd(in_a: u64, in_b: u64) -> u64 {
    let mut a = in_a;
    let mut b = in_b;
    while a != b && a != 0 && b != 0 {
        if a > b { 
            a %= b; 
        } else if a < b { 
            b %= a; 
        }
    }
    if a != 0 {
        return a;
    } else {
        return b;
    }
}
