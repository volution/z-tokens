

use crate::prelude::*;




define_error! (pub EntroyError, result : EntropyResult);




pub struct Entropy {
	accumulator_log2 : f64,
	accumulator : u128,
}


impl Entropy {
	
	pub fn none () -> Self {
		Self {
				accumulator_log2 : 0.0,
				accumulator : 0,
			}
	}
	
	pub fn for_choice (_count : usize) -> Self {
		Self {
				accumulator_log2 : 0.0,
				accumulator : _count as u128,
			}
	}
	
	pub fn multiply (&mut self, _other : &Entropy) -> EntropyResult {
		if (self.accumulator_log2 != 0.0) || (_other.accumulator_log2 != 0.0) {
			self.accumulator_log2 = self.bits () + _other.bits ();
			self.accumulator = 0;
		} else {
			assert! (self.accumulator_log2 == 0.0);
			assert! (_other.accumulator_log2 == 0.0);
			if self.accumulator == 0 {
				self.accumulator = _other.accumulator;
			} else if _other.accumulator == 0 {
				// NOP
			} else if (self.accumulator < u64::MAX as u128) && (_other.accumulator < u64::MAX as u128) {
				self.accumulator *= _other.accumulator;
			} else {
				self.accumulator_log2 = self.bits () + _other.bits ();
				self.accumulator = 0;
			}
		}
		Ok (())
	}
	
	pub fn bits (&self) -> f64 {
		if self.accumulator != 0 {
			assert! (self.accumulator_log2 == 0.0);
			(self.accumulator as f64).log2 ()
		} else {
			assert! (self.accumulator == 0);
			self.accumulator_log2
		}
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
	}
}

