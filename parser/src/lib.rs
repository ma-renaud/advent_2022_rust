pub fn read_lines(path: &str) -> color_eyre::Result<String>
{
    std::fs::read_to_string(path).map_err(From::from)
}
