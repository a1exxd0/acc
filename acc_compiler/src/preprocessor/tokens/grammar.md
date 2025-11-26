# Tokens

Preprocessing tokens consist of:

```
preprocessing-token ::= header-name
        | identifier
        | pp-number
        | character-constant
        | string-literal
        | punctuator
        | "each none white-space character that can not be one of the above"

header-name ::= <[^>\n]+>|"[^"\n]+"
identifier ::= [a-zA-Z_\u0080-\uFFFF][a-zA-Z0-9_\u0080-\uFFFF]*
pp-number ::= \.?[0-9]([0-9]|[a-zA-Z_]|[eEpP][+-]|\.)*
character-constant ::= ^('(
    [^'\\\n]                # ordinary character
  | \\['"\\?abfnrtv]        # simple escapes
  | \\[0-7]{1,3}            # octal
  | \\x[0-9A-Fa-f]+         # hex
  | \\u[0-9A-Fa-f]{4}       # UCN short
  | \\U[0-9A-Fa-f]{8}       # UCN long
)+')$
string-literal ::= ^(
    (u8|u|U|L)?              # optional encoding prefix
    "(
        [^"\\\n]             # ordinary char
      | \\['"\\?abfnrtv]     # simple escapes
      | \\[0-7]{1,3}         # octal
      | \\x[0-9A-Fa-f]+      # hex
      | \\u[0-9A-Fa-f]{4}    # UCN short
      | \\U[0-9A-Fa-f]{8}    # UCN long
    )*"
)$
punctuator ::= one of:
    [ ] ( ) { } . ->
    ++ -- & * + - ~ !
    / % << >> < > <= >= == != ^ | && ||
    ? : ; ...
    = *= /= %= += -= <<= >>= &= ^= |=
    , # ##
    <: :> <% %> %: %:%:

```
