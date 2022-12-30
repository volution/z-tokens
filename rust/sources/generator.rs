

use crate::prelude::*;




define_error! (pub GeneratorError, result : GeneratorResult);




pub fn generate_token (_pattern : impl AsRef<TokenPattern>, _randomizer : &mut impl Randomizer) -> GeneratorResult<Token> {
	
	let mut _collector = Vec::new ();
	
	generate_token_push (_pattern, _randomizer, &mut _collector) ?;
	
	let _token = Token {
			atoms : RbList::from_vec_rb (_collector),
		};
	
	Ok (_token)
}


pub fn generate_token_push (_pattern : impl AsRef<TokenPattern>, _randomizer : &mut impl Randomizer, _collector : &mut Vec<Rb<Atom>>) -> GeneratorResult {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		
		TokenPattern::Named (_identifier, _pattern) =>
			generate_token_push (_pattern, _randomizer, _collector),
		
		TokenPattern::Atom (_pattern) => {
			let _atom = generate_atom (_pattern, _randomizer) ?;
			_collector.push (_atom);
			Ok (())
		}
		
		TokenPattern::Sequence (_patterns, _separator) => {
			let _count = _patterns.len ();
			for (_index, _pattern) in _patterns.iter () .enumerate () {
				let (_before, _after) = generate_separator (_separator, _index, _count, _randomizer) ?;
				if let Some (_separator) = _before {
					let _atom = Rb::new (Atom::Separator (_separator));
					_collector.push (_atom);
				}
				generate_token_push (_pattern, _randomizer, _collector) ?;
				if let Some (_separator) = _after {
					let _atom = Rb::new (Atom::Separator (_separator));
					_collector.push (_atom);
				}
			}
			Ok (())
		}
		
		TokenPattern::Repeat (_pattern, _separator, _count) => {
			let _count = *_count;
			for _index in 0 .. _count {
				let (_before, _after) = generate_separator (_separator, _index, _count, _randomizer) ?;
				if let Some (_separator) = _before {
					let _atom = Rb::new (Atom::Separator (_separator));
					_collector.push (_atom);
				}
				generate_token_push (_pattern, _randomizer, _collector) ?;
				if let Some (_separator) = _after {
					let _atom = Rb::new (Atom::Separator (_separator));
					_collector.push (_atom);
				}
			}
			Ok (())
		}
		
		TokenPattern::Empty =>
			Ok (()),
	}
}




pub fn generate_separator (_pattern : impl AsRef<SeparatorPattern>, _index : usize, _count : usize, _randomizer : &mut impl Randomizer) -> GeneratorResult<(Option<Rb<Separator>>, Option<Rb<Separator>>)> {
	assert! (_count > 0);
	assert! (_index < _count);
	let _pattern = _pattern.as_ref ();
	let _separators = match _pattern {
		
		SeparatorPattern::None =>
			(None, None),
		
		SeparatorPattern::Prefix (_prefix) =>
			(
				if _index == 0 { Some (_prefix.clone ()) } else { None },
				None,
			),
		
		SeparatorPattern::Suffix (_suffix) =>
			(
				None,
				if _index == (_count - 1) { Some (_suffix.clone ()) } else { None },
			),
		
		SeparatorPattern::Bracket (_prefix, _suffix) =>
			(
				if _index == 0 { Some (_prefix.clone ()) } else { None },
				if _index == (_count - 1) { Some (_suffix.clone ()) } else { None },
			),
		
		SeparatorPattern::Infix (_separator) =>
			(
				if _index > 0 { Some (_separator.clone ()) } else { None },
				None,
			),
		
		SeparatorPattern::InfixEach (_separator, _each) =>
			(
				if (_index > 0) && ((_index % _each) == 0) { Some (_separator.clone ()) } else { None },
				None,
			),
	};
	Ok (_separators)
}




pub fn generate_atom (_pattern : impl AsRef<AtomPattern>, _randomizer : &mut impl Randomizer) -> GeneratorResult<Rb<Atom>> {
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
			let _glyph = generate_glyph (_pattern, _randomizer) ?;
			Ok (Rb::new (Atom::Glyph (_glyph)))
		}
	}
}




pub fn generate_glyph (_pattern : impl AsRef<GlyphPattern>, _randomizer : &mut impl Randomizer) -> GeneratorResult<Rb<Glyph>> {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		
		GlyphPattern::Set (_patterns) => {
			let _count = _patterns.len ();
			let _index = _randomizer.choose (_count) .else_wrap (0x5079d3d3) ?;
			let _glyph = _patterns[_index] .clone ();
			Ok (_glyph)
		}
		
		GlyphPattern::Integer (_lower, _upper, _format) => {
			let (_lower, _upper) = (*_lower, *_upper);
			if _lower > _upper {
				fail! (0xb8a08c0e);
			}
			let _delta = (_upper - _lower) + 1;
			if _delta > (usize::MAX as u128) {
				fail! (0x4cc61b73);
			}
			let _index = _randomizer.choose (_delta as usize) .else_wrap (0x5079d3d3) ?;
			let _value = _lower + (_index as u128);
			let _glyph = Glyph::Integer (_value, *_format);
			let _glyph = Rb::new (_glyph);
			Ok (_glyph)
		}
	}
}

