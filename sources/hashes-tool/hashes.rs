

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;

use crate::model::*;
use crate::inputs::Input;

use ::digest::{
		self as digest,
		Digest as _,
		VariableOutput as _,
	};




define_error! (pub HashError, result : HashResult);




pub fn hash (_algorithm : Algorithm, _output_size : usize, _input : impl Input) -> HashResult<Vec<u8>> {
	
	if _output_size == 0 {
		fail! (0x93d0f3af);
	}
	if _output_size > OUTPUT_SIZE_MAX {
		fail! (0x32c196e2);
	}
	
	let mut _output = vec! [0u8; _output_size];
	
	match _algorithm {
		
		Algorithm::MD5 =>
			hash_fixed (::md5::Md5::new (), _input, &mut _output) ?,
		
		Algorithm::SHA1 =>
			hash_fixed (::sha1::Sha1::new (), _input, &mut _output) ?,
		
		Algorithm::SHA2_224 =>
			hash_fixed (::sha2::Sha224::new (), _input, &mut _output) ?,
		Algorithm::SHA2_256 =>
			hash_fixed (::sha2::Sha256::new (), _input, &mut _output) ?,
		Algorithm::SHA2_384 =>
			hash_fixed (::sha2::Sha384::new (), _input, &mut _output) ?,
		Algorithm::SHA2_512 =>
			hash_fixed (::sha2::Sha512::new (), _input, &mut _output) ?,
		
		Algorithm::SHA3_224 =>
			hash_fixed (::sha3::Sha3_224::new (), _input, &mut _output) ?,
		Algorithm::SHA3_256 =>
			hash_fixed (::sha3::Sha3_256::new (), _input, &mut _output) ?,
		Algorithm::SHA3_384 =>
			hash_fixed (::sha3::Sha3_384::new (), _input, &mut _output) ?,
		Algorithm::SHA3_512 =>
			hash_fixed (::sha3::Sha3_512::new (), _input, &mut _output) ?,
		
		Algorithm::GitSHA1 =>
			fail! (0x64e83dae),
		
		Algorithm::Blake2s =>
			hash_variable (::blake2::Blake2sVar::new (_output_size) .else_wrap (0xfb4c3bb9) ?, _input, &mut _output) ?,
		Algorithm::Blake2b =>
			hash_variable (::blake2::Blake2bVar::new (_output_size) .else_wrap (0x6e7b8e58) ?, _input, &mut _output) ?,
		
		Algorithm::Blake3 =>
			hash_extendable (::blake3::Hasher::new (), _input, &mut _output) ?,
		
	}
	
	Ok (_output)
}




fn hash_fixed <Hasher> (mut _hasher : Hasher, _input : impl Input, _output : &mut [u8]) -> HashResult
		where Hasher : digest::FixedOutput + digest::Update
{
	hash_update (&mut _hasher, _input) .else_wrap (0x3322631d) ?;
	
	let _output_size = _output.len ();
	let _hash_full = _hasher.finalize_fixed ();
	if _hash_full.len () < _output_size {
		fail! (0x529b2c3f);
	}
	
	_output.copy_from_slice (&_hash_full[0.._output_size]);
	
	Ok (())
}


fn hash_variable <Hasher> (mut _hasher : Hasher, _input : impl Input, _output : &mut [u8]) -> HashResult
		where Hasher : digest::VariableOutput + digest::Update
{
	hash_update (&mut _hasher, _input) .else_wrap (0xccfa4243) ?;
	
	_hasher.finalize_variable (_output) .else_wrap (0x52d8d078) ?;
	
	Ok (())
}


fn hash_extendable <Hasher> (mut _hasher : Hasher, _input : impl Input, _output : &mut [u8]) -> HashResult
		where Hasher : digest::ExtendableOutput + digest::Update
{
	
	hash_update (&mut _hasher, _input) .else_wrap (0x5df214fb) ?;
	
	_hasher.finalize_xof_into (_output);
	
	Ok (())
}


fn hash_update <Hasher> (_hasher : &mut Hasher, mut _input : impl Input) -> HashResult
		where Hasher : digest::Update
{
	
	while let Some (_data) = _input.input () .else_wrap (0x17507faa) ? {
		_hasher.update (_data);
	}
	
	Ok (())
}

