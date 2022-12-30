

use crate::prelude::*;




define_error! (pub GeneratorError, result : GeneratorResult);




pub fn generate_token (_pattern : impl AsRef<TokenPattern>) -> GeneratorResult<Token> {
	
	let mut _collector = Vec::new ();
	
	generate_token_push (_pattern, &mut _collector) ?;
	
	let _token = Token {
			atoms : RbList::from_vec_rb (_collector),
		};
	
	Ok (_token)
}


pub fn generate_token_push (_pattern : impl AsRef<TokenPattern>, _collector : &mut Vec<Rb<Atom>>) -> GeneratorResult {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		
		TokenPattern::Named (_identifier, _pattern) =>
			generate_token_push (_pattern, _collector),
		
		TokenPattern::Atom (_pattern) => {
			let _atom = generate_atom (_pattern) ?;
			_collector.push (_atom);
			Ok (())
		}
		
		TokenPattern::Sequence (_patterns, _separator) => {
			let mut _is_first = true;
			for _pattern in _patterns.iter () {
				if ! _is_first {
					if let Some (_separator) = _separator {
						let _separator = _separator.clone ();
						let _atom = Rb::new (Atom::Separator (_separator));
						_collector.push (_atom);
					}
				}
				generate_token_push (_pattern, _collector) ?;
				_is_first = false;
			}
			Ok (())
		}
		
		TokenPattern::Repeat (_pattern, _separator, _count) => {
			let mut _is_first = true;
			for _index in 0 .. *_count {
				if ! _is_first {
					if let Some (_separator) = _separator {
						let _separator = _separator.clone ();
						let _atom = Rb::new (Atom::Separator (_separator));
						_collector.push (_atom);
					}
				}
				generate_token_push (_pattern, _collector) ?;
				_is_first = false;
			}
			Ok (())
		}
		
		TokenPattern::Empty =>
			Ok (()),
	}
}




pub fn generate_atom (_pattern : impl AsRef<AtomPattern>) -> GeneratorResult<Rb<Atom>> {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		
		AtomPattern::Separator (_separator) => {
			let _separator = _separator.clone ();
			Ok (Rb::new (Atom::Separator (_separator)))
		}
		
		AtomPattern::Constant (_text) => {
			let _text = _text.clone ();
			Ok (Rb::new (Atom::Constant (_text)))
		}
		
		AtomPattern::Glyph (_pattern) => {
			let _glyph = generate_glyph (_pattern) ?;
			Ok (Rb::new (Atom::Glyph (_glyph)))
		}
	}
}




pub fn generate_glyph (_pattern : impl AsRef<GlyphPattern>) -> GeneratorResult<Rb<Glyph>> {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		GlyphPattern::Set (_patterns) => {
			let _count = _patterns.len ();
			let _index = random_usize_range (.. _count);
			let _glyph = _patterns[_index] .clone ();
			Ok (_glyph)
		}
	}
}

