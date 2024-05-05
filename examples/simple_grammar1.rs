use pilator::parser::*;

fn main() {
    let s = NaiveParser::new();
    match s.parse("a", None) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
    println!("Hello, world!")
}

// cargo run --emaple simple_grammar1.rs