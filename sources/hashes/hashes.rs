

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;

use crate::model::*;
use crate::inputs::*;

use ::z_tokens_runtime::crates::{
		
		byteorder::{
				BigEndian,
				ByteOrder as _,
			},
	};

use ::z_tokens_runtime_hashes::crates::{
		
		digest,
		
		blake2,
		blake3,
		
		sha3,
		sha2,
		sha1,
		
		md5,
		
		siphasher,
		seahash,
		
		crc,
		adler,
		highway,
		xxhash,
		twox_hash,
		hashers,
		
		argon2,
		scrypt,
		
		digest::{
				Digest as _,
				VariableOutput as _,
			},
	};




define_error! (pub HashError, result : HashResult);




pub fn hash (_algorithm : Algorithm, _input : impl Input, _output_parameters : &OutputParameters) -> HashResult<Vec<u8>> {
	
	if _output_parameters.hash_size == 0 {
		fail! (0xc54f7d98);
	}
	if _output_parameters.truncate_size == 0 {
		fail! (0x93d0f3af);
	}
	if _output_parameters.truncate_size > _output_parameters.hash_size {
		fail! (0x9afe94bc);
	}
	if _output_parameters.truncate_size > OUTPUT_SIZE_MAX {
		fail! (0x32c196e2);
	}
	
	let mut _output = vec! [0u8; _output_parameters.truncate_size];
	
	match _algorithm {
		
		Algorithm::MD5 =>
			hash_fixed (md5::Md5::new (), _input, &mut _output, _output_parameters) ?,
		
		Algorithm::SHA1 =>
			hash_fixed (sha1::Sha1::new (), _input, &mut _output, _output_parameters) ?,
		
		Algorithm::SHA2_224 =>
			hash_fixed (sha2::Sha224::new (), _input, &mut _output, _output_parameters) ?,
		Algorithm::SHA2_256 =>
			hash_fixed (sha2::Sha256::new (), _input, &mut _output, _output_parameters) ?,
		Algorithm::SHA2_384 =>
			hash_fixed (sha2::Sha384::new (), _input, &mut _output, _output_parameters) ?,
		Algorithm::SHA2_512 =>
			hash_fixed (sha2::Sha512::new (), _input, &mut _output, _output_parameters) ?,
		
		Algorithm::SHA3_224 =>
			hash_fixed (sha3::Sha3_224::new (), _input, &mut _output, _output_parameters) ?,
		Algorithm::SHA3_256 =>
			hash_fixed (sha3::Sha3_256::new (), _input, &mut _output, _output_parameters) ?,
		Algorithm::SHA3_384 =>
			hash_fixed (sha3::Sha3_384::new (), _input, &mut _output, _output_parameters) ?,
		Algorithm::SHA3_512 =>
			hash_fixed (sha3::Sha3_512::new (), _input, &mut _output, _output_parameters) ?,
		
		Algorithm::Shake_128 =>
			hash_extendable (sha3::Shake128::default (), _input, &mut _output, _output_parameters) ?,
		Algorithm::Shake_256 =>
			hash_extendable (sha3::Shake256::default (), _input, &mut _output, _output_parameters) ?,
		
		Algorithm::GitSHA1 =>
			hash_git_sha1 (_input, &mut _output, _output_parameters) ?,
		Algorithm::GitSHA2 =>
			hash_git_sha2 (_input, &mut _output, _output_parameters) ?,
		
		Algorithm::Blake2s =>
			hash_variable (blake2::Blake2sVar::new (_output_parameters.hash_size) .else_wrap (0xfb4c3bb9) ?, _input, &mut _output, _output_parameters) ?,
		Algorithm::Blake2b =>
			hash_variable (blake2::Blake2bVar::new (_output_parameters.hash_size) .else_wrap (0x6e7b8e58) ?, _input, &mut _output, _output_parameters) ?,
		
		Algorithm::Blake3 =>
			hash_extendable (blake3::Hasher::new (), _input, &mut _output, _output_parameters) ?,
		
		Algorithm::Scrypt =>
			hash_scrypt (_input, &mut _output, _output_parameters) ?,
		
		Algorithm::Argon2d =>
			hash_argon (argon2::Algorithm::Argon2d, _input, &mut _output, _output_parameters) ?,
		Algorithm::Argon2i =>
			hash_argon (argon2::Algorithm::Argon2i, _input, &mut _output, _output_parameters) ?,
		Algorithm::Argon2id =>
			hash_argon (argon2::Algorithm::Argon2id, _input, &mut _output, _output_parameters) ?,
		
		Algorithm::SipHash_64 =>
			hash_siphash_64 (_input, &mut _output, _output_parameters) ?,
		Algorithm::SipHash_128 =>
			hash_siphash_128 (_input, &mut _output, _output_parameters) ?,
		
		Algorithm::SeaHash =>
			hash_seahash (_input, &mut _output, _output_parameters) ?,
		
		Algorithm::HighwayHash_64 =>
			hash_highwayhash_64 (_input, &mut _output, _output_parameters) ?,
		Algorithm::HighwayHash_128 =>
			hash_highwayhash_128 (_input, &mut _output, _output_parameters) ?,
		Algorithm::HighwayHash_256 =>
			hash_highwayhash_256 (_input, &mut _output, _output_parameters) ?,
		
		Algorithm::XxHash_32 =>
			hash_xxhash_32 (_input, &mut _output, _output_parameters) ?,
		Algorithm::XxHash_64 =>
			hash_xxhash_64 (_input, &mut _output, _output_parameters) ?,
		
		Algorithm::Xxh3_64 =>
			hash_xxh3_64 (_input, &mut _output, _output_parameters) ?,
		Algorithm::Xxh3_128 =>
			hash_xxh3_128 (_input, &mut _output, _output_parameters) ?,
		
		Algorithm::Djb2 =>
			hash_djb2 (_input, &mut _output, _output_parameters) ?,
		Algorithm::SDBM =>
			hash_sdbm (_input, &mut _output, _output_parameters) ?,
		
		Algorithm::FNV1a_32 =>
			hash_fnv1a_32 (_input, &mut _output, _output_parameters) ?,
		Algorithm::FNV1a_64 =>
			hash_fnv1a_64 (_input, &mut _output, _output_parameters) ?,
		
		Algorithm::CRC8 =>
			hash_crc8_any (crc::CRCu8::crc8 (), _input, &mut _output, _output_parameters) ?,
		Algorithm::CRC16 =>
			hash_crc16_any (crc::CRCu16::crc16 (), _input, &mut _output, _output_parameters) ?,
		Algorithm::CRC32 =>
			hash_crc32_any (crc::CRCu32::crc32 (), _input, &mut _output, _output_parameters) ?,
		Algorithm::CRC32C =>
			hash_crc32_any (crc::CRCu32::crc32c (), _input, &mut _output, _output_parameters) ?,
		Algorithm::CRC64 =>
			hash_crc64_any (crc::CRCu64::crc64 (), _input, &mut _output, _output_parameters) ?,
		
		Algorithm::Adler32 =>
			hash_adler32 (_input, &mut _output, _output_parameters) ?,
		
	}
	
	Ok (_output)
}








fn hash_fixed <Hasher> (mut _hasher : Hasher, _input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult
		where Hasher : digest::FixedOutput + digest::Update
{
	hash_update_digest (&mut _hasher, _input) ?;
	
	let _hash = _hasher.finalize_fixed ();
	
	copy_output_from_slice (&_hash, _output, _output_parameters)
}


fn hash_variable <Hasher> (mut _hasher : Hasher, _input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult
		where Hasher : digest::VariableOutput + digest::Update
{
	hash_update_digest (&mut _hasher, _input) ?;
	
	if _output.len () == _output_parameters.hash_size {
		
		_hasher.finalize_variable (_output) .else_wrap (0x52d8d078) ?;
		
		if _output_parameters.reversed {
			_output.reverse ();
		}
		
		Ok (())
		
	} else {
		
		let mut _hash = vec! [0u8; _output_parameters.hash_size];
		_hasher.finalize_variable (&mut _hash) .else_wrap (0x5ed68bcb) ?;
		
		copy_output_from_slice (&_hash, _output, _output_parameters)
	}
}


fn hash_extendable <Hasher> (mut _hasher : Hasher, _input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult
		where Hasher : digest::ExtendableOutput + digest::Update
{
	hash_update_digest (&mut _hasher, _input) ?;
	
	if _output.len () == _output_parameters.hash_size {
		
		_hasher.finalize_xof_into (_output);
		
		if _output_parameters.reversed {
			_output.reverse ();
		}
		
		Ok (())
		
	} else {
		
		let mut _hash = vec! [0u8; _output_parameters.hash_size];
		_hasher.finalize_xof_into (&mut _hash);
		
		copy_output_from_slice (&_hash, _output, _output_parameters)
	}
}








fn hash_siphash_64 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = siphasher::sip::SipHasher24::new ();
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher);
	
	copy_output_from_u64 (_hash_value, _output, _output_parameters)
}


fn hash_siphash_128 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = siphasher::sip128::SipHasher24::new ();
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = siphasher::sip128::Hasher128::finish128 (&_hasher) .as_u128 ();
	
	copy_output_from_u128 (_hash_value, _output, _output_parameters)
}




fn hash_seahash (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = seahash::SeaHasher::new ();
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher);
	
	copy_output_from_u64 (_hash_value, _output, _output_parameters)
}




fn hash_highwayhash_64 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = highway::HighwayHasher::new (Default::default ());
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = highway::HighwayHash::finalize64 (_hasher);
	
	copy_output_from_u64 (_hash_value, _output, _output_parameters)
}


fn hash_highwayhash_128 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = highway::HighwayHasher::new (Default::default ());
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_values = highway::HighwayHash::finalize128 (_hasher);
	
	copy_output_from_u64s (&_hash_values, _output, _output_parameters)
}


fn hash_highwayhash_256 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = highway::HighwayHasher::new (Default::default ());
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_values = highway::HighwayHash::finalize256 (_hasher);
	
	copy_output_from_u64s (&_hash_values, _output, _output_parameters)
}




fn hash_xxhash_32 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = twox_hash::XxHash32::with_seed (0);
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher) as u32;
	
	copy_output_from_u32 (_hash_value, _output, _output_parameters)
}


fn hash_xxhash_64 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = twox_hash::XxHash64::with_seed (0);
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher);
	
	copy_output_from_u64 (_hash_value, _output, _output_parameters)
}




fn hash_xxh3_64 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = xxhash::xxh3::Xxh3::with_seed (0);
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = xxhash::xxh3::Xxh3::digest (&_hasher);
	
	copy_output_from_u64 (_hash_value, _output, _output_parameters)
}


fn hash_xxh3_128 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = xxhash::xxh3::Xxh3::with_seed (0);
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = xxhash::xxh3::Xxh3::digest128 (&_hasher);
	
	copy_output_from_u128 (_hash_value, _output, _output_parameters)
}




fn hash_djb2 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = hashers::oz::DJB2Hasher::default ();
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher) as u32;
	
	copy_output_from_u32 (_hash_value, _output, _output_parameters)
}


fn hash_sdbm (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = hashers::oz::SDBMHasher::default ();
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher) as u32;
	
	copy_output_from_u32 (_hash_value, _output, _output_parameters)
}




fn hash_fnv1a_32 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = hashers::fnv::FNV1aHasher32::default ();
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher) as u32;
	
	copy_output_from_u32 (_hash_value, _output, _output_parameters)
}


fn hash_fnv1a_64 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = hashers::fnv::FNV1aHasher64::default ();
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher);
	
	copy_output_from_u64 (_hash_value, _output, _output_parameters)
}




fn hash_crc8_any (mut _hasher : crc::CRCu8, _input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	hash_update_fn (|_data| { _hasher.digest (_data); Ok (()) }, _input) ?;
	let _hash_value = crc::CRCu8::get_crc (&_hasher);
	
	copy_output_from_u8 (_hash_value, _output, _output_parameters)
}


fn hash_crc16_any (mut _hasher : crc::CRCu16, _input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	hash_update_fn (|_data| { _hasher.digest (_data); Ok (()) }, _input) ?;
	let _hash_value = crc::CRCu16::get_crc (&_hasher);
	
	copy_output_from_u16 (_hash_value, _output, _output_parameters)
}


fn hash_crc32_any (mut _hasher : crc::CRCu32, _input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	hash_update_fn (|_data| { _hasher.digest (_data); Ok (()) }, _input) ?;
	let _hash_value = crc::CRCu32::get_crc (&_hasher);
	
	copy_output_from_u32 (_hash_value, _output, _output_parameters)
}


fn hash_crc64_any (mut _hasher : crc::CRCu64, _input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	hash_update_fn (|_data| { _hasher.digest (_data); Ok (()) }, _input) ?;
	let _hash_value = crc::CRCu64::get_crc (&_hasher);
	
	copy_output_from_u64 (_hash_value, _output, _output_parameters)
}




fn hash_adler32 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hasher = adler::Adler32::new ();
	hash_update_fn (|_data| { _hasher.write_slice (_data); Ok (()) }, _input) ?;
	let _hash_value = adler::Adler32::checksum (&_hasher);
	
	copy_output_from_u32 (_hash_value, _output, _output_parameters)
}








fn hash_git_sha1 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	hash_git_fixed (sha1::Sha1::new (), _input, _output, _output_parameters)
}


fn hash_git_sha2 (_input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	hash_git_fixed (sha2::Sha256::new (), _input, _output, _output_parameters)
}


fn hash_git_fixed <Hasher> (mut _hasher : Hasher, _input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult
		where Hasher : digest::FixedOutput + digest::Update + io::Write
{
	// FIXME:  Optimize the code, so that if only one data item is returned, no buffering is needed.
	
	let mut _buffer = Vec::new ();
	
	hash_update_fn (
			|_data| {
				if (_buffer.len () + _data.len ()) > INPUT_SIZE_MAX {
					fail! (0x9d4af86c);
				}
				_buffer.extend_from_slice (_data);
				Ok (())
			},
			_input) ?;
	
	write! (_hasher, "blob {}\0", _buffer.len ()) .else_wrap (0x6160a28b) ?;
	
	let _input = InputFromBytesBoxes::from_vec (_buffer);
	
	hash_fixed (_hasher, _input, _output, _output_parameters)
}








fn hash_update_digest <Hasher> (_hasher : &mut Hasher, mut _input : impl Input) -> HashResult
		where Hasher : digest::Update
{
	hash_update_fn (|_data| { _hasher.update (_data); Ok (()) }, _input)
}


fn hash_update_std <Hasher> (_hasher : &mut Hasher, mut _input : impl Input) -> HashResult
		where Hasher : hash::Hasher
{
	hash_update_fn (|_data| { _hasher.write (_data); Ok (()) }, _input)
}


fn hash_update_fn <Update> (mut _update : Update, mut _input : impl Input) -> HashResult
		where Update : FnMut (&[u8]) -> HashResult
{
	while let Some (_data) = _input.input () .else_wrap (0x397076d5) ? {
		_update (_data) ?;
	}
	
	Ok (())
}








fn copy_output_from_u8 (_hash_value : u8, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hash_bytes = [ _hash_value ];
	
	copy_output_from_slice (&_hash_bytes, _output, _output_parameters)
}


fn copy_output_from_u16 (_hash_value : u16, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hash_bytes = [0u8; 2];
	BigEndian::write_u16 (&mut _hash_bytes, _hash_value);
	
	copy_output_from_slice (&_hash_bytes, _output, _output_parameters)
}


fn copy_output_from_u32 (_hash_value : u32, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hash_bytes = [0u8; 4];
	BigEndian::write_u32 (&mut _hash_bytes, _hash_value);
	
	copy_output_from_slice (&_hash_bytes, _output, _output_parameters)
}


fn copy_output_from_u64 (_hash_value : u64, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hash_bytes = [0u8; 8];
	BigEndian::write_u64 (&mut _hash_bytes, _hash_value);
	
	copy_output_from_slice (&_hash_bytes, _output, _output_parameters)
}


fn copy_output_from_u64s <const SIZE : usize> (_hash_values : &[u64; SIZE], _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	// FIXME:  We can't write `8 * SIZE`...
	let mut _hash_bytes = [0u8; 8 * 4];
	BigEndian::write_u64_into (_hash_values, &mut _hash_bytes[.. 8 * SIZE]);
	
	copy_output_from_slice (&_hash_bytes, _output, _output_parameters)
}


fn copy_output_from_u128 (_hash_value : u128, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let mut _hash_bytes = [0u8; 16];
	BigEndian::write_u128 (&mut _hash_bytes, _hash_value);
	
	copy_output_from_slice (&_hash_bytes, _output, _output_parameters)
}


fn copy_output_from_slice (_hash : &[u8], _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let _hash_len = _hash.len ();
	let _output_len = _output.len ();
	
	if _hash_len < _output_len {
		fail! (0xedf869a5);
	}
	
	if _output_parameters.discard_right {
		_output.copy_from_slice (&_hash[.. _output_len]);
	} else {
		_output.copy_from_slice (&_hash[(_hash_len - _output_len) ..]);
	}
	
	if _output_parameters.reversed {
		_output.reverse ();
	}
	
	Ok (())
}








fn hash_scrypt (mut _input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let _hash_size = _output_parameters.hash_size;
	let (_m_cost, _t_cost, _p_cost) = hash_passwords_cost (_hash_size) ?;
	
	// NOTE:  =>  https://words.filippo.io/the-scrypt-parameters/
	let _r_cost = 8;
	let _p_cost = _p_cost * u32::max (_t_cost / 2, 1);
	let _n_cost : u32 = ((_m_cost as u64 * 1024) / _r_cost as u64 / 128) .try_into () .else_wrap (0xb9f46bf5) ? ;
	let _n_log2_cost : u8 = _n_cost.ilog2 () .try_into () .else_wrap (0xd3015b16) ?;
	
	// ::std::eprintln! ("size {} {} || m {} / t {} || n {} / r {} / p {}", _hash_size, _hash_size * 8, _m_cost / 1024, _t_cost, _n_cost, _r_cost, _p_cost);
	
	const INPUT_HASH_SIZE : usize = 32;
	let mut _input_hash = [0u8; INPUT_HASH_SIZE];
	hash_fixed (sha2::Sha256::new (), _input, &mut _input_hash, & OutputParameters { hash_size : INPUT_HASH_SIZE, truncate_size : INPUT_HASH_SIZE, discard_right : true, reversed : false, }) ?;
	
	let _hasher_parameters = scrypt::Params::new (_n_log2_cost, _r_cost, _p_cost, scrypt::Params::RECOMMENDED_LEN) .else_wrap (0x5c5ead84) ?;
	
	let mut _hash = vec! [0u8; _hash_size];
	scrypt::scrypt (&_input_hash, &_input_hash, &_hasher_parameters, &mut _hash) .else_wrap (0x532d2ed3) ?;
	
	copy_output_from_slice (&_hash, _output, _output_parameters)
}




fn hash_argon (_algorithm : argon2::Algorithm, mut _input : impl Input, _output : &mut [u8], _output_parameters : &OutputParameters) -> HashResult {
	
	let _hash_size = _output_parameters.hash_size;
	let (_m_cost, _t_cost, _p_cost) = hash_passwords_cost (_hash_size) ?;
	
	const INPUT_HASH_SIZE : usize = 64;
	let mut _input_hash = [0u8; INPUT_HASH_SIZE];
	hash_fixed (blake2::Blake2b512::new (), _input, &mut _input_hash, & OutputParameters { hash_size : INPUT_HASH_SIZE, truncate_size : INPUT_HASH_SIZE, discard_right : true, reversed : false, }) ?;
	
	// ::std::eprintln! ("size {} {} || m {} / t {} / p {}", _hash_size, _hash_size * 8, _m_cost / 1024, _t_cost, _p_cost);
	
	let _hasher_parameters = argon2::Params::new (_m_cost, _t_cost, _p_cost, Some (_hash_size)) .else_wrap (0x8acd25cd) ?;
	let _hasher = argon2::Argon2::new (_algorithm, argon2::Version::V0x13, _hasher_parameters);
	
	let mut _hash = vec! [0u8; _hash_size];
	_hasher.hash_password_into (&_input_hash, &_input_hash, &mut _hash) .else_wrap (0xce42692d) ?;
	
	copy_output_from_slice (&_hash, _output, _output_parameters)
}




fn hash_passwords_cost (_hash_size : usize) -> HashResult<(u32, u32, u32)> {
	
	if _hash_size > PASSWORD_SIZE_MAX {
		fail! (0x5daf563e);
	}
	
	let _hash_size = usize::max (_hash_size / 4, 1);
	let _hash_size : u32 = _hash_size.try_into () .else_wrap (0x55109be8) ?;
	
	const M_COST_MAX : u32 = 1 * 1024 * 1024;
	const M_COST_BASE : u32 = 16 * 1024;
	const T_COST_BASE : u32 = 16;
	const P_COST : u32 = 1;
	
	let _m_cost = u64::min (_hash_size as u64 * M_COST_BASE as u64, M_COST_MAX as u64) as u32;
	let _t_cost = u64::max (_hash_size as u64 * T_COST_BASE as u64 / (M_COST_MAX / M_COST_BASE / 4) as u64, T_COST_BASE as u64) as u32;
	
	Ok ((_m_cost, _t_cost, P_COST))
}


