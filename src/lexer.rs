mod token;

pub struct Lexer<'a> {
    input: &'a [u8],
    start: usize,
    end: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a std::string::String) -> Lexer<'a> {
        let bytes = input.as_bytes();
        Lexer {
            input: bytes,
            start: 0,
            end: 0,
        }
    }

    fn read_char(&mut self) -> Option<u8> {
        let byte: u8;
        if self.end >= self.input.len() {
            return None
        }
        byte = self.input[self.end];
        self.start = self.end;
        self.end += 1;
        return Some(byte)
    }

    fn is_special_initial(c: char) -> bool {
        match c {
        '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' | '>' | '?' | '^'
            | '_' | '~' => true,
        _ => false,
        }
    }

    fn is_initial(c: char) -> bool {
        c.is_ascii() && (c.is_alphabetic() || Lexer::is_special_initial(c))
    }

    fn is_special_subsequent(c: char) -> bool {
        match c {
            '+' | '-' | '.' | '@' => true,
            _ => false,
        }
    }

    fn is_subsequent(c: char) -> bool {
        Lexer::is_initial(c) || c.is_ascii_digit() || Lexer::is_special_subsequent(c)
    }

    fn read_symbol(&mut self, c: char) -> Option<token::Atom> {
        let mut string = format!("{}", c);
        while let Some(c) = self.read_char() {
            let c = c as char;
            if !Lexer::is_subsequent(c) {
                self.end -= 1;
                break;
            }
            string.push(c);
        }
        if string.to_lowercase() == "nil" {
            return Some(token::Atom::Nil)
        }
        Some(token::Atom::Symbol(string))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = token::Token;

    fn next(&mut self) -> Option<token::Token> {
        match self.read_char()? as char {
            '\0' => None,
            '(' => Some(token::Token::Lparen),
            ')' => Some(token::Token::Rparen),
            '.' => Some(token::Token::Period),
            symbol if Lexer::is_initial(symbol) =>
                Some(token::Token::Atom(self.read_symbol(symbol)?)),
            skip if (skip as char).is_whitespace() => self.next(),
            _ => Some(token::Token::Illegal)
        }
    }
}