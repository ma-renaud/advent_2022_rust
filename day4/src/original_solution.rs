use std::ops::RangeInclusive;

pub fn original_solution(lines: &Vec<String>) -> (i32, i32) {
    let mut fully_contained_pair = 0;
    let mut overlapping_pair = 0;

    for line in lines {
        let ranges = line
            .split(",")
            .map(parse_range)
            .collect::<Vec<RangeInclusive<u16>>>();

        let [range1, range2] = ranges.as_slice() else {
            core::panic!("Invalid line: {}", line);
        };

        if range_contains_range(range1, range2) {
            fully_contained_pair += 1;
        }

        if range_overlaps_range(range1, range2) {
            overlapping_pair += 1;
        }
    }

    println!("Fully contained pair(s): {}", fully_contained_pair);
    println!("Overlapping pair(s): {}", overlapping_pair);

    (fully_contained_pair, overlapping_pair)
}

fn range_contains_range(
    range1: &RangeInclusive<u16>,
    range2: &RangeInclusive<u16>,
) -> bool {
    (*range1.start() >= *range2.start()
        && *range1.start() <= *range2.end()
        && *range1.end() <= *range2.end())
        || (*range2.start() >= *range1.start()
            && *range2.start() <= *range1.end()
            && *range2.end() <= *range1.end())
}

fn range_overlaps_range(
    range1: &RangeInclusive<u16>,
    range2: &RangeInclusive<u16>,
) -> bool {
    *range1.start() >= *range2.start() && *range1.start() <= *range2.end()
        || *range2.start() >= *range1.start()
            && *range2.start() <= *range1.end()
}

fn parse_range(range_str: &str) -> RangeInclusive<u16> {
    let range_split = range_str.split("-").collect::<Vec<_>>();
    let [first_str, second_str] = range_split.as_slice() else {
        core::panic!("Invalid range: {}", range_str);
    };
    let [start, end] = [first_str, second_str].map(|r| {
        r.parse()
            .expect(&format!("parse range part: {} of {}", r, range_str))
    });
    start..=end
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn sample_test() {
        let vec_lines: Vec<String> =
            EXAMPLE.lines().map(str::to_string).collect();
        let res = original_solution(&vec_lines);

        assert_eq!(res, (2, 4));
    }
}
