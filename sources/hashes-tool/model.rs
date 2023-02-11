

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;




define_error! (pub AlgorithmError, result : AlgorithmResult);




pub const OUTPUT_SIZE_MAX : usize = 1024 * 1024 * 1024;




#[ derive (Copy, Clone) ]
#[ allow (non_camel_case_types) ]
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
	
	Argon2,
	Argon2d,
	Argon2i,
	Argon2id,
	
	SipHash,
	SeaHash,
	HighwayHash,
	
	XxHash,
	Xxh3,
	
	Djb2,
	SDBM,
	FNV1a,
	
	CRC,
	CRC32C,
	
}


#[ derive (Copy, Clone) ]
#[ allow (non_camel_case_types) ]
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
	
	Argon2d,
	Argon2i,
	Argon2id,
	
	SipHash_64,
	SipHash_128,
	
	SeaHash,
	
	HighwayHash_64,
	HighwayHash_128,
	HighwayHash_256,
	
	XxHash_32,
	XxHash_64,
	
	Xxh3_64,
	Xxh3_128,
	
	Djb2,
	SDBM,
	FNV1a_32,
	FNV1a_64,
	
	CRC8,
	CRC16,
	CRC32,
	CRC32C,
	CRC64,
	
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
			
			Family::Argon2 |
			Family::Argon2d |
			Family::Argon2i |
			Family::Argon2id => (4, OUTPUT_SIZE_MAX, 32),
			
			Family::SipHash => (1, 128 / 8, 64 / 8),
			Family::SeaHash => (1, 64 / 8, 64 / 8),
			Family::HighwayHash => (1, 256 / 8, 64 / 8),
			
			Family::XxHash => (1, 64 / 8, 64 / 8),
			Family::Xxh3 => (1, 128 / 8, 64 / 8),
			
			Family::Djb2 => (1, 32 / 8, 32 / 8),
			Family::SDBM => (1, 32 / 8, 32 / 8),
			Family::FNV1a => (1, 64 / 8, 32 / 8),
			
			Family::CRC => (1, 64 / 8, 32 / 8),
			Family::CRC32C => (1, 32 / 8, 32 / 8),
			
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
			
			Family::Argon2 => Ok (Algorithm::Argon2id),
			Family::Argon2d => Ok (Algorithm::Argon2d),
			Family::Argon2i => Ok (Algorithm::Argon2i),
			Family::Argon2id => Ok (Algorithm::Argon2id),
			
			Family::SipHash if _output_size <= 64 / 8 => Ok (Algorithm::SipHash_64),
			Family::SipHash if _output_size <= 128 / 8 => Ok (Algorithm::SipHash_128),
			
			Family::SeaHash if _output_size <= 64 / 8 => Ok (Algorithm::SeaHash),
			
			Family::HighwayHash if _output_size <= 64 / 8 => Ok (Algorithm::HighwayHash_64),
			Family::HighwayHash if _output_size <= 128 / 8 => Ok (Algorithm::HighwayHash_128),
			Family::HighwayHash if _output_size <= 256 / 8 => Ok (Algorithm::HighwayHash_256),
			
			Family::XxHash if _output_size <= 32 / 8 => Ok (Algorithm::XxHash_32),
			Family::XxHash if _output_size <= 64 / 8 => Ok (Algorithm::XxHash_64),
			
			Family::Xxh3 if _output_size <= 64 / 8 => Ok (Algorithm::Xxh3_64),
			Family::Xxh3 if _output_size <= 128 / 8 => Ok (Algorithm::Xxh3_128),
			
			Family::Djb2 if _output_size <= 32 / 8 => Ok (Algorithm::Djb2),
			Family::SDBM if _output_size <= 32 / 8 => Ok (Algorithm::SDBM),
			
			Family::FNV1a if _output_size <= 32 / 8 => Ok (Algorithm::FNV1a_32),
			Family::FNV1a if _output_size <= 64 / 8 => Ok (Algorithm::FNV1a_64),
			
			Family::CRC if _output_size <= 8 / 8 => Ok (Algorithm::CRC8),
			Family::CRC if _output_size <= 16 / 8 => Ok (Algorithm::CRC16),
			Family::CRC if _output_size <= 32 / 8 => Ok (Algorithm::CRC32),
			Family::CRC if _output_size <= 64 / 8 => Ok (Algorithm::CRC64),
			Family::CRC32C if _output_size <= 32 / 8 => Ok (Algorithm::CRC32C),
			
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


