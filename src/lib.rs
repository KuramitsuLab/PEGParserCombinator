use std::collections::HashMap;

#[derive(Debug,Clone)]
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
            &Exp::Char{ref c } => {  //cを使ってマッチするかどうか確かめる
                if p.input_len == p.pos{    //inputのposで指しているバイト配列の値とcをキャストしたものを比較
                   false
                }else if p.input[p.pos] == (*c as u8){
                    p.pos += 1;
                    true   
                }else{
                    false
                }
            },
            &Exp::AnyChar => {
                if p.input_len == p.pos{    //空、配列のn番目とpos数が同じならそこは空（配列は0から始まるから）
                    false
                }else{
                    p.pos += 1;
                    true    //空だとfalse
                }
            }, 
            &Exp::Symbol{ref sym} =>{
                let old = p.clone();
                let p_rule = p.clone();
                match p_rule.rules.get(sym){
                    Some(ref e) => if e.parse(p){
                        true
                    }else{
                        *p = old;
                        false
                    },
                    None => panic!("There is no rule. {}",sym),
                }
            },
            &Exp::Seq{ref e1, ref e2} =>{
                let old = p.clone();   
                if e1.parse(p){ //parse関数がe1のメソッド呼び
                    e2.parse(p)
                }else{
                    *p = old;
                    false
                }
            },
            &Exp::Choice{ref e1, ref e2} =>{  //バックトラックがある
                let old = p.clone();
                if e1.parse(p){
                    true
                }else{
                    *p = old;
                    e2.parse(p)
                }
            },
            &Exp::Rep{ref e} =>{
                loop{
                    let old = p.clone();
                    if !e.parse(p){
                        *p = old;
                        break;
                    }
                }
                true
            },
            &Exp::Opt{ref e} =>{
                let old = p.clone();
                if e.parse(p){
                    true
                }else{
                    *p = old;
                    true
                }
            },
            &Exp::Not{ref e} =>{
                let old = p.clone();
                if e.parse(p){
                    *p = old;
                    false
                }else{
                    *p = old;
                    true
                }
            },
            _ => panic!("{:?}: This expression is undefined!", self)
        }
    }
}
#[derive(Debug,Clone)]
pub struct ParserContext{
    pub input: Vec<u8>, //バイト配列
    pub input_len: usize,
    pub pos: usize,
    pub rules: HashMap<&'static str,Exp>,
}

impl ParserContext{
    pub fn new(input: Vec<u8>,rules: HashMap<&'static str,Exp>) -> ParserContext{
        ParserContext{
            input_len: input.len(),
            input: input,
            pos: 0,
            rules: rules,
        }
    }

}