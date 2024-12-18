use crate::compiler::lexer::types::KeyWord;
use once_cell::sync::Lazy;
use std::collections::HashSet;

macro_rules! initialize_keywords {
    ($($name:ident => $str:expr),* $(,)?) => {{
        let mut set = HashSet::new();
        $(
            set.insert(($str, KeyWord::$name));
        )*
        set
    }};
}

static RESERVED_KEYWORDS: Lazy<HashSet<(&'static str, KeyWord)>> = Lazy::new(|| {
    initialize_keywords!(
        Auto        => "auto",
        Double      => "double",
        Int         => "int",
        Struct      => "struct",
        Break       => "break",
        Else        => "else",
        Long        => "long",
        Switch      => "switch",
        Case        => "case",
        Enum        => "enum",
        Register    => "register",
        Typedef     => "typedef",
        Char        => "char",
        Extern      => "extern",
        Return      => "return",
        Union       => "union",
        Const       => "const",
        Float       => "float",
        Short       => "short",
        Unsigned    => "unsigned",
        Continue    => "continue",
        For         => "for",
        Signed      => "signed",
        Void        => "void",
        Default     => "default",
        Goto        => "goto",
        Sizeof      => "sizeof",
        Volatile    => "volatile",
        Do          => "do",
        If          => "if",
        Static      => "static",
        While       => "while",
    )
});
