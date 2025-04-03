use std::env;
use std::process;

mod creation;
mod parser;

fn print_usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!(
        "  {} create                - Create sample VCD files.",
        program_name
    );
    eprintln!(
        "  {} parse <filename>      - Parse the specified VCD file.",
        program_name
    );
    eprintln!(
        "  {} parse_all             - Parse all VCD files in the designated directory.",
        program_name
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];

    if args.len() < 2 {
        print_usage(program_name);
        process::exit(1);
    }

    match args[1].as_str() {
        "create" => {
            println!("Creating sample VCD files...");
            if let Err(e) = creation::generate_multiple_vcd_files() {
                eprintln!("Error creating VCD files: {}", e);
                process::exit(1);
            } else {
                println!("VCD files created successfully.");
            }
        }
        "parse" => {
            if args.len() < 3 {
                eprintln!("Error: You must supply a filename to parse.");
                print_usage(program_name);
                process::exit(1);
            }
            let filename = &args[2];
            if let Err(e) = parser::parse_vcd_file(filename) {
                eprintln!("Error parsing VCD file {}: {}", filename, e);
                process::exit(1);
            }
        }
        "parse_all" => {
            println!("Parsing all VCD files in the directory...");
            // Adjust the directory path if needed.
            let vcd_dir = "/Users/ziad/Desktop/ML/Books/ProgrammingRust/projs/vcd/src/vcd_files";
            if let Err(e) = parser::parse_all_vcd_files(vcd_dir) {
                eprintln!("Error parsing VCD files: {}", e);
                process::exit(1);
            } else {
                println!("All VCD files parsed successfully.");
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage(program_name);
            process::exit(1);
        }
    }
}
