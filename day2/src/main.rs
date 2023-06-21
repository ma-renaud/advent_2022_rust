use crate::RockPaperScissor::{Paper, Rock, Scissor};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq)]
enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

fn get_move(character: char) -> Option<RockPaperScissor> {
    match character {
        'A' => Some(Rock),
        'X' => Some(Rock),
        'B' => Some(Paper),
        'Y' => Some(Paper),
        'C' => Some(Scissor),
        'Z' => Some(Scissor),
        _ => None,
    }
}

fn get_strategy(line: &str) -> Option<(RockPaperScissor, RockPaperScissor)> {
    if line.len() == 3 {
        let expected = get_move(line.as_bytes()[0] as char);
        let response = get_move(line.as_bytes()[2] as char);

        if !expected.is_none() && !response.is_none() {
            Some((expected.unwrap(), response.unwrap()))
        } else {
            None
        }
    } else {
        None
    }
}

fn is_winning(elve_hand: RockPaperScissor, player_hand: RockPaperScissor) -> bool {
    player_hand == Rock && elve_hand == Scissor ||
        player_hand == Paper && elve_hand == Rock ||
        player_hand == Scissor && elve_hand == Paper
}

fn get_score(elve_hand: RockPaperScissor, player_hand: RockPaperScissor) -> u8 {
    let hand_score = match player_hand {
        Rock => 1,
        Paper => 2,
        Scissor => 3
    };

    if player_hand == elve_hand {
        return hand_score + 3;
    } else if is_winning(elve_hand, player_hand) {
        return  hand_score + 6;
    }

    hand_score
}

fn main() {
    let lines = match read_lines("day2/data.txt") {
        Err(e) => panic!("Problem opening the file: {:?}", e),
        Ok(l) => l,
    };

    let mut sum: u32 = 0;
    for line in lines {
        if let Ok(line_str) = line {
            let strategy = get_strategy(&line_str).unwrap();
            sum += get_score(strategy.0, strategy.1) as u32;
        }
    }

    // let mut line_iter = lines.into_iter();
    // for _ in 0..10 {
    //     if let Ok(line_str) = line_iter.next().unwrap() {
    //         let strategy = get_strategy(&line_str).unwrap();
    //         println!("{} Given {:?} do {:?} +{}", line_str, &strategy.0, &strategy.1, get_score(&strategy.0, &strategy.1));
    //     }
    // }

    println!("Total score is {}", sum);
}
