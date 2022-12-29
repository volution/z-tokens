

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
		
		TokenPattern::Atom (_pattern) => {
			let _atom = generate_atom (_pattern) ?;
			_collector.push (_atom);
			Ok (())
		}
		
		TokenPattern::Sequence (_patterns) => {
			for _pattern in _patterns.iter () {
				generate_token_push (_pattern, _collector) ?;
			}
			Ok (())
		}
		
		TokenPattern::Repeat (_pattern, _count) => {
			for _index in 0 .. *_count {
				generate_token_push (_pattern, _collector) ?
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

