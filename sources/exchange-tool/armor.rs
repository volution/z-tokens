

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use crate::coding::*;








define_error! (pub ArmorError, result : ArmorResult);




pub(crate) const ARMOR_DECODED_SIZE_MAX : usize = 128 * 1024 * 1024;

pub(crate) const ARMOR_ENCODED_SIZE_MAX : usize =
		(
			(
				(
					ARMOR_DECODED_SIZE_MAX
					+ (ARMOR_DECODED_SIZE_MAX * COMPRESSION_OVERHEAD_FRACTION / COMPRESSION_OVERHEAD_DIVIDER) + COMPRESSION_OVERHEAD_EXTRA
					+ 4 + ARMOR_ENCODED_HASH + ARMOR_ENCODED_SALT
				) / CODING_CHUNK_DECODED_SIZE
			) + 1
		) * (CODING_CHUNK_ENCODED_SIZE + 1);


pub(crate) const ARMOR_ENCODED_HASH : usize = CODING_CHUNK_DECODED_SIZE * 2 - 4;
pub(crate) const ARMOR_ENCODED_SALT : usize = 16;


static ARMOR_AONT_KEY_CONTEXT : &str = "z-tokens exchange armor aont key (2023a)";
static ARMOR_PIN_KEY_CONTEXT : &str = "z-tokens exchange armor pin key (2023a)";


static ARMOR_PIN_ARGON_SALT : &str = "z-tokens exchange armor pin salt (2023a)";

const ARMOR_PIN_ARGON_ALGORITHM : ::argon2::Algorithm = ::argon2::Algorithm::Argon2id;
const ARMOR_PIN_ARGON_VERSION : ::argon2::Version = ::argon2::Version::V0x13;

const ARMOR_PIN_ARGON_M_COST : u32 = 128 * 1024;
const ARMOR_PIN_ARGON_T_COST : u32 = 8;
const ARMOR_PIN_ARGON_P_COST : u32 = 1;








pub fn armor (_decoded : &[u8], _encoded : &mut Vec<u8>, _pin : Option<&[u8]>) -> ArmorResult {
	
	let _decoded_len = _decoded.len ();
	
	if _decoded_len > ARMOR_DECODED_SIZE_MAX {
		fail! (0x3463e357);
	}
	
	// NOTE:  compressing...
	
	let _compress_capacity = compress_capacity_max (_decoded_len) .else_wrap (0xd7e27086) ?;
	let _compress_capacity = _compress_capacity + 4 + ARMOR_ENCODED_HASH + ARMOR_ENCODED_SALT;
	
	let mut _compress_buffer = Vec::with_capacity (_compress_capacity);
	compress (_decoded, &mut _compress_buffer) .else_wrap (0x08e19178) ?;
	
	// NOTE:  wrapping...
	
	encode_u32_push (_decoded_len as u32, &mut _compress_buffer);
	
	let _fingerprint = apply_fingerprint (&_compress_buffer) ?;
	_compress_buffer.extend_from_slice (&_fingerprint);
	
	// NOTE:  all-or-nothing...
	
	let mut _salt = generate_salt () ?;
	
	apply_all_or_nothing_encryption (&_salt, &mut _compress_buffer, _pin) ?;
	apply_all_or_nothing_mangling (&mut _salt, &_compress_buffer) ?;
	
	_compress_buffer.extend_from_slice (&_salt);
	
	// NOTE:  encoding...
	
	let _encode_capacity = encode_capacity_max (_compress_buffer.len ()) .else_wrap (0x00bf84c9) ?;
	
	let mut _encode_buffer = Vec::with_capacity (_encode_capacity);
	encode (&_compress_buffer, &mut _encode_buffer) .else_wrap (0x080c7733) ?;
	
	assert! (_encode_buffer.len () <= ARMOR_ENCODED_SIZE_MAX, "[e14aea63]  {} <= {}", _encode_buffer.len (), ARMOR_ENCODED_SIZE_MAX);
	
	// NOTE:  finalizing...
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_encoded.extend_from_slice (&_encode_buffer);
	
	Ok (())
}




pub fn dearmor (_encoded : &[u8], _decoded : &mut Vec<u8>, _pin : Option<&[u8]>) -> ArmorResult {
	
	let _encoded_len = _encoded.len ();
	
	if _encoded_len > ARMOR_ENCODED_SIZE_MAX {
		fail! (0xe141a81a);
	}
	
	// NOTE:  decoding...
	
	let _decode_capacity = decode_capacity_max (_encoded_len) .else_wrap (0x7321f5b4) ?;
	
	let mut _decode_buffer = Vec::with_capacity (_decode_capacity);
	decode (_encoded, &mut _decode_buffer) .else_wrap (0x6432ccd9) ?;
	
	// NOTE:  all-or-nothing...
	
	let mut _salt = bytes_pop::<ARMOR_ENCODED_SALT> (&mut _decode_buffer) .else_wrap (0xcfdbfbc3) ?;
	
	apply_all_or_nothing_mangling (&mut _salt, &_decode_buffer) ?;
	apply_all_or_nothing_encryption (&_salt, &mut _decode_buffer, _pin) ?;
	
	// NOTE:  unwrapping...
	
	let _fingerprint_expected = bytes_pop::<ARMOR_ENCODED_HASH> (&mut _decode_buffer) .else_wrap (0x80825bd8) ?;
	
	let _fingerprint_actual = apply_fingerprint (&_decode_buffer) ?;
	
	if ! ::constant_time_eq::constant_time_eq (&_fingerprint_actual, &_fingerprint_expected) {
		fail! (0x7c3ab20d);
	}
	
	let _decoded_len = decode_u32_pop (&mut _decode_buffer) .else_wrap (0xa8d32a02) ? as usize;
	
	if _decoded_len > ARMOR_DECODED_SIZE_MAX {
		fail! (0xd0db488b);
	}
	
	// NOTE:  decompressing...
	
	let mut _decompress_buffer = Vec::with_capacity (_decoded_len);
	decompress (&_decode_buffer, &mut _decompress_buffer) .else_wrap (0x70f5d0b4) ?;
	
	if _decompress_buffer.len () != _decoded_len {
		fail! (0xc763571b);
	}
	
	// NOTE:  finalizing...
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_decoded.extend_from_slice (&_decompress_buffer);
	
	Ok (())
}








fn apply_all_or_nothing_encryption (_salt : &[u8; ARMOR_ENCODED_SALT], _data : &mut [u8], _pin : Option<&[u8]>) -> ArmorResult {
	
	use ::chacha20::cipher::KeyIvInit as _;
	use ::chacha20::cipher::StreamCipher as _;
	
	let _pin = apply_argon (_pin) ?;
	
	let _pin : [u8; 32] =
			::blake3::Hasher::new_derive_key (ARMOR_PIN_KEY_CONTEXT)
			.update (&_pin)
			.finalize ()
			.into ();
	
	let _key : [u8; 32] =
			::blake3::Hasher::new_derive_key (ARMOR_AONT_KEY_CONTEXT)
			.update (&_pin)
			.update (_salt)
			.finalize ()
			.into ();
	
	let _nonce = [0u8; 12];
	
	let _key = ::chacha20::Key::from_slice (&_key);
	let _nonce = ::chacha20::Nonce::from (_nonce);
	
	let mut _cipher = ::chacha20::ChaCha20::new (&_key, &_nonce);
	
	_cipher.try_apply_keystream (_data) .else_wrap (0x1f4e248a) ?;
	
	Ok (())
}


fn apply_all_or_nothing_mangling (_salt : &mut [u8; ARMOR_ENCODED_SALT], _data : &[u8]) -> ArmorResult {
	
	let _hash =
			::blake3::Hasher::new ()
			.update (_data)
			.finalize ();
	
	let _hash = _hash.as_bytes ();
	
	for _index in 0 .. ARMOR_ENCODED_SALT {
		_salt[_index] ^= _hash[_index];
	}
	
	Ok (())
}




fn apply_fingerprint (_data : &[u8]) -> ArmorResult<[u8; ARMOR_ENCODED_HASH]> {
	
	let _hash =
			::blake3::Hasher::new ()
			.update (_data)
			.finalize ();
	
	let mut _fingerprint = [0u8; ARMOR_ENCODED_HASH];
	_fingerprint.copy_from_slice (& _hash.as_bytes () [.. ARMOR_ENCODED_HASH]);
	
	Ok (_fingerprint)
}




fn apply_argon (_pin : Option<&[u8]>) -> ArmorResult<[u8; 32]> {
	
	let mut _output = [0u8; 32];
	
	let Some (_pin) = _pin
		else {
			return Ok (_output);
		};
	
	if _pin.is_empty () {
		return Ok (_output);
	}
	
	let _parameters = ::argon2::Params::new (
				ARMOR_PIN_ARGON_M_COST,
				ARMOR_PIN_ARGON_T_COST,
				ARMOR_PIN_ARGON_P_COST,
				Some (_output.len ()),
			) .else_wrap (0x23aba478) ?;
	
	let _hasher = ::argon2::Argon2::new (
				ARMOR_PIN_ARGON_ALGORITHM,
				ARMOR_PIN_ARGON_VERSION,
				_parameters,
			);
	
	_hasher.hash_password_into (_pin, ARMOR_PIN_ARGON_SALT.as_bytes (), &mut _output) .else_wrap (0x6ab7506e) ?;
	
	Ok (_output)
}




fn generate_salt () -> ArmorResult<[u8; ARMOR_ENCODED_SALT]> {
	use ::rand::RngCore as _;
	let mut _salt = [0u8; ARMOR_ENCODED_SALT];
	::rand::rngs::OsRng.fill_bytes (&mut _salt);
	Ok (_salt)
}


