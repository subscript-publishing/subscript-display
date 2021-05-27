// use crate::lexer::Token;
use std::fmt;
use crate::font::{AtomType};
use crate::ast::symbols::Symbol;

pub type LayoutResult<T> = ::std::result::Result<T, LayoutError>;
pub type ParseResult<'a, T> = ::std::result::Result<T, ParseError<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutError {
    Font(FontError)
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontError {
    MissingGlyphCodepoint(char),
    MissingGlyphGID(u16),
}
impl From<FontError> for LayoutError {
    fn from(e: FontError) -> Self {
        LayoutError::Font(e)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError<'a> {
    UnrecognizedCommand(&'a str),
    UnrecognizedSymbol(char),
    UnrecognizedDimension,
    UnrecognizedColor(&'a str),

    // ExpectedMathField(Token<'a>),
    // ExpectedTokenFound(Token<'a>, Token<'a>),
    ExpectedOpen(Symbol),
    ExpectedClose(Symbol),
    ExpectedAtomType(AtomType, AtomType),
    // ExpectedSymbol(Token<'a>),
    ExpectedOpenGroup,

    MissingSymbolAfterDelimiter,
    MissingSymbolAfterAccent,
    LimitsMustFollowOperator,
    RequiredMacroArg,
    NoClosingBracket,
    StackMustFollowGroup,
    AccentMissingArg(&'a str),
    // FailedToParse(Token<'a>),
    ExcessiveSubscripts,
    ExcessiveSuperscripts,

    // UnexpectedEof(Token<'a>),

    Todo
}
#[derive(Debug, Clone, PartialEq)]
pub enum Error<'a> {
    Parse(ParseError<'a>),
    Layout(LayoutError)
}
impl<'a> From<ParseError<'a>> for Error<'a> {
    fn from(e: ParseError<'a>) -> Self {
        Error::Parse(e)
    }
}
impl<'a> From<LayoutError> for Error<'a> {
    fn from(e:LayoutError) -> Self {
        Error::Layout(e)
    }
}


impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FontError::*;
        match *self {
            MissingGlyphCodepoint(cp) =>
                write!(f, "missing glyph for codepoint'{}'", cp),
            MissingGlyphGID(gid) =>
                write!(f, "missing glyph with gid {}", gid),
        }
    }
}
