mod cursor;
pub mod pretty_print;
mod spec;

use crate::preprocessor::character_mapping::{cursor::Cursor, cursor::EOF_CHAR, spec::TRIGRAPH_MAP};

use spec::{ALPHABET, WHITESPACE_SET};

/// Implementation for translation phase 1, mapping characters to source
/// language alphabet. Characters not part of the source alphabet are ignored.
///
/// # Examples
/// ```rust
/// use acc_compiler::preprocessor::character_mapping::{CharMapper, MappedChar};
/// let my_str = "xyz\r\n3";
/// let mut mapper = CharMapper::new(my_str);
/// let vec: Vec<MappedChar> = mapper.into_iter().collect();
///
/// assert_eq!(vec[0].chr().unwrap(), b'x');
/// assert_eq!(vec[3].chr().unwrap(), b'\n');
/// assert_eq!(vec[4].chr().unwrap(), b'3');
/// ```
/// ```rust
/// use acc_compiler::preprocessor::character_mapping::{CharMapper, MappedChar};
///
/// let mut mapper = CharMapper::new("hey\n??(x");
/// assert_eq!(mapper.next().unwrap().chr(), Some(b'h'));
/// assert_eq!(mapper.next().unwrap().chr(), Some(b'e'));
/// assert_eq!(mapper.next().unwrap().chr(), Some(b'y'));
/// mapper.next();
///
/// let next = mapper.next().unwrap();
/// assert_eq!(next.pos(), (1, 0));
/// assert_eq!(next.chr(), Some((b'[')));
///
/// let final_char = mapper.next().unwrap();
/// assert_eq!(final_char.pos(), (1, 3));
/// assert_eq!(final_char.chr(), Some(b'x'));
///
/// assert_eq!(mapper.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct CharMapper<'a> {
    source: Cursor<'a>,
    row: u64,
    col: u64,
}

/// Char wrapper with positional information too needed if trigraphs are used.
///
/// # Example
/// ```rust
/// use acc_compiler::preprocessor::character_mapping::MappedChar;
/// let chr = MappedChar::new(Some(b'a'), 10, 3);
///
/// assert_eq!(chr.chr(), Some(b'a'));
/// assert_eq!(chr.pos(), (10, 3));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct MappedChar {
    chr: Option<u8>,
    row: u64,
    col: u64,
}

impl<'a> CharMapper<'a> {
    /// Constructor for character mapping, taking string slice as input.
    pub fn new(input: &'a str) -> CharMapper<'a> {
        CharMapper {
            source: Cursor::new(input),
            row: 0,
            col: 0,
        }
    }

    pub fn first(&self) -> Option<MappedChar> {
        let mut it = self.clone();
        it.next()
    }

    pub fn second(&self) -> Option<MappedChar> {
        let mut it = self.clone();
        it.next()?;
        it.next()
    }

    pub fn third(&self) -> Option<MappedChar> {
        let mut it = self.clone();
        it.next()?;
        it.next()?;
        it.next()
    }
}

impl Iterator for CharMapper<'_> {
    type Item = MappedChar;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.source.first(), self.source.second(), self.source.third()) {
            (EOF_CHAR, _, _) => None,
            (a, b, c) if TRIGRAPH_MAP.contains_key(&(a, b, c)) => {
                let chr = TRIGRAPH_MAP[&(a, b, c)];
                let result = Some(MappedChar::new(Some(chr), self.row, self.col));

                self.col += 3;
                self.source.bump_n(3);
                return result;
            }
            ('\\', '\r', '\n') => {
                let result = Some(MappedChar::new(None, self.row, self.col));

                self.row += 1;
                self.col = 0;
                self.source.bump_n(3);
                return result;
            }
            ('\\', '\n', _) => {
                let result = Some(MappedChar::new(None, self.row, self.col));

                self.row += 1;
                self.col = 0;
                self.source.bump_n(2);
                return result;
            }
            ('\r', '\n', _) => {
                let result = Some(MappedChar::new(Some(b'\n'), self.row, self.col));

                self.row += 1;
                self.col = 0;
                self.source.bump_n(2);
                return result;
            }
            ('\n', _, _) => {
                let result = Some(MappedChar::new(Some(b'\n'), self.row, self.col));

                self.row += 1;
                self.col = 0;
                self.source.bump();
                return result;
            }
            (a, _, _) if WHITESPACE_SET.contains(&a) => {
                let result = Some(MappedChar::new(Some(b' '), self.row, self.col));

                self.col += 1;
                self.source.bump();
                return result;
            }
            (a, _, _) if ALPHABET.contains(&a) => {
                let result = Some(MappedChar::new(Some(a as u8), self.row, self.col));

                self.col += 1;
                self.source.bump();
                return result;
            }
            (_, _, _) => {
                let result = Some(MappedChar::new(None, self.row, self.col));

                self.col += 1;
                self.source.bump();
                return result;
            }
        }
    }
}

impl MappedChar {
    pub fn new(chr: Option<u8>, row: u64, col: u64) -> MappedChar {
        MappedChar { chr, row, col }
    }

    /// Return Option<u8> of character it is holding. This will be None if the character
    /// is not part of the source alphabet or if it is a backslash-newline combination.
    pub fn chr(&self) -> Option<u8> {
        self.chr
    }

    /// Return position character was found at/started at
    pub fn pos(&self) -> (u64, u64) {
        (self.row, self.col)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn map_str_and_print() {
        let mapper = super::CharMapper::new("HelloWorld! \r\n Alex??(1");
        let mapped_chars: Vec<super::MappedChar> = mapper.into_iter().collect();

        let printed = super::pretty_print::mapped_chars_to_str(&mapped_chars);
        assert_eq!(printed, "HelloWorld! \n Alex[1");
        assert_eq!(
            super::pretty_print::verbose_mapped_chars_to_str(&mapped_chars)
                .lines()
                .next(),
            Some("H (0, 0)")
        );
    }
}
