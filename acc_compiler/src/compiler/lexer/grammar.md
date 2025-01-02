# Lexical elements

A token is the minimal lexical element of the language.

```
<token> ::= <keyword> | <identifier> | 
            <constant> | <string-literal> | 
            <operator> | <punctuator>

<preprocessing-token> 
        ::= <header-name> | <identifier> |
            <pp-number> | <character-constant> |
            <string-literal> | <operator> |
            <punctuator>
```

## Keywords
```
<keyword> 
        ::= auto | double | int | struct |
            break | else | long | switch |
            case | enum | register | typedef |
            char | extern | return | union |
            const | float | short | unsigned |
            continue | for | signed | void |
            default | goto | sizeof | volatile |
            do | if | static | while
```

## Identifiers
Keywords should be identified before identifiers (no identifier should consist of the same sequence of letters as a keyword).
```
<identifier>
        ::= <nondigit> | <identifier> <nondigit> |
            <identifier> <digit>

<nondigit>
        ::= [a-zA-Z_]

<digit> ::= [0-9]
```

## Constants
```
<constant>
        ::= <floating-constant> | <integer-constant> |
            <enumeration-constant> | <character-constant>
```
### Floating constants

```
<floating-constant>
        ::= <fractional-constant> <<opt> exponent-part> <<opt> floating-suffix> |
            <digit-sequence> <exponent-part> <<opt> floating-suffix> |

<fractional-constant>
        ::= <<opt> digit-sequence> . <digit-sequence> |
            <digit-sequence> .

<exponent-part>
        ::= e <<opt> sign> <digit-sequence> |
            E <<opt> sign> <digit-sequence>

<sign>  ::= + | -

<digit-sequence>
        ::= <digit> | <digit-sequence> <digit>

<floating-suffix> ::= f | l | F | L
```
### Integer constants
```
<integer-constant>
        ::= <decimal-constant> <<opt> integer-suffix> |
            <octal-constant> <<opt> integer-suffix> |
            <hexadecimal-constant> <<opt> integer-suffix>

<decimal-constant>
        ::= <nonzero-digit> | <decimal-constant> <digit>

<octal-constant>
        ::= 0 | <octal-constant> <octal-digit>

<hexadecimal-constant>
        ::= 0x <hexadecimal-digit> | 0X <hexadecimal-digit> |
            <hexadecimal-constant> <hexadecimal-digit>

<nonzero-digit>
        ::= [1-9]

<octal-digit>
        ::= [0-7]

<hexadecimal-digit>
        ::= [0-9a-fA-F]

<integer-suffix>
        ::= <unsigned-suffix> <<opt> long-suffix> |
            <long-suffix> <<opt> unsigned-suffix>

<unsigned-suffix> 
        ::= u | U

<long-suffix>
        ::= l | L
```
### Enumeration constants
```
<enumeration-constant>
        ::= <identifier>
```
### Character constants
```
<character-constant>
        ::= ' <c-char-sequence> ' | L ' <c-char-sequence> '

<c-char-sequence>
        ::= <c-char> | <c-char-sequence> <c-char>

<c-char>
        ::= source character set except (', \, newline)

<escape-sequence>
        ::= <simple-escape-sequence> | <octal-escape-sequence> |
            <hexadecimal-escape-sequence>

<simple-escape-sequence>
        ::= One of (
            \' \" \? \\ \a \b \f \n \r \t \v
        )

<octal-escape-sequence>
        ::= \ <octal-digit> | \ <octal-digit> <octal-digit> |
            \ <octal-digit> <octal-digit> <octal-digit>

<hexadecimal-escape-sequence>
        ::= \x <hexadecimal-digit> |
            <hexadecimal-escape-sequence> <hexadecimal-digit>
```

## String literals
```
<string-literal>
        ::= " <<opt> s-char-sequence> " | L <<opt> s-char-sequence> "

<s-char-sequence>
        ::= <s-char> | <s-char-sequence> <s-char>

<s-char>
        ::= source character set except (', \, newline) |
            <escape-sequence>
```

## Operators
```
<operator>
    ::= one of (
        [ ] ( ) . ->
        ++ -- & * + - ~ ! sizeof
        / % << >> < > <= >= == != ^ | && || ? :
        = *= /= %= += -= <<= >>= &= ^= |= , # ##
    )
```

## Punctuators
```
<punctuator>
        ::= one of (
            [ ] ( ) { } * , : = ; ... #
        )
```

## Header names
```
<header-name> 
        ::= < <h-char-sequence> > | " <q-char-sequence> "

<h-char-sequence>
        ::= <h-char> | <h-char-sequence> <h-char>

<h-char> 
        ::= source character set except newline character and >

<q-char-sequence>
        ::= <q-char> | <q-char-sequence> <q-char>

<q-char>
        ::= source character set except newline character and "
```

## Preprocessing numbers
```
<pp-number>
        ::= <digit> | . <digit> | <pp-number> <digit> |
            <pp-number> <nondigit> | <pp-number> e <sign> |
            <pp-number> E <sign> | <pp-number> .
```