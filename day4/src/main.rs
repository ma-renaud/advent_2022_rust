mod original_solution;
mod fasterthanlime_solution;

use original_solution::original_solution;
use fasterthanlime_solution::fasterthanlime_solution;

fn main() -> color_eyre::Result<()> {
    let text = parser::read_lines("day4/data.txt")?;

    let vec_lines: Vec<String> =
        text.lines().map(|l| l.to_string()).collect();


    original_solution(&vec_lines);
    fasterthanlime_solution(&vec_lines);

    Ok(())
}
