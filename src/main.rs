extern crate peg_parser_combinator;

use std::collections::HashMap;
use peg_parser_combinator::{ParserContext, Exp,Tree};
use peg_parser_combinator::*;

fn main() {
//    println!("{}", Exp::Empty.parse(&mut ParserContext::new("hello".to_string().into_bytes(),HashMap::new())));

    let s: &'static str = "(((((1)))))";
    let mut rules = HashMap::new();
    rules.insert("P",choice(seq(char('('),seq(sym("A"),char(')'))),char('1')));
    rules.insert("A",choice(seq(sym("P"),seq(char('+'),sym("A"))),choice(seq(sym("P"),seq(char('-'),sym("A"))),sym("P"))));
    let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),rules.clone());
    Exp::Symbol{sym:&"A"}.parse(&mut p);


/*
    let s: &'static str = "abc";
    let mut rules = HashMap::new();
        rules.insert("S",seq(sym("A"),sym("BC")));
        rules.insert("A",char('a'));
        rules.insert("BC",seq(char('b'),char('c')));  
     let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),rules.clone());
    Exp::Symbol{sym:&"S"}.parse(&mut p);
*/
    println!("{}",p.tree[0].two_string());
    println!("memosu{:?}",p.memos);

        
}


