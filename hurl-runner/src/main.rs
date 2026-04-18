use std::env;
use std::fs;
use std::process::{exit, Command};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

extern "C" {
    fn tree_sitter_hurl() -> Language;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: hurl-runner <file> <row>");
        exit(1);
    }

    let file = &args[1];
    let row: usize = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Error: row must be a number");
        exit(1);
    });

    let content = fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", file, e);
        exit(1);
    });

    let language = unsafe { tree_sitter_hurl() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let tree = parser.parse(&content, None).unwrap_or_else(|| {
        eprintln!("Failed to parse file");
        exit(1);
    });

    let query = Query::new(&language, "(entry) @entry").unwrap();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

    // collect all entry start lines (1-based to match ZED_ROW)
    let mut entry_lines: Vec<usize> = Vec::new();
    while let Some(m) = matches.next() {
        let line = m.captures[0].node.start_position().row + 1;
        entry_lines.push(line);
    }

    entry_lines.sort();

    // count how many entries start at or before our row
    let entry = entry_lines.iter().filter(|&&l| l <= row).count();

    if entry == 0 {
        eprintln!("No entry found at row {}", row);
        exit(1);
    }

    let status = Command::new("hurl")
        .arg("--from-entry")
        .arg(entry.to_string())
        .arg("--to-entry")
        .arg(entry.to_string())
        .arg(file)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Failed to run hurl: {}", e);
            exit(1);
        });

    exit(status.code().unwrap_or(1));
}
