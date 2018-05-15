enum Exp{
    Empty,
    Char{c:char},
    AnyChar,
    Symbol{sym:&`static str},
    Seq{e1:Box<Exp>, e2:Box<Exp>},
    Choice{e1:Box<Exp>, e2:Box<Exp>},
    Rep{e:Box<Exp>},
    Opt{e:Box<Exp>},
    Not{e:Box<Exp>},
}



fn main() {
    println!("Hello, world!!!!");
}
