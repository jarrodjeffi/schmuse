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

    fn is_explicit_sign(c: char) -> bool {
        c == '+' || c == '-'
    }

    fn is_special_subsequent(c: char) -> bool {
        match c {
            sign if Lexer::is_explicit_sign(sign) => true,
            '.' | '@' => true,
            _ => false,
        }
    }

    fn is_subsequent(c: char) -> bool {
        Lexer::is_initial(c) || c.is_ascii_digit()
            || Lexer::is_special_subsequent(c)
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = token::Token;

    fn next(&mut self) -> Option<token::Token> {
        match self.read_char() {
            Some(c) => match c as char {
                skip if (skip as char).is_ascii_whitespace() => self.next(),
                '(' => Some(token::Token::Lparen),
                ')' => Some(token::Token::Rparen),
                symbol if Lexer::is_initial(symbol) =>
                    Some(token::Token::Atom(self.read_symbol(symbol)?)),
                _ => Some(token::Token::Illegal)
            },
            None => Some(token::Token::EOF),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        struct Test<'a> {
            input: &'a String,
            expected: &'a [token::Token],
        };

        let tests = &[
            // Empty
            Test {
                input: &String::from(""),
                expected: &[token::Token::EOF],
            },
            // Whitespace
            Test {
                input: &String::from(" \t\r\n"),
                expected: &[token::Token::EOF],
            },
            // Keyword
            Test {
                input: &String::from("nil"),
                expected: &[
                    token::Token::Atom(token::Atom::Nil),
                    token::Token::EOF,
                ],
            },
            // Symbol
            Test {
                input: &String::from("foo"),
                expected: &[
                    token::Token::Atom(token::Atom::Symbol(String::from("foo"))),
                    token::Token::EOF,
                ],
            },
            // Parentheses
            Test {
                input: &String::from("()"),
                expected: &[
                    token::Token::Lparen,
                    token::Token::Rparen,
                    token::Token::EOF,
                ],
            },
        ];

        for i in 0..tests.len() {
            let mut lexer = Lexer::new(tests[i].input);
            for j in 0..tests[i].expected.len() {
                assert_eq!(lexer.next().unwrap(), tests[i].expected[j])
            }
        }
    }
}