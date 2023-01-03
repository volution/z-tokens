

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
	Integer (u128, IntegerFormat),
	Bytes (Rb<Bytes>, BytesFormat),
	Timestamp (u128, TimestampFormat),
}


pub enum Separator {
	Mandatory (Rb<Text>),
	Optional (Rb<Text>),
}




pub enum TokenPattern {
	Named (&'static str, Rb<TokenPattern>),
	Atom (Rb<AtomPattern>),
	Sequence (RbList<TokenPattern>, Rb<SeparatorPattern>),
	Repeat (Rb<TokenPattern>, Rb<SeparatorPattern>, usize),
	Empty,
}


pub enum SeparatorPattern {
	None,
	Prefix (Rb<Separator>),
	Suffix (Rb<Separator>),
	Bracket (Rb<Separator>, Rb<Separator>),
	Infix (Rb<Separator>),
	InfixEach (Rb<Separator>, usize),
}


pub enum AtomPattern {
	Separator (Rb<Separator>),
	Constant (Rb<Text>),
	Glyph (Rb<GlyphPattern>),
}


pub enum GlyphPattern {
	Set (RbList<Glyph>),
	Integer (u128, u128, IntegerFormat),
	Bytes (usize, BytesFormat),
	Timestamp (TimestampFormat),
}




pub enum Text {
	Char (char),
	Str (&'static str),
	String (String),
}


pub enum Bytes {
	Static (&'static [u8]),
	Boxed (Box<[u8]>),
}


#[ derive (Copy, Clone) ]
pub enum IntegerFormat {
	Decimal,
	DecimalPadded (usize),
	Hex,
	HexPadded (usize),
}


#[ derive (Copy, Clone) ]
pub enum BytesFormat {
	Hex,
}


#[ derive (Copy, Clone) ]
pub enum TimestampFormat {
	Decimal (u128, u128, u128, usize),
	Hex (u128, u128, u128, usize),
	Strftime (&'static str, bool),
}

