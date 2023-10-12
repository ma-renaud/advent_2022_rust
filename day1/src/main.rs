extern crate parser;
use itertools::Itertools;

fn main() {
    let lines = parser::read_lines("day1/data.txt").unwrap();

    let vec_lines: Vec<String> = lines
        .map(|l| l.expect("Could not parse line"))
        .collect();

    original_solution(&vec_lines);
    fasterthanlime_btree_solution(&vec_lines);
    fasterthanlime_itertools_solution(&vec_lines);
}

fn original_solution(lines: &Vec<String>) {
    use std::time::Instant;
    let now = Instant::now();

    let calories_max;
    let mut current_count: u32 = 0;
    let mut calories = std::collections::BTreeSet::<_>::new();

    for line in lines {
        if line.is_empty() {
            calories.insert(current_count);
            current_count = 0;
        } else {
            current_count += line.parse::<u32>().unwrap();
        }
    }

    calories.insert(current_count);
    calories_max = *calories.last().unwrap();


    let mut sum: u32 = 0;
    let mut calories_iter = calories.iter();
    for _ in 0..3 {
        sum += calories_iter.next_back().unwrap();
    }

    let elapsed = now.elapsed();

    println!("\nOriginal Solution");
    println!("Top calories: {}", calories_max);
    println!("Top three calories: {:?}", sum);
    println!("Elapsed: {:.2?}", elapsed);
}

struct GroupSumIter<I> {
    inner: I,
}

impl<I> Iterator for GroupSumIter<I>
    where
        I: Iterator<Item=Option<u64>>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = loop {
            match self.inner.next() {
                Some(Some(v)) => break v,
                Some(None) => {
                    // huh, weird, didn't expect a separator there
                    // but let's just skip it
                }
                // we've reached the end of the inner iterator
                None => return None,
            }
        };

        loop {
            match self.inner.next() {
                Some(Some(v)) => sum += v,
                Some(None) | None => {
                    // reached a separator or the end of the iterator
                    break Some(sum);
                }
            }
        }
    }
}

fn fasterthanlime_btree_solution(lines: &Vec<String>) {
    use std::time::Instant;
    let now = Instant::now();

    let lines = lines.iter()
        .map(|v| v.parse::<u64>().ok());

    let btree: std::collections::BTreeSet::<_> = GroupSumIter { inner: lines }.collect();
    let sorted_vec = btree.iter().rev().map(|v| *v).collect::<Vec<_>>();

    let calories_max = *sorted_vec.first().unwrap();
    let sum = sorted_vec.iter().take(3).sum::<u64>();

    let elapsed = now.elapsed();

    println!("\nFasterthanlime Btree Solution");
    println!("Top calories: {}", calories_max);
    println!("Top three calories: {:?}", sum);
    println!("Elapsed: {:.2?}", elapsed);
}

fn fasterthanlime_itertools_solution(lines: &Vec<String>) {
    use std::time::Instant;
    let now = Instant::now();

    let sorted_vec = lines.iter()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .sorted_by_key(|&v| std::cmp::Reverse(v))
        .take(3)
        .collect::<Vec<_>>();

    let calories_max = *sorted_vec.first().unwrap();
    let sum = sorted_vec.iter().take(3).sum::<u64>();

    let elapsed = now.elapsed();

    println!("\nFasterthanlime Btree Solution");
    println!("Top calories: {}", calories_max);
    println!("Top three calories: {:?}", sum);
    println!("Elapsed: {:.2?}", elapsed);
}
