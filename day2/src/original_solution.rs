use self::RockPaperScissor::{ Rock, Paper, Scissor };
use self::GameResult::{ Win, Lose, Draw };

#[derive(Debug, PartialEq, Eq)]
enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, PartialEq, Eq)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

pub fn original_solution(lines: &Vec<String>) {
    use std::time::Instant;
    let now = Instant::now();

    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;
    for line in lines {
        let strategy = get_strategy(&line).unwrap();
        let real_strategy = get_real_strategy(&line).unwrap();
        sum_part1 += get_score(&strategy.0, &strategy.1) as u32;
        sum_part2 += get_score(&real_strategy.0, &real_strategy.1) as u32;
    }

    let elapsed = now.elapsed();

    println!("\nOriginal Solution");
    println!("Total score part1 is {}", sum_part1);
    println!("Total score part2 is {}", sum_part2);
    println!("Elapsed: {:.2?}", elapsed);
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

fn get_move_from_result(elve_hand: &RockPaperScissor, character: char) -> Option<RockPaperScissor> {
    let result = match character {
        'X' => Some(Lose),
        'Y' => Some(Draw),
        'Z' => Some(Win),
        _ => None,
    };

    if let Some(result) = result {
        match (elve_hand, result) {
            (Rock, Lose) => Some(Scissor),
            (Rock, Draw) => Some(Rock),
            (Rock, Win) => Some(Paper),
            (Paper, Lose) => Some(Rock),
            (Paper, Draw) => Some(Paper),
            (Paper, Win) => Some(Scissor),
            (Scissor, Lose) => Some(Paper),
            (Scissor, Draw) => Some(Scissor),
            (Scissor, Win) => Some(Rock)
        }
    } else { None }
}

fn get_real_strategy(line: &str) -> Option<(RockPaperScissor, RockPaperScissor)> {
    if line.len() == 3 {
        if let Some(expected) = get_move(line.chars().nth(0).unwrap()) {
            if let Some(response) = get_move_from_result(&expected, line.chars().nth(2).unwrap()) {
                Some((expected, response))
            } else { None }
        } else { None }
    } else { None }
}

fn is_winning(elve_hand: &RockPaperScissor, player_hand: &RockPaperScissor) -> bool {
    *player_hand == Rock && *elve_hand == Scissor ||
        *player_hand == Paper && *elve_hand == Rock ||
        *player_hand == Scissor && *elve_hand == Paper
}

fn get_score(elve_hand: &RockPaperScissor, player_hand: &RockPaperScissor) -> u8 {
    let hand_score = match *player_hand {
        Rock => 1,
        Paper => 2,
        Scissor => 3
    };

    if player_hand == elve_hand {
        return hand_score + 3;
    } else if is_winning(elve_hand, player_hand) {
        return hand_score + 6;
    }

    hand_score
}