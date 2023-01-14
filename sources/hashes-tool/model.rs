

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;




define_error! (pub AlgorithmError, result : AlgorithmResult);




pub const OUTPUT_SIZE_MAX : usize = 1024 * 1024 * 1024;




#[ derive (Copy, Clone) ]
pub enum Family {
	
	MD5,
	
	SHA1,
	SHA2,
	SHA3,
	GitSHA1,
	
	Blake3,
	
	Blake2,
	Blake2s,
	Blake2b,
}


#[ derive (Copy, Clone) ]
pub enum Algorithm {
	
	MD5,
	
	SHA1,
	
	SHA2_224,
	SHA2_256,
	SHA2_384,
	SHA2_512,
	
	SHA3_224,
	SHA3_256,
	SHA3_384,
	SHA3_512,
	
	GitSHA1,
	
	Blake2s,
	Blake2b,
	Blake3,
}




impl FromStr for Family {
	
	type Err = AlgorithmError;
	
	fn from_str (_string : &str) -> AlgorithmResult<Self> {
		fail! (0x1c3de566);
	}
}




impl Family {
	
	fn output_size_range (&self) -> (usize, usize, usize) {
		match self {
			Family::MD5 => (1, 128 / 8, 128 / 8),
			Family::SHA1 |
			Family::GitSHA1 => (1, 160 / 8, 160 / 8),
			Family::SHA2 => (1, 512 / 8, 256 / 8),
			Family::SHA3 => (1, 512 / 8, 256 / 8),
			Family::Blake2 => (1, 64, 64),
			Family::Blake2s => (1, 32, 32),
			Family::Blake2b => (1, 64, 64),
			Family::Blake3 => (1, OUTPUT_SIZE_MAX, 256 / 8),
		}
	}
	
	fn algorithm_for_output_size (&self, _output_size : usize) -> AlgorithmResult<Algorithm> {
		match self {
			
			Family::MD5 if _output_size <= 128 / 8 => Ok (Algorithm::MD5),
			
			Family::SHA1 if _output_size <= 160 / 8 => Ok (Algorithm::SHA1),
			
			Family::SHA2 if _output_size <= 224 / 8 => Ok (Algorithm::SHA2_224),
			Family::SHA2 if _output_size <= 256 / 8 => Ok (Algorithm::SHA2_256),
			Family::SHA2 if _output_size <= 384 / 8 => Ok (Algorithm::SHA2_384),
			Family::SHA2 if _output_size <= 512 / 8 => Ok (Algorithm::SHA2_512),
			
			Family::SHA3 if _output_size <= 224 / 8 => Ok (Algorithm::SHA3_224),
			Family::SHA3 if _output_size <= 256 / 8 => Ok (Algorithm::SHA3_256),
			Family::SHA3 if _output_size <= 384 / 8 => Ok (Algorithm::SHA3_384),
			Family::SHA3 if _output_size <= 512 / 8 => Ok (Algorithm::SHA3_512),
			
			Family::Blake2 if _output_size <= 32 => Ok (Algorithm::Blake2s),
			Family::Blake2s if _output_size <= 32 => Ok (Algorithm::Blake2s),
			Family::Blake2 if _output_size <= 64 => Ok (Algorithm::Blake2b),
			Family::Blake2b if _output_size <= 64 => Ok (Algorithm::Blake2b),
			
			Family::Blake3 => Ok (Algorithm::Blake3),
			
			_ =>
				fail! (0x36b18fd0),
		}
	}
}




pub fn choose_algorithm (_family : Option<Family>, _output_size : Option<usize>) -> AlgorithmResult<(Algorithm, usize)> {
	
	let _family = _family.unwrap_or (Family::Blake3);
	
	let (_output_minimum, _output_maximum, _output_default) = _family.output_size_range ();
	
	let _output_size = _output_size.unwrap_or (_output_default);
	
	if _output_size < _output_minimum {
		fail! (0xbc5e1f69);
	}
	if _output_size > _output_maximum {
		fail! (0x2166c769);
	}
	
	let _algorithm = _family.algorithm_for_output_size (_output_size) ?;
	
	Ok ((_algorithm, _output_size))
}


