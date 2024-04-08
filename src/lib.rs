pub mod lang;
mod test;
pub mod value;
pub mod vm;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"] // Path to your grammar file
pub struct LanguageParser;
