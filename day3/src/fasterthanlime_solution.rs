use color_eyre::eyre::Result;
use item::Item;
use std::collections::HashSet;
use itertools::Itertools;

mod item {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Item(u8);

    impl TryFrom<u8> for Item {
        type Error = color_eyre::Report;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err(color_eyre::eyre::eyre!(
                "{} is not a valid item",
                value as char
            )),
            }
        }
    }

    impl Item {
        pub(crate) fn score(self) -> usize {
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
                _ => unreachable!(),
            }
        }
    }

    impl std::fmt::Debug for Item {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0 as char)
        }
    }
}

pub fn fasterthanlime_solution(lines: &Vec<String>) -> Result<()> {
    use std::time::Instant;
    let now = Instant::now();

    let sum = lines.iter()
        .map(|line| -> color_eyre::Result<_> {
            let (first, second) = line.split_at(line.len() / 2);
            let first_items = first
                .bytes()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()?;
            itertools::process_results(second.bytes().map(Item::try_from), |mut it| {
                it.find(|&item| first_items.contains(&item))
                    .map(|item| item.score())
                    .ok_or_else(|| color_eyre::eyre::eyre!("compartments have no items in common"))
            })?
        })
        .sum::<color_eyre::Result<usize>>()?;

    let rucksacks = lines.iter().map(|line| {
        line.bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()
    });

    let badges_sum = itertools::process_results(rucksacks, |rs| {
        rs.tuples()
            .map(|(a, b, c)| {
                a.iter()
                    .copied()
                    .find(|i| b.contains(i) && c.contains(i))
                    .map(|i| i.score())
                    .unwrap_or_default()
            })
            .sum::<usize>()
    })?;

    let elapsed = now.elapsed();

    println!("\nFasterthanlime Solution");
    println!("Priority sum is {}", sum);
    println!("Priority badges is {}", badges_sum);
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
