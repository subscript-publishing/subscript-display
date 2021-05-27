//! This is a collection of tools used for converting ParseNodes into LayoutNodes.

use crate::font::{Glyph, Direction, VariantGlyph};
use crate::dimensions::{*};
use crate::layout::LayoutSettings;

use super::{Style};
use super::builders;
use super::{LayoutNode, LayoutVariant, LayoutGlyph};
use crate::ast::nodes::Rule;
use crate::error::LayoutResult;

pub trait AsLayoutNode<'f> {
    fn as_layout<'a>(&self, config: LayoutSettings<'a, 'f>) -> LayoutResult<LayoutNode<'f>>;
}

impl<'f> AsLayoutNode<'f> for Glyph<'f> {
    fn as_layout<'a>(
        &self,
        config: LayoutSettings<'a, 'f>
    ) -> LayoutResult<LayoutNode<'f>> {
        Ok(LayoutNode {
            height: self.height().scaled(config),
            width:  self.advance.scaled(config),
            depth:  self.depth().scaled(config),
            node:   LayoutVariant::Glyph(LayoutGlyph {
                font: self.font,
                gid: self.gid,
                size: Length::new(1.0, Em).scaled(config),
                attachment: self.attachment.scaled(config),
                italics: self.italics.scaled(config),
                offset:  Length::zero(),
            })
        })
    }
}

impl<'f> AsLayoutNode<'f> for Rule {
    fn as_layout<'a>(&self, config: LayoutSettings<'a, 'f>) -> LayoutResult<LayoutNode<'f>> {
        Ok(LayoutNode {
            node:   LayoutVariant::Rule,
            width:  self.width .scaled(config),
            height: self.height.scaled(config),
            depth:  Length::zero(),
        })
    }
}

impl<'f> AsLayoutNode<'f> for VariantGlyph {
    fn as_layout<'a>(&self, config: LayoutSettings<'a, 'f>) -> LayoutResult<LayoutNode<'f>> {
        match *self {
            VariantGlyph::Replacement(gid) => {
                let glyph = config.ctx.glyph_from_gid(gid)?;
                glyph.as_layout(config)
            },

            VariantGlyph::Constructable(dir, ref parts) => {
                match dir {
                    Direction::Vertical => {
                        let mut contents = builders::VBox::new();
                        for instr in parts {
                            let glyph = config.ctx.glyph_from_gid(instr.gid)?;
                            contents.insert_node(0, glyph.as_layout(config)?);
                            if instr.overlap != 0 {
                                let overlap = Length::new(instr.overlap, Font);
                                let kern = -(overlap + glyph.depth()).scaled(config);
                                contents.add_node(kern!(vert: kern));
                            }
                        }

                        Ok(contents.build())
                    },

                    Direction::Horizontal => {
                        let mut contents = builders::HBox::new();
                        for instr in parts {
                            let glyph = config.ctx.glyph_from_gid(instr.gid)?;
                            if instr.overlap != 0 {
                                let kern = -Length::new(instr.overlap, Font).scaled(config);
                                contents.add_node(kern!(horz: kern));
                            }
                            contents.add_node(glyph.as_layout(config)?);
                        }

                        Ok(contents.build())
                    }
                }
            },
        }
    }
}

impl<'a, 'f> LayoutSettings<'a, 'f> {
    fn scale_factor(&self) -> f64 {
        match self.style {
            Style::Display |
            Style::DisplayCramped |
            Style::Text |
            Style::TextCramped
                => 1.0,

            Style::Script |
            Style::ScriptCramped
                => self.ctx.constants.script_percent_scale_down,

            Style::ScriptScript |
            Style::ScriptScriptCramped
                => self.ctx.constants.script_script_percent_scale_down,
        }
    }
    fn scale_font_unit(&self, length: Length<Font>) -> Length<Px> {
        length / self.ctx.units_per_em * self.font_size
    }
    pub fn to_font(&self, length: Length<Px>) -> Length<Font> {
        length / self.font_size * self.ctx.units_per_em
    }
}
pub trait Scaled {
    fn scaled(self, config: LayoutSettings) -> Length<Px>;
}

impl Scaled for Length<Font> {
    fn scaled(self, config: LayoutSettings) -> Length<Px> {
        config.scale_font_unit(self) * config.scale_factor()
    }
}

impl Scaled for Length<Px> {
    fn scaled(self, config: LayoutSettings) -> Length<Px> {
        self * config.scale_factor()
    }
}
impl Scaled for Length<Em> {
    fn scaled(self, config: LayoutSettings) -> Length<Px> {
        self * config.font_size * config.scale_factor()
    }
}
impl Scaled for Unit {
    fn scaled(self, config: LayoutSettings) -> Length<Px> {
        let length = match self {
            Unit::Em(em) => Length::new(em, Em) * config.font_size,
            Unit::Px(px) => Length::new(px, Px)
        };
        length * config.scale_factor()
    }
}
