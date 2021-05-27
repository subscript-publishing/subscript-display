use crate::error::{ParseError, ParseResult};
use crate::font::{Style, style_symbol, AtomType};
// use crate::lexer::{Lexer, Token};
use crate::ast::{
    nodes::{Delimited, ParseNode, Accent, Scripts},
    symbols::Symbol,
    color::RGBA
};
// use crate::functions::get_command;
use crate::environments::Environment;
use crate::dimensions::*;

/// Helper function for determining an atomtype based on a given codepoint.
/// This is primarily used for characters while processing, so may give false
/// negatives when used for other things.
fn codepoint_atom_type(codepoint: char) -> Option<AtomType> {
    Some(match codepoint {
             'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | 'Α' ..= 'Ω' | 'α' ..= 'ω' => AtomType::Alpha,
             '*' | '+' | '-' => AtomType::Binary,
             '[' | '(' => AtomType::Open,
             ']' | ')' | '?' | '!' => AtomType::Close,
             '=' | '<' | '>' | ':' => AtomType::Relation,
             ',' | ';' => AtomType::Punctuation,
             '|' => AtomType::Fence,
             '/' | '@' | '.' | '"' => AtomType::Alpha,
             _ => return None,
         })
}
