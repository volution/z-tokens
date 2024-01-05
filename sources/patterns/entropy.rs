

use crate::prelude::*;


use ::z_tokens_runtime::crates::num_bigint::{
		BigUint,
	};

use ::z_tokens_runtime::crates::num_traits::{
		Zero as _,
		One as _,
		ToPrimitive as _,
		Pow as _,
	};




define_error! (pub EntroyError, result : EntropyResult);




pub struct Entropy {
	accumulator : BigUint,
	accumulator_log2 : f64,
}


impl Entropy {
	
	pub fn none () -> Self {
		Self {
				accumulator : BigUint::zero (),
				accumulator_log2 : 0.0,
			}
	}
	
	pub fn for_bits (_count : usize) -> Self {
		if _count == 0 {
			return Self::none ();
		}
		Self {
				accumulator : BigUint::one () << _count,
				accumulator_log2 : _count as f64,
			}
	}
	
	pub fn for_choice (_count : u128) -> Self {
		if _count == 0 {
			return Self::none ();
		}
		Self {
				accumulator : BigUint::from (_count),
				accumulator_log2 : (_count as f64) .log2 (),
			}
	}
	
	pub fn for_choice_repeat (_count : usize, _repeats : usize) -> Self {
		if (_count == 0) || (_repeats == 0) {
			return Self::none ();
		}
		if _repeats == 1 {
			return Self::for_choice (_count as u128);
		}
		Self {
				accumulator : BigUint::from (_count) .pow (_repeats),
				accumulator_log2 : ((_count as f64) * (_repeats as f64)) .log2 (),
			}
	}
	
	pub fn multiply (&mut self, _other : &Entropy) -> EntropyResult {
		if self.accumulator.is_zero () {
			self.accumulator = _other.accumulator.clone ();
			self.accumulator_log2 = _other.accumulator_log2;
		} else if _other.accumulator.is_zero () {
			// NOP
		} else {
			self.accumulator *= &_other.accumulator;
			self.accumulator_log2 += _other.accumulator_log2;
		}
		Ok (())
	}
	
	pub fn bits (&self) -> f64 {
		if self.accumulator.is_zero () {
			return 0.0;
		}
		let _value = self.accumulator.to_f64 ();
		let _bits = if let Some (_value) = _value {
				if _value.is_finite () {
					Some (_value.log2 ())
				} else {
					None
				}
			} else {
				None
			};
		let _bits = if let Some (_bits) = _bits {
				_bits
			} else {
				self.accumulator_log2
			};
		debug_assert! (! _bits.is_nan (), "[0375f69f]");
		debug_assert! (_bits.is_finite (), "[86036975]");
		_bits
	}
	
	pub fn bits_exact (&self) -> (f64, bool) {
		if self.accumulator.is_zero () {
			return (0.0, true);
		}
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
}




pub fn entropy_token (_pattern : impl AsRef<TokenPattern>) -> EntropyResult<Entropy> {
	
	let mut _entropy = Entropy::none ();
	
	entropy_token_push (_pattern, &mut _entropy) ?;
	
	Ok (_entropy)
}


pub fn entropy_token_push (_pattern : impl AsRef<TokenPattern>, _collector : &mut Entropy) -> EntropyResult {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		
		TokenPattern::Tagged (_pattern, _) =>
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
			Ok (Entropy::for_choice (_patterns.len () as u128)),
		
		GlyphPattern::Integer (_lower, _upper, _format) => {
			let (_lower, _upper) = (*_lower, *_upper);
			if _lower > _upper {
				fail! (0xb6347bbc);
			}
			let _delta = u128::abs_diff (_lower, _upper);
			if _delta == u128::MAX {
				Ok (Entropy::for_bits (128))
			} else {
				Ok (Entropy::for_choice (_delta + 1))
			}
		}
		
		GlyphPattern::Bytes (_size, _format) =>
			Ok (Entropy::for_choice_repeat (256, *_size)),
		
		GlyphPattern::Timestamp (_) =>
			Ok (Entropy::none ()),
	}
}

