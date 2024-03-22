use clap::Parser;
use log::{debug, warn};

mod file_util;
mod content_parser;
mod csv_util;

// Structs define here
/// A simple program to read an enum written in java and convert it to csv
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The input file to read
    #[arg(short, long)]
    input: String,
    /// The output file to write
    #[arg(short, long, default_value = "data/output.csv")]
    output: String,
}

fn main() {
    init_logger();
    debug!("Starting program");
    let args = parse_args();
    let content = file_util::read_file(&args.input);
    let csv_data = content_parser::parse_content(&content);
    csv_util::write_csv(&csv_data, &args.output);
}

fn parse_args() -> Args{
    let args = Args::parse();
    check_args(&args);
    args
}

fn check_args(args: &Args) {
    let input = &args.input;
    debug!("input: {}", input);
    // Check input file exists
    if !file_util::file_exists(input) {
        warn!("Input file does not exist: {}", input);
        std::process::exit(1);
    }
    // Check input file is a java file
    if !input.ends_with(".java") {
        warn!("Input file is not a java file: {}", input);
        std::process::exit(1);
    }
}

fn init_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file() {
        let file_path = "src/file_util.rs";
        let file_contents = file_util::read_file(file_path);
        assert!(file_contents.contains("read_file"));
    }

    #[test]
    #[should_panic(expected = "Input file does not exist: test_input.java")]
    fn test_check_args() {
        // arrange
        let args = Args {
            input: "test_input.java".to_string(),
            output: "output.csv".to_string(),
        };

        // act
        check_args(&args);
    }
}
