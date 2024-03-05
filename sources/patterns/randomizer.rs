

use crate::prelude::*;


use ::z_tokens_runtime_random::{
		
		crates::vrl_random::random_usize_range_from,
		crates::vrl_random::random_u128_from,
		
		crates::rand::RngCore,
		crates::rand::SeedableRng,
		crates::rand_chacha::ChaCha20Rng,
		
		crates::rand::rngs::OsRng,
		crates::rand::rngs::StdRng,
		
	};


use ::z_tokens_runtime_hashes::{
		
		crates::blake3::hash as blake3_hash,
		
	};


use ::z_tokens_runtime::crates::chrono;




define_error! (pub RandomError, result : RandomResult);




pub trait Randomizer {
	
	fn choose (&mut self, _count : usize) -> RandomResult<usize>;
	
	fn bytes (&mut self, _buffer : &mut [u8]) -> RandomResult;
	
	fn value_u128 (&mut self) -> RandomResult<u128>;
	
	fn timestamp (&mut self) -> RandomResult<u128>;
	
	fn reset (&mut self) -> RandomResult;
	
	fn advance (&mut self) -> RandomResult;
}




pub struct OsRandomizer {
	delegate : RngRandomizer<OsRng>,
}


pub struct SeedRandomizer {
	delegate : RngRandomizer<StdRng>,
	seed : u64,
}


pub struct ChaCha20Randomizer {
	delegate : RngRandomizer<ChaCha20Rng>,
	seed : [u8; 32],
}


pub struct RngRandomizer <Core : RngCore> {
	rng : Core,
	timestamp : Option<u128>,
}




impl OsRandomizer {
	
	pub fn from_os () -> RandomResult<Self> {
		let _delegate = RngRandomizer {
				rng : OsRng,
				timestamp : None,
			};
		let _self = Self {
				delegate : _delegate,
			};
		Ok (_self)
	}
}


impl Randomizer for OsRandomizer {
	
	fn choose (&mut self, _count : usize) -> RandomResult<usize> {
		self.delegate.choose (_count)
	}
	
	fn bytes (&mut self, _buffer : &mut [u8]) -> RandomResult {
		self.delegate.bytes (_buffer)
	}
	
	fn value_u128 (&mut self) -> RandomResult<u128> {
		self.delegate.value_u128 ()
	}
	
	fn timestamp (&mut self) -> RandomResult<u128> {
		self.delegate.timestamp ()
	}
	
	fn reset (&mut self) -> RandomResult {
		self.delegate.timestamp = None;
		Ok (())
	}
	
	fn advance (&mut self) -> RandomResult {
		Ok (())
	}
}




impl SeedRandomizer {
	
	pub fn for_testing () -> RandomResult<Self> {
		Self::for_testing_with_seed (0)
	}
	
	pub fn for_testing_with_seed (_seed : u64) -> RandomResult<Self> {
		let _delegate = RngRandomizer {
				rng : StdRng::seed_from_u64 (_seed),
				timestamp : Some (1640873716789789789),
			};
		let _self = Self {
				delegate : _delegate,
				seed : _seed,
			};
		Ok (_self)
	}
}


impl Randomizer for SeedRandomizer {
	
	fn choose (&mut self, _count : usize) -> RandomResult<usize> {
		self.delegate.choose (_count)
	}
	
	fn bytes (&mut self, _buffer : &mut [u8]) -> RandomResult {
		self.delegate.bytes (_buffer)
	}
	
	fn value_u128 (&mut self) -> RandomResult<u128> {
		self.delegate.value_u128 ()
	}
	
	fn timestamp (&mut self) -> RandomResult<u128> {
		self.delegate.timestamp ()
	}
	
	fn reset (&mut self) -> RandomResult {
		self.delegate.rng = StdRng::seed_from_u64 (self.seed);
		Ok (())
	}
	
	fn advance (&mut self) -> RandomResult {
		Ok (())
	}
}




impl ChaCha20Randomizer {
	
	pub fn new_with_key (_key : &[u8]) -> RandomResult<Self> {
		let _seed = blake3_hash (_key) .as_bytes () .clone ();
		Self::new_with_seed (_seed)
	}
	
	pub fn new_with_seed (_seed : [u8; 32]) -> RandomResult<Self> {
		let _delegate = RngRandomizer {
				rng : ChaCha20Rng::from_seed (_seed),
				timestamp : None,
			};
		let _self = Self {
				delegate : _delegate,
				seed : _seed,
			};
		Ok (_self)
	}
}


impl Randomizer for ChaCha20Randomizer {
	
	fn choose (&mut self, _count : usize) -> RandomResult<usize> {
		self.delegate.choose (_count)
	}
	
	fn bytes (&mut self, _buffer : &mut [u8]) -> RandomResult {
		self.delegate.bytes (_buffer)
	}
	
	fn value_u128 (&mut self) -> RandomResult<u128> {
		self.delegate.value_u128 ()
	}
	
	fn timestamp (&mut self) -> RandomResult<u128> {
		self.delegate.timestamp ()
	}
	
	fn reset (&mut self) -> RandomResult {
		self.delegate.rng = ChaCha20Rng::from_seed (self.seed);
		Ok (())
	}
	
	fn advance (&mut self) -> RandomResult {
		Ok (())
	}
}




impl <Core : RngCore> RngRandomizer<Core> {
	
	fn choose (&mut self, _count : usize) -> RandomResult<usize> {
		Ok (random_usize_range_from (.. _count, &mut self.rng))
	}
	
	fn bytes (&mut self, _buffer : &mut [u8]) -> RandomResult {
		self.rng.try_fill_bytes (_buffer) .else_wrap (0xbee63cdc)
	}
	
	fn value_u128 (&mut self) -> RandomResult<u128> {
		Ok (random_u128_from (&mut self.rng))
	}
	
	fn timestamp (&mut self) -> RandomResult<u128> {
		if let Some (_timestamp) = self.timestamp {
			Ok (_timestamp)
		} else {
			let _time = chrono::Utc::now () .naive_utc ();
			let _seconds = _time.timestamp () as u128;
			let _subsec_nanoseconds = _time.timestamp_subsec_nanos () as u128;
			let _timestamp = (_seconds * 1_000_000_000) + _subsec_nanoseconds;
			self.timestamp = Some (_timestamp);
			Ok (_timestamp)
		}
	}
}


