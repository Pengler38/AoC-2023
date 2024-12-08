//ppengler
//day9.rs

use std::env;
use std::fs;

fn main() {
    //Get arguments, find input file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./(executable) path/to/input.txt");
        return;
    }

    let file_input = fs::read_to_string(args[1].as_str()).expect("File not found");
    let input: Vec<&str> = file_input.lines().collect();

    let mut sum_next = 0;
    let mut sum_prev = 0;
    for line in input {
        let next_nums = next( 
            line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect()
            );

        sum_next += next_nums.0;
        sum_prev += next_nums.1;
    }

    println!("{sum_next}\n{sum_prev}");
}

//Finds the next number in the set
//Recursive function
fn next(nums: Vec<i64>) -> (i64, i64) {
    //Base case, if all num in nums are 0
    if nums.iter().find(|&&x| x != 0) == None {
        return (0, 0);
    }

    let mut differences: Vec<i64> = vec![];
    for i in 1..nums.len() {
        differences.push(nums[i] - nums[i-1]);
    }

    let next_nums = next(differences);
    return (nums.last().unwrap() + next_nums.0, nums.first().unwrap() - next_nums.1);
}
