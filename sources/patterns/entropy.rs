

use crate::prelude::*;


use ::z_tokens_runtime::crates::num_bigint::{
		BigUint,
	};

use ::z_tokens_runtime::crates::num_integer::{
		Integer as _,
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
	accurate : bool,
	timestamp : bool,
}


impl Entropy {
	
	pub fn none () -> Self {
		Self {
				accumulator : BigUint::zero (),
				accumulator_log2 : 0.0,
				accurate : true,
				timestamp : false,
			}
	}
	
	pub fn timestamp () -> Self {
		let mut _self = Self::none ();
		_self.timestamp = true;
		_self
	}
	
	pub fn for_bits (_count : usize) -> Self {
		if _count == 0 {
			return Self::none ();
		}
		Self {
				accumulator : BigUint::one () << _count,
				accumulator_log2 : _count as f64,
				accurate : true,
				timestamp : false,
			}
	}
	
	pub fn for_set (_count : u128) -> Self {
		if _count == 0 {
			return Self::none ();
		}
		Self {
				accumulator : BigUint::from (_count),
				accumulator_log2 : (_count as f64) .log2 (),
				accurate : true,
				timestamp : false,
			}
	}
	
	pub fn for_set_repeat (_count : u128, _repeats : u128) -> Self {
		if (_count == 0) || (_repeats == 0) {
			return Self::none ();
		}
		if _repeats == 1 {
			return Self::for_set (_count);
		}
		Self {
				accumulator : BigUint::from (_count) .pow (_repeats),
				accumulator_log2 : ((_count as f64) * (_repeats as f64)) .log2 (),
				accurate : true,
				timestamp : false,
			}
	}
	
	pub fn for_permutation (_count : u128, _subset : u128) -> EntropyResult<Self> {
		// FIXME:  Verify correctness!
		if _subset > _count {
			fail! (0x0d457506);
		}
		if (_count == 0) || (_subset == 0) {
			return Ok (Self::none ());
		}
		let mut _accumulator = BigUint::one ();
		let mut _accumulator_log2 = 0.0;
		let _factor_start = max (2, (_count - _subset) + 1);
		for _factor in _factor_start ..= _count {
			_accumulator *= _factor;
			_accumulator_log2 += (_factor as f64) .log2 ();
		}
		let _self = Self {
				accumulator : _accumulator,
				accumulator_log2 : _accumulator_log2,
				accurate : true,
				timestamp : false,
			};
		Ok (_self)
	}
	
	pub fn multiply_by (&mut self, _factor : u128) -> EntropyResult {
		if _factor == 0 {
			fail! (0xa0097d3d);
		}
		if self.accumulator.is_zero () || (_factor == 1) {
			return Ok (());
		}
		self.accumulator *= _factor;
		self.accumulator_log2 += (_factor as f64) .log2 ();
		Ok (())
	}
	
	pub fn divide_by (&mut self, _divisor : u128) -> EntropyResult {
		if _divisor == 0 {
			fail! (0x5be4a3df);
		}
		if self.accumulator.is_zero () || (_divisor == 1) {
			return Ok (());
		}
		let (_quotient, _reminder) = self.accumulator.div_rem (& BigUint::from (_divisor));
		if ! _quotient.is_zero () {
			self.accumulator = _quotient;
			self.accumulator_log2 -= (_divisor as f64) .log2 ();
		} else {
			self.accumulator = BigUint::zero ();
			self.accumulator_log2 = 0.0;
		}
		if ! _reminder.is_zero () {
			self.accurate = false;
		}
		Ok (())
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
		if ! _other.accurate {
			self.accurate = false;
		}
		if _other.timestamp {
			self.timestamp = true;
		}
		Ok (())
	}
	
	pub fn addition (&mut self, _other : &Entropy) -> EntropyResult {
		if self.accumulator.is_zero () {
			self.accumulator = _other.accumulator.clone ();
			self.accumulator_log2 = _other.accumulator_log2;
		} else if _other.accumulator.is_zero () {
			// NOP
		} else if self.accumulator == _other.accumulator {
			self.accumulator <<= 1;
			self.accumulator_log2 += 1.0;
		} else {
			self.accumulator += &_other.accumulator;
			// NOTE:  =>  <https://cdsmithus.medium.com/the-logarithm-of-a-sum-69dd76199790>
			let x = f64::max (self.accumulator_log2, _other.accumulator_log2);
			let y = f64::min (self.accumulator_log2, _other.accumulator_log2);
			let s = x + f64::log2 (1.0 + f64::exp2 (y - x));
			self.accumulator_log2 = s;
		}
		if ! _other.accurate {
			self.accurate = false;
		}
		if _other.timestamp {
			self.timestamp = true;
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
		if ! self.accurate {
			return (self.bits (), false);
		}
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
	
	pub fn is_timestamp (&self) -> bool {
		self.timestamp
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
		
		TokenPattern::Tagged (_pattern, _) => {
			entropy_token_push (_pattern, _collector)
		}
		
		TokenPattern::Atom (_pattern) => {
			let _entropy = entropy_atom (_pattern) ?;
			_collector.multiply (&_entropy) ?;
			Ok (())
		}
		
		TokenPattern::Sequence (_patterns, _separator) => {
			for _pattern in _patterns.iter () {
				entropy_token_push (_pattern, _collector) ?;
			}
			Ok (())
		}
		
		TokenPattern::Permutation (_patterns, _separator) => {
			let _count = _patterns.len ();
			{
				let _entropy_permutation = Entropy::for_permutation (_count as u128, _count as u128) ?;
				_collector.multiply (&_entropy_permutation) ?;
			}
			for _pattern in _patterns.iter () {
				entropy_token_push (_pattern, _collector) ?;
			}
			Ok (())
		}
		
		TokenPattern::PermutationPartial (_patterns, _separator, _subset) => {
			let _count = _patterns.len ();
			let _subset = *_subset;
			if _subset > _count {
				fail! (0x0c280a6f);
			}
			{
				let _entropy_permutation = Entropy::for_permutation (_count as u128, _subset as u128) ?;
				_collector.multiply (&_entropy_permutation) ?;
			}
			// FIXME:  This is just an estimate!
			let _entropy_avg = {
					let mut _entropy_avg = Entropy::none ();
					_entropy_avg.accurate = false;
					for _pattern in _patterns.iter () {
						let mut _entropy = Entropy::none ();
						entropy_token_push (_pattern, &mut _entropy) ?;
						_entropy_avg.addition (&_entropy) ?;
					}
					_entropy_avg.divide_by (_count as u128) ?;
					_entropy_avg
				};
			for _ in 0 .. _subset {
				_collector.multiply (&_entropy_avg) ?;
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
		
		TokenPattern::Choice (_patterns) => {
			let mut _entropy_sum = Entropy::none ();
			for _pattern in _patterns.iter () {
				let mut _entropy = Entropy::none ();
				entropy_token_push (_pattern, &mut _entropy) ?;
				_entropy_sum.addition (&_entropy) ?;
			}
			_collector.multiply (&_entropy_sum) ?;
			Ok (())
		}
		
		TokenPattern::Empty => {
			Ok (())
		}
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
			Ok (Entropy::for_set (_patterns.len () as u128)),
		
		GlyphPattern::Integer (_lower, _upper, _format) => {
			let (_lower, _upper) = (*_lower, *_upper);
			if _lower > _upper {
				fail! (0xb6347bbc);
			}
			let _delta = u128::abs_diff (_lower, _upper);
			if _delta == u128::MAX {
				Ok (Entropy::for_bits (128))
			} else {
				Ok (Entropy::for_set (_delta + 1))
			}
		}
		
		GlyphPattern::Bytes (_size, _format) =>
			Ok (Entropy::for_set_repeat (256, *_size as u128)),
		
		GlyphPattern::Timestamp (_) =>
			Ok (Entropy::timestamp ()),
	}
}

