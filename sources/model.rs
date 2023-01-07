

use crate::prelude::*;




#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub struct Token {
	pub atoms : RbList<Atom>,
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum Atom {
	Separator (Rb<Separator>),
	Constant (Rb<Text>),
	Glyph (Rb<Glyph>),
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum Glyph {
	Text (Rb<Text>),
	Integer (u128, IntegerFormat),
	Bytes (Rb<Bytes>, BytesFormat),
	Timestamp (u128, TimestampFormat),
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum Separator {
	Mandatory (Rb<Text>),
	Optional (Rb<Text>),
}




#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum TokenPattern {
	Named (&'static str, &'static [&'static str], Rb<TokenPattern>),
	Atom (Rb<AtomPattern>),
	Sequence (RbList<TokenPattern>, Rb<SeparatorPattern>),
	Repeat (Rb<TokenPattern>, Rb<SeparatorPattern>, usize),
	Empty,
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum SeparatorPattern {
	None,
	Prefix (Rb<Separator>),
	Suffix (Rb<Separator>),
	Bracket (Rb<Separator>, Rb<Separator>),
	Infix (Rb<Separator>),
	InfixEach (Rb<Separator>, usize),
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum AtomPattern {
	Separator (Rb<Separator>),
	Constant (Rb<Text>),
	Glyph (Rb<GlyphPattern>),
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum GlyphPattern {
	Set (RbList<Glyph>),
	Integer (u128, u128, IntegerFormat),
	Bytes (usize, BytesFormat),
	Timestamp (TimestampFormat),
}




#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum Text {
	Char (char),
	Str (&'static str),
	String (String),
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum Bytes {
	Static (&'static [u8]),
	Boxed (Box<[u8]>),
}


#[ derive (Copy, Clone) ]
#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum IntegerFormat {
	Decimal,
	DecimalPadded (usize),
	Hex,
	HexPadded (usize),
}


#[ derive (Copy, Clone) ]
#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum BytesFormat {
	Hex,
}


#[ derive (Copy, Clone) ]
#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum TimestampFormat {
	Decimal (u128, u128, u128, usize),
	Hex (u128, u128, u128, usize),
	Strftime (&'static str, bool),
}

