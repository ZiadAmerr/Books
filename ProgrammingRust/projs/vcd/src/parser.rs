use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

/// Parses a single VCD file and returns a tuple (event_count, duration_ms)
/// where duration_ms is the time taken in milliseconds.
pub fn parse_vcd_file(file_name: &str) -> io::Result<(usize, u64)> {
    let start = Instant::now();
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut event_count = 0;
    // Count events by checking lines that start with '0' or '1' and end with '!'
    for line in reader.lines() {
        let line = line?;
        if (line.starts_with('0') || line.starts_with('1')) && line.ends_with('!') {
            event_count += 1;
        }
    }
    let duration = start.elapsed().as_millis() as u64;
    Ok((event_count, duration))
}

/// Parses all VCD files under the given directory (including subdirectories),
/// grouping them by simulation size (using the subdirectory name)
/// and computing the wall-clock parsing time per category.
pub fn parse_all_vcd_files(vcd_dir: &str) -> io::Result<()> {
    // Create a hash map to group file paths by simulation size.
    let mut groups: HashMap<u64, Vec<String>> = HashMap::new();

    // Iterate over subdirectories inside vcd_dir.
    for entry in read_dir(vcd_dir)? {
        let entry = entry?;
        let dir_path = entry.path();
        if dir_path.is_dir() {
            if let Some(dir_name) = dir_path.file_name().and_then(|s| s.to_str()) {
                if let Ok(sim_steps) = dir_name.parse::<u64>() {
                    // Iterate over files in each subdirectory.
                    for file_entry in read_dir(&dir_path)? {
                        let file_entry = file_entry?;
                        let file_path = file_entry.path();
                        if file_path.is_file() {
                            if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                                if ext == "vcd" {
                                    groups
                                        .entry(sim_steps)
                                        .or_default()
                                        .push(file_path.to_str().unwrap().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Process each category in parallel.
    for (sim_steps, files) in groups {
        let category_start = Instant::now();
        // Process files concurrently.
        let results: Vec<_> = files
            .par_iter()
            .map(|file| match parse_vcd_file(file) {
                Ok((events, _)) => (events, 0), // individual durations are ignored for wall-clock time
                Err(e) => {
                    eprintln!("Error parsing {}: {}", file, e);
                    (0, 0)
                }
            })
            .collect();

        let wall_clock_time = category_start.elapsed().as_millis() as u64;
        let total_files = results.len();

        // println!("Category: {} simulation steps", sim_steps);
        // println!("  Processed {} files.", total_files);
        // println!("  Wall-clock parsing time: {} ms", wall_clock_time);
        // println!("  Average wall-clock time per file: {} ms", if total_files > 0 { wall_clock_time / (total_files as u64) } else { 0 });
        let average_time = if total_files > 0 {
            wall_clock_time / (total_files as u64)
        } else {
            0
        };
        println!(
            "Category {} simulation steps took total {} ms to parse {} files with average of {} ms per file",
            sim_steps, wall_clock_time, total_files, average_time
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_file() {
        // Adjust the file name as needed for testing.
        let test_file = "/Users/ziad/Desktop/ML/Books/ProgrammingRust/projs/vcd/src/vcd_files/100/vcd_output_100_1.vcd";
        match parse_vcd_file(test_file) {
            Ok((events, dur)) => {
                println!("Test file parsed {} events in {} ms", events, dur);
            }
            Err(e) => panic!("Error parsing file {}: {}", test_file, e),
        }
    }
}
