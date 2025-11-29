mod original_solution;
use original_solution::original_solution;

fn main() {
    let lines = parser::read_lines("day4/data.txt").unwrap();

    let vec_lines: Vec<String> =
        lines.map(|l| l.expect("Could not parse line")).collect();


    original_solution(&vec_lines);
}
