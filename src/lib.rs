#[derive(Debug)]
pub enum Exp{
    Empty,
    Char{c:char},
    AnyChar,
    Symbol{sym:&'static str},
    Seq{e1:Box<Exp>, e2:Box<Exp>},
    Choice{e1:Box<Exp>, e2:Box<Exp>},
    Rep{e:Box<Exp>},
    Opt{e:Box<Exp>},
    Not{e:Box<Exp>},
}

impl Exp{
    pub fn parse(&self, p: &mut ParserContext) -> bool{
        match self {
            &Exp::Empty => true,
            _ => panic!("{:?}: This expression is undefined!", self)
        }
    }
}

pub struct ParserContext{
    pub input: Vec<u8>,
    pub input_len: usize,
    pub pos: usize,
}

impl ParserContext{
    pub fn new(input: Vec<u8>) -> ParserContext{
        ParserContext{
            input_len: input.len(),
            input: input,
            pos: 0,
        }
    }
}