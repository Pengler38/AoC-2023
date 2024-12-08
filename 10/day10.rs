//ppengler
//day10.rs

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

    //2D array that tracks which slots are pipes or not for Part 2
    let mut pipes: Vec<Vec<bool>> = vec![];
    for line in &input {
        let mut temp_vec: Vec<bool> = vec![];
        for _ in line.chars() {
            temp_vec.push(false);
        }
        pipes.push(temp_vec);
    }

    //Find the start position
    let mut start: (i32, i32) = (0, 0);
    for i in 0..input.len(){
        let pos = input[i].chars().position(|x| x == 'S');
        if pos != None {
            start = (pos.unwrap() as i32, i as i32);
        }
    };

    pipes[start.1 as usize][start.0 as usize] = true;
    println!("start: {}, {}", start.0, start.1);

    //Scan for pipes connecting to the Start
    let pos = scan(start, &input);

    let mut pos1 = pos[0];
    pipes[pos[0].1 as usize][pos[0].0 as usize] = true;
    let mut lastpos1 = start;
    let mut pos2 = pos[1];
    pipes[pos[1].1 as usize][pos[1].0 as usize] = true;
    let mut lastpos2 = start;
    let mut i = 1;
    println!("pos1: {}, {}", pos1.0, pos1.1);
    println!("pos2: {}, {}", pos2.0, pos2.1);
    while pos1 != pos2 {
        let newpos1 = *scan(pos1, &input).iter().find(|&&x| x != lastpos1).unwrap();
        lastpos1 = pos1;
        pos1 = newpos1;

        let newpos2 = *scan(pos2, &input).iter().find(|&&x| x != lastpos2).unwrap();
        lastpos2 = pos2;
        pos2 = newpos2;

        //Add pipes to the pipe array
        pipes[newpos1.1 as usize][newpos1.0 as usize] = true;
        pipes[newpos2.1 as usize][newpos2.0 as usize] = true;

        i += 1;
    }

    println!("{i}");

    //Find the area by looking at the pipes
    //First, 
    //TODO this is invalid because it doesn't take into account verticality, see example on the
    //website
    for line in &mut pipes {
        for c in &mut *line {
            if *c == true {break;}
            *c = true;
        }
        for c in line.into_iter().rev() {
            if *c == true {break;}
            *c = true;
        }
    }
    //Second, sum the number of all the false slots
    let area = pipes.iter().fold(0, |sum, list| sum + list.iter().fold(0, |sum, &p| if p == false { sum + 1} else {sum} ));
    println!("{area}");
}


//Scan for the connections
fn scan(pos: (i32, i32), input: &Vec<&str>) -> Vec<(i32, i32)>{
    let current_char = input[pos.1 as usize].chars().nth(pos.0 as usize).unwrap();

    let connections: Vec<(i32, i32)> = match current_char {
            '|' => vec![
                (pos.0, pos.1-1),
                (pos.0, pos.1+1)],
            '-' => vec![
                (pos.0-1, pos.1),
                (pos.0+1, pos.1)],
            '7' => vec![
                (pos.0-1, pos.1),
                (pos.0, pos.1+1)],
            'F' => vec![
                (pos.0+1, pos.1),
                (pos.0, pos.1+1)],
            'L' => vec![
                (pos.0+1, pos.1),
                (pos.0, pos.1-1)],
            'J' => vec![
                (pos.0-1, pos.1),
                (pos.0, pos.1-1)],
            'S' => vec![
                (pos.0-1, pos.1),
                (pos.0+1, pos.1),
                (pos.0, pos.1-1),
                (pos.0, pos.1+1)],
            _ => panic!(),
    };

    let mut result: Vec<(i32, i32)> = vec![];
    for (x, y) in connections {
        let line = input.get(y as usize);
        if line == None{ continue; }
        let c = line.unwrap().chars().nth(x as usize);
        if c == None{ continue; }

        let catchpipe = match c.unwrap() {
            '|' if x == pos.0 => true,
            '-' if y == pos.1 => true,
            '7' if (y == pos.1-1) || (x == pos.0+1) => true,
            'F' if (y == pos.1-1) || (x == pos.0-1) => true,
            'L' if (y == pos.1+1) || (x == pos.0-1) => true,
            'J' if (y == pos.1+1) || (x == pos.0+1) => true,
            'S' => true,
            _ => false,
        };

        if catchpipe == true {
            result.push((x, y));
        }
    }
    if result.len() != 2 {
        println!("Error: result.len() = {}", result.len());
        for r in &result {
            println!("{}, {}", r.0, r.1);
        }
    }
    result
}

//Part 2 gameplan: have a 2d array of bool, fill true with the pipe pieces, and then magically find
//the enclosed portion >:(
//fill outer portions with true as well. front and back.
//T -> T F ... F T <- T
