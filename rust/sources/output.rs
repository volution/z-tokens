

use crate::prelude::*;




define_error! (pub OutputError, result : OutputResult);




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
			if false {
				output_text (_text, _stream)
			} else {
				Ok (())
			},
	}
}




pub fn output_glyph (_glyph : impl AsRef<Glyph>, _stream : impl Write) -> OutputResult {
	let _glyph = _glyph.as_ref ();
	match _glyph {
		Glyph::Text (_text) =>
			output_text (_text, _stream),
	}
}




pub fn output_text (_text : impl AsRef<Text>, mut _stream : impl Write) -> OutputResult {
	let _text = _text.as_ref ();
	match _text {
		Text::Char (_char) =>
			write! (_stream, "{}", _char),
		Text::Static (_string) =>
			write! (_stream, "{}", _string),
		Text::String (_string) =>
			write! (_stream, "{}", _string),
	} .else_wrap (0x180d200b)
}

