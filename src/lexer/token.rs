#[derive(Debug, Hash)]
pub enum Atom {
    Nil,
    Symbol(String),
}

#[derive(Debug, Hash)]
pub enum Token {
    Illegal,
    Atom(Atom),
    Lparen,
    Rparen,
    Period,
}