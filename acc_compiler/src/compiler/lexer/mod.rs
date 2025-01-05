//! The lexer module is responsible for the construction of preprocessing tokens
//! Due to the lexer not being entirely context free (we must handle certain
//! differentiations), it requires some record of previously accessed tokens.
//!
//! To stick to idiomatic rust, this will produce an iterator.

use crate::preprocessor::character_mapping::MappedChar;

pub mod types;

/// TODO! finish an iterator/decide on an implementation to intertwine lexing and otherwise.
struct Lexer {}
