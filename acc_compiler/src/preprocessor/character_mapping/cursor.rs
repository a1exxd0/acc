use std::str::Chars;

pub(in crate::preprocessor) const EOF_CHAR: char = '\0';

/// Peekable iterator over a character sequence
///
/// # Example
/// ```rust
/// use acc_compiler::preprocessor::character_mapping::cursor::Cursor;
/// let mut cursor = Cursor::new("hello");
///
/// let peeked_second = cursor.second();
/// assert_eq!(peeked_second, 'e');
///
/// let first_read = cursor.bump();
/// assert_eq!(first_read, Some('h'));
/// ```
#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    /// Constructor for cursor using a string slice.
    ///
    /// Lifetime of string must match cursor.
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor { chars: input.chars() }
    }

    /// Return unconsumed characters as a slice.
    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    /// Peek first character not processed.
    ///
    /// # Example
    /// ```rust
    /// use acc_compiler::preprocessor::character_mapping::cursor::Cursor;
    /// let mut cursor = Cursor::new("Hello World!");
    ///
    /// // Move forward one character, next bump will produce 'e'
    /// cursor.bump();
    /// assert_eq!(cursor.first(), 'e');
    ///
    /// /// If you peek past the end of a character sequence, a null
    /// /// terminator ('\0') is returned.
    /// let mut short_cursor = Cursor::new("X");
    /// assert_eq!(short_cursor.bump(), Some('X'));
    /// assert_eq!(short_cursor.first(), '\0');
    /// ```
    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Peeks second character not processed.
    ///
    /// # Example
    /// ```rust
    /// use acc_compiler::preprocessor::character_mapping::cursor::Cursor;
    /// let mut cursor = Cursor::new("Hello World!");
    ///
    /// // Move forward one character, next bump will produce 'e'
    /// cursor.bump();
    /// assert_eq!(cursor.second(), 'l');
    ///
    /// /// If you peek past the end of a character sequence, a null
    /// /// terminator ('\0') is returned.
    /// let short_cursor = Cursor::new("X");
    /// assert_eq!(short_cursor.second(), '\0');
    /// ```
    pub fn second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    /// Peeks third character not processed.
    ///
    /// # Example
    /// ```rust
    /// use acc_compiler::preprocessor::character_mapping::cursor::Cursor;
    /// let mut cursor = Cursor::new("Hello World!");
    ///
    /// // Move forward one character, next bump will produce 'e'
    /// cursor.bump();
    /// assert_eq!(cursor.third(), 'l');
    ///
    /// /// If you peek past the end of a character sequence, a null
    /// /// terminator ('\0') is returned.
    /// let short_cursor = Cursor::new("X");
    /// assert_eq!(short_cursor.third(), '\0');
    /// ```
    pub fn third(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    // Returns true if the character sequence is empty.
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Advances the cursor and returns the character pointed
    /// to, or `None` if the character sequence is empty.
    ///
    /// # Example
    /// ```rust
    /// use acc_compiler::preprocessor::character_mapping::cursor::Cursor;
    ///
    /// let mut short_cursor = Cursor::new("X");
    /// assert_eq!(short_cursor.bump(), Some('X'));
    /// assert_eq!(short_cursor.bump(), None);
    /// ```
    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Advances the cursor `n` times and returns the character
    /// pointed to, or `None` if the character sequence is empty.
    ///
    /// # Example
    /// ```rust
    /// use acc_compiler::preprocessor::character_mapping::cursor::Cursor;
    ///
    /// let mut short_cursor = Cursor::new("Hello!");
    /// assert_eq!(short_cursor.bump_n(5), Some('o'));
    /// ```
    pub fn bump_n(&mut self, n: usize) -> Option<char> {
        for _ in 1..n {
            self.bump();
        }

        self.bump()
    }
}
