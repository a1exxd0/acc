//! This aims to wrap around the char-mapper; i.e. every time next is called here,
//! it calls next on the underlying to process that too.

use crate::preprocessor::character_mapping::CharMapper;
use crate::preprocessor::PreprocessingToken;
use std::collections::VecDeque;

/// Output pp-tokens.
struct Lexer<'a> {
    char_mapper: CharMapper<'a>,

    /// Record of previously found tokens. Front of the deque
    /// represents most-recently-parsed.
    previous_tokens: VecDeque<<CharMapper<'a> as IntoIterator>::Item>,
}

impl Iterator for Lexer<'_> {
    type Item = Result<PreprocessingToken, String>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}
