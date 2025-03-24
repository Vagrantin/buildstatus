# XML Status Extractor

A Rust CLI tool that extracts specific data from XML files in a directory and outputs the results to a CSV file.

## Features

- Extracts `itemCode` values (preferring non-digit-only values) from XML files
- Finds the most recent status from `StatusHistoryRow` elements
- Outputs results to a CSV file with columns for itemCode, filename, and status
- Includes verbose mode for debugging

## Installation

### Prerequisites

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))

### Building from Source

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/xml-status-extractor.git
   cd xml-status-extractor
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

The compiled binary will be available at `target/release/xml_status_extractor`.

## Usage

```bash
xml_status_extractor --input-dir <INPUT_DIRECTORY> --output-file <OUTPUT_CSV>
```

### Command Line Options

- `-i, --input-dir <INPUT_DIRECTORY>` - Directory containing XML files to process (required)
- `-o, --output-file <OUTPUT_CSV>` - Path for the output CSV file (default: "status_output.csv")
- `-v, --verbose` - Enable verbose output for debugging
- `-h, --help` - Display help information
- `-V, --version` - Display version information

## XML Format

The tool expects XML files with a structure similar to this:

```xml
<root>
    <itemCode t="ws">ITEM123</itemCode>
    <!-- other tags -->
    <StatusHistory>
        <StatusHistoryRow date.td="2025-02-21T07:11:06.553">
            <comment t="ws">Create</comment>
            <userFirstName t="ws">UserName</userFirstName>
            <user t="ws"></user>
            <status t="ws">Status1</status>
        </StatusHistoryRow>
        <StatusHistoryRow date.td="2025-02-21T08:30:00.000">
            <comment t="ws">Update</comment>
            <userFirstName t="ws">UserName</userFirstName>
            <user t="ws"></user>
            <status t="ws">Status2</status>
        </StatusHistoryRow>
    </StatusHistory>
</root>
```

## Output Format

The tool generates a CSV file with the following columns:

1. `itemCode` - The value from the first non-digit-only `<itemCode>` tag (or first non-empty one if all contain only digits)
2. `filename` - The name of the XML file
3. `status` - The status value from the last (most recent) `<StatusHistoryRow>`

Example output:
```csv
itemCode,filename,status
ITEM123,file1.xml,Status2
ITEM456,file2.xml,In Progress
```

## Dependencies

- roxmltree - XML parsing
- csv - CSV file generation
- anyhow - Error handling
- clap - Command-line argument parsing
- regex - Pattern matching for digits-only check

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0) - see the LICENSE file for details. 

The GPL-3.0 is a strong copyleft license that requires anyone who distributes your code or a derivative work to make the source available under the same terms, ensuring that all modified and extended versions of the program remain free software.
