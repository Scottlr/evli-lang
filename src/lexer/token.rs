use super::slidingwindow::SlidingWindow;

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub line_parsed: usize,
    pub line_offset_parsed: usize,
    pub kind: TokenKind
}

impl Token {
    pub fn construct(token_kind: TokenKind, source_code: &mut SlidingWindow) -> Token {
        Token {
            line_parsed: source_code.current_line,
            line_offset_parsed: source_code.relative_line_pos,
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

    //Travia
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
