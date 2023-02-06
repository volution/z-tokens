

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use crate::coding::*;
use crate::macros::*;








define_error! (pub ArmorError, result : ArmorResult);




pub const ARMOR_DECODED_SIZE_MAX : usize = 128 * 1024 * 1024;


pub const ARMOR_ENCODED_SIZE_MAX : usize =
		(
			(
				(
					ARMOR_DECODED_SIZE_MAX
					+ 4 + ARMOR_ENCODED_FINGERPRINT
				) / CODING_CHUNK_DECODED_SIZE
				+ 1
			) / CODING_CHUNKS_PER_LINE
			+ 1
		) * (
			9 + 4 + 1
			+ CODING_CHUNKS_PER_LINE * (CODING_CHUNKS_PER_LINE + CODING_CHUNK_ENCODED_SIZE + 1)
		);


const ARMOR_ENCODED_FINGERPRINT : usize = CODING_CHUNK_DECODED_SIZE * 2 - 4;


define_cryptographic_context! (ARMOR_AONT_KEY_CONTEXT, armor, aont_key);








pub fn armor (_decoded : &[u8], _encoded : &mut Vec<u8>) -> ArmorResult {
	
	let _decoded_len = _decoded.len ();
	
	if _decoded_len > ARMOR_DECODED_SIZE_MAX {
		fail! (0x3463e357);
	}
	
	// NOTE:  compressing...
	
	let _compress_capacity = compress_capacity_max (_decoded_len) .else_wrap (0xd7e27086) ?;
	let _compress_capacity = _compress_capacity + 4 + ARMOR_ENCODED_FINGERPRINT;
	
	let mut _intermediate_buffer = Vec::with_capacity (_compress_capacity);
	compress (_decoded, &mut _intermediate_buffer) .else_wrap (0x08e19178) ?;
	
	if _intermediate_buffer.len () >= _decoded_len {
		
		_intermediate_buffer.clear ();
		_intermediate_buffer.extend_from_slice (_decoded);
	}
	
	// NOTE:  wrapping...
	
	encode_u32_push (_decoded_len as u32, &mut _intermediate_buffer);
	
	// NOTE:  all-or-nothing...
	
	let mut _fingerprint = apply_fingerprint (&_intermediate_buffer) ?;
	
	apply_all_or_nothing_encryption (&_fingerprint, &mut _intermediate_buffer) ?;
	apply_all_or_nothing_mangling (&mut _fingerprint, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (&_fingerprint);
	
	// NOTE:  encoding...
	
	assert! (_intermediate_buffer.len () <= (_decoded_len + 4 + ARMOR_ENCODED_FINGERPRINT), "[8c327ecd]");
	
	let _encode_capacity = encode_capacity_max (_intermediate_buffer.len ()) .else_wrap (0x00bf84c9) ?;
	
	let mut _encode_buffer = Vec::with_capacity (_encode_capacity);
	encode (&_intermediate_buffer, &mut _encode_buffer) .else_wrap (0x080c7733) ?;
	
	assert! (_encode_buffer.len () <= ARMOR_ENCODED_SIZE_MAX, "[e14aea63]  {} <= {}", _encode_buffer.len (), ARMOR_ENCODED_SIZE_MAX);
	
	// NOTE:  finalizing...
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_encoded.extend_from_slice (&_encode_buffer);
	
	Ok (())
}




pub fn dearmor (_encoded : &[u8], _decoded : &mut Vec<u8>) -> ArmorResult {
	
	let _encoded_len = _encoded.len ();
	
	if _encoded_len > ARMOR_ENCODED_SIZE_MAX {
		fail! (0xe141a81a);
	}
	
	// NOTE:  decoding...
	
	let _decode_capacity = decode_capacity_max (_encoded_len) .else_wrap (0x7321f5b4) ?;
	
	let mut _intermediate_buffer = Vec::with_capacity (_decode_capacity);
	decode (_encoded, &mut _intermediate_buffer) .else_wrap (0x6432ccd9) ?;
	
	// NOTE:  all-or-nothing...
	
	let mut _fingerprint = bytes_pop::<ARMOR_ENCODED_FINGERPRINT> (&mut _intermediate_buffer) .else_wrap (0xcfdbfbc3) ?;
	
	apply_all_or_nothing_mangling (&mut _fingerprint, &_intermediate_buffer) ?;
	apply_all_or_nothing_encryption (&_fingerprint, &mut _intermediate_buffer) ?;
	
	let _fingerprint_actual = apply_fingerprint (&_intermediate_buffer) ?;
	
	if ! ::constant_time_eq::constant_time_eq (&_fingerprint_actual, &_fingerprint) {
		fail! (0x7c3ab20d);
	}
	
	// NOTE:  unwrapping...
	
	let _decoded_len = decode_u32_pop (&mut _intermediate_buffer) .else_wrap (0xa8d32a02) ? as usize;
	
	if _decoded_len > ARMOR_DECODED_SIZE_MAX {
		fail! (0xd0db488b);
	}
	
	// NOTE:  decompressing...
	
	let _decompress_buffer = if _decoded_len > _intermediate_buffer.len () {
		
		let mut _decompress_buffer = Vec::with_capacity (_decoded_len);
		decompress (&_intermediate_buffer, &mut _decompress_buffer) .else_wrap (0x70f5d0b4) ?;
		
		_decompress_buffer
	} else {
		_intermediate_buffer
	};
	
	if _decompress_buffer.len () != _decoded_len {
		fail! (0xc763571b);
	}
	
	// NOTE:  finalizing...
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_decoded.extend_from_slice (&_decompress_buffer);
	
	Ok (())
}








fn apply_all_or_nothing_encryption (_fingerprint : &[u8; ARMOR_ENCODED_FINGERPRINT], _data : &mut [u8]) -> ArmorResult {
	
	use ::chacha20::cipher::KeyIvInit as _;
	use ::chacha20::cipher::StreamCipher as _;
	
	let _key : [u8; 32] =
			::blake3::Hasher::new_derive_key (ARMOR_AONT_KEY_CONTEXT)
			.update (_fingerprint)
			.finalize ()
			.into ();
	
	let _nonce = [0u8; 12];
	
	let _key = ::chacha20::Key::from (_key);
	let _nonce = ::chacha20::Nonce::from (_nonce);
	
	let mut _cipher = ::chacha20::ChaCha20::new (&_key, &_nonce);
	
	_cipher.try_apply_keystream (_data) .else_wrap (0x1f4e248a) ?;
	
	Ok (())
}


fn apply_all_or_nothing_mangling (_fingerprint : &mut [u8; ARMOR_ENCODED_FINGERPRINT], _data : &[u8]) -> ArmorResult {
	
	let _hash =
			::blake3::Hasher::new ()
			.update (_data)
			.finalize ();
	
	let _hash = _hash.as_bytes ();
	
	for _index in 0 .. ARMOR_ENCODED_FINGERPRINT {
		_fingerprint[_index] ^= _hash[_index];
	}
	
	Ok (())
}




fn apply_fingerprint (_data : &[u8]) -> ArmorResult<[u8; ARMOR_ENCODED_FINGERPRINT]> {
	
	let _hash =
			::blake3::Hasher::new ()
			.update (_data)
			.finalize ();
	
	let mut _fingerprint = [0u8; ARMOR_ENCODED_FINGERPRINT];
	_fingerprint.copy_from_slice (& _hash.as_bytes () [.. ARMOR_ENCODED_FINGERPRINT]);
	
	Ok (_fingerprint)
}


