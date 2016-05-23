#[derive(PartialEq, Copy, Clone)]
pub enum TokenType {
    Operator,
    WhiteSpace,
    Identifier
}

#[derive(PartialEq, Clone, Debug)]
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
    StartOfIdentifier,
    Identifier(String),
    //Keywords
    AwaitKeyword,
    FuncKeyword,
    PublicModifierKeyword,
    IntKeyword,
    FloatKeyword,
    StringKeyword,
}
