use std::fs;
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "typescript.pest"] // Ensure this points to your grammar file
struct TypeScriptParser;

fn main() {
    let file_contents = fs::read_to_string("test.zts") // The TypeScript file to parse
        .expect("Unable to read the TypeScript file");

    match TypeScriptParser::parse(Rule::program, &file_contents) {
        Ok(pairs) => println!("Parsed successfully: {:?}", pairs),
        Err(e) => println!("Failed to parse: {}", e),
    }
}
