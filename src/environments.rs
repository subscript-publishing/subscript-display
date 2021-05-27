// use crate::lexer::{Lexer, Token};
use crate::font::{Style, AtomType};
use crate::ast::{self, ParseNode, symbols::Symbol};
use crate::error::{ParseResult, ParseError};

/// An enumeration of recognized enviornmnets.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Environment {
    Array,
    Matrix,
    PMatrix,
    BMatrix,
    BbMatrix,
    VMatrix,
    VvMatrix,
}

impl Environment {
    /// Attempt to parse an `&str` type into a an `Enviornment`.
    pub fn try_from_str(name: &str) -> Option<Environment> {
        match name {
            "array" => Some(Environment::Array),
            "matrix" => Some(Environment::Matrix),
            "pmatrix" => Some(Environment::PMatrix),
            "bmatrix" => Some(Environment::BMatrix),
            "Bmatrix" => Some(Environment::BbMatrix),
            "vmatrix" => Some(Environment::VMatrix),
            "Vmatrix" => Some(Environment::VvMatrix),
            _ => None,
        }
    }
}

/// The horizontal positioning of an array.  These are parsed as an optional
/// argument for the Array environment. The default value is `Centered` along
/// the x-axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayVerticalAlign {
    /// Centered along the x-axis.
    Centered,

    /// Align the top with the baseline.
    Top,

    /// Align the bottom with the baseline.
    Bottom,
}

impl Default for ArrayVerticalAlign {
    fn default() -> ArrayVerticalAlign {
        ArrayVerticalAlign::Centered
    }
}

// TODO: since we use default values, we should make the argument optional?
/// Array column alignent.  These are parsed as a required macro argument
/// for the array enviornment. The default value is `Centered`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayColumnAlign {
    /// Column is centered
    Centered,

    /// Column is left aligned.
    Left,

    /// Column is right aligned.
    Right,
}

impl Default for ArrayColumnAlign {
    fn default() -> ArrayColumnAlign {
        ArrayColumnAlign::Centered
    }
}

/// Formatting options for a single column.  This includes both the horizontal
/// alignment of the column (clr), and optional vertical bar spacers (on the left).
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArraySingleColumnFormatting {
    /// The alignment of the column.  Defaults to Centered.
    alignment: ArrayColumnAlign,

    /// The number of vertical marks before column.
    left_vert: u8,
}

/// The collection of column formatting for an array.  This includes the vertical
/// alignment for each column in an array along with optional vertical bars
/// positioned to the right of the last column.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ArrayColumnsFormatting {
    /// The formatting specifications for each column
    columns: Vec<ArraySingleColumnFormatting>,

    /// The number of vertical marks after the last column.
    right_vert: u8,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Array {
    /// The formatting arguments (clr) for each row.  Default: center.
    pub col_format: ArrayColumnsFormatting,

    /// A collection of rows.  Each row consists of one `Vec<Expression>`.
    pub rows: Vec<Vec<Expression>>,

    /// The left delimiter for the array (optional).
    pub left_delimiter: Option<Symbol>,

    /// The right delimiter for the array (optional).
    pub right_delimiter: Option<Symbol>,
}

type Expression = Vec<ParseNode>;

