mod original_solution;
mod fasterthanlime_solution;

use original_solution::original_solution;
use fasterthanlime_solution::fasterthanlime_solution;


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let lines = parser::read_lines("day3/data.txt").unwrap();

    let vec_lines: Vec<String> = lines
        .map(|l| l.expect("Could not parse line"))
        .collect();

    original_solution(&vec_lines);
    fasterthanlime_solution(&vec_lines)?;

    Ok(())
}