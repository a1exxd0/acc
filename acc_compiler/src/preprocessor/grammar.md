# Preprocessing directives
```
<preprocessing-file> 
        ::= group_o

<group> ::= <group-part> | <group> <group-part>

<group-part>
        ::= <<opt> pp-tokens> \n | 
            <if-section> |
            <control-line>

<if-section>
        ::= <if-group> <<opt> elif-groups> <<opt> else-group> <endif-line> 

<if-group>
        ::= # if <constant-expression> \n <<opt> group> |
            # ifdef <identifier> \n <<opt> group> |
            # ifndef <identifier> \n <<opt> group> 

<elif-groups>
        ::= <elif-group> | <elif-groups> <elif-group>

<elif-group>
        ::= # elif <constant-expression> \n <<opt> group>

<else-group>
        ::= # else \n <<opt> group>

<endif-line>
        ::= # endif \n

<control-line>
        ::= # include <pp-tokens> <newline> | 
            # define <identifier> <replacement-list> <newline> |
            # define <identifier> <lparen> <<opt> identifier-list> ) <replacement-list> \n |
            # undef <identifier> \n | # line <pp-tokens> \n |
            # error <<opt> pp-tokens> \n | # pragma <<opt> pp-tokens> \n |
            \n

<identifier-list>
        ::= <identifier> |
            <identifier-list> , <identifier>

<lparen>
        ::= left parentheses without preceding whitespace

<replacement-list>
        ::= <<opt> pp-tokens>

<pp-tokens>
        ::= <preprocessing-token> | <pp-tokens> <preprocessing-token>
```