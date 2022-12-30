

use crate::prelude::*;




define_error! (pub OutputError, result : OutputResult);




pub fn output_token_to_string (_token : impl AsRef<Token>) -> OutputResult<String> {
	
	let mut _buffer = Vec::with_capacity (1024);
	
	output_token (_token, &mut _buffer) ?;
	
	let _string = String::from_utf8 (_buffer) .else_wrap (0xb126b3c8) ?;
	
	Ok (_string)
}



pub fn output_token (_token : impl AsRef<Token>, mut _stream : impl Write) -> OutputResult {
	let _token = _token.as_ref ();
	for _atom in _token.atoms.iter () {
		output_atom (_atom, &mut _stream) ?;
	}
	Ok (())
}




pub fn output_atom (_atom : impl AsRef<Atom>, _stream : impl Write) -> OutputResult {
	let _atom = _atom.as_ref ();
	match _atom {
		Atom::Separator (_separator) =>
			output_separator (_separator, _stream),
		Atom::Constant (_text) =>
			output_text (_text, _stream),
		Atom::Glyph (_glyph) =>
			output_glyph (_glyph, _stream),
	}
}




pub fn output_separator (_separator : impl AsRef<Separator>, _stream : impl Write) -> OutputResult {
	let _separator = _separator.as_ref ();
	match _separator {
		Separator::Mandatory (_text) =>
			output_text (_text, _stream),
		Separator::Optional (_text) =>
			if true {
				output_text (_text, _stream)
			} else {
				Ok (())
			},
	}
}




pub fn output_glyph (_glyph : impl AsRef<Glyph>, mut _stream : impl Write) -> OutputResult {
	let _glyph = _glyph.as_ref ();
	match _glyph {
		Glyph::Text (_text) =>
			output_text (_text, _stream),
		Glyph::Integer (_value, _format) =>
			match *_format {
				IntegerFormat::Decimal =>
					write! (_stream, "{0:}", *_value) .else_wrap (0x6d36e225),
				IntegerFormat::DecimalPadded (_width) =>
					write! (_stream, "{0:1$}", *_value, _width) .else_wrap (0x6044048b),
				IntegerFormat::Hex =>
					write! (_stream, "{0:x}", *_value) .else_wrap (0xe1154407),
				IntegerFormat::HexPadded (_width) =>
					write! (_stream, "{0:1$x}", *_value, _width) .else_wrap (0x33322c70),
			}
	}
}




pub fn output_text (_text : impl AsRef<Text>, mut _stream : impl Write) -> OutputResult {
	let _text = _text.as_ref ();
	match _text {
		Text::Char (_char) =>
			write! (_stream, "{}", _char),
		Text::Str (_string) =>
			write! (_stream, "{}", _string),
		Text::String (_string) =>
			write! (_stream, "{}", _string),
	} .else_wrap (0x180d200b)
}

