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
        ::= <nondigit> | <identifier><nondigit> |
            <identifier> | digit

<nondigit>
        ::= [a-zA-Z_]

<digit> ::= [0-9]
```