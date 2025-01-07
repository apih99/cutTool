use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Fields to select
    #[arg(short = 'f')]
    fields: String,

    /// Field delimiter
    #[arg(short = 'd', default_value = "\t")]
    delimiter: String,

    /// Input file
    #[arg(value_name = "FILE")]
    file: String,
}

fn parse_field_list(s: &str) -> Result<Vec<usize>, String> {
    s.split(',')
        .map(|field| {
            field
                .parse::<usize>()
                .map_err(|_| format!("Invalid field number: {}", field))
        })
        .collect()
}

fn process_file(args: &Args) -> Result<()> {
    let fields = parse_field_list(&args.fields)
        .map_err(|e| anyhow::anyhow!("Failed to parse fields: {}", e))?;
    
    let file = File::open(&args.file)
        .with_context(|| format!("Failed to open file: {}", args.file))?;

    if args.delimiter == "," {
        // Handle CSV files with proper CSV parsing
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(file);

        for result in csv_reader.records() {
            let record = result?;
            let mut output = String::new();
            for (i, &field_index) in fields.iter().enumerate() {
                if i > 0 {
                    output.push_str(&args.delimiter);
                }
                if let Some(field) = record.get(field_index - 1) {
                    output.push_str(field);
                }
            }
            println!("{}", output);
        }
    } else {
        // Handle other delimited files
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let line_fields: Vec<_> = line.split(&args.delimiter).collect();
            let mut output = String::new();
            for (i, &field_index) in fields.iter().enumerate() {
                if i > 0 {
                    output.push_str(&args.delimiter);
                }
                if let Some(&field) = line_fields.get(field_index - 1) {
                    output.push_str(field);
                }
            }
            println!("{}", output);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    process_file(&args)?;
    Ok(())
}
