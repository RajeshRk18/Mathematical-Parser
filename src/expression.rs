use std::iter::Peekable;

#[derive(Debug)]
pub struct Lexer<T: Iterator<Item=char>> {
    pub chars: Peekable<T>
}

impl <T: Iterator<Item=char>> Lexer<T>  {
    pub fn from_str(source: T) -> Self {
        Self {chars: source.peekable()}
    }
}

impl <T: Iterator<Item=char>> Iterator for Lexer<T> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {

        while let Some(_) = self.chars.next_if(|x| x.is_whitespace()) {}

        if let Some(token) = self.chars.next() {

            match token {
                '(' => Some(Token { text: token.to_string(), kind: Tokenkind::OpenParen }),
                ')' => Some(Token { text: token.to_string(), kind: Tokenkind::CloseParen }),
                '=' | '+' | '-' | '*' | '/' => Some(Token { text: token.to_string(), kind: Tokenkind::Sign }),
                ',' => Some(Token { text: token.to_string(), kind: Tokenkind::Comma }),
                '^' => Some(Token { text: token.to_string(), kind: Tokenkind::Cap}),
                 _  => {
                    if token.is_alphabetic() {
                        let mut text = String::new();
                        text.push(token);

                        while let Some(ident) = self.chars.next_if(|ident| ident.is_alphabetic()) {
                            text.push(ident)
                        }
                        return Some(Token {text, kind: Tokenkind::Identifier});

                    } else {
                        let mut numbers = String::new();
                        numbers.push(token);
                        while let Some(num) = self.chars.next_if(|x| x.is_numeric()) {
                            numbers.push(num);
                        }
                        return Some(Token { text: numbers, kind: Tokenkind::Int });
                    }
                 }
            }
        } else {
            None
        }
    }
}


#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub kind: Tokenkind
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tokenkind {
    OpenParen, 
    CloseParen,
    Identifier,  
    Sign,        
    Comma,
    Cap, 
    Int,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Exp {
    Nil,
    Var(String),
    Op(String),
    Int(String),
    Exponent(String),
    Cap(String),
    Fn(String, Vec<Exp>),
}

impl Exp {
    fn generate_exp(lexer: &mut Peekable<impl Iterator<Item=Token>>) -> Vec<Self> {
        let mut symbols = Vec::<Exp>::new();
        while let Some(token) = lexer.next() {
            match token.kind {
                Tokenkind::Identifier => {
                    symbols.push(Exp::Var(token.text));
                    },
                
                Tokenkind::Comma => {
                    if let Some(char) = lexer.next() {
                        match char.kind {
                            Tokenkind::Identifier => {
                                symbols.push(Exp::Var(char.text));
                            },
                            Tokenkind::Int => {
                                symbols.push(Exp::Int(char.text));
                            },
                            _ => {
                                continue;
                            },
                        }
                    } else { continue;}
                },

                Tokenkind::Sign => {
                    if lexer.peek().unwrap().kind == Tokenkind::Identifier || lexer.peek().unwrap().kind == Tokenkind::Int {
                        symbols.push(Exp::Op(token.text));
                    } else {
                        panic!("Sign should only be followed by int or identifier!");
                    }
                },

                Tokenkind::Cap => {
                    match symbols.last() {
                        Some(val) => {
                            match val {
                                Exp::Var(_) | Exp::Int(_)=>  {
                                    if let Some(char) = lexer.next_if(|x| x.kind == Tokenkind::Int) {
                                        symbols.push(Exp::Cap(token.text));
                                        symbols.push(Exp::Exponent(char.text));
                                    } else if let Some(sign) = lexer.next_if(|c| c.kind == Tokenkind::Sign) {
                                        if (sign.text == "+") | (sign.text == "-") {
                                            symbols.push(Exp::Cap(token.text));
                                            symbols.push(Exp::Op(sign.text));
                                        }
                                    }                               
                                },

                                _ => {
                                    panic!("Cap should only be followed by int or identifier!");
                                }
                            }
                        },
                        None => {panic!("Cap should only be followed by int or identifier!");},
                    }
                },

                Tokenkind::Int => {
                    symbols.push(Exp::Int(token.text));
                },

                _ => {continue;}
            }
        }
        symbols
    }

    pub fn parse(lexer: &mut impl Iterator<Item=Token>) -> Vec<Self> {
        Self::generate_exp(&mut lexer.peekable())   
    }
}
