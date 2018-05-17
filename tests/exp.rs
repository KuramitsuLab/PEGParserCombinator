extern crate peg_parser_combinator;

#[cfg(test)]
mod exp{

    use peg_parser_combinator::{ParserContext, Exp};

    #[test]
    fn anychar1() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::AnyChar;
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }

    #[test]
    fn anychar2() {
        let s: &'static str = "aaa";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Seq{e1: Box::new(Exp::AnyChar), e2: Box::new(Exp::Seq{e1: Box::new(Exp::AnyChar), e2: Box::new(Exp::AnyChar)})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }

    #[test]
    fn char1() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Char{c: 'a'};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }

    #[test]
    fn char2() {
        let s: &'static str = "aaa";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Seq{e1: Box::new(Exp::Char{c: 'a'}), e2: Box::new(Exp::Seq{e1: Box::new(Exp::Char{c: 'a'}), e2: Box::new(Exp::Char{c: 'a'})})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
}