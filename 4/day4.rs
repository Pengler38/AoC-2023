//ppengler
//day4.rs

//use std::env;
use std::fs;
use regex::Regex;




fn main() {
    //split input.txt by line
    let input: String = fs::read_to_string("../../input.txt").expect("File not found");
    //let inputs: Vec<&str> = input.split('\n').collect();

    let inputs = input.split('\n');

    let reg: regex::Regex = Regex::new(r"([0-9]+)|(\|)").unwrap();

    //let caps: regex::Captures = reg.captures(inputs[0]).unwrap();
    enum State{
        AddWinningNumbers,
        CheckNumbers
    }


    let mut points = 0;

    let mut i = 0;

    for scratch_card in inputs {
        let mut iter: regex::CaptureMatches  = reg.captures_iter(scratch_card);
        iter.next(); //The first match is the game number, use next to ignore it

        i += 1;
        print!("Game {i}: ");

        let mut winning_nums: u32 = 0;
        let mut vec: Vec<String> = Vec::new();
        let mut cur_state: State = State::AddWinningNumbers;

        for number_matches in iter {

            let asdf = number_matches[0].to_string();

            if matches!(cur_state, State::AddWinningNumbers){
                //If the delimiter | is found, switch to checking numbers against the winning numbers
                print!("{asdf:>2} ");
                if number_matches[0].chars().nth(0) == Some('|')
                {
                    cur_state = State::CheckNumbers; 
                    continue;
                }


                //Add the current number to the vec
                vec.push(number_matches[0].to_string());
            }
            else if vec.contains(&number_matches[0].to_string()) {
                print!(".{asdf:>2}. ");
                winning_nums += 1;
            }
            else{
                print!(" {asdf:>2}  ");
            }
        }

        if winning_nums > 0
        {
            points += 2_i32.pow(winning_nums-1);
            println!(": {}, {}", winning_nums, 2_i32.pow(winning_nums-1));
        }
        else{
            println!(": {}, {}", winning_nums, 0);
        }

    }

    //let first: regex::Captures = iter.next().unwrap();
    //println!("test,\n{}", &first[0]);

    //println!("test,\n{}", &caps[0]);

    //println!("test:\n{first}");

    

    println!("Points: {points}");
}
