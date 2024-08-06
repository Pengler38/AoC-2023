//ppengler
//day5.rs 

use std::fs;
use std::env;

struct Map{
    dest: u32,
    src: u32,
    range: u32,
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
    println!("{}", lineparse[0]);
    let mut seeds: Vec<u32> = vec![];
    for word in lineparse {
        seeds.push(word.parse::<u32>().unwrap());
    }

    //Parse in the maps
    let mut maps: Vec<Vec<Map>> = Vec::new();
    for _x in 0..7{
       let mut map: Vec<Map> = Vec::new();
       parse(":", &mut map, &mut lines);
       maps.push(map);
    }
}

//calls lines.next() until start is found, then on the next line, read in the map values until a
//\n\n is found or the end of lines
fn parse(start: &str, arr: &mut Vec<Map>, lines: &mut std::str::Lines){
    while !lines.next().unwrap().contains(start) { }
    loop {
        let next : &str; 
        //Ensure we have a next line
        match lines.next() {
            Some(x) => next = x,
            None => break,
        };
        //Ensure the line is not empty
        if !next.contains(char::is_numeric) {return;}

        println!("{next}");
        let numbers: Vec<&str> = next.split(' ').collect();
        arr.push(
            Map{
            dest: numbers[0].parse().unwrap(),
            src: numbers[1].parse().unwrap(),
            range: numbers[2].parse().unwrap(),
            }
            );
    }
}
