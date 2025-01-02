// Transform unicode/byte sequences to this.
//
// a) 5 whitespace characters (space, horizontal tab, vertical tab, form feed, new-line)
// b) 10 digit characters from '0' to '9'
// c) 52 letters from 'a' to 'z' and from 'A' to 'Z'
// d) 29 punctuation characters: _ { } [ ] # ( ) < > % : ; . ? * + - / ^ & | ~ ! = , \ " '

use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

pub(super) static TRIGRAPH_MAP: Lazy<HashMap<(char, char, char), u8>> = Lazy::new(|| {
    HashMap::from([
        (('?', '?', '='), b'#'),
        (('?', '?', '('), b'['),
        (('?', '?', '/'), b'\\'),
        (('?', '?', ')'), b']'),
        (('?', '?', '\''), b'^'),
        (('?', '?', '<'), b'{'),
        (('?', '?', '!'), b'|'),
        (('?', '?', '>'), b'}'),
        (('?', '?', '-'), b'~'),
    ])
});

pub(super) static WHITESPACE_SET: Lazy<HashSet<char>> =
    Lazy::new(|| HashSet::from([' ', '\t', '\x0B', '\x0C', '\n']));

pub(super) static ALPHABET: Lazy<HashSet<char>> = Lazy::new(|| {
    let mut alphabet = HashSet::new();
    alphabet.extend('0'..='9');
    alphabet.extend('a'..='z');
    alphabet.extend('A'..='Z');

    alphabet.extend([
        '_', '{', '}', '[', ']', '#', '(', ')', '<', '>', '%', ':', ';', '.', '?', '*', '+', '-', '/',
        '^', '&', '|', '~', '!', '=', ',', '\\', '"', '\'',
    ]);

    alphabet
});
