use super::{PreprocessingSymbol, PREPROCESSING_SYMBOL_MAP};
use crate::preprocessor::{CharMapper, Symbol};

/// Intended to to ease preprocessing. Grammar is
/// <base-token>
///         ::= <preprocessing-symbol> | <consecutive-numbers> |
///             <consecutive-characters> | <whitespace> |
///             <new-line>
///
/// Where whitespace is any whitespace except newline, and the
/// rest are self explanatory. This is a regular language.
///
/// The order of priority is:
/// 1) Whitespace
/// 2) Newline
/// 3) Consecutive characters
/// 4) Consecutive nums
/// 5) Symbols
pub(crate) enum BaseTokenInner {
    Whitespace,
    Newline,
    Symbol { val: PreprocessingSymbol },
    ConsecutiveNumbers { val: Vec<u8> },
    ConsecutiveCharacters { val: Vec<u8> },
}

/// Wrapper around BaseTokenInner to ease preprocessing.
///
/// The preprocessing language is not a regular language, and
/// therefore to simplify implementation, we will tokenise with a regular
/// language and then parse with much more ease.
pub(crate) struct BaseToken {
    token: BaseTokenInner,
    row: u64,
    col: u64,
}

impl BaseToken {
    pub fn new(token: BaseTokenInner, row: u64, col: u64) -> BaseToken {
        BaseToken { token, row, col }
    }
}

pub(crate) struct PreprocessingTokenizer<'a> {
    char_mapper: CharMapper<'a>,
}

impl<'a> PreprocessingTokenizer<'a> {
    fn consume_whitespace(&mut self) -> Option<(u64, u64)> {
        let tuple;
        if let Some(chr) = self.char_mapper.first() {
            if chr.chr() != Some(b' ') {
                return None;
            }
            tuple = chr.pos();
        } else {
            return None;
        }

        while let Some(chr) = self.char_mapper.first() {
            if chr.chr() != Some(b' ') {
                break;
            }

            self.char_mapper.next();
        }

        Some(tuple)
    }

    fn consume_newline(&mut self) -> Option<(u64, u64)> {
        if let Some(chr) = self.char_mapper.first() {
            if chr.chr() != Some(b'\n') {
                return None;
            } else {
                self.char_mapper.next();
                return Some(chr.pos());
            }
        }

        None
    }

    fn consume_consec_chars(&mut self) -> Option<(Vec<u8>, (u64, u64))> {
        let tuple = self.char_mapper.first()?.pos();
        let mut tok = Vec::new();

        while let Some(mapped_chr) = self.char_mapper.first() {
            if let Some(chr) = mapped_chr.chr() {
                if chr <= b'z' && chr >= b'A' {
                    tok.push(chr);
                } else {
                    break;
                }
            } else {
                break;
            }
            self.char_mapper.next();
        }

        match tok.len() {
            0 => None,
            _ => Some((tok, tuple)),
        }
    }

    fn consume_consec_nums(&mut self) -> Option<(Vec<u8>, (u64, u64))> {
        let tuple = self.char_mapper.first()?.pos();
        let mut tok = Vec::new();

        while let Some(mapped_chr) = self.char_mapper.first() {
            if let Some(chr) = mapped_chr.chr() {
                if chr <= b'9' && chr >= b'0' {
                    tok.push(chr);
                } else {
                    break;
                }
            } else {
                break;
            }
            self.char_mapper.next();
        }

        match tok.len() {
            0 => None,
            _ => Some((tok, tuple)),
        }
    }

    fn consume_single(&mut self) -> Option<(Option<u8>, (u64, u64))> {
        let tuple = self.char_mapper.first()?.pos();
        let chr = self.char_mapper.next()?.chr();

        Some((chr, tuple))
    }
}

impl Iterator for PreprocessingTokenizer<'_> {
    type Item = BaseToken;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((row, col)) = self.consume_whitespace() {
            Some(BaseToken::new(BaseTokenInner::Whitespace, row, col))
        } else if let Some((row, col)) = self.consume_newline() {
            Some(BaseToken::new(BaseTokenInner::Newline, row, col))
        } else if let Some((val, (row, col))) = self.consume_consec_chars() {
            Some(BaseToken::new(
                BaseTokenInner::ConsecutiveCharacters { val },
                row,
                col,
            ))
        } else if let Some((val, (row, col))) = self.consume_consec_nums() {
            Some(BaseToken::new(
                BaseTokenInner::ConsecutiveNumbers { val },
                row,
                col,
            ))
        } else if let Some((val, (row, col))) = self.consume_single() {
            if let Some(chr) = val {
                Some(BaseToken::new(
                    BaseTokenInner::Symbol {
                        val: PREPROCESSING_SYMBOL_MAP
                            .get(&chr)
                            .unwrap_or(&PreprocessingSymbol::Unknown)
                            .clone(),
                    },
                    row,
                    col,
                ))
            } else {
                Some(BaseToken::new(
                    BaseTokenInner::Symbol { val: PreprocessingSymbol::Unknown },
                    row,
                    col,
                ))
            }
        } else {
            None
        }
    }
}
