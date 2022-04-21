use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let query = &args[2];

    let file_content = read_file(filename);
    let final_result = find_query(&file_content, &query);

    println!("Found {} results!", final_result.len());
    final_result.iter().for_each(|x| {
        println!(
            "{} | {}",
            x.line_number,
            x.line
                .replace(query, format!("\x1b[32m{}\x1b[0m", query).as_str())
        )
    });
}

fn read_file(filename: &String) -> String {
    let content =
        fs::read_to_string(filename).expect("Something went wrong while reading the file");
    content
}

struct SearchResult {
    line_number: usize,
    line: String,
}

impl SearchResult {
    fn new(line_number: usize, line: String) -> SearchResult {
        SearchResult { line_number, line }
    }
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
