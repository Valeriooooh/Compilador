use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct LangParser;

fn main() {
    let unparsed_file = std::fs::read_to_string("test.op").expect("cannot read file");

    let file = LangParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();
}

// fn foo() -> i32 {
//     4
// }
