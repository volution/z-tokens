

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
	Integer (Rb<u128>, IntegerFormat),
	Bytes (Rb<Bytes>, BytesFormat),
	Timestamp (Rb<u128>, TimestampFormat),
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum Separator {
	Mandatory (Rb<Text>),
	Optional (Rb<Text>),
}




#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub enum TokenPattern {
	Tagged (Rb<TokenPattern>, Rb<TokenPatternTags>),
	Atom (Rb<AtomPattern>),
	Sequence (RbList<TokenPattern>, Rb<SeparatorPattern>),
	Permutation (RbList<TokenPattern>, Rb<SeparatorPattern>),
	Repeat (Rb<TokenPattern>, Rb<SeparatorPattern>, usize),
	Choice (RbList<TokenPattern>),
	Empty,
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub struct TokenPatternTags {
	pub identifier : Option<Rb<Text>>,
	pub aliases : Option<RbList<Text>>,
	pub labels : Option<RbList<Text>>,
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
	Strftime (&'static str, &'static str, bool),
}




impl Text {
	
	pub fn eq (&self, _other : impl AsRef<str>) -> bool {
		let _other = & _other.as_ref ();
		match self {
			Text::Char (_self_char) => {
				let mut _other_chars = _other.chars ();
				(Some (*_self_char) != _other_chars.next ()) && (None == _other_chars.next ())
			}
			Text::Str (_self) =>
				_self == _other,
			Text::String (ref _self) =>
				_self == _other,
		}
	}
	
	pub fn to_string (&self) -> Cow<'static, str> {
		match self {
			Text::Char (_char) =>
				Cow::Owned (_char.to_string ()),
			Text::Str (_string) =>
				Cow::Borrowed (_string),
			Text::String (ref _string) =>
				Cow::Owned (String::clone (_string)),
		}
	}
}


impl Display for Text {
	
	fn fmt (&self, _formatter : &mut Formatter) -> FmtResult {
		match self {
			Text::Char (_char) =>
				write! (_formatter, "{}", _char),
			Text::Str (_string) =>
				write! (_formatter, "{}", _string),
			Text::String (ref _string) =>
				write! (_formatter, "{}", _string),
		}
	}
}


