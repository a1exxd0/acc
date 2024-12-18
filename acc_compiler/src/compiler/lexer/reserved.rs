use crate::compiler::lexer::types::KeyWord;
use once_cell::sync::Lazy;
use std::collections::HashMap;

macro_rules! initialize_keywords {
    ($($name:ident => $str:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $(
            map.insert($str, KeyWord::$name);
        )*
        map
    }};
}

static RESERVED_KEYWORDS: Lazy<HashMap<&'static str, KeyWord>> = Lazy::new(|| {
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
