#![allow(unused)]

use clap::Parser;
use radix_fmt::*;

/// Convert a number from one base to another
#[derive(Parser)]
struct Cli {
    /// The base to convert to
    dest_base: String,
    /// The number to convert
    input: String,
    /// The base of the input number/base to convert from
    org_base: Option<String>,
}

/// Determine the base of the input number, if not specified
fn input_type(input: &str) -> u32 {
    match input {
        s if s.starts_with("0x") => 16,
        s if s.starts_with("0o") => 8,
        s if s.starts_with("0b") => 2,
        _ => 10,
    }
}

/// Parse the destination base, and panic if it is invalid
fn parse_dest_base(dest_base_str: &str) -> u8 {
    match dest_base_str.parse() {
        Ok(n) => {
            if n < 2 || n > 36 {
                panic!("Invalid destination base: {}", n);
            }
            n
        },
        Err(_) => panic!("Invalid destination base: {}", dest_base_str),
    }
}



fn main() {

    /// arguments are accepted in the form of:
    ///    <dest_base> <input> <org_base>
    /// or <dest_base> <input>
    let args = Cli::parse();

    // convert dest_base to integer
    // check if dest_base is valid, ie, between 2 and 36

    let dest_base: u8 = parse_dest_base(&args.dest_base);


    // define variables for input and org_base
    let mut input: String = args.input.clone();
    let mut org_base: u8 = 0;

    // if org_base is specified, convert it to integer
    // if not specified, determine the base of the input number and store as org_base

    if args.org_base.is_some() {
        org_base = args.org_base.unwrap().parse().unwrap();

        if org_base < 2 || org_base > 36 {
            panic!("Invalid origin base: {}", org_base);
        }
        input = input.to_string();
    } else {
        org_base = input_type(&args.input) as u8;
        if org_base != 10 {
            input = input[2..].to_string();
        }
    }

    // convert input from org_base to an integer in base 10
    // if from_str_radix fails, tell the user that the input is invalid

    let input: u64 = match u64::from_str_radix(&input, org_base as u32) {
        Ok(n) => n,
        Err(_) => panic!("Invalid input: {}", args.input),
    };


    // convert modified input to dest_base
    let output: Radix<u64> = radix(input, dest_base as u32 as u8);

    // print converted input
    println!("Converted Input: {}", output);
}
