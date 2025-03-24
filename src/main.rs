use std::fs::{self};
use std::path::{Path, PathBuf};
use roxmltree::{Document, Node};
use csv::Writer;
use anyhow::{Result, Context, bail};
use clap::{Parser, ArgAction};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input directory with XML files
    #[arg(short, long)]
    input_dir: String,

    /// Output CSV file
    #[arg(short, long, default_value = "status_output.csv")]
    output_file: String,

    /// Verbose mode
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

#[derive(Debug)]
struct StatusInfo {
    item_code: String,
    filename: String,
    status: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let input_dir = Path::new(&args.input_dir);
    if !input_dir.exists() || !input_dir.is_dir() {
        bail!("The input directory doesn't exist or is not a directory: {}", args.input_dir);
    }
    
    let mut result_data = Vec::new();
    
    // Process each XML file in the directory
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("xml") {
            if args.verbose {
                println!("Processing file: {}", path.display());
            }
            
            match process_file(&path) {
                Ok(status_info) => {
                    if args.verbose {
                        println!("  Item Code: {}", status_info.item_code);
                        println!("  Status: {}", status_info.status);
                    }
                    result_data.push(status_info);
                },
                Err(err) => {
                    eprintln!("Error processing file {}: {}", path.display(), err);
                }
            }
        }
    }
    
    // Write results to CSV
    write_to_csv(&args.output_file, &result_data)?;
    
    println!("Successfully processed {} files. Results saved to {}", 
             result_data.len(), args.output_file);
    
    Ok(())
}

fn is_digits_only(s: &str) -> bool {
    let re = Regex::new(r"^\d+$").unwrap();
    re.is_match(s.trim())
}

fn process_file(file_path: &PathBuf) -> Result<StatusInfo> {
    // Read file content
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;
    
    // Parse XML
    let doc = Document::parse(&content)
        .with_context(|| format!("Failed to parse XML in file: {}", file_path.display()))?;
    
    // Find the first itemCode that is not digits-only
    let item_codes: Vec<String> = doc.descendants()
        .filter(|n| n.has_tag_name("itemCode"))
        .filter_map(|n| n.text().map(|t| t.to_string()))
        .collect();
    
    let item_code = item_codes.iter()
        .find(|code| !code.is_empty() && !is_digits_only(code))
        .cloned()
        .unwrap_or_else(|| {
            // If no non-digit itemCode found, use the first non-empty one
            item_codes.iter()
                .find(|code| !code.is_empty())
                .cloned()
                .unwrap_or_default()
        });
    
    // Find all StatusHistoryRow nodes
    let status_rows: Vec<Node> = doc.descendants()
        .filter(|n| n.has_tag_name("StatusHistoryRow"))
        .collect();
    
    // Get the last StatusHistoryRow (most recent status)
    let last_row = status_rows.last()
        .with_context(|| format!("No StatusHistoryRow found in file: {}", file_path.display()))?;
    
    // Find the status within this row
    let status = last_row.descendants()
        .find(|n| n.has_tag_name("status"))
        .and_then(|n| n.text())
        .with_context(|| format!("No status tag found in the last StatusHistoryRow in file: {}", file_path.display()))?;
    
    // Extract filename from path
    let filename = file_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    Ok(StatusInfo {
        item_code,
        filename,
        status: status.to_string(),
    })
}

fn write_to_csv(output_path: &str, data: &[StatusInfo]) -> Result<()> {
    let mut wtr = Writer::from_path(output_path)?;
    
    // Write header
    wtr.write_record(&["itemCode", "filename", "status"])?;
    
    // Write data
    for item in data {
        wtr.write_record(&[&item.item_code, &item.filename, &item.status])?;
    }
    
    wtr.flush()?;
    Ok(())
}
