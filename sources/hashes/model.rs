

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;
use ::z_tokens_runtime_flags::*;




define_error! (pub AlgorithmError, result : AlgorithmResult);




pub const INPUT_SIZE_MAX : usize = 1024 * 1024 * 1024;
pub const OUTPUT_SIZE_MAX : usize = 1024 * 1024 * 1024;
pub const PASSWORD_SIZE_MAX : usize = 4096;




#[ derive (Clone) ]
pub struct OutputParameters {
	pub hash_size : usize,
	pub truncate_size : usize,
	pub discard_right : bool,
	pub reversed : bool,
}




#[ derive (Debug) ]
#[ derive (Copy, Clone) ]
#[ allow (non_camel_case_types) ]
pub enum Family {
	
	MD5,
	
	SHA1,
	
	SHA2,
	SHA2_224,
	SHA2_256,
	SHA2_384,
	SHA2_512,
	
	SHA3,
	SHA3_224,
	SHA3_256,
	SHA3_384,
	SHA3_512,
	
	Shake_128,
	Shake_256,
	
	GitSHA1,
	GitSHA2,
	
	Blake2,
	Blake2s,
	Blake2b,
	
	Blake3,
	
	SipHash,
	SipHash_64,
	SipHash_128,
	
	SeaHash,
	
	HighwayHash,
	HighwayHash_64,
	HighwayHash_128,
	HighwayHash_256,
	
	XxHash,
	XxHash_32,
	XxHash_64,
	
	Xxh3,
	Xxh3_64,
	Xxh3_128,
	
	Djb2,
	SDBM,
	
	FNV1a,
	FNV1a_32,
	FNV1a_64,
	
	CRC,
	CRC8,
	CRC16,
	CRC32,
	CRC32C,
	CRC64,
	
	Adler,
	Adler32,
	
	Scrypt,
	
	Argon2,
	Argon2d,
	Argon2i,
	Argon2id,
	
}




#[ derive (Debug) ]
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
	
	Shake_128,
	Shake_256,
	
	GitSHA1,
	GitSHA2,
	
	Blake2s,
	Blake2b,
	
	Blake3,
	
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
	
	Adler32,
	
	Scrypt,
	
	Argon2d,
	Argon2i,
	Argon2id,
	
}




impl Family {
	
	
	pub fn hash_size_range (&self) -> (usize, usize, usize) {
		match self {
			
			Family::MD5 => (1, 128 / 8, 128 / 8),
			
			Family::SHA1 => (1, 160 / 8, 160 / 8),
			
			Family::SHA2 =>  (1, 512 / 8, 256 / 8),
			Family::SHA2_224 => (1, 224 / 8, 224 / 8),
			Family::SHA2_256 => (1, 256 / 8, 256 / 8),
			Family::SHA2_384 => (1, 384 / 8, 384 / 8),
			Family::SHA2_512 => (1, 512 / 8, 512 / 8),
			
			Family::SHA3 => (1, 512 / 8, 256 / 8),
			Family::SHA3_224 => (1, 224 / 8, 224 / 8),
			Family::SHA3_256 => (1, 256 / 8, 256 / 8),
			Family::SHA3_384 => (1, 384 / 8, 384 / 8),
			Family::SHA3_512 => (1, 512 / 8, 512 / 8),
			
			Family::Shake_128 => (1, OUTPUT_SIZE_MAX, 128 / 8),
			Family::Shake_256 => (1, OUTPUT_SIZE_MAX, 256 / 8),
			
			Family::GitSHA1 => (1, 160 / 8, 160 / 8),
			Family::GitSHA2 => (1, 256 / 8, 256 / 8),
			
			Family::Blake2 => (1, 64, 64),
			Family::Blake2s => (1, 32, 32),
			Family::Blake2b => (1, 64, 64),
			
			Family::Blake3 => (1, OUTPUT_SIZE_MAX, 256 / 8),
			
			Family::SipHash => (1, 128 / 8, 64 / 8),
			Family::SipHash_64 => (1, 64 / 8, 64 / 8),
			Family::SipHash_128 => (1, 128 / 8, 128 / 8),
			
			Family::SeaHash => (1, 64 / 8, 64 / 8),
			
			Family::HighwayHash => (1, 256 / 8, 64 / 8),
			Family::HighwayHash_64 => (1, 64 / 8, 64 / 8),
			Family::HighwayHash_128 => (1, 128 / 8, 128 / 8),
			Family::HighwayHash_256 => (1, 256 / 8, 256 / 8),
			
			Family::XxHash => (1, 64 / 8, 64 / 8),
			Family::XxHash_32 => (1, 32 / 8, 32 / 8),
			Family::XxHash_64 => (1, 64 / 8, 64 / 8),
			
			Family::Xxh3 => (1, 128 / 8, 64 / 8),
			Family::Xxh3_64 => (1, 64 / 8, 64 / 8),
			Family::Xxh3_128 => (1, 128 / 8, 128 / 8),
			
			Family::Djb2 => (1, 32 / 8, 32 / 8),
			Family::SDBM => (1, 32 / 8, 32 / 8),
			
			Family::FNV1a => (1, 64 / 8, 32 / 8),
			Family::FNV1a_32 => (1, 32 / 8, 32 / 8),
			Family::FNV1a_64 => (1, 64 / 8, 64 / 8),
			
			Family::CRC => (1, 64 / 8, 32 / 8),
			Family::CRC8 => (1, 8 / 8, 8 / 8),
			Family::CRC16 => (1, 16 / 8, 16 / 8),
			Family::CRC32 |
			Family::CRC32C => (1, 32 / 8, 32 / 8),
			Family::CRC64 => (1, 64 / 8, 64 / 8),
			
			Family::Adler |
			Family::Adler32 => (1, 32 / 8, 32 / 8),
			
			Family::Scrypt => (4, PASSWORD_SIZE_MAX, 32),
			
			Family::Argon2 |
			Family::Argon2d |
			Family::Argon2i |
			Family::Argon2id => (4, PASSWORD_SIZE_MAX, 32),
			
		}
	}
	
	
	pub fn algorithm_for_hash_size (&self, _hash_size : usize) -> AlgorithmResult<Algorithm> {
		#[ allow (unused_parens) ]
		match self {
			
			Family::MD5 if _hash_size <= 128 / 8 => Ok (Algorithm::MD5),
			Family::MD5 => fail! (0x7ffeffd3),
			
			Family::SHA1 if _hash_size <= 160 / 8 => Ok (Algorithm::SHA1),
			Family::SHA1 => fail! (0xe2bba0e0),
			
			(Family::SHA2 | Family::SHA2_224) if _hash_size <= 224 / 8 => Ok (Algorithm::SHA2_224),
			(Family::SHA2 | Family::SHA2_256) if _hash_size <= 256 / 8 => Ok (Algorithm::SHA2_256),
			(Family::SHA2 | Family::SHA2_384) if _hash_size <= 384 / 8 => Ok (Algorithm::SHA2_384),
			(Family::SHA2 | Family::SHA2_512) if _hash_size <= 512 / 8 => Ok (Algorithm::SHA2_512),
			(Family::SHA2 | Family::SHA2_224 | Family::SHA2_256 | Family::SHA2_384 | Family::SHA2_512) => fail! (0xbaab0fdf),
			
			(Family::SHA3 | Family::SHA3_224) if _hash_size <= 224 / 8 => Ok (Algorithm::SHA3_224),
			(Family::SHA3 | Family::SHA3_256) if _hash_size <= 256 / 8 => Ok (Algorithm::SHA3_256),
			(Family::SHA3 | Family::SHA3_384) if _hash_size <= 384 / 8 => Ok (Algorithm::SHA3_384),
			(Family::SHA3 | Family::SHA3_512) if _hash_size <= 512 / 8 => Ok (Algorithm::SHA3_512),
			(Family::SHA3 | Family::SHA3_224 | Family::SHA3_256 | Family::SHA3_384 | Family::SHA3_512) => fail! (0xfa6ca215),
			
			Family::Shake_128 if _hash_size <= OUTPUT_SIZE_MAX => Ok (Algorithm::Shake_128),
			Family::Shake_256 if _hash_size <= OUTPUT_SIZE_MAX => Ok (Algorithm::Shake_256),
			(Family::Shake_128 | Family::Shake_256) => fail! (0x3d8e25a8),
			
			Family::GitSHA1 if _hash_size <= 160 / 8 => Ok (Algorithm::GitSHA1),
			Family::GitSHA1 => fail! (0x8e8add74),
			Family::GitSHA2 if _hash_size <= 256 / 8 => Ok (Algorithm::GitSHA2),
			Family::GitSHA2 => fail! (0x85fd3470),
			
			(Family::Blake2 | Family::Blake2s) if _hash_size <= 32 => Ok (Algorithm::Blake2s),
			(Family::Blake2 | Family::Blake2b) if _hash_size <= 64 => Ok (Algorithm::Blake2b),
			(Family::Blake2 | Family::Blake2s | Family::Blake2b) => fail! (0x9cd36d05),
			
			Family::Blake3 if _hash_size <= OUTPUT_SIZE_MAX => Ok (Algorithm::Blake3),
			Family::Blake3 => fail! (0x88a2c2cb),
			
			(Family::SipHash | Family::SipHash_64) if _hash_size <= 64 / 8 => Ok (Algorithm::SipHash_64),
			(Family::SipHash | Family::SipHash_128) if _hash_size <= 128 / 8 => Ok (Algorithm::SipHash_128),
			(Family::SipHash | Family::SipHash_64 | Family::SipHash_128) => fail! (0xdf55d2f8),
			
			Family::SeaHash if _hash_size <= 64 / 8 => Ok (Algorithm::SeaHash),
			Family::SeaHash => fail! (0xba92317e),
			
			(Family::HighwayHash | Family::HighwayHash_64) if _hash_size <= 64 / 8 => Ok (Algorithm::HighwayHash_64),
			(Family::HighwayHash | Family::HighwayHash_128) if _hash_size <= 128 / 8 => Ok (Algorithm::HighwayHash_128),
			(Family::HighwayHash | Family::HighwayHash_256) if _hash_size <= 256 / 8 => Ok (Algorithm::HighwayHash_256),
			(Family::HighwayHash | Family::HighwayHash_64 | Family::HighwayHash_128 | Family::HighwayHash_256) => fail! (0xdf7f0a14),
			
			(Family::XxHash | Family::XxHash_32) if _hash_size <= 32 / 8 => Ok (Algorithm::XxHash_32),
			(Family::XxHash | Family::XxHash_64) if _hash_size <= 64 / 8 => Ok (Algorithm::XxHash_64),
			(Family::XxHash | Family::XxHash_32 | Family::XxHash_64) => fail! (0x99f883ef),
			
			(Family::Xxh3 | Family::Xxh3_64) if _hash_size <= 64 / 8 => Ok (Algorithm::Xxh3_64),
			(Family::Xxh3 | Family::Xxh3_128) if _hash_size <= 128 / 8 => Ok (Algorithm::Xxh3_128),
			(Family::Xxh3 | Family::Xxh3_64 | Family::Xxh3_128) => fail! (0x10f1551f),
			
			Family::Djb2 if _hash_size <= 32 / 8 => Ok (Algorithm::Djb2),
			Family::SDBM if _hash_size <= 32 / 8 => Ok (Algorithm::SDBM),
			(Family::Djb2 | Family::SDBM) => fail! (0x34e1a6df),
			
			(Family::FNV1a | Family::FNV1a_32) if _hash_size <= 32 / 8 => Ok (Algorithm::FNV1a_32),
			(Family::FNV1a | Family::FNV1a_64) if _hash_size <= 64 / 8 => Ok (Algorithm::FNV1a_64),
			(Family::FNV1a | Family::FNV1a_32 | Family::FNV1a_64) => fail! (0xe1307d7d),
			
			(Family::CRC | Family::CRC8) if _hash_size <= 8 / 8 => Ok (Algorithm::CRC8),
			(Family::CRC | Family::CRC16) if _hash_size <= 16 / 8 => Ok (Algorithm::CRC16),
			(Family::CRC | Family::CRC32) if _hash_size <= 32 / 8 => Ok (Algorithm::CRC32),
			(Family::CRC | Family::CRC64) if _hash_size <= 64 / 8 => Ok (Algorithm::CRC64),
			Family::CRC32C if _hash_size <= 32 / 8 => Ok (Algorithm::CRC32C),
			(Family::CRC | Family::CRC8 | Family::CRC16 | Family::CRC32 | Family::CRC32C | Family::CRC64) => fail! (0x9df52825),
			
			(Family::Adler | Family::Adler32) if _hash_size <= 32 / 8 => Ok (Algorithm::Adler32),
			(Family::Adler | Family::Adler32) => fail! (0x1c8456b5),
			
			Family::Scrypt if _hash_size <= PASSWORD_SIZE_MAX => Ok (Algorithm::Scrypt),
			Family::Scrypt => fail! (0x9a7484ca),
			
			Family::Argon2 if _hash_size <= PASSWORD_SIZE_MAX => Ok (Algorithm::Argon2id),
			Family::Argon2d if _hash_size <= PASSWORD_SIZE_MAX => Ok (Algorithm::Argon2d),
			Family::Argon2i if _hash_size <= PASSWORD_SIZE_MAX => Ok (Algorithm::Argon2i),
			Family::Argon2id if _hash_size <= PASSWORD_SIZE_MAX => Ok (Algorithm::Argon2id),
			(Family::Argon2 | Family::Argon2d | Family::Argon2i | Family::Argon2id) => fail! (0xa684303b),
		}
	}
}




pub fn choose_algorithm (_family : Option<Family>, _hash_size : Option<usize>) -> AlgorithmResult<(Algorithm, usize)> {
	
	let _family = _family.unwrap_or (Family::Blake3);
	
	let (_hash_size_minimum, _hash_size_maximum, _hash_size_default) = _family.hash_size_range ();
	
	let _hash_size = _hash_size.unwrap_or (_hash_size_default);
	
	if _hash_size < _hash_size_minimum {
		fail! (0xbc5e1f69);
	}
	if _hash_size > _hash_size_maximum {
		fail! (0x2166c769);
	}
	
	let _algorithm = _family.algorithm_for_hash_size (_hash_size) ?;
	
	Ok ((_algorithm, _hash_size))
}








impl FlagValue for Family {}

impl FlagValueDisplay for Family {
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		Debug::fmt (self, _formatter) .else_wrap (0x609b3f4a)
	}
}

impl FlagValueParsable for Family {
	fn parse_string (_string : String) -> FlagValueParseResult<Self> {
		fail! (0x1bd9cb5d);
	}
}




impl FlagValue for Algorithm {}

impl FlagValueDisplay for Algorithm {
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		Debug::fmt (self, _formatter) .else_wrap (0x18dd1207)
	}
}

impl FlagValueParsable for Algorithm {
	fn parse_string (_string : String) -> FlagValueParseResult<Self> {
		fail! (0xc247f344);
	}
}


