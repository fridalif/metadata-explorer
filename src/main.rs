mod png;


use std::env;
use png::png::PngParser;

fn print_usage() {
    println!("Usage: mde <file_type> <options>");
    println!("\tFile types:");
    println!("\t\t--png");
    println!("For more informations about options write:\n\tmde <file_type> --help");
}

fn main() {
    let args = env::args().collect();

    for arg in &args {
        if arg == "--png" {
            let mut png_parser = PngParser::new();
            png_parser.parse(&args);
            png_parser.run();
            return;
        }
    }
    print_usage();
}
