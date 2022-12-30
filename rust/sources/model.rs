

use crate::prelude::*;




pub struct Token {
	pub atoms : RbList<Atom>,
}


pub enum Atom {
	Separator (Rb<Separator>),
	Constant (Rb<Text>),
	Glyph (Rb<Glyph>),
}


pub enum Glyph {
	Text (Rb<Text>),
}


pub enum Separator {
	Mandatory (Rb<Text>),
	Optional (Rb<Text>),
}




pub enum TokenPattern {
	Named (&'static str, Rb<TokenPattern>),
	Atom (Rb<AtomPattern>),
	Sequence (RbList<TokenPattern>),
	Repeat (Rb<TokenPattern>, usize),
	Empty,
}


pub enum AtomPattern {
	Separator (Rb<Separator>),
	Constant (Rb<Text>),
	Glyph (Rb<GlyphPattern>),
}


pub enum GlyphPattern {
	Set (RbList<Glyph>),
}




pub enum Text {
	Char (char),
	String (String),
	Static (&'static str),
}


