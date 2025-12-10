#![allow(unused)]
mod original_solution;
mod fasterthanlime_solution;

use original_solution::original_solution;
use fasterthanlime_solution::fasterthanlime_solution;

fn main() -> color_eyre::Result<()> {
    let text = parser::read_lines("day5/test.txt")?;

    original_solution(&text);
    fasterthanlime_solution(&text);

    Ok(())
}
