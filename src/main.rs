extern crate peg_parser_combinator;

use std::collections::HashMap;
use peg_parser_combinator::{ParserContext, Exp,Tree};
use peg_parser_combinator::*;

fn main() {
//    println!("{}", Exp::Empty.parse(&mut ParserContext::new("hello".to_string().into_bytes(),HashMap::new())));

    let s: &'static str = "((1))";
    let mut rules = HashMap::new();
    let mut child = Vec::new();
    rules.insert("P",choice(seq(char('('),seq(sym("A"),char(')'))),char('1')));
    rules.insert("A",choice(seq(sym("P"),seq(char('+'),sym("A"))),choice(seq(sym("P"),seq(char('-'),sym("A"))),sym("P"))));
    let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),rules.clone());
    rules.get(&"A").unwrap().parse(&mut p,&mut child);

    println!("{}",child[0].two_string());
 //   println!("memosu{:?}",p.memos);

        
}


