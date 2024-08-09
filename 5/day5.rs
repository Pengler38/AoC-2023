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
    let many_seeds_min = many_seeds_to_lowest_location(&seeds, &maps);
    println!("Many Seed Lowest location: {many_seeds_min}");

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


//Store start almanac number and end (inclusive)
struct Entry{
    start: u64,
    end: u64,
}

fn many_seeds_to_lowest_location(seeds: &Vec<u64>, maps: &Vec<Vec<Map>>) -> u64 {
    let mut almanac: Vec<Entry> = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        almanac.push(
            Entry{
                start: seeds[i],
                end: seeds[i] + seeds[i+1] - 1, //end is inclusive
            });
        i += 2;
    }

    //For each map, transform the almanac numbers according to the mappings
    //If the value isn't explicitly mapped, no transformation needed
    eprintln!();
    let mut i = 0;
    for map in maps{

        while i < almanac.len() {

            let entry = &mut almanac[i];
            for mapping in map{

                let start_in_range = entry.start >= mapping.src && entry.start < mapping.src + mapping.range;
                let end_in_range = entry.end >= mapping.src && entry.end < mapping.src + mapping.range;

                if start_in_range && end_in_range {
                    //Easy case, behaves as before
                    entry.start = mapping.dest + (entry.start - mapping.src);
                    entry.end = mapping.dest + (entry.end - mapping.src);
                    break;
                }
                else if start_in_range || end_in_range {
                    //split the entry into 2 ranges, add the second range, and incr. i as to not
                    //try to double map the second range

                    let new_entry;
                    if start_in_range {
                        let split = mapping.src + mapping.range; //split number belongs to the
                                                                 //upper range
                        new_entry = Entry{
                            start: entry.start,
                            end: split - 1,
                        };

                        entry.start = split;

                    }
                    else { //end_in_range = true 
                        let split = mapping.src; //split number belongs to the upper range

                        new_entry = Entry{
                            start: split,
                            end: entry.end,
                        };

                        entry.end = split - 1;
                    }
                    
                    almanac.insert(i+1, new_entry);
                    i += 1;
                    break;
                }

            }

            i += 1;
        }

        // for x in &almanac{
        //     eprint!("{}-{}, ", x.start, x.end);
        // }
        // eprintln!();
    }

    let mut min: u64 = u64::MAX;
    for x in almanac {
        if x.start < min { 
            min = x.start;
        }
    }

    min
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
