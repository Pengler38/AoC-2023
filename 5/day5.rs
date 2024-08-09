//ppengler
//day5.rs 

use std::fs;
use std::env;

struct Map{
    dest: u64,
    src: u64,
    range: u64,
}

fn main() {

    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("Usage: ./day5rs.exe ");
        return;
    }
    //let input = fs::read_to_string(env::args().collect::<Vec<String>>()[0]).expect("File not found");
    let input = fs::read_to_string(args[1].as_mut_str()).expect("File not found");
    let mut lines = input.lines();
    //let mut lines = input.lines();
    
    //eprint!("{}\n", lines.peek().unwrap());


    //Parse in the seed numbers
    let mut lineparse: Vec<&str> = lines.next().unwrap().split(' ').collect();
    lineparse.remove(0); //remove the "seeds:"
    eprintln!("{}", lineparse[0]);
    let mut seeds: Vec<u64> = vec![];
    for word in lineparse {
        seeds.push(word.parse::<u64>().unwrap());
    }

    //Parse in the mappings
    let mut maps: Vec<Vec<Map>> = Vec::new();
    for _x in 0..7{
       let mut map: Vec<Map> = Vec::new();
       parse(":", &mut map, &mut lines);
       maps.push(map);
    }


    let single_seed_locations = seeds_to_locations(&seeds, &maps);

    print!("Single Seed Locations found: ");
    for x in &single_seed_locations{
        print!("{}, ", *x);
    }
    println!();

    //Print lowest location number
    let mut min: u64 = u64::MAX;
    for x in &single_seed_locations{
        if *x < min { 
            min = *x;
        }
    }
    println!("Single Seed Lowest location: {min}");

    //Find the location when using many seeds
    let mut manyseeds = vec![];
    let mut x = 0;
    while x < seeds.len() {
        let mut seed = seeds[x];
        let mut range = seeds[x+1];

        while range > 0{
            manyseeds.push(seed);
            seed += 1;
            range -= 1;
        }

        x += 2;
    }
    let many_seed_locations = seeds_to_locations(&manyseeds, &maps);


    print!("Many Seed Locations found: ");
    for x in &many_seed_locations{
        print!("{}, ", *x);
    }
    println!();

    //Print lowest location number
    let mut min: u64 = u64::MAX;
    for x in &many_seed_locations{
        if *x < min { 
            min = *x;
        }
    }
    println!("Many Seed Lowest location: {min}");

}

fn seeds_to_locations(seeds: &Vec<u64>, maps: &Vec<Vec<Map>>) -> Vec<u64> {
    let mut almanac = (*seeds.clone()).to_vec();

    //For each map, transform the almanac numbers according to the mappings
    //If the value isn't explicitly mapped, no transformation needed
    eprintln!();
    for map in maps{
        for number in &mut almanac{
            for mapping in map{
                if (*number >= mapping.src) && (*number < mapping.src + mapping.range) {
                    *number = mapping.dest + (*number - mapping.src);
                    break; //Prevent mapping the same number multiple times
                }
            }
        }

        for x in &almanac{
            eprint!("{x}, ");
        }
        eprintln!();
    }

    almanac
}

//calls lines.next() until start is found, then on the next line, read in the map values until a
//\n\n is found or the end of lines
fn parse(start: &str, arr: &mut Vec<Map>, lines: &mut std::str::Lines){
    while !lines.next().unwrap().contains(start) { }
    loop {
        //Ensure we have a next line
        let next : &str = match lines.next() {
            Some(x) => x,
            None => break,
        };
        //Ensure the line is not empty
        if !next.contains(char::is_numeric) {break;}

        eprintln!("{next}");
        let numbers: Vec<&str> = next.split(' ').collect();
        arr.push(
            Map{
                dest: numbers[0].parse().unwrap(),
                src: numbers[1].parse().unwrap(),
                range: numbers[2].parse().unwrap(),
            }
            );
    }
    eprintln!();
}
