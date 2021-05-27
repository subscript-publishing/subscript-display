use crate::dimensions::{Unit};
use crate::layout::Style;
use crate::error::{ParseResult, ParseError};
use super::color::RGBA;
use crate::environments::Array;
use crate::font::{AtomType};
use super::symbols::Symbol;

// TODO: It might be worth letting the `Group` variant
// to have an atomtype associated with it.  By default,
// it will be a `Ordinal`.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseNode {
    Symbol(Symbol),
    Delimited(Delimited),
    Radical(Radical),
    GenFraction(GenFraction),
    Scripts(Scripts),
    Rule(Rule),
    Kerning(Unit),
    Accent(Accent),
    Style(Style),
    AtomChange(AtomChange),
    Color(Color),
    Group(Vec<ParseNode>),
    Stack(Stack),
    Extend(char, Unit),
    Array(Array),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Stack {
    pub atom_type: AtomType,
    pub lines: Vec<Vec<ParseNode>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Delimited {
    pub left: Symbol,
    pub right: Symbol,
    pub inner: Vec<ParseNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scripts {
    pub base: Option<Box<ParseNode>>,
    pub superscript: Option<Vec<ParseNode>>,
    pub subscript: Option<Vec<ParseNode>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AtomChange {
    pub at: AtomType,
    pub inner: Vec<ParseNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Accent {
    pub symbol: Symbol,
    pub nucleus: Vec<ParseNode>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rule {
    pub width: Unit,
    pub height: Unit,
    //pub depth:  Unit,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Radical {
    pub inner: Vec<ParseNode>,
    // pub superscript: Vec<ParseNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenFraction {
    pub numerator: Vec<ParseNode>,
    pub denominator: Vec<ParseNode>,
    pub bar_thickness: BarThickness,
    pub left_delimiter: Option<Symbol>,
    pub right_delimiter: Option<Symbol>,
    pub style: MathStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub color: RGBA,
    pub inner: Vec<ParseNode>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BarThickness {
    Default,
    None,
    Unit(Unit),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MathStyle {
    Display,
    Text,
    NoChange,
}

impl ParseNode {
    pub fn expect_left(self) -> ParseResult<'static, Symbol> {
        if let ParseNode::Symbol(sym) = self {
            if sym.atom_type == AtomType::Open || sym.atom_type == AtomType::Fence ||
               sym.codepoint == '.' {
                return Ok(sym);
            } else {
                return Err(ParseError::ExpectedOpen(sym));
            }
        } else {
            unreachable!()
        }
    }

    pub fn expect_right(self) -> ParseResult<'static, Symbol> {
        if let ParseNode::Symbol(sym) = self {
            if sym.atom_type == AtomType::Close || sym.atom_type == AtomType::Fence ||
               sym.codepoint == '.' {
                return Ok(sym);
            } else {
                return Err(ParseError::ExpectedClose(sym));
            }
        } else {
            unreachable!()
        }
    }

    pub fn set_atom_type(&mut self, at: AtomType) {
        match *self {
            ParseNode::Symbol(ref mut sym) => sym.atom_type = at,
            ParseNode::Scripts(Scripts { ref mut base, .. }) => {
                if let Some(ref mut b) = *base {
                    b.set_atom_type(at);
                }
            }
            ParseNode::AtomChange(ref mut node) => node.at = at,
            ParseNode::Stack(Stack { ref mut atom_type, .. }) => *atom_type = at,
            _ => (),
        }
    }

    pub fn is_symbol(&self) -> Option<Symbol> {
        match *self {
            ParseNode::Symbol(sym) => Some(sym),
            ParseNode::Scripts(Scripts { ref base, .. }) =>
                base.as_ref().and_then(|b| b.is_symbol()),
            ParseNode::Accent(ref acc) => is_symbol(&acc.nucleus),
            ParseNode::AtomChange(ref ac) => is_symbol(&ac.inner),
            ParseNode::Color(ref clr) => is_symbol(&clr.inner),
            _ => None,
        }
    }

    pub fn atom_type(&self) -> AtomType {
        match *self {
            ParseNode::Symbol(ref sym)  => sym.atom_type,
            ParseNode::Delimited(_)     => AtomType::Inner,
            ParseNode::Radical(_)       => AtomType::Alpha,
            ParseNode::GenFraction(_)   => AtomType::Inner,
            ParseNode::Group(_)         => AtomType::Alpha,
            ParseNode::Scripts(ref scr) => scr.base.as_ref()
                .map(|base| base.atom_type())
                .unwrap_or(AtomType::Alpha),

            ParseNode::Rule(_)          => AtomType::Alpha,
            ParseNode::Kerning(_)       => AtomType::Transparent,
            ParseNode::Accent(ref acc)  => acc.nucleus.first()
                .map(|acc| acc.atom_type())
                .unwrap_or(AtomType::Alpha),

            ParseNode::Style(_)         => AtomType::Transparent,
            ParseNode::AtomChange(ref ac) => ac.at,
            ParseNode::Color(ref clr)     => clr.inner.first()
                .map(|first| first.atom_type())
                .unwrap_or(AtomType::Alpha),

            ParseNode::Extend(_,_)   => AtomType::Inner,
            ParseNode::Array(_)      => AtomType::Inner,
            ParseNode::Stack(ref s)  => s.atom_type,
        }
    }
}

pub fn is_symbol(contents: &[ParseNode]) -> Option<Symbol> {
    if contents.len() != 1 {
        return None;
    }

    contents[0].is_symbol()
}