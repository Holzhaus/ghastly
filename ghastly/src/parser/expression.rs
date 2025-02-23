// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

/// A Token consisting of a value and a token kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    value: &'a str,
    kind: TokenKind,
}

impl Token<'_> {
    #[inline]
    #[expect(dead_code)]
    pub fn value(&self) -> &str {
        self.value
    }

    #[inline]
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    #[inline]
    pub fn string(value: &str) -> Token<'_> {
        Token {
            value,
            kind: TokenKind::String,
        }
    }

    #[inline]
    pub fn expression(value: &str) -> Token<'_> {
        Token {
            value,
            kind: TokenKind::Expression,
        }
    }
}

/// The kind of token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// A string literal (outside of an expression).
    String,
    /// A GitHub expression (inside `${{ ... }}`)
    Expression,
}

/// Tokenize a string to differentiate normal strings from GitHub expressions.
pub fn tokenize(text: &str) -> impl Iterator<Item = Token<'_>> + '_ {
    let mut remainder = text;
    let mut current_token_kind = TokenKind::String;
    let mut eof_found = false;
    std::iter::from_fn(move || {
        if eof_found {
            return None;
        }

        if current_token_kind == TokenKind::Expression {
            match remainder.split_once("}}") {
                Some((before, after)) => {
                    let value = Token::expression(before);
                    remainder = after;
                    current_token_kind = TokenKind::String;
                    Some(value)
                }
                None => {
                    // Missing end of expression. Should we fail here?
                    let value = Token::expression(remainder);
                    remainder = "";
                    eof_found = true;
                    Some(value)
                }
            }
        } else {
            match remainder.split_once("${{") {
                Some((before, after)) => {
                    let value = Token::string(before);
                    remainder = after;
                    current_token_kind = TokenKind::Expression;
                    Some(value)
                }
                None => {
                    // EOF encountered when looking for start of expression.
                    let value = Token::string(remainder);
                    remainder = "";
                    eof_found = true;
                    Some(value)
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("hello world!").collect::<Vec<_>>(),
            vec![Token::string("hello world!")]
        );
        assert_eq!(
            tokenize("${{ foo.bar }}").collect::<Vec<_>>(),
            vec![
                Token::string(""),
                Token::expression(" foo.bar "),
                Token::string("")
            ]
        );
        assert_eq!(
            tokenize("hello ${{ foo.bar }} world!").collect::<Vec<_>>(),
            vec![
                Token::string("hello "),
                Token::expression(" foo.bar "),
                Token::string(" world!")
            ]
        );
        assert_eq!(
            tokenize("${{ foo.bar }} and '${{ toJSON(bar.baz) }}'!").collect::<Vec<_>>(),
            vec![
                Token::string(""),
                Token::expression(" foo.bar "),
                Token::string(" and '"),
                Token::expression(" toJSON(bar.baz) "),
                Token::string("'!")
            ]
        );
    }
}
