extern crate peg_parser_combinator;

use std::collections::HashMap;
use peg_parser_combinator::{ParserContext, Exp};

fn main() {
    println!("{}", Exp::Empty.parse(&mut ParserContext::new("hello".to_string().into_bytes(),HashMap::new())));
}
