use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(name = "Rusty Cat")]
#[command(author = "Otavio G.")]
#[command(version = "1.0")]
#[command(about = "A clone of the cat command written in Rust", long_about = None)]
pub struct Cli {
    #[arg(default_value("-"), num_args(1..), help("Input file(s)"))]
    files: Vec<String>,
    #[arg(short('n'), long("number"), help("Number lines"))]
    number_of_lines: bool,
    #[arg(short('b'), long("number-nonblank"), help("Number nonblack lines"))]
    number_nonblank_lines: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    number_of_lines: bool,
    number_nonblank_lines: bool,
}

/// Parse Cli arguments and generate config struct.
pub fn get_args() -> MyResult<Config> {
    let matches = Cli::parse();

    Ok(Config {
        files: matches.files,
        number_of_lines: matches.number_of_lines,
        number_nonblank_lines: matches.number_nonblank_lines,
    })
}

/// Executes the program given the arguments parsed.
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                
                if config.number_of_lines {
                    print_with_line_numbers(buffer)
                } else if config.number_nonblank_lines {
                    print_with_line_numbers_skipping_blank_lines(buffer)
                } else {
                    print_without_line_numbers(buffer)
                }
            },
        }
    }
    Ok(())
}

/// Open the file and return the File object if successful.
pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

/// Print the contents of the files with line numbers, including brank lines.
pub fn print_with_line_numbers(string_buffer: String) {
    for (n_line, line) in string_buffer.lines().enumerate() {
       println!("{:>6}\t{}", n_line + 1, line);
    }
}

/// Print the contents of the files with line numbers but skips blank lines.
pub fn print_with_line_numbers_skipping_blank_lines(string_buffer: String) {
    let mut count_blank_lines = 0;
    for (n_line, line) in string_buffer.lines().enumerate() {
        if line.is_empty() {
            count_blank_lines += 1;
            println!();
        } else { 
            println!("{:>6}\t{}", n_line + 1 - count_blank_lines, line);
        }
    }
}

/// Prints the contents of the files without the line numbers.
pub fn print_without_line_numbers(string_buffer: String) {
    for line in string_buffer.lines() {
       println!("{}", line);
    }
}

