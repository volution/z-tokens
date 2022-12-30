

use crate::prelude::*;


use ::num_bigint::{
		BigUint,
	};

use ::num_traits::{
		Zero as _,
		ToPrimitive as _,
	};




define_error! (pub EntroyError, result : EntropyResult);




pub struct Entropy {
	accumulator : BigUint,
}


impl Entropy {
	
	pub fn none () -> Self {
		Self {
				accumulator : BigUint::zero (),
			}
	}
	
	pub fn for_choice (_count : usize) -> Self {
		Self {
				accumulator : BigUint::from (_count),
			}
	}
	
	pub fn multiply (&mut self, _other : &Entropy) -> EntropyResult {
		if self.accumulator.is_zero () {
			self.accumulator = _other.accumulator.clone ();
		} else if _other.accumulator.is_zero () {
			// NOP
		} else {
			self.accumulator *= &_other.accumulator;
		}
		Ok (())
	}
	
	pub fn bits (&self) -> f64 {
		self.accumulator.to_f64 () .else_panic (0xf18aae78) .log2 ()
	}
	
	pub fn bits_exact (&self) -> (f64, bool) {
		let _bits = self.accumulator.bits ();
		let mut _exact = true;
		for _bit in 0 .. (_bits - 1) {
			if self.accumulator.bit (_bit) {
				_exact = false;
				break;
			}
		}
		if _exact {
			((_bits - 1) as f64, true)
		} else {
			(self.bits (), false)
		}
	}
	
	pub(crate) fn accumulator (&self) -> Option<u128> {
		self.accumulator.to_u128 ()
	}
}




pub fn entropy_token (_pattern : impl AsRef<TokenPattern>) -> EntropyResult<Entropy> {
	
	let mut _entropy = Entropy::none ();
	
	entropy_token_push (_pattern, &mut _entropy) ?;
	
	Ok (_entropy)
}


pub fn entropy_token_push (_pattern : impl AsRef<TokenPattern>, _collector : &mut Entropy) -> EntropyResult {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		
		TokenPattern::Named (_identifier, _pattern) =>
			entropy_token_push (_pattern, _collector),
		
		TokenPattern::Atom (_pattern) => {
			let _entropy = entropy_atom (_pattern) ?;
			_collector.multiply (&_entropy)
		}
		
		TokenPattern::Sequence (_patterns, _separator) => {
			for _pattern in _patterns.iter () {
				entropy_token_push (_pattern, _collector) ?;
			}
			Ok (())
		}
		
		TokenPattern::Repeat (_pattern, _separator, _count) => {
			let _count = *_count;
			for _index in 0 .. _count {
				entropy_token_push (_pattern, _collector) ?;
			}
			Ok (())
		}
		
		TokenPattern::Empty =>
			Ok (()),
	}
}




pub fn entropy_atom (_pattern : impl AsRef<AtomPattern>) -> EntropyResult<Entropy> {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		
		AtomPattern::Separator (_separator) =>
			Ok (Entropy::none ()),
		
		AtomPattern::Constant (_text) =>
			Ok (Entropy::none ()),
		
		AtomPattern::Glyph (_pattern) => {
			entropy_glyph (_pattern)
		}
	}
}




pub fn entropy_glyph (_pattern : impl AsRef<GlyphPattern>) -> EntropyResult<Entropy> {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		
		GlyphPattern::Set (_patterns) =>
			Ok (Entropy::for_choice (_patterns.len ())),
		
		GlyphPattern::Integer (_lower, _upper, _format) => {
			let (_lower, _upper) = (*_lower, *_upper);
			if _lower > _upper {
				fail! (0xb6347bbc);
			}
			let _delta = u128::abs_diff (_lower, _upper) + 1;
			if _delta > (usize::MAX as u128) {
				fail! (0x2a1640d7);
			}
			Ok (Entropy::for_choice (_delta as usize))
		}
	}
}

