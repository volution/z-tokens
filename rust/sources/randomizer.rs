

use crate::prelude::*;


use ::rand::{
		
		self as rand,
		
		RngCore,
		CryptoRng,
		SeedableRng,
		
		rngs::OsRng,
		rngs::StdRng,
		
	};




define_error! (pub RandomError, result : RandomResult);




pub trait Randomizer {
	
	fn choose (&mut self, _count : usize) -> RandomResult<usize>;
	
	fn bytes (&mut self, _buffer : &mut [u8]) -> RandomResult;
}




pub struct RngRandomizer <Core : RngCore> {
	rng : Core,
}


impl <Core : RngCore> Randomizer for RngRandomizer<Core> {
	
	fn choose (&mut self, _count : usize) -> RandomResult<usize> {
		Ok (random_usize_range_from (.. _count, &mut self.rng))
	}
	
	fn bytes (&mut self, _buffer : &mut [u8]) -> RandomResult {
		self.rng.try_fill_bytes (_buffer) .else_wrap (0xbee63cdc)
	}
}


impl RngRandomizer <OsRng> {
	
	pub fn from_os () -> RandomResult<Self> {
		let _self = Self {
				rng : OsRng,
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
			};
		Ok (_self)
	}
}

