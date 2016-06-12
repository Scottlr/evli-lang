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
    Whitespace,
    NewLine,
    CarraigeReturn,

    //Identifier and it's value.
    Identifier(String),
    StringValue(String),
    
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
    WhileKeyword,
    StructKeyword,
    ClassKeyword,
    UseKeyword,
    LetKeyword,
    //Conditionals
    ConditionalEquals
}
