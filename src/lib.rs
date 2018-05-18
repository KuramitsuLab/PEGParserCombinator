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
/*
Expの修正とparse関数の追加、ParserContextの追加を行いました。
このparser関数でマッチされるExpを増やすようにして開発を進めていきましょう。
とりあえずEmptyだけ書いておきました。(未完成)
*/
impl Exp{
    pub fn parse(&self, p: &mut ParserContext) -> bool{
        match self {
            &Exp::Empty => true,
            &Exp::Char{ref c} => {  //cを使ってマッチするかどうか確かめる
                if p.input_len == p.pos{
                   false
                }else{
                    p.pos += 1;
                    true   
                }
            },
            &Exp::AnyChar => {
                if p.input_len == p.pos{
                    false
                }else{
                    p.pos += 1;
                    true    //空だとfalse
                }
            }, 
//            &Exp::Symbol{ref sym} =>,
            &Exp::Seq{ref e1, ref e2} =>{   
                if e1.parse(p){ //parse関数がe1のメソッド呼び
                    e2.parse(p)
                }else{
                    false
                }
            },
/*            &Exp::Choice{ref e1, ref e2} =>{
                if {
                    true
                }else if {
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
*/
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