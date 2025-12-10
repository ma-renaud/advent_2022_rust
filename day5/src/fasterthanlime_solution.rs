use itertools::Itertools;
use nom::{
    Finish, IResult,
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
};
use std::fmt;

pub(crate) fn fasterthanlime_solution(
    text: &str,
) -> color_eyre::Result<(String, String)> {
    use std::time::Instant;
    let now = Instant::now();

    let mut lines = text.lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();

    let mut piles = Piles(transpose_rev(crate_lines));

    assert!(lines.next().unwrap().is_empty());

    let mut saved_line_iter = lines.clone();
    let mut saved_piles = piles.clone();

    for ins in lines
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
    {
        piles.apply_9000(ins);
    }

    let ans_part1 = piles.0.iter().map(|pile| pile.last().unwrap()).join("");

    for ins in saved_line_iter
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
    {
        saved_piles.apply_9001(ins);
    }

    let ans_part2 = saved_piles.0.iter().map(|pile| pile.last().unwrap()).join("");

    let elapsed = now.elapsed();
    println!("answer part 1 = {}", ans_part1);
    println!("answer part 2 = {}", ans_part2);
    println!("Elapsed: {:.2?}", elapsed);

    Ok((ans_part1, ans_part2))
}

#[derive(Clone, Copy)]
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
struct Piles(Vec<Vec<Crate>>);

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

impl Piles {
    fn apply_9000(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let el = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(el);
        }
    }

    fn apply_9001(&mut self, ins: Instruction) {
        let (src, dst) = self.0.borrow_two_mut(ins.src, ins.dst);

        for krate in (0..ins.quantity)
            .map(|_| src.pop().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            dst.push(krate);
        }
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    // `drop` takes a value and returns nothing, which is
    // perfect for our case
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) =
            opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

// convert from 1-indexed to 0-indexed
fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub trait BorrowTwoMut<T> {
    fn borrow_two_mut(&mut self, a: usize, b: usize) -> (&mut T, &mut T);
}

impl<T> BorrowTwoMut<T> for [T] {
    fn borrow_two_mut(&mut self, a: usize, b: usize) -> (&mut T, &mut T) {
        assert!(a != b);
        if a < b {
            let (l, r) = self.split_at_mut(b);
            (&mut l[a], &mut r[0])
        } else {
            let (l, r) = self.split_at_mut(a);
            (&mut r[0], &mut l[b])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fasterthanlime_solution::BorrowTwoMut;

    #[test]
    fn borrow_two_mut_works() {
        let mut arr = [0, 1, 2, 3, 4, 5];
        {
            let (a, b) = arr.borrow_two_mut(1, 4);
            assert_eq!(*a, 1);
            assert_eq!(*b, 4);
        }
        {
            let (a, b) = arr.borrow_two_mut(5, 2);
            assert_eq!(*a, 5);
            assert_eq!(*b, 2);
        }
    }

    #[test]
    #[should_panic]
    fn borrow_two_mut_panics_on_same_index() {
        let mut arr = [0, 1, 2, 3, 4, 5];
        let (_, _) = arr.borrow_two_mut(1, 1);
    }
}
