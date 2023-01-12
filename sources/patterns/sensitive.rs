

use crate::prelude::*;




include! ("./sensitive_macros.in");








// NOTE:  Has only public information.
impl_sensitive_nop! (TokenPattern);
impl_sensitive_nop! (TokenPatternTags);
impl_sensitive_nop! (SeparatorPattern);
impl_sensitive_nop! (AtomPattern);
impl_sensitive_nop! (GlyphPattern);








impl_sensitive! (Token => |self| {
	self.atoms.erase ();
});

impl_sensitive! (Atom => |self| {
	match self {
		Atom::Separator (_separator) =>
			_separator.erase (),
		Atom::Constant (_constant) =>
			_constant.erase (),
		Atom::Glyph (_glyph) =>
			_glyph.erase (),
	}
});

impl_sensitive! (Glyph => |self| {
	match self {
		Glyph::Text (_text) =>
			_text.erase (),
		Glyph::Integer (_integer, _) =>
			_integer.erase (),
		Glyph::Bytes (_bytes, _) =>
			_bytes.erase (),
		Glyph::Timestamp (_timestamp, _) =>
			_timestamp.erase (),
	}
});

impl_sensitive! (Separator => |self| {
	match self {
		Separator::Mandatory (_text) =>
			_text.erase (),
		Separator::Optional (_text) =>
			_text.erase (),
	}
});


impl_sensitive! (Text => |self| {
	match self {
		Text::Char (_char) =>
			_char.erase (),
		Text::Str (_str) =>
			_str.erase (),
		Text::String (_string) =>
			_string.erase (),
	}
});

impl_sensitive! (Bytes => |self| {
	match self {
		Bytes::Static (_slice) =>
			_slice.erase (),
		Bytes::Boxed (_box) =>
			_box.as_mut () .erase (),
	}
});


