//！ 具体的Token类型，以及对应用作正则匹配标识符的字符串常量

#[derive(Debug, PartialEq, Eq)]
/// 运算符类Token
pub enum Operator {
    // `+`
    Add,
    // `-`
    Minus,
    // `*`
    Times,
    // `/`
    Divide,
    // `=`
    Assign,
    // `==`
    Equal,
    // `!=`
    Nonequal,
    // '>'
    Larger,
    // '>='
    NotSmaller,
    // '<'
    Smaller,
    // '<='
    NotLarger,
    // `!`
    Not,
    // `(`
    LeftPeren,
    // `)`
    RightPeren,
    // `[`,
    LeftSqket,
    // `]`
    RightSqket,
    // `{`
    LeftCuket,
    // `}`
    RightCuket,
    // `.`
    Call,
    // `&`
    And,
    // `|`
    Or,
}
// 关键词
#[derive(Debug, PartialEq, Eq)]
/// 关键词类Token
pub enum Keyword {
    // `let`
    Let,
    // `if`
    If,
    // `else`
    Else,
    // `false`
    False,
    // `true`
    True,
    // `null`
    Null,
    // `func`
    Func,
    // `for`
    For,
    // `while`
    While,
    // `return`
    Return,
    // `this`
    This,
    // `super`
    Super,
}
// 字面量
#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    // `"string"`，字符串字面量
    String(String),
    // `114514`，数字字面量
    Number(f64),
}

#[derive(Debug, PartialEq)]
/// 总Token类
pub enum Token {
    // 用户定义的标识符
    Id(String),
    LiteralValue(LiteralValue),
    Keyword(Keyword),
    Operator(Operator),
    // ` `或`// 1919810`，无意义块
    Empty,
    // `;`，语句结束标志
    End,
}
