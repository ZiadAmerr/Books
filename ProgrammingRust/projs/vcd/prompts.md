Below is the updated main module that supports three commands – "create", "parse", and "parse_all". The "parse_all" command will use the parser module to recursively read all VCD files from your specified directory and process them in parallel.

------------------------------------------------

------------------------------------------------

Explanation:

1. The program's entry point collects command-line arguments and checks if at least one command was given.
2. If "create" is passed, it calls creation::generate_multiple_vcd_files() (which, as updated in your creation module, creates 20 files per simulation size) to create the VCD files.
3. If "parse" is passed, the program expects a filename after the command and calls parser::parse_vcd_file() on that file.
4. If "parse_all" is passed, it calls parser::parse_all_vcd_files() with your fixed VCD files directory. This command will read all VCD files (processing them in parallel using Rayon) and report per-category parsing times.
5. Usage instructions are printed if the input is invalid.

Make sure your project's Cargo.toml includes the appropriate dependencies (such as Rayon and Chrono) and that your modules “creation” and “parser” have been updated per your requirements.