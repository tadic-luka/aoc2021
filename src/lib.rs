use std::{env::args_os, fs::File, io::{self, Error, Read, ErrorKind}, path::PathBuf, str::FromStr, fmt::Display};

use solution::day04::{Board, COLS, ROWS, SIZE};

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

pub fn parse_day4_boards(input: String) -> io::Result<(Vec<usize>, Vec<Board>)> {
    let mut lines = input.lines();

    let numbers = match lines.next() {
        Some(line) => line
            .split(',')
            .map(|val| val.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| {
                Error::new(
                    ErrorKind::InvalidData,
                    "Unable to parse numbers from first line".to_string(),
                )
            })?,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Missing first line of numbers".to_string(),
            ));
        }
    };
    let mut boards = Vec::new();

    loop {
        // skip empty line
        if lines.next().is_none() {
            break;
        }
        let mut data = [0; SIZE];
        for i in 0..ROWS {
            let line = lines.next().ok_or(Error::new(
                ErrorKind::InvalidInput,
                "Missing board row".to_string(),
            ))?;
            let mut nums = line.split_whitespace();
            for j in 0..COLS {
                let num = nums.next().ok_or(Error::new(
                    ErrorKind::InvalidData,
                    "Missing board column".to_string(),
                ))?;
                let num = num.parse::<usize>().map_err(|_| {
                    Error::new(ErrorKind::InvalidInput, "Malformed input, number not given")
                })?;
                data[i * COLS + j] = num;
            }
        }
        boards.push(Board::new(data));
    }

    Ok((numbers, boards))
}
