//! 词法分析器：将源代码字符串转化为一系列程序可以理解的Token，以便进一步处理

use lazy_static::lazy_static;
use regex::Regex;

// 引入`Token`类型
mod tokens;
pub use tokens::Token;
use tokens::*;

use super::errors::SyntaxError;

impl Token {
    /// 将一串源代码解析为`Vec<Token>`
    /// # 错误
    /// 如源代码不合法，则返回`SyntaxError`
    pub fn from(code: &String) -> Result<Vec<Token>, SyntaxError> {
        // 生成&编译正则表达式
        lazy_static! {
            // 匹配各类Token的正则表达式
            static ref RE_LIST: Vec<(&'static str, &'static str)> = vec![
                (LITERAL_VALUES_STRING_NAME, "\"([^\"])*\""),
                (LITERAL_VALUES_NUMBER_NAME, r"\d+(\.\d+)?"),
                (KEYWORDS_NAME, r"let"),
                (ID_NAME, r"[a-z_A-Z][a-z_A-Z0-9]*"),
                (END_NAME, r";"),
                (EMPTY_NAME, r"( +)|(//.*)"),
                (
                    COMMON_OPS_NAME,
                    r"[+\-*/%]|!|&&|\|\||<=|==|>=|[<>\(\)\[\]]|,"
                ),
                (ASSIGN_OPS_NAME, r"=")
            ];
            static ref RE: Regex = {
                let re_str =
                    String::from_iter(RE_LIST.iter().map(|x| format!("(?P<{}>{})|", x.0, x.1)));
                Regex::new(&re_str[0..re_str.len() - 1]).unwrap()
            };
        }
        let caps = RE.captures_iter(code.as_str());
        let mut tokens = Vec::new();
        let mut last_pos = 0;
        for cap in caps {
            for re in RE_LIST.iter() {
                let name = re.0;
                if let Some(mat) = cap.name(name) {
                    // 确保每一个Token都是首尾相连的，没有匹配不到的字符
                    if mat.start() != last_pos {
                        return Err(SyntaxError::UnsolvableChar { offset: last_pos });
                    }
                    last_pos = mat.end();
                    tokens.push(solve_match_to_token(name, mat.as_str()))
                }
            }
        }
        if last_pos == code.len() {
            Ok(tokens)
        } else {
            Err(SyntaxError::UnsolvableChar { offset: last_pos })
        }
    }
}
// 将单个字符串解析为`Token`
fn solve_match_to_token(name: &str, str: &str) -> Token {
    match name {
        LITERAL_VALUES_STRING_NAME => {
            let mut s = String::from(str);
            s.pop();
            s.remove(0);
            Token::LiteralValues(LiteralValues::String(s))
        }
        LITERAL_VALUES_NUMBER_NAME => {
            Token::LiteralValues(LiteralValues::Number(str.parse::<f64>().unwrap()))
        }
        KEYWORDS_NAME => Token::Keywords(solve_keywords_match(str)),
        ID_NAME => Token::Id(String::from(str)),
        END_NAME => Token::End,
        EMPTY_NAME => Token::Empty,
        COMMON_OPS_NAME => Token::Ops(solve_common_ops_match(str)),
        ASSIGN_OPS_NAME => Token::Ops(Ops::Assign),
        _ => panic!("Cant find token type {}", name),
    }
}
// 解析常见操作符的Token
fn solve_common_ops_match(str: &str) -> Ops {
    match str {
        "+" => Ops::Add,
        "-" => Ops::Minus,
        "==" => Ops::Equal,
        "!=" => Ops::Nonequal,
        _ => panic!("Cant find common op {}", str),
    }
}
// 解析关键字的Token
fn solve_keywords_match(str: &str) -> Keywords {
    match str {
        "let" => Keywords::Declare,
        _ => panic!("Cant find keyword {}", str),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tokenize_normally() {
        let code = String::from("let a =\"你是一个一个1919810\";let b = 114.514; a==b;");
        let tokens = Token::from(&code).unwrap();
        let ans = vec![
            Token::Keywords(Keywords::Declare),
            Token::Empty,
            Token::Id(String::from("a")),
            Token::Empty,
            Token::Ops(Ops::Assign),
            Token::LiteralValues(LiteralValues::String(String::from("你是一个一个1919810"))),
            Token::End,
            Token::Keywords(Keywords::Declare),
            Token::Empty,
            Token::Id(String::from("b")),
            Token::Empty,
            Token::Ops(Ops::Assign),
            Token::Empty,
            Token::LiteralValues(LiteralValues::Number(114.514)),
            Token::End,
            Token::Empty,
            Token::Id(String::from("a")),
            Token::Ops(Ops::Equal),
            Token::Id(String::from("b")),
            Token::End,
        ];
        assert_eq!(tokens.len(), tokens.len());
        for i in 0..tokens.len() {
            assert_eq!(tokens[i], ans[i]);
        }
    }
}
