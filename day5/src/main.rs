#![allow(unused)]
mod original_solution;

use original_solution::original_solution;

fn main() -> color_eyre::Result<()> {
    let text = parser::read_lines("day5/data.txt")?;

    // let vec_lines: Vec<String> =
    //     lines.map(|l| l.expect("Could not parse line")).collect();
    //
    original_solution(&text);

    Ok(())
}
