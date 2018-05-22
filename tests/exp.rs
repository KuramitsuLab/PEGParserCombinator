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
    #[test]
    fn choice1() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Choice{e1: Box::new(Exp::Char{c: 'a'}),e2: Box::new(Exp::Char{c: 'b'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn choice2() {
        let s: &'static str = "b";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Choice{e1: Box::new(Exp::Char{c: 'a'}),e2: Box::new(Exp::Char{c: 'b'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn zero_or_more1() {
        let s: &'static str = "";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Rep{e: Box::new(Exp::Char{c: 'a'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn zero_or_more2() {
        let s: &'static str = "aaaaaaaaaaaaaaaaaaaa";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Rep{e: Box::new(Exp::Char{c: 'a'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn one_or_more1() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Seq{e1: Box::new(Exp::Char{c: 'a'}),e2: Box::new(Exp::Rep{e: Box::new(Exp::Char{c: 'a'})})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn one_or_more2() {
        let s: &'static str = "aaaaaaaaaaaaaaaaaaa";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Seq{e1: Box::new(Exp::Char{c: 'a'}),e2: Box::new(Exp::Rep{e: Box::new(Exp::Char{c: 'a'})})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn opt1() {
        let s: &'static str = "";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Opt{e: Box::new(Exp::Char{c: 'a'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn opt2() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Opt{e: Box::new(Exp::Char{c: 'a'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn not1() {
        let s: &'static str = "b";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Not{e: Box::new(Exp::Char{c: 'a'})};
        assert!(peg.parse(&mut p) && (p.pos == 0));
    }
    #[test]
    fn and1() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes());
        let peg: Exp = Exp::Not{e: Box::new(Exp::Not{e: Box::new(Exp::Char{c: 'a'})})};
        assert!(peg.parse(&mut p) && (p.pos == 0));
    }     
}