use crate::dimensions::Unit;
use crate::font::{Weight, Family, AtomType, Style, style_symbol};
use crate::layout::Style as LayoutStyle;
// use crate::lexer::{Lexer, Token};
use crate::ast as parse;
use crate::ast::nodes::{ParseNode, Radical, MathStyle, GenFraction, Rule, BarThickness, AtomChange,
                    Color, Stack};
use crate::ast::color::RGBA;
use crate::error::{ParseError, ParseResult};
use crate::ast::symbols::Symbol;


macro_rules! sym {
    (@at ord) => { AtomType::Ordinal };
    (@at bin) => { AtomType::Binary };
    (@at op)  => { AtomType::Operator };
    (@at open) => { AtomType::Open };
    (@at close) => { AtomType::Close };

    ($code:expr, $ord:ident) => ({
        Some(Symbol {
            codepoint: $code,
            atom_type: sym!(@at $ord),
        })
    });
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Radical,
    Rule,
    VExtend,
    Color,
    ColorLit(RGBA),
    Fraction(Option<Symbol>, Option<Symbol>, BarThickness, MathStyle),
    DelimiterSize(u8, AtomType),
    Kerning(Unit),
    Style(LayoutStyle),
    AtomChange(AtomType),
    TextOperator(&'static str, bool),
    SubStack(AtomType),
}
