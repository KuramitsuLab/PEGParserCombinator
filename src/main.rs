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
/*
Expの修正とparse関数の追加、ParserContextの追加を行いました。
このparser関数でマッチされるExpを増やすようにして開発を進めていきましょう。
とりあえずEmptyだけ書いておきました。(未完成)
*/
impl Exp{
    fn parse(&self, p: &mut ParserContext) -> bool{
        match self {
            &Exp::Empty => true,
            &Evp::Char{ref c} => ,  //空だとfalse
            &Exp::AnyChar => true,  //空だとfalse
            &Exp::Symbol{ref sym} =>,
            &Exp::Seq{ref e1, ref e2} =>{
                if e1 && e2{
                    true
                }else{
                    false
                }
            },
            &Exp::Choice{ref e1, ref e2} =>{
                if e1{
                    true
                }else if e2{
                    true
                }else{
                    false
                }
            },
            &Exp::Rep{ref e} =>,
            &Exp::Opt{ref e} =>{
                if{

                }else{
                    true
                }
            },
            &Exp::Not{ref e} =>,
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
