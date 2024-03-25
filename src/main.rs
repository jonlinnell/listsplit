use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The number spaces to insert before each line
    #[arg(short, long, default_value_t = 4)]
    indent_depth: usize,

    /// The number of elements to display per line
    #[arg(short = 'c', long, default_value_t = 5)]
    max_elements_per_line: usize,
}

fn main() {
    let args = Args::parse();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut input = String::new();
    handle.read_line(&mut input).expect("Failed to read line");

    input.pop();

    let mut elements = input.split(',');
    let mut line = String::new();

    let mut element_count = 0;
    let mut total_count = 0;
    let mut count = 0;

    for _ in elements.clone().by_ref() {
        element_count += 1;
    }

    while let Some(element) = elements.next() {
        line.push_str(element);

        count += 1;
        total_count += 1;

        if total_count != element_count {
            line.push(',');
        }

        if count == args.max_elements_per_line {
            println!("{:indent$}{}", " ", line, indent = args.indent_depth);
            line.clear();
            count = 0;
        }
    }

    if !line.is_empty() {
        println!("{:indent$}{}", " ", line, indent = args.indent_depth);
    }
}
