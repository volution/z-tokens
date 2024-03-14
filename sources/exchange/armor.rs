

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use ::z_tokens_runtime::{
		crates::constant_time_eq::constant_time_eq,
	};


use ::z_tokens_runtime_hashes::{
		crates::blake3,
	};


use ::z_tokens_runtime_crypto::{
		crates::chacha20,
		define_cryptographic_purpose,
	};


use crate::coding::*;








define_error! (pub ArmorError, result : ArmorResult);




pub const ARMOR_DECODED_SIZE_MAX : usize = 128 * 1024 * 1024;


pub const ARMOR_ENCODED_SIZE_MAX : usize =
		(
			(
				(
					ARMOR_DECODED_SIZE_MAX
					+ ARMOR_ENCODED_HEADER_SIZE
					+ ARMOR_ENCODED_TRAILER_SIZE
				) / CODING_CHUNK_DECODED_SIZE
				+ 1
			) / CODING_CHUNKS_PER_LINE
			+ 1
		) * (
			9 + 4 + 1
			+ CODING_CHUNKS_PER_LINE * (CODING_CHUNKS_PER_LINE + CODING_CHUNK_ENCODED_SIZE + 1)
		);


pub(crate) const ARMOR_ENCODED_SCHEMA_SIZE : usize = 4;
pub(crate) const ARMOR_ENCODED_LENGTH_SIZE : usize = 4;
pub(crate) const ARMOR_ENCODED_FINGERPRINT_SIZE : usize = 16;


pub(crate) const ARMOR_ENCODED_HEADER_SIZE : usize = ARMOR_ENCODED_SCHEMA_SIZE + ARMOR_ENCODED_LENGTH_SIZE;
pub(crate) const ARMOR_ENCODED_TRAILER_SIZE : usize = ARMOR_ENCODED_FINGERPRINT_SIZE;

pub(crate) const ARMOR_SCHEMA_V1_VALUE : u32 = 0x51c8e38b;


define_cryptographic_purpose! (pub(crate) ARMOR_AONT_KEY_PURPOSE, armor, aont_key);








pub fn armor (_decoded : &[u8], _encoded : &mut Vec<u8>) -> ArmorResult {
	
	let _decoded_len = _decoded.len ();
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  sanity check...
	
	if _decoded_len > ARMOR_DECODED_SIZE_MAX {
		fail! (0x3463e357);
	}
	
	let _compress_capacity = compress_capacity_max (_decoded_len) .else_wrap (0xd7e27086) ?;
	let _compress_capacity = _compress_capacity + ARMOR_ENCODED_HEADER_SIZE + ARMOR_ENCODED_TRAILER_SIZE;
	
	let mut _intermediate_buffer = Vec::with_capacity (_compress_capacity);
	
	// NOTE:  schema...
	
	encode_u32_push (ARMOR_SCHEMA_V1_VALUE, &mut _intermediate_buffer);
	
	// NOTE:  length...
	
	encode_u32_push (_decoded_len as u32, &mut _intermediate_buffer);
	
	// NOTE:  compressing...
	
	compress (_decoded, &mut _intermediate_buffer) .else_wrap (0x08e19178) ?;
	
	if (_intermediate_buffer.len () + ARMOR_ENCODED_HEADER_SIZE) >= _decoded_len {
		
		_intermediate_buffer.truncate (ARMOR_ENCODED_HEADER_SIZE);
		_intermediate_buffer.extend_from_slice (_decoded);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  fingerprinting...
	
	let mut _fingerprint = apply_fingerprint (&_intermediate_buffer) ?;
	
	// NOTE:  all-or-nothing...
	
	apply_all_or_nothing_encryption (&_fingerprint, &mut _intermediate_buffer) ?;
	apply_all_or_nothing_mangling (&mut _fingerprint, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (&_fingerprint);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  encoding...
	
	assert! (_intermediate_buffer.len () <= (_decoded_len + ARMOR_ENCODED_HEADER_SIZE + ARMOR_ENCODED_TRAILER_SIZE), "[8c327ecd]");
	
	let _encode_capacity = encode_capacity_max (_intermediate_buffer.len ()) .else_wrap (0x00bf84c9) ?;
	
	let mut _encode_buffer = Vec::with_capacity (_encode_capacity);
	encode (&_intermediate_buffer, &mut _encode_buffer) .else_wrap (0x080c7733) ?;
	
	assert! (_encode_buffer.len () <= ARMOR_ENCODED_SIZE_MAX, "[e14aea63]  {} <= {}", _encode_buffer.len (), ARMOR_ENCODED_SIZE_MAX);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_encoded.extend_from_slice (&_encode_buffer);
	
	Ok (())
}




pub fn dearmor (_encoded : &[u8], _decoded : &mut Vec<u8>) -> ArmorResult {
	
	let _encoded_len = _encoded.len ();
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  sanity check...
	
	if _encoded_len > ARMOR_ENCODED_SIZE_MAX {
		fail! (0xe141a81a);
	}
	
	// NOTE:  decoding...
	
	let _decode_capacity = decode_capacity_max (_encoded_len) .else_wrap (0x7321f5b4) ?;
	
	let mut _intermediate_buffer = Vec::with_capacity (_decode_capacity);
	decode (_encoded, &mut _intermediate_buffer) .else_wrap (0x6432ccd9) ?;
	
	if _intermediate_buffer.len () < (ARMOR_ENCODED_HEADER_SIZE + ARMOR_ENCODED_TRAILER_SIZE) {
		fail! (0xf70b9d32);
	}
	
	// NOTE:  schema...
	
	let _schema_value = decode_u32_slice (&_intermediate_buffer[..ARMOR_ENCODED_SCHEMA_SIZE]);
	
	if _schema_value != ARMOR_SCHEMA_V1_VALUE {
		fail! (0xdf40ef71);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  all-or-nothing...
	
	let mut _fingerprint = bytes_pop::<ARMOR_ENCODED_FINGERPRINT_SIZE> (&mut _intermediate_buffer) .else_wrap (0xcfdbfbc3) ?;
	
	apply_all_or_nothing_mangling (&mut _fingerprint, &_intermediate_buffer) ?;
	apply_all_or_nothing_encryption (&_fingerprint, &mut _intermediate_buffer) ?;
	
	// NOTE:  fingerprinting...
	
	let _fingerprint_actual = apply_fingerprint (&_intermediate_buffer) ?;
	
	if ! constant_time_eq (&_fingerprint_actual, &_fingerprint) {
		fail! (0x7c3ab20d);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  length...
	
	let _decoded_len = decode_u32_slice (&_intermediate_buffer[ARMOR_ENCODED_SCHEMA_SIZE .. ARMOR_ENCODED_SCHEMA_SIZE + ARMOR_ENCODED_LENGTH_SIZE]) as usize;
	
	if _decoded_len > ARMOR_DECODED_SIZE_MAX {
		fail! (0xd0db488b);
	}
	
	// NOTE:  decompressing...
	
	let mut _decompress_buffer = Vec::with_capacity (_decoded_len);
	if _decoded_len > (_intermediate_buffer.len () - ARMOR_ENCODED_HEADER_SIZE) {
		
		decompress (&_intermediate_buffer[ARMOR_ENCODED_HEADER_SIZE..], &mut _decompress_buffer) .else_wrap (0x70f5d0b4) ?;
		
	} else {
		
		_decompress_buffer.extend_from_slice (&_intermediate_buffer[ARMOR_ENCODED_HEADER_SIZE..]);
	}
	
	_intermediate_buffer.truncate (ARMOR_ENCODED_HEADER_SIZE);
	
	if _decompress_buffer.len () != _decoded_len {
		fail! (0xc763571b);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  length...
	
	if _decoded_len != decode_u32_pop (&mut _intermediate_buffer) .else_wrap (0xa8d32a02) ? as usize {
		panic! (unreachable, 0xb847a182);
	}
	
	// NOTE:  schema...
	
	if _schema_value != decode_u32_pop (&mut _intermediate_buffer) .else_wrap (0xcae8663c) ? {
		panic! (unreachable, 0x6fa5f798);
	}
	
	// NOTE:  finalizing...
	
	if ! _intermediate_buffer.is_empty () {
		fail! (0x7c104fb9);
	}
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_decoded.extend_from_slice (&_decompress_buffer);
	
	Ok (())
}








pub(crate) fn apply_all_or_nothing_encryption (_fingerprint : &[u8; ARMOR_ENCODED_FINGERPRINT_SIZE], _data : &mut [u8]) -> ArmorResult {
	
	use chacha20::cipher::KeyIvInit as _;
	use chacha20::cipher::StreamCipher as _;
	
	let _key : [u8; 32] =
			blake3::Hasher::new_derive_key (ARMOR_AONT_KEY_PURPOSE)
			.update (_fingerprint)
			.finalize ()
			.into ();
	
	let _nonce = [0u8; 12];
	
	let _key = chacha20::Key::from (_key);
	let _nonce = chacha20::Nonce::from (_nonce);
	
	let mut _cipher = chacha20::ChaCha20::new (&_key, &_nonce);
	
	assert! (_data.len () >= ARMOR_ENCODED_HEADER_SIZE, "[73aae7e5]");
	
	_cipher.try_apply_keystream (&mut _data[ARMOR_ENCODED_SCHEMA_SIZE..]) .else_wrap (0x1f4e248a) ?;
	
	Ok (())
}


pub(crate) fn apply_all_or_nothing_mangling (_fingerprint : &mut [u8; ARMOR_ENCODED_FINGERPRINT_SIZE], _data : &[u8]) -> ArmorResult {
	
	let _hash =
			blake3::Hasher::new ()
			.update (_data)
			.finalize ();
	
	let _hash = _hash.as_bytes ();
	
	assert! (_hash.len () >= ARMOR_ENCODED_FINGERPRINT_SIZE, "[6059b917]");
	
	for _index in 0 .. ARMOR_ENCODED_FINGERPRINT_SIZE {
		_fingerprint[_index] ^= _hash[_index];
	}
	
	Ok (())
}




pub(crate) fn apply_fingerprint (_data : &[u8]) -> ArmorResult<[u8; ARMOR_ENCODED_FINGERPRINT_SIZE]> {
	
	let _hash =
			blake3::Hasher::new ()
			.update (_data)
			.finalize ();
	
	let _hash = _hash.as_bytes ();
	
	assert! (_hash.len () >= ARMOR_ENCODED_FINGERPRINT_SIZE, "[884359e7]");
	
	let mut _fingerprint = [0u8; ARMOR_ENCODED_FINGERPRINT_SIZE];
	_fingerprint.copy_from_slice (&_hash[.. ARMOR_ENCODED_FINGERPRINT_SIZE]);
	
	Ok (_fingerprint)
}


