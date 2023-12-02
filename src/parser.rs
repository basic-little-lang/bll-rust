use colored::Colorize;

use self::tokens::Token;


pub mod tokens;

#[derive(Debug)]
pub enum ParsedTokens {
    Add,
    Subtract,
    Divide,
    Multiplie,
    Modulo,
    Power,
    Equal,
    Number(f64),
    Print,
    Var(String),
}

fn str_to_keyword(str: &str) -> Option<ParsedTokens> {
    match str {
        "print" => Some(ParsedTokens::Print),
        _ => None,
    }
}

impl ParsedTokens {
    pub fn convert(dirent_tokens: &Vec<Token>) -> Result<Vec<ParsedTokens>, String> {
        let mut parsered_tokens = Vec::new();
        let mut buffer: Vec<&Token> = Vec::new();

        for (index, token) in dirent_tokens.iter().enumerate() {
            match token {
                Token::Char(chr) => {
                    buffer.push(token);
                    
                    let mut string = String::new();
                    for buff in &buffer {
                        string.push(match buff {
                            Token::Char(ch) => ch.to_owned(),
                            Token::Point => '.',
                            Token::Negitive => '-',
                            _ => 0 as char,
                        })
                    }

                    let next_is_whitespace = match dirent_tokens.get(index + 1) {
                        Some(n) => {
                            if let Token::Whitespace = n {
                                true
                            } else {
                                false
                            }
                        },
                        None => true,
                    };

                    if chr.is_numeric() && next_is_whitespace {
                        let is_negitive = string.contains("-");
                        let string = string.replace("-", "");
                        let mut number: f64 = match string.parse() {
                            Ok(n) => n,
                            Err(err) => {
                                return Err(format!("{}: cannot parse number {}: {}", "parsing error".red(), string, err.to_string()));
                            }
                        };
                        if is_negitive {
                            number *= -1 as f64;
                        }

                        parsered_tokens.push(ParsedTokens::Number(number));

                        buffer = Vec::new();
                    } else if next_is_whitespace {
                        match str_to_keyword(&string) {
                            Some(token) => parsered_tokens.push(token),
                            None => parsered_tokens.push(ParsedTokens::Var(string)),
                        }

                        buffer = Vec::new();
                    }

                },
                Token::Add => parsered_tokens.push(ParsedTokens::Add),
                Token::Subtract => parsered_tokens.push(ParsedTokens::Subtract),
                Token::Multiplie => parsered_tokens.push(ParsedTokens::Multiplie),
                Token::Divide => parsered_tokens.push(ParsedTokens::Divide),
                Token::Modulo => parsered_tokens.push(ParsedTokens::Modulo),
                Token::Power => parsered_tokens.push(ParsedTokens::Power),
                Token::Equal => parsered_tokens.push(ParsedTokens::Equal),
                Token::Negitive => buffer.push(token),
                Token::Point => buffer.push(token),
                Token::Whitespace => {},
            }
        }

        Ok(parsered_tokens)
    }
}
