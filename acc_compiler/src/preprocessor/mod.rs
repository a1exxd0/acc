/// Deals with translation phases 1 and 2:
/// 1) Character mapping to source set
/// 2) Concatenation of backslash-newlines into one line
pub mod character_mapping;
mod parse_help;
use crate::preprocessor::character_mapping::CharMapper;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::VecDeque;

/// Output pp-tokens.
struct Preprocessor<'a> {
    char_mapper: CharMapper<'a>,

    /// Record of previously found tokens. Front of the deque
    /// represents most-recently-parsed.
    previous_tokens: VecDeque<PreprocessingToken>,

    /// On "includes", we fill into the buffer and return until the buffer
    /// is empty. The iterator empties this if nonempty, front to back.
    ///
    /// Push to back.
    buffer: VecDeque<PreprocessingToken>,
}

impl<'a> Preprocessor<'a> {
    fn return_for_iterator(&mut self, value: PreprocessingToken) -> Option<PreprocessingToken> {
        if self.previous_tokens.len() > 3 {
            self.previous_tokens.pop_back();
            self.previous_tokens.push_front(value.clone());
        }

        return Some(value);
    }
}

impl Iterator for Preprocessor<'_> {
    type Item = PreprocessingToken;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.buffer.pop_front() {
            return self.return_for_iterator(item);
        }

        todo!();
    }
}

/// Preprocessing token spec implementation
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum PreprocessingToken {
    HeaderName { local_scoped: bool, path: String },
    Itentifier { val: String },
    PPNumber { val: String },
    CharacterConstant { wide: bool, val: String },
    StringLiteral { wide: bool, val: String },
    Symbol { val: Symbol },
    PreprocessingError { err_msg: String, row: u64, col: u64 },
}

/// Union of punctuators and operators.
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Symbol {
    LeftSquareBracket,
    RightSquareBracket,
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
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
    Unknown,
}

macro_rules! preprocessing_symbol_map {
    ($($variant:ident => $value:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $(
            map.insert($value, PreprocessingSymbol::$variant);
        )*
        map
    }};
}

pub(crate) static PREPROCESSING_SYMBOL_MAP: Lazy<HashMap<u8, PreprocessingSymbol>> = Lazy::new(|| {
    preprocessing_symbol_map!(
        LeftSquareBracket => b'[',
        RightSquareBracket => b']',
        LeftBracket => b'(',
        RightBracket => b')',
        LeftCurlyBracket => b'{',
        RightCurlyBracket => b'}',
        Period => b'.',
        Ampersand => b'&',
        Asterisk => b'*',
        Plus => b'+',
        Minus => b'-',
        Tilde => b'~',
        Bang => b'!',
        ForwardSlash => b'/',
        Percent => b'%',
        LessThan => b'<',
        GreaterThan => b'>',
        Hat => b'^',
        Pipe => b'|',
        QuestionMark => b'?',
        Colon => b':',
        Equals => b'=',
        Comma => b',',
        Hash => b'#',
        Semicolon => b';',
    )
});

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub(crate) enum PreprocessingSymbol {
    LeftSquareBracket,
    RightSquareBracket,
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Period,
    Ampersand,
    Asterisk,
    Plus,
    Minus,
    Tilde,
    Bang,
    ForwardSlash,
    Percent,
    LessThan,
    GreaterThan,
    Hat,
    Pipe,
    QuestionMark,
    Colon,
    Equals,
    Comma,
    Hash,
    Semicolon,
    Unknown,
}
