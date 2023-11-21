use std::convert::TryFrom;
use color_eyre::eyre::Result;
use itertools::Itertools;

pub fn fasterthanlime_solution(lines: &Vec<String>) -> Result<()> {
    use std::time::Instant;
    let now = Instant::now();

    let sum_part1: usize = itertools::process_results(
        lines.iter()
            .map(|line| str_to_round(line))
            .map_ok(|r| r.our_score()),
        |it| it.sum(),
    )?;


    let sum_part2: usize = itertools::process_results(
        lines.iter()
            .map(|line| Round::try_from(line))
            .map_ok(|r| r.our_score()),
        |it| it.sum(),
    )?;

    let elapsed = now.elapsed();

    println!("\nFasterthanlime Solution");
    println!("Total score part1 is {}", sum_part1);
    println!("Total score part2 is {}", sum_part2);
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move beats us")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("we beat at least one move")
    }

    fn drawing_move(self) -> Self {
        self
    }

    /// How many points do we get for picking that move?
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    fn our_score(self) -> usize {
        self.ours.inherent_points() + self.outcome().inherent_points()
    }
}

fn str_to_round(s: &String) -> Result<Round> {
    let mut chars = s.chars();
    let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
        return Err(color_eyre::eyre::eyre!("expected <theirs>SP<ours>EOF, got {s:?}"));
    };

    Ok(Round {
        theirs: theirs.try_into()?,
        ours: ours.try_into()?,
    })
}

impl TryFrom<&String> for Round {
    type Error = color_eyre::Report;

    fn try_from(data: &String) -> Result<Self, Self::Error> {
        let mut chars = data.chars();
        let (Some(theirs), Some(' '), Some(outcome), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <theirs>SP<ours>EOF, got {data:?}"));
        };

        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome::try_from(outcome)?;
        let ours = outcome.matching_move(theirs);

        Ok(Self { theirs, ours })
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win => theirs.winning_move(),
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Loss => theirs.losing_move(),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {c:?}")),
        }
    }
}