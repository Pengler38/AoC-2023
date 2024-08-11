//ppengler
//day6.rs 

use std::fs;
use std::env;

struct Race {
    time: u32,
    distance: u32,
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
    let mut results: Vec<u32> = Vec::new();

    for race in races {
        print!("Time: {}, Distance: {} -> ", race.time, race.distance);

        //Go through each potential split of race.time and find the min and max times to hold the
        //button that work 
        let mut min: Option<u32> = None;
        let mut max: Option<u32> = None;

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


//Parse in the inputs properly
fn parse(races: &mut Vec<Race>, lines: Vec<&str>) {
    for word in lines[0].split(' ') {
        let parsed = match word.parse::<u32>() {
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
        let parsed = match word.parse::<u32>() {
            Ok(x) => x,
            Err(_) => continue,
        };
        races[i].distance = parsed;
        i += 1;
    }
}
