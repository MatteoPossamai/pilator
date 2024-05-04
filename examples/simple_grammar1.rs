use pilator::parser::SimpleParser;
use pilator::parser::Parser;

fn main() {
    let s = SimpleParser {};
    match s.parse("a", ()) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
    println!("Hello, world!")
}

// cargo run --emaple simple_grammar1.rs