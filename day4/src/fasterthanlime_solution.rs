use std::ops::RangeInclusive;
use itertools::Itertools;

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

pub fn fasterthanlime_solution(lines: &Vec<String>) -> (i32, i32) {
    use std::time::Instant;
    let now = Instant::now();

    let assignment = lines.into_iter()
        .map(|l| {
            l.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().expect("range start/end should be u32"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("each range should have a start and end")
                })
                .collect_tuple::<(_, _)>()
                .expect("each line must have a pair of ranges")
        });

    let redundant = assignment.clone()
        .filter(|(a, b)| a.contains_or_is_contained(b))
        .count();

    let overlapped = assignment
        .filter(|(a, b)| a.overlaps_or_is_overlapped(b))
        .count();

    let elapsed = now.elapsed();

    dbg!(redundant);
    dbg!(overlapped);
    println!("Elapsed: {:.2?}", elapsed);

    (redundant as i32, overlapped as i32)
}
