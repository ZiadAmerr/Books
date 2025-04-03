use chrono::Utc;
use rayon::prelude::*;
use std::fs::{File, create_dir_all};
use std::io::{self, BufWriter, Write};

/// Creates one VCD file with a simple header and a single toggling signal.
/// The VCD file will have num_steps simulation time steps.
pub fn create_vcd_file(file_name: &str, num_steps: u64) -> io::Result<()> {
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);

    // Write header information.
    writeln!(writer, "$date\n   {}\n$end", Utc::now())?;
    writeln!(writer, "$version\n   Rust VCD file generator\n$end")?;
    writeln!(writer, "$timescale 1ns $end")?;
    writeln!(writer, "$scope module top $end")?;
    writeln!(writer, "$var wire 1 ! signal1 $end")?;
    writeln!(writer, "$upscope $end")?;
    writeln!(writer, "$enddefinitions $end")?;

    // Write initial dump variables.
    writeln!(writer, "$dumpvars")?;
    writeln!(writer, "0!")?;
    writeln!(writer, "$end")?;

    // Write simulation data:
    // For simplicity, the signal toggles each timestep.
    let mut current_val = false;
    for t in 0..num_steps {
        writeln!(writer, "#{}", t)?;
        current_val = !current_val;
        let val_char = if current_val { '1' } else { '0' };
        writeln!(writer, "{}!", val_char)?;
    }

    writer.flush()
}

/// Generates 20 VCD files for each simulation length.
/// Files are placed into separate directories under "./vcd_files/{simulation_length}".
/// This version creates files in parallel using Rayon.
pub fn generate_multiple_vcd_files() -> io::Result<()> {
    // Define the simulation lengths.
    let simulation_lengths = [100_u64, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];

    // Base directory for all VCD files.
    let target_dir = "./vcd_files";
    create_dir_all(target_dir)?;

    // Create a vector to collect errors during parallel file creation.
    let errors: Vec<String> = simulation_lengths
        .par_iter()
        .flat_map(|&steps| {
            // Create a separate directory for each simulation length.
            let category_dir = format!("{}/{}", target_dir, steps);
            if let Err(e) = create_dir_all(&category_dir) {
                return vec![format!("Error creating {}: {}", category_dir, e)];
            }
            (1..=20)
                .into_par_iter()
                .filter_map(move |file_index| {
                    let file_name =
                        format!("{}/vcd_output_{}_{}.vcd", category_dir, steps, file_index);
                    println!("Creating {} with {} steps...", file_name, steps);
                    if let Err(e) = create_vcd_file(&file_name, steps) {
                        Some(format!("Error creating {}: {}", file_name, e))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    if !errors.is_empty() {
        // If any errors occurred, return the first error wrapped in an io::Error.
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Errors occurred:\n{}", errors.join("\n")),
        ));
    }

    Ok(())
}
