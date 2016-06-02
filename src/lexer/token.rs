#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
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
    AsyncKeyword,
    FuncKeyword,
    PublicModifierKeyword,
    IntKeyword,
    FloatKeyword,
    StringKeyword,
    ForKeyword,
    InKeyword,
    IsKeyword,
    WhereKeyword,
    LoopKeyword,
    WhileKeyword,
    StructKeyword,
    ClassKeyword,
    UseKeyword

}
