//ppengler
//day6.rs 

use std::fs;
use std::env;

struct Race {
    time: u64,
    distance: u64,
}

fn main() {

    //Get arguments, find input file
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("Usage: ./day6rs.exe path/to/input.txt");
        return;
    }

    let input = fs::read_to_string(args[1].as_mut_str()).expect("File not found");
    let lines = input.lines().collect();

    let mut races: Vec<Race> = Vec::new();

    //Parse the input
    parse(&mut races, lines);


    //Solve the race problem
    race_solve(&races);


    //Solve the race problem, treating it as 1 big race
    println!("\nSingle big race:");
    let big_race = vec![single_race_parse(&races)];
    race_solve(&big_race);
}

fn race_solve(races: &Vec<Race>) {
    let mut results: Vec<u64> = Vec::new();

    for race in races {
        print!("Time: {}, Distance: {} -> ", race.time, race.distance);

        //Go through each potential split of race.time and find the min and max times to hold the
        //button that work 
        let mut min: Option<u64> = None;
        let mut max: Option<u64> = None;

        for x in 0..race.time {
            let winning = x * (race.time - x) > race.distance;

            if min == None && winning {
                min = Some(x);
            }
            else if min != None && !winning{
                max = Some(x);
                break;
            }
        }
        let max = max.unwrap();
        let min = min.unwrap();

        //Put the number of ways you can win in the results array
        results.push(max-min);

        println!("{}", max-min);
    }

    let mut ret = 1;
    for x in results {
        ret *= x;
    }
    println!("\nFinal multiply: {}", ret);
}

//Reads the races array as a single race
fn single_race_parse(races: &Vec<Race>) -> Race{
    let mut time = String::new();
    let mut distance = String::new();

    for race in races {
        time.push_str(&race.time.to_string());
        distance.push_str(&race.distance.to_string());
    }

    Race {
        time: time.parse::<u64>().unwrap(),
        distance: distance.parse::<u64>().unwrap(),
    }
}


//Parse in the inputs properly
fn parse(races: &mut Vec<Race>, lines: Vec<&str>) {
    for word in lines[0].split(' ') {
        let parsed = match word.parse::<u64>() {
            Ok(x) => x,
            Err(_) => continue,
        };
        races.push(
            Race{
                time: parsed,
                distance: 0,
            });
    }
    let mut i = 0;
    for word in lines[1].split(' ') {
        let parsed = match word.parse::<u64>() {
            Ok(x) => x,
            Err(_) => continue,
        };
        races[i].distance = parsed;
        i += 1;
    }
}
