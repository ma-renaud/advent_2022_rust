use color_eyre::eyre::Result;
pub fn fasterthanlime_solution(_lines: &Vec<String>) -> Result<()> {
    use std::time::Instant;
    let now = Instant::now();



    let elapsed = now.elapsed();

    println!("\nFasterthanlime Solution");
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
