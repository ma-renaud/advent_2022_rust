use std::fs::File;
use std::io::{self, BufRead};

pub struct PathedIoError {
    path: String,
    inner: io::Error,
}

impl std::fmt::Debug for PathedIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "For file {:?}: {}", self.path, self.inner)
    }
}

pub fn read_lines(path: &str) -> Result<io::Lines<io::BufReader<File>>, PathedIoError>
{
    match File::open(path) {
        Ok(s) => Ok(io::BufReader::new(s).lines()),
        Err(e) => Err(PathedIoError {
            path: path.into(),
            inner: e,
        }),
    }
}
