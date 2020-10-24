#[derive(Debug, PartialEq, Hash)]
pub enum Atom {
    Nil,
    Symbol(String),
}

#[derive(Debug, PartialEq, Hash)]
pub enum Token {
    Atom(Atom),
    Lparen,
    Rparen,
    Illegal,
    EOF,
}