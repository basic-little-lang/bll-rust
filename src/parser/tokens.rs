use colored::Colorize;

#[derive(Debug)]
pub enum Token {
    Add,
    Subtract,
    Divide,
    Multiplie,
    Modulo,
    Power,
    Negitive,
    Equal,
    Char(char),
    Point,
    Whitespace,
}

pub const COMMENT_CHAR: char = '#';

pub fn tokenize_str(str: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    let mut skip_comment = false;

    let chars: Vec<char> = str.chars().collect();

    for (index, ch) in str.chars().enumerate() {
        if ch == '\n' {
            if skip_comment {
                skip_comment = false;
                continue;
            }
            tokens.push(Token::Whitespace);
            continue;
        }
        if skip_comment {
            continue;
        }
        if ch == COMMENT_CHAR {
            skip_comment = true;
            continue;
        }
        if ch.is_whitespace() {
            tokens.push(Token::Whitespace);
            continue;
        }

        match ch {
            '+' => tokens.push(Token::Add),
            '-' => {
                let next = chars.get(index + 1);

                if let Some(c) = next {
                    if c.is_whitespace() {
                        tokens.push(Token::Subtract);
                    } else {
                        tokens.push(Token::Negitive);
                    }
                } else {
                    return Err(format!("{}: Cannot get next char, this maybe the end of the file", "syntax error".red()));
                }
            },
            '*' => tokens.push(Token::Multiplie),
            '/' => tokens.push(Token::Divide),
            '%' => tokens.push(Token::Modulo),
            '^' => tokens.push(Token::Power),
            '=' => tokens.push(Token::Equal),
            '.' => tokens.push(Token::Point),
            _ => tokens.push(Token::Char(ch)),
        }
    }

    Ok(tokens)
}
