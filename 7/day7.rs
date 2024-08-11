//ppengler
//day7.rs 

//e.g. cards = "AAA22", strength = 4 (full house)
//Strength ranges from 0 to 6

use std::env;
use std::fs;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
enum HandType{
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
struct Hand<'a>{
    cards: &'a str,
    strength: HandType,
    bet: u32,
}

fn main() {
    //Get arguments, find input file
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("Usage: ./day6rs.exe path/to/input.txt");
        return;
    }

    let input = fs::read_to_string(args[1].as_mut_str()).expect("File not found");
    let lines = input.lines();

    let mut hands: Vec<Hand> = Vec::new();
    parse(lines, &mut hands);

    //Sort the hands in ascending strength, according to HandType strength and first cards
    hands.sort_unstable_by(hand_sort);

    //Find the winnings by multiplying the bets and hand ranks
    let mut i = 0;
    let mut winnings: u64 = 0;
    while i < hands.len() {
        let rank = i + 1;

        winnings += (hands[i].bet as u64) * (rank as u64);

        i += 1;
    }

    println!("\nTotal Winnings: {}", winnings);
}

fn hand_sort(a: &Hand, b: &Hand) -> Ordering {
    if (a.strength as u32) < (b.strength as u32) {
        return Ordering::Less;
    } else if (a.strength as u32) > (b.strength as u32) {
        return Ordering::Greater;
    } else {
        //Hand type strengths are equal, check each card sequentially for which is greater
        let mut i = 0;
        while i < a.cards.len() {
            match card_compare(a.cards.as_bytes()[i], b.cards.as_bytes()[i]) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                //If Ordering::Equal, continue the loop
                _ => {},
            }
            i += 1;
        }
        //Hands are exactly equivalent, should not be possible
        println!("Error! Equivalent hands, both with: {}", a.cards);
        return Ordering::Equal;
    }

    //Inner function for convenience
    fn card_compare(a: u8, b: u8) -> Ordering {
        let a_val = card_value(a);
        let b_val = card_value(b);
        if a_val < b_val {
            return Ordering::Less;
        } else if a_val > b_val {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }

        fn card_value(c: u8) -> u8{
            match c {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'J' => 9,
                b'T' => 8,
                b'9' => 7,
                b'8' => 6,
                b'7' => 5,
                b'6' => 4,
                b'5' => 3,
                b'4' => 2,
                b'3' => 1,
                b'2' => 0,
                _ => panic!("Received a non-card value: {}", c),
            }
        }
    }
}

//Parse input, populating Hand structs into hands
fn parse<'a, I>(lines: I, hands: &mut Vec<Hand<'a>>)
    where
    I : Iterator<Item = &'a str>,

{
    for line in lines {
        let words: Vec<&str> = line.split(' ').collect();
        let (cards, bet) = (words[0], words[1].parse::<u32>().unwrap());

        //Find the strength of the hand
        //Count how many of each card type are in th hand, store the amounts in card_count
        let mut card_count: Vec<(char, u32)> = Vec::new();
        for card in cards.chars() {
            let mut found = false;
            //Search if the card is already counted, if so add one more card
            for entry in &mut card_count {
                if entry.0 == card {
                    found = true;
                    entry.1 += 1;
                    break;
                }
            }
            //else if the card isn't in card_count already, add it in
            if !found {
                card_count.push((card, 1));
            }
        }

        //Use card_count to determine hand strength. First reverse sort card_count to ensure the highest
        //card counts are at earlier array positions
        card_count.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
        //Next get hand_strength by the matching HandType
        let hand_strength = if card_count[0].1 == 5 {
            HandType::FiveOfAKind
        } else if card_count[0].1 == 4 {
            HandType::FourOfAKind
        } else if card_count[0].1 == 3 && card_count[1].1 == 2 {
            HandType::FullHouse
        } else if card_count[0].1 == 3 {
            HandType::ThreeOfAKind
        } else if card_count[0].1 == 2 && card_count[1].1 == 2 {
            HandType::TwoPair
        } else if card_count[0].1 == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        let hand = Hand{
            cards: cards,
            bet: bet,
            strength: hand_strength,
        };
        println!("{}, {}, {:?}", hand.cards, hand.bet, hand.strength);
        hands.push(hand);
    }
}
