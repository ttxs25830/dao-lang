//！ 具体的Token类型，以及对应用作正则匹配标识符的字符串常量

pub const COMMON_OPS_NAME: &'static str = "OPS";
pub const ASSIGN_OPS_NAME: &'static str = "ASSIGN_OPS";
#[derive(Debug, PartialEq)]
/// 运算符类Token
pub enum Ops {
    // `+`， 加法运算符
    Add,
    // `-`， 减法运算符
    Minus,
    // `=`， 赋值运算符
    Assign,
    // `==`，相等运算符
    Equal,
    // `!=`， 不等运算符
    Nonequal,
}
// 关键词
pub const KEYWORDS_NAME: &'static str = "KEYWORDS";
#[derive(Debug, PartialEq)]
/// 关键词类Token
pub enum Keywords {
    // `let`，声明变量
    Declare,
}
// 字面量
pub const LITERAL_VALUES_STRING_NAME: &'static str = "LVS_STRING";
pub const LITERAL_VALUES_NUMBER_NAME: &'static str = "LVS_NUMBER";
#[derive(Debug, PartialEq)]
/// 字面量运算符
pub enum LiteralValues {
    // `"string"`，字符串字面量
    String(String),
    // `114514`，数字字面量
    Number(f64),
}

pub const ID_NAME: &'static str = "ID";
pub const EMPTY_NAME: &'static str = "EMPTY";
pub const END_NAME: &'static str = "END";
#[derive(Debug, PartialEq)]
/// 总Token类
pub enum Token {
    // 用户定义的标识符
    Id(String),
    LiteralValues(LiteralValues),
    Keywords(Keywords),
    Ops(Ops),
    // ` `或`// 1919810`，无意义块
    Empty,
    // `;`，语句结束标志
    End,
}
