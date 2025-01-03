//! Deals with AST tokens. During lexing, there is no guarantee we have
//! the correct version of certain items.
//!
//! Therefore, the implementation I have chosen is to split the input into
//! a couple of key categories, being "identifiers", "punctuators/operators",
//! but this is the concern of the lexer.
//!

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum KeyWord {
    Auto,
    Double,
    Int,
    Struct,
    Break,
    Else,
    Long,
    Switch,
    Case,
    Enum,
    Register,
    Typedef,
    Char,
    Extern,
    Return,
    Union,
    Const,
    Float,
    Short,
    Unsigned,
    Continue,
    For,
    Signed,
    Void,
    Default,
    Goto,
    Sizeof,
    Volatile,
    Do,
    If,
    Static,
    While,
}
