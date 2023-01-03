

use crate::prelude::*;


use ::rand::{
		
		self as rand,
		
		RngCore,
		CryptoRng,
		SeedableRng,
		
		rngs::OsRng,
		rngs::StdRng,
		
	};


use ::chrono;




define_error! (pub RandomError, result : RandomResult);




pub trait Randomizer {
	
	fn choose (&mut self, _count : usize) -> RandomResult<usize>;
	
	fn bytes (&mut self, _buffer : &mut [u8]) -> RandomResult;
	
	fn timestamp (&mut self) -> RandomResult<u128>;
}




pub struct RngRandomizer <Core : RngCore> {
	rng : Core,
	timestamp : Option<u128>,
}


impl <Core : RngCore> Randomizer for RngRandomizer<Core> {
	
	fn choose (&mut self, _count : usize) -> RandomResult<usize> {
		Ok (random_usize_range_from (.. _count, &mut self.rng))
	}
	
	fn bytes (&mut self, _buffer : &mut [u8]) -> RandomResult {
		self.rng.try_fill_bytes (_buffer) .else_wrap (0xbee63cdc)
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


impl RngRandomizer <OsRng> {
	
	pub fn from_os () -> RandomResult<Self> {
		let _self = Self {
				rng : OsRng,
				timestamp : None,
			};
		Ok (_self)
	}
}


impl RngRandomizer <StdRng> {
	
	pub fn for_testing () -> RandomResult<Self> {
		Self::for_testing_with_seed (0)
	}
	
	pub fn for_testing_with_seed (_seed : u64) -> RandomResult<Self> {
		let _self = Self {
				rng : StdRng::seed_from_u64 (_seed),
				timestamp : None,
			};
		Ok (_self)
	}
}

