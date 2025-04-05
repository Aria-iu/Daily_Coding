pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    // stores both a foreground and background color in a single struct
    pub color: ColorPair,
    // store a single character or glyph
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player ;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy ;