use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
struct Cli {
    /// The input files (optional, reads from stdin if not provided)
    files: Vec<String>,
    /// Include empty lines in the output
    #[clap(short, long)]
    empty: bool,
    /// Align numeric columns to the right (default is left alignment)
    #[clap(short = 'l', long)]
    left_align_numeric: bool,
}

fn main() {
    let args = Cli::parse();
    // println!("Args: {:?}", args);  // 添加这行来打印 args
    let mut all_lines = Vec::new();
    
    if args.files.is_empty() {
        // Read from stdin if no files are provided
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                process_line(&mut all_lines, &line, args.empty);
            }
        }
    } else {
        // Read from provided files
        for file_name in &args.files {
            match File::open(file_name) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            process_line(&mut all_lines, &line, args.empty);
                        }
                    }
                },
                Err(e) => eprintln!("Failed to open file '{}': {}", file_name, e),
            }
        }
    }

    if !all_lines.is_empty() {
        let formatted = format_columns(&all_lines, args.left_align_numeric);
        print!("{}", formatted);
    }
}

fn process_line(all_lines: &mut Vec<String>, line: &str, include_empty: bool) {
    if !line.trim().is_empty() || include_empty {
        all_lines.push(line.to_string());
    }
}

fn format_columns(lines: &[String], left_align_numeric: bool) -> String {
    let mut columns: Vec<Vec<String>> = Vec::new();
    let mut max_widths: Vec<usize> = Vec::new();
    let mut is_numeric: Vec<bool> = Vec::new();

    // Split lines into columns and find max width for each column
    for (line_index, line) in lines.iter().enumerate() {
        let parts: Vec<String> = line.split_whitespace().map(String::from).collect();
        for (i, part) in parts.iter().enumerate() {
            if i >= columns.len() {
                columns.push(Vec::new());
                max_widths.push(0);
                is_numeric.push(true); // Initialize as true
            }
            columns[i].push(part.clone());
            max_widths[i] = max_widths[i].max(part.len());
            if line_index > 0 { // Only check for numeric after the header row
                is_numeric[i] &= is_numeric_string(part);
            }
        }
    }

    // Debug output
    // println!("Is numeric: {:?}", is_numeric);
    // println!("Max widths: {:?}", max_widths);

    // Format output with aligned columns
    let mut output = String::new();
    for i in 0..columns[0].len() {
        let mut line = String::new();
        for (j, column) in columns.iter().enumerate() {
            if i < column.len() {
                let align_left = left_align_numeric && is_numeric[j] && i > 0; // Don't right-align header
                let formatted = if align_left {
                    format!("{:<width$}", column[i], width = max_widths[j])
                } else {
                    format!("{:>width$}", column[i], width = max_widths[j])
                };
                // Debug output
                // println!("Column {}, value: '{}', align_left: {}, formatted: '{}'", j, column[i], align_left, formatted);
                line.push_str(&formatted);
                line.push_str("  "); // Two spaces between columns
            }
        }
        line = line.trim_end().to_string(); // Remove trailing spaces
        output.push_str(&line);
        output.push('\n');
    }

    output
}

fn is_numeric_string(s: &str) -> bool {
    let result = s.replace(',', "").parse::<f64>().is_ok();
    // println!("Checking if '{}' is numeric: {}", s, result); // Debug output
    result
}
