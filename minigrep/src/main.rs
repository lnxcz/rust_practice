use std::env;
use std::fs;

use minigrep::{Input, SearchResult};

fn main() {
    let args: Vec<String> = env::args().collect();

    let inputs = Input::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let file_content = read_file(&inputs.filename);
    let final_result = find_query(&file_content, &inputs.query);

    println!("Found {} results!", final_result.len());
    final_result.iter().for_each(|x| {
        println!(
            "{} | {}",
            x.line_number,
            x.line.replace(
                inputs.query.as_str(),
                format!("\x1b[32m{}\x1b[0m", inputs.query).as_str()
            )
        )
    });
}

fn read_file(filename: &String) -> String {
    let content =
        fs::read_to_string(filename).expect("Something went wrong while reading the file");
    content
}

fn find_query(content: &String, query: &String) -> Vec<SearchResult> {
    let content: Vec<&str> = content.split("\n").collect();
    let mut results: Vec<SearchResult> = Vec::new();
    for (line_number, line) in content.iter().enumerate() {
        if line.split_whitespace().any(|word| word == query) {
            results.push(SearchResult::new(line_number, line.to_string()));
        }
    }
    results
}
