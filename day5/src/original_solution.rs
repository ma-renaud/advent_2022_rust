use std::collections::VecDeque;
use std::slice;
use std::str::FromStr;

pub(crate) fn original_solution(
    text: &String,
) -> color_eyre::Result<(String, String)> {
    use std::time::Instant;
    let now = Instant::now();

    if (text.is_empty()) {
        panic!("Input is empty")
    }

    let mut port = Port::default();

    let line_offset = port.parse_stack(text)?;

    for line in text.lines().skip(line_offset - 1) {
        port.moves.push(line.parse::<Move>()?);
    }

    let mut part1 = port.clone();

    part1.apply_moves_9000();
    let tops_part1 = part1.get_tops();

    port.apply_moves_9001();
    let tops_part2 = port.get_tops();

    let elapsed = now.elapsed();
    println!("Part 1: {}", tops_part1);
    println!("Part 2: {}", tops_part2);
    println!("Elapsed: {:.2?}", elapsed);

    Ok((tops_part1, tops_part2))
}

#[derive(Debug, Clone)]
struct Move {
    how_many: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let (Some(_), Some(how_many), Some(_), Some(from), Some(_), Some(to)) = (
            words.next(),
            words.next(),
            words.next(),
            words.next(),
            words.next(),
            words.next(),
        ) else {
            return Err(color_eyre::eyre::eyre!(
                "expected move <move> from <from> to <to>, got {s:?}"
            ));
        };

        Ok(Self {
            how_many: how_many.parse()?,
            from: from.parse::<usize>()? - 1,
            to: to.parse::<usize>()? - 1,
        })
    }
}

#[derive(Debug, Default, Clone)]
struct Port {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

impl Port {
    pub fn apply_moves_9000(&mut self) {
        for a_move in self.moves.iter() {
            for i in 0..a_move.how_many {
                if let Some(a_crate) = self.stacks[a_move.from].pop_back() {
                    self.stacks[a_move.to].push_back(a_crate);
                }
            }
        }
    }

    pub fn apply_moves_9001(&mut self) {
        for a_move in self.moves.iter() {
            if let Ok([from, to]) =
                self.stacks.get_disjoint_mut([a_move.from, a_move.to])
            {
                let crane_arm = from.drain(from.len()-a_move.how_many..);
                to.extend(crane_arm);
            }
        }
    }

    pub fn get_tops(&self) -> String {
        let mut tops = String::from("");

        for stack in self.stacks.iter() {
            if let Some(a_crate) = stack.back() {
                tops += &a_crate.to_string();
            }
        }

        tops
    }

    pub fn parse_stack(&mut self, text: &String) -> color_eyre::Result<usize> {
        let mut eof = false;
        let mut line_offset = 0;

        for line in text.lines() {
            let mut chars = line.chars();
            let mut pos = 0;

            if eof {
                break;
            }

            while let Some(char) = chars.next() {
                match char {
                    ' ' => {
                        pos += 1;
                    }
                    '[' => {
                        self.add_crate(chars.next().unwrap_or_default(), pos);
                        pos += 3;
                    }
                    'm' => {
                        if pos == 0 {
                            eof = true;
                            break;
                        }
                    }
                    _ => (),
                }
            }

            line_offset += 1;
        }

        Ok(line_offset)
    }

    fn add_crate(&mut self, ident: char, pos: i32) {
        let stack_pos: usize = (pos / 4) as usize;
        if stack_pos >= self.stacks.len() {
            self.add_stack(stack_pos);
        }
        self.stacks[stack_pos].push_front(ident);
    }

    fn add_stack(&mut self, pos: usize) {
        for i in 0..=pos {
            if i >= self.stacks.len() {
                self.stacks.push(VecDeque::new())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn sample_test() {
        let vec_lines: Vec<String> =
            EXAMPLE.lines().map(str::to_string).collect();
        let res = original_solution(&EXAMPLE.to_string()).unwrap();

        assert_eq!(res, ("CMZ".to_string(), "MCD".to_string()));
    }
}
