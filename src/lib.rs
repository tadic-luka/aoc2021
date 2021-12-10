use std::{env::args_os, fs::File, io::{self, Error, Read, ErrorKind}, path::PathBuf, str::FromStr, fmt::Display};

pub mod solution;

pub fn get_file() -> io::Result<File> {
    args_os().skip(1)
        .next()
        .map(PathBuf::from)
        .ok_or(Error::new(io::ErrorKind::Other, "Input path not provided"))
        .and_then(File::open)
}


pub fn get_input_contents(mut file: File) -> io::Result<String> {
    let mut input = String::new();
    file.read_to_string(&mut input)
        .map(|_| input)
}

pub fn get_lines<T>(contents: String) -> io::Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Display
{
    contents
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err: <T as FromStr>::Err| Error::new(ErrorKind::InvalidData, err.to_string()))
}
