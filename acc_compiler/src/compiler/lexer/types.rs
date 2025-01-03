//! Here, we implement the lexer types. Lexer specifications require production of "preprocessing"
//! tokens, but we will need to handle general disambiguations between types.
//!
//! In the case of header names, we expect a header name after the tokens {#} {include}

/// Preprocessing token spec implementation
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum PreprocessingToken {
    HeaderName { path: String },
    Itentifier { val: String },
    PPNumber { val: String },
    CharacterConstant { wide: bool, val: String },
    StringLiteral { wide: bool, val: String },
    Symbol { val: Symbol },
}

/// Union of punctuators and operators.
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Symbol {
    LeftSquareBracket,
    RightSquareBracket,
    LeftBracket,
    RightBracket,
    Period,
    RightArrow,
    Increment,
    Decrement,
    Ampersand,
    Asterisk,
    Plus,
    Minus,
    Tilde,
    Bang,
    ForwardSlash,
    Percent,
    LeftShift,
    RightShift,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    EqualTo,
    NotEqualTo,
    Hat,
    Pipe,
    DoubleAnd,
    DoublePipe,
    QuestionMark,
    Colon,
    Assign,
    AsteriskAssign,
    ForwardSlashAssign,
    PercentAssign,
    PlusAssign,
    MinusAssign,
    LeftShiftAssign,
    RightShiftAssign,
    AmpersandAssign,
    HatAssign,
    PipeAssign,
    Comma,
    Hash,
    DoubleHash,
    Semicolon,
    Ellipses,
}
