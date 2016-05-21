#[derive(PartialEq, Copy, Clone)]
pub enum TokenType {
    Operator,
    WhiteSpace,
    Identifier
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Token {
    SemiColon,
    OpenParen,
    CloseParen,
    Equals,
    OpenBrace,
    CloseBrace,
    RightAngle,
    LeftAngle,
    Hyphen,
    Comma,
    Colon,
    Asterix,
    Plus,
    PlusEquals,
    MinusEquals,
    MultiplicationEquals,
    DivideEquals,
    BackSlash,
    ForwardSlash,
    Character,
    QoutationMark,

    //Keywords
    AwaitKeyword,
    FuncKeyword,
    PublicKeyword,
    IntKeyword,
    FloatKeyword,
    StringKeyword

}
