extern crate peg_parser_combinator;

use std::collections::HashMap;
use peg_parser_combinator::{ParserContext, Exp};
use peg_parser_combinator::*;

/*
fn print(s: &'static str,p: &mut ParserContext){
    let mut p = &mut peg_parser_combinator::ParserContext::new;
    let mut p.tree = Vec::new();

    p.tree.push(Tree::Noed{})
}
*/

fn main() {
//    println!("{}", Exp::Empty.parse(&mut ParserContext::new("hello".to_string().into_bytes(),HashMap::new())));


    let mut rules = HashMap::new();
    rules.insert("S",seq(sym("A"),sym("BC")));
    rules.insert("A",char('a'));
    rules.insert("BC",seq(char('b'),char('c')));      
    let mut tree = Vec::new();

    println!("{}", Exp::Symbol{sym: &"S"}.parse(&mut ParserContext::new("abc".to_string().into_bytes(),rules), &mut tree));
    println!("{:?}",tree);
        
}


