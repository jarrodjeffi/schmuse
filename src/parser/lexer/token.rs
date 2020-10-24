#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Atom {
    Nil,
    Define,
    Symbol(String),
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Token {
    Atom(Atom),
    Lparen,
    Rparen,
    Illegal(char),
    EOF,
}