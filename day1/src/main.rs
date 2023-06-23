extern crate parser;

fn main() {
    let lines = match parser::read_lines("day1/data_part1.txt") {
        Err(e) => panic!("Problem opening the file: {:?}", e),
        Ok(l) => l,
    };

    let mut current_count: u32 = 0;
    let mut calories = std::collections::BTreeSet::<_>::new();

    for line in lines {
        if let Ok(line_str) = line {
            if line_str.is_empty() {
                calories.insert(current_count);
                current_count = 0;
            } else {
                current_count += line_str.parse::<u32>().unwrap();
            }
        }
    }

    calories.insert(current_count);

    println!("Top calories: {}", calories.last().unwrap());

    let mut sum: u32 = 0;
    let mut calories_iter = calories.iter();
    for _ in 0..3 {
        sum += calories_iter.next_back().unwrap();
    }
    println!("Top three calories: {:?}", sum);
}
