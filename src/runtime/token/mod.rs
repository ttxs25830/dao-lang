//! 词法分析器：将源代码字符串转化为一系列程序可以理解的Token，以便进一步处理

// 引入`Token`类型
mod tokens;
use std::str::Chars;

pub use tokens::Token;
use tokens::*;

use super::SyntaxError;

struct FutureStr<'a> {
    s: Chars<'a>,
    cache: Option<char>,
}
impl<'a> Iterator for FutureStr<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.is_some() {
            let tmp = self.cache;
            self.cache = None;
            tmp
        } else {
            self.s.next()
        }
    }
}
impl<'a> FutureStr<'a> {
    fn from_str(s: &'a str) -> Self {
        Self {
            s: s.chars(),
            cache: None,
        }
    }
    fn until(&mut self, c: char) {
        while self.next() != Some(c) {}
    }
    fn peek(&mut self) -> Option<char> {
        if let Some(c) = self.cache {
            Some(c)
        } else {
            if let Some(c) = self.s.next() {
                self.cache = Some(c);
                Some(c)
            } else {
                None
            }
        }
    }
    fn gen_error(&self, reason: &'static str) -> SyntaxError {
        SyntaxError::new(0, reason)
    }
}
pub fn str_to_token(s: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut s = FutureStr::from_str(s);
    let mut ans = Vec::new();

    loop {
        let c = {
            match s.next() {
                Some(c) => c,
                None => break,
            }
        };
        let token = match c {
            '(' => Token::Operator(Operator::LeftPeren),
            ')' => Token::Operator(Operator::RightPeren),
            '[' => Token::Operator(Operator::LeftSqket),
            ']' => Token::Operator(Operator::RightSqket),
            '{' => Token::Operator(Operator::LeftCuket),
            '}' => Token::Operator(Operator::RightCuket),

            ';' => Token::End,
            '.' => Token::Operator(Operator::Call),

            '&' => Token::Operator(Operator::And),
            '|' => Token::Operator(Operator::Or),

            '+' => Token::Operator(Operator::Add),
            '-' => Token::Operator(Operator::Minus),
            '*' => Token::Operator(Operator::Times),
            '/' => {
                if s.peek() == Some('/') {
                    s.next();
                    s.until('\n');
                    Token::Empty
                } else {
                    Token::Operator(Operator::Divide)
                }
            }

            '!' => {
                if s.peek() == Some('=') {
                    s.next();
                    Token::Operator(Operator::Nonequal)
                } else {
                    Token::Operator(Operator::Not)
                }
            }

            '=' => {
                if s.peek() == Some('=') {
                    s.next();
                    Token::Operator(Operator::Equal)
                } else {
                    Token::Operator(Operator::Assign)
                }
            }

            '>' => {
                if s.peek() == Some('=') {
                    s.next();
                    Token::Operator(Operator::NotSmaller)
                } else {
                    Token::Operator(Operator::Larger)
                }
            }
            '<' => {
                if s.peek() == Some('=') {
                    s.next();
                    Token::Operator(Operator::NotLarger)
                } else {
                    Token::Operator(Operator::Smaller)
                }
            }
            ' ' => Token::Empty,
            '\r' => Token::Empty,
            '\t' => Token::Empty,
            '\n' => Token::Empty,
            '"' => {
                let mut str = String::new();
                loop {
                    if let Some(c) = s.next() {
                        match c {
                            '\n' => {
                                return Err(s.gen_error("broken string"));
                            }
                            '"' => {
                                break Token::LiteralValue(LiteralValue::String(str));
                            }
                            c => str.push(c),
                        }
                    } else {
                        return Err(s.gen_error("broken string"));
                    }
                }
            }
            c if c.is_digit(10) => {
                let mut str = String::new();
                str.push(c);
                let mut doted = false;
                loop {
                    match s.peek() {
                        Some(c) if c.is_digit(10) => {
                            str.push(c);
                            s.next();
                        }
                        Some('.') => {
                            if !doted {
                                doted = true;
                                str.push('.');
                                s.next();
                            } else {
                                return Err(s.gen_error("unknow number"));
                            }
                        }
                        _ => {
                            if str.ends_with('.') {
                                return Err(s.gen_error("unknow number"));
                            }
                            break Token::LiteralValue(LiteralValue::Number(str.parse().unwrap()));
                        }
                    }
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut str = String::new();
                str.push(c);
                loop {
                    match s.peek() {
                        Some(c) if c.is_alphabetic() || c.is_digit(10) || c == '_' => {
                            str.push(c);
                            s.next().unwrap();
                        }
                        _ => {
                            break match str.as_str() {
                                "let" => Token::Keyword(Keyword::Let),
                                "if" => Token::Keyword(Keyword::If),
                                "else" => Token::Keyword(Keyword::Else),
                                "true" => Token::Keyword(Keyword::True),
                                "false" => Token::Keyword(Keyword::False),
                                "for" => Token::Keyword(Keyword::For),
                                "while" => Token::Keyword(Keyword::While),
                                "null" => Token::Keyword(Keyword::Null),
                                "func" => Token::Keyword(Keyword::Func),
                                "return" => Token::Keyword(Keyword::Return),
                                "this" => Token::Keyword(Keyword::This),
                                "super" => Token::Keyword(Keyword::Super),
                                _ => Token::Id(str),
                            }
                        }
                    }
                }
            }

            _ => return Err(s.gen_error("unknow char")),
        };
        if token != Token::Empty {
            ans.push(token);
        }
    }

    Ok(ans)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn scan_test() {
        assert_eq!(
            str_to_token(
                "let _你好a = \"hello啊\"; let letb = a+123.01; while(true) a = b+a; return a*b;"
            )
            .unwrap(),
            vec![
                Token::Keyword(Keyword::Let),
                Token::Id(String::from("_你好a")),
                Token::Operator(Operator::Assign),
                Token::LiteralValue(LiteralValue::String(String::from("hello啊"))),
                Token::End,
                Token::Keyword(Keyword::Let),
                Token::Id(String::from("letb")),
                Token::Operator(Operator::Assign),
                Token::Id(String::from("a")),
                Token::Operator(Operator::Add),
                Token::LiteralValue(LiteralValue::Number(123.01)),
                Token::End,
                Token::Keyword(Keyword::While),
                Token::Operator(Operator::LeftPeren),
                Token::Keyword(Keyword::True),
                Token::Operator(Operator::RightPeren),
                Token::Id(String::from("a")),
                Token::Operator(Operator::Assign),
                Token::Id(String::from("b")),
                Token::Operator(Operator::Add),
                Token::Id(String::from("a")),
                Token::End,
                Token::Keyword(Keyword::Return),
                Token::Id(String::from("a")),
                Token::Operator(Operator::Times),
                Token::Id(String::from("b")),
                Token::End,
            ]
        );
    }
}
