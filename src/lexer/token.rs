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

    //Parse to identifier if keyword or identifier...
    StartOfIdentifierOrKeyword,

    //Identifier and it's value.
    Identifier(String),
    
    //Keywords
    AwaitKeyword,
    FuncKeyword,
    PublicModifierKeyword,
    IntKeyword,
    FloatKeyword,
    StringKeyword,
}
