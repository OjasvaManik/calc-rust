use crate::tokens::Token;

pub fn lex(string: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = string.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::FSlash),
            '%' => tokens.push(Token::Percent),
            '(' => tokens.push(Token::LBracket),
            ')' => tokens.push(Token::RBracket),

            ' ' | '\t' | '\n' | '\r' => continue,

            '0'..='9' | '.' => {
                let mut word = String::from(c);

                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() || next == '.' {
                        word.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if word.contains('.') {
                    if let Ok(num) = word.parse::<f64>() {
                        tokens.push(Token::NumFloat(num));
                    }
                } else {
                    if let Ok(num) = word.parse::<i64>() {
                        tokens.push(Token::NumInteger(num));
                    }
                }
            }

            ':' => {
                let mut command = String::new();

                while let Some(&next) = chars.peek() {
                    if next.is_ascii_alphabetic() {
                        command.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                match command.as_str() {
                    "exit" => tokens.push(Token::Exit),
                    _ => println!("Unknown command: {}", command),
                }
            }

            _ => println!("Unknown character: {}", c),
        }
    }
    tokens
}
