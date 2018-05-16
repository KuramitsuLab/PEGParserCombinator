#[derive(Debug)]
enum Exp{
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
    fn parse(&self, p: &mut ParserContext) -> bool{
        match self {
            &Exp::Empty => true,
            _ => panic!("{:?}: This expression is undefined!", self)
        }
    }
}

struct ParserContext{
    input: Vec<u8>,
}

fn main() {
    println!("{}", Exp::Empty.parse(&mut ParserContext{input: "hello".to_string().into_bytes()}));
}
