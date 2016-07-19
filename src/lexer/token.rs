use super::slidingwindow::SlidingWindow;

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub metadata: TokenMetaData
}

#[derive(PartialEq, Clone, Debug)]
pub struct TokenMetaData {
    pub parsed_on_line: usize,
    pub relative_line_pos: usize
}

impl Token {
    pub fn construct(token_kind: TokenKind, source_code: &mut SlidingWindow) -> Token {
        Token {
            metadata: source_code.get_metadata(),
            kind: token_kind 
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum TokenKind {
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
    IncrementOperator,
    DecrementOperator,
    PointerArrow,

    //Trivia
    Whitespace,
    NewLine,
    CarraigeReturn,

    //Identifier and it's value.
    Identifier(String),
    StringValue(String),
    NumericalValue(String),
    
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
