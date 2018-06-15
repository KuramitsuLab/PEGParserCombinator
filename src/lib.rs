/*
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

//parseの引数減らして、parsercontextにtreeにいれて

impl Exp{
    pub fn parse(&self, p: &mut ParserContext,mut child: &mut Vec<Tree>) -> bool{ 
        
        match self {
            &Exp::Empty => true,
            &Exp::Char{ref c } => {  //cを使ってマッチするかどうか確かめる
println!("char"); 
                if p.input_len == p.pos{    //inputのposで指しているバイト配列の値とcをキャストしたものを比較
                   false
                }else if p.input[p.pos] == (*c as u8){
                    p.pos += 1;
                    child.push(Tree::Leaf{val: *c});
                    true   
                }else{
                    false
                }
            },
            &Exp::AnyChar => {
println!("anychar");
                if p.input_len == p.pos{    //空、配列のn番目とpos数が同じならそこは空（配列は0から始まるから）
                    false
                }else{
                    p.pos += 1;
                    true    
                }
            }, 
            &Exp::Symbol{ref sym} =>{
println!("symbol");
                let old = p.clone();
                let p_rule = p.clone();
                let mut newtree = Vec::new();

                match p.memos.clone().get(&(p.pos,sym)) { 
                    None => {
                        match p_rule.rules.get(sym){
                            Some(ref e) => if e.parse(p,&mut newtree){
                                child.push(Tree::Node{sym: sym,child: newtree.clone()});
//                                p.memos.insert((p.pos, sym), Memos::Memo{pos: p.pos,child: child.clone()});
println!("{:?}",child);
                                true
                            }else{
                                *p = old;
                                false
                            },
                            None => panic!("There is no rule. {}",sym),
                        }
                    },
                    Some(ref memo) => {
                        println!("*");
                        match memo{
                            &Memos::Fail => return false,
                            &Memos::Memo{ref pos,child: ref newchild} => {
                                child.push(Tree::Node{sym: sym,child: newchild.clone()});
                                true
                            }, 
                        }
                    },  
                }
            },
            &Exp::Seq{ref e1, ref e2} =>{
println!("seq");
                let old = p.clone();   
                if e1.parse(p,&mut child){ //parse関数がe1のメソッド呼び
                    if e2.parse(p,&mut child){
                        true
                    }else{
                        *p = old;
                        false
                    }
                }else{
                    *p = old;
                    false
                }
            },
            &Exp::Choice{ref e1, ref e2} =>{  
                let old = p.clone();
                if e1.parse(p,&mut child){
                    true
                }else{
                    *p = old.clone();
                    if e2.parse(p,&mut child){
                        true
                    }else{
                        *p = old;
                        false
                    }
                }
            },
            &Exp::Rep{ref e} =>{
                loop{
                    let old = p.clone();
                    if !e.parse(p,&mut child){
                        *p = old;
                        break;
                    }
                }
                true
            },
            &Exp::Opt{ref e} =>{
                let old = p.clone();
                if e.parse(p,&mut child){
                    true
                }else{
                    *p = old;
                    true
                }
            },
            &Exp::Not{ref e} =>{
                let old = p.clone();
                if e.parse(p,&mut child){
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

    pub fn emp() -> Exp{
        Exp::Empty
    }
    pub fn char(c: char) -> Exp{
        Exp::Char{c: c}
    }
    pub fn any() -> Exp{
        Exp::AnyChar
    }
    pub fn sym(sym: &'static str) -> Exp{
        Exp::Symbol{sym: sym}
    }
    pub fn seq(e1: Exp,e2: Exp) -> Exp{
        Exp::Seq{e1: Box::new(e1),e2: Box::new(e2)}
    }
    pub fn choice(e1: Exp,e2: Exp) -> Exp{
        Exp::Choice{e1: Box::new(e1),e2: Box::new(e2)}
    }
    pub fn rep0(e: Exp) -> Exp{
        Exp::Rep{e: Box::new(e)}
    }
    pub fn rep1(e: Exp) -> Exp{
        Exp::Seq{e1: Box::new(e.clone()),e2: Box::new(Exp::Rep{e: Box::new(e)})}
    }
    pub fn opt(e: Exp) -> Exp{
        Exp::Opt{e: Box::new(e)}
    }
    pub fn not(e: Exp) -> Exp{
        Exp::Not{e: Box::new(e)}
    }

#[derive(Debug,Clone)]
pub struct ParserContext{
    pub input: Vec<u8>, //バイト配列
    pub input_len: usize,
    pub pos: usize,
    pub rules: HashMap<&'static str,Exp>,
    pub memos: HashMap<(usize,&'static str),Memos>,
}

impl ParserContext{
    pub fn new(input: Vec<u8>,rules: HashMap<&'static str,Exp>) -> ParserContext{
        ParserContext{
            input_len: input.len(),
            input: input,
            pos: 0,
            rules: rules,
            memos: HashMap::new(),
        }
    }
}

#[derive(Debug,Clone)]
pub enum Tree{
    Node{sym: &'static str, child: Vec<Tree>},
    Leaf{val: char}
}

impl Tree{
    pub fn two_string(&self) -> String{
        match self{
            &Tree::Leaf{ref val} => format!("{}",val),
            &Tree::Node{ref sym, ref child} => format!("[{}{}]",sym,{
                //アキュムレーターにくっつけた結果をパシパシ
                child.iter().fold("".to_string(), |acc,child| format!("{} {}",acc,child.two_string()))
            }),
        }
    }
}
#[derive(Debug,Clone)]
pub enum Memos{
    Fail,
    Memo{pos: usize, child: Vec<Tree>},
}
*/



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

//parseの引数減らして、parsercontextにtreeにいれて

impl Exp{
    pub fn parse(&self, p: &mut ParserContext) -> bool{ 
        
        match self {
            &Exp::Empty => true,
            &Exp::Char{ref c } => {  //cを使ってマッチするかどうか確かめる
                if p.input_len == p.pos{    //inputのposで指しているバイト配列の値とcをキャストしたものを比較
                   false
                }else if p.input[p.pos] == (*c as u8){
                    p.pos += 1;
                    p.tree.push(Tree::Leaf{val: *c});
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
                    true    
                }
            }, 
            &Exp::Symbol{ref sym} =>{
                let old = p.clone();
                let p_rule = p.clone();
                let mut initialtree = p.tree.clone();

                match p.memos.clone().get(&(p.pos,sym)) { 
                    None => {
                        p.tree = Vec::new();
                        match p_rule.rules.get(sym){
                            Some(ref e) => if e.parse(p){
                               initialtree.push(Tree::Node{sym: sym, child: p.tree.clone()});
                                p.memos.insert((p.pos, sym), Memos::Memo{pos: p.pos,child: p.tree.clone()});
                                p.tree = initialtree.clone();
                            //    println!("{:?}",p.tree);
                                true
                            }else{
                                *p = old;
                                false
                            },
                            None => panic!("There is no rule. {}",sym),
                        }
                    },
                    Some(ref memo) => {
                        println!("*");
                        match memo{
                            &Memos::Fail => return false,
                            &Memos::Memo{ref pos,child: ref newchild} => {
                                p.tree.push(Tree::Node{sym: sym,child: newchild.clone()});
                                true
                            }, 
                        }
                    },  
                }
            },
            &Exp::Seq{ref e1, ref e2} =>{
                let old = p.clone();   
                if e1.parse(p){ //parse関数がe1のメソッド呼び
                    if e2.parse(p){
                        true
                    }else{
                        *p = old;
                        false
                    }
                }else{
                    *p = old;
                    false
                }
            },
            &Exp::Choice{ref e1, ref e2} =>{  
                let old = p.clone();
                if e1.parse(p){
                    true
                }else{
                    *p = old.clone();
                    if e2.parse(p){
                        true
                    }else{
                        *p = old;
                        false
                    }
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

    pub fn emp() -> Exp{
        Exp::Empty
    }
    pub fn char(c: char) -> Exp{
        Exp::Char{c: c}
    }
    pub fn any() -> Exp{
        Exp::AnyChar
    }
    pub fn sym(sym: &'static str) -> Exp{
        Exp::Symbol{sym: sym}
    }
    pub fn seq(e1: Exp,e2: Exp) -> Exp{
        Exp::Seq{e1: Box::new(e1),e2: Box::new(e2)}
    }
    pub fn choice(e1: Exp,e2: Exp) -> Exp{
        Exp::Choice{e1: Box::new(e1),e2: Box::new(e2)}
    }
    pub fn rep0(e: Exp) -> Exp{
        Exp::Rep{e: Box::new(e)}
    }
    pub fn rep1(e: Exp) -> Exp{
        Exp::Seq{e1: Box::new(e.clone()),e2: Box::new(Exp::Rep{e: Box::new(e)})}
    }
    pub fn opt(e: Exp) -> Exp{
        Exp::Opt{e: Box::new(e)}
    }
    pub fn not(e: Exp) -> Exp{
        Exp::Not{e: Box::new(e)}
    }

#[derive(Debug,Clone)]
pub struct ParserContext{
    pub input: Vec<u8>, //バイト配列
    pub input_len: usize,
    pub pos: usize,
    pub tree: Vec<Tree>,
    pub rules: HashMap<&'static str,Exp>,
    pub memos: HashMap<(usize,&'static str),Memos>,
}

impl ParserContext{
    pub fn new(input: Vec<u8>,rules: HashMap<&'static str,Exp>) -> ParserContext{
        ParserContext{
            input_len: input.len(),
            input: input,
            pos: 0,
            tree: Vec::new(),
            rules: rules,
            memos: HashMap::new(),
        }
    }
}

#[derive(Debug,Clone)]
pub enum Tree{
    Node{sym: &'static str, child: Vec<Tree>},
    Leaf{val: char}
}

impl Tree{
    pub fn two_string(&self) -> String{
        match self{
            &Tree::Leaf{ref val} => format!("{}",val),
            &Tree::Node{ref sym, ref child} => format!("[{}{}]",sym,{
                //アキュムレーターにくっつけた結果をパシパシ
                child.iter().fold("".to_string(), |acc,child| format!("{} {}",acc,child.two_string()))
            }),
        }
    }
}
#[derive(Debug,Clone)]
pub enum Memos{
    Fail,
    Memo{pos: usize, child: Vec<Tree>},
}



