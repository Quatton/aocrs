use clap::Parser;
use std::{fs, io, path::PathBuf};

pub const INPUT_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../input");

#[derive(Parser)]
pub struct Args {
    #[arg(default_value = "real")]
    pub input: String,
}

pub fn read_input_arg(year: u16, day: u8) -> io::Result<String> {
    read_input(year, day, &Args::parse().input)
}

pub fn read_input(year: u16, day: u8, name: &str) -> io::Result<String> {
    let path = PathBuf::from(INPUT_ROOT)
        .join(year.to_string())
        .join(day.to_string())
        .join(format!("{name}.txt"));

    fs::read_to_string(path)
}

// Reset code
pub const RESET: &str = "\x1b[0m";

// Foreground colors
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";

// Styles
pub const BOLD: &str = "\x1b[1m";
pub const UNDERLINE: &str = "\x1b[4m";
