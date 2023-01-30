

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use crate::coding::*;




define_error! (pub ArmorError, result : ArmorResult);




pub(crate) const ARMOR_DECODED_SIZE_MAX : usize = 128 * 1024 * 1024;
pub(crate) const ARMOR_ENCODED_SIZE_MAX : usize = ((ARMOR_DECODED_SIZE_MAX / CODING_CHUNK_DECODED_SIZE) + 1) * (CODING_CHUNK_ENCODED_SIZE + 1) + 4;




pub fn armor (_decoded : &[u8], _encoded : &mut Vec<u8>) -> ArmorResult {
	
	let _decoded_len = _decoded.len ();
	
	if _decoded_len > ARMOR_DECODED_SIZE_MAX {
		fail! (0x3463e357);
	}
	
	let _compress_capacity = compress_capacity_max (_decoded_len) .else_wrap (0xd7e27086) ?;
	let _compress_capacity = _compress_capacity + 4;
	
	let mut _compress_buffer = Vec::with_capacity (_compress_capacity);
	compress (_decoded, &mut _compress_buffer) .else_wrap (0x08e19178) ?;
	
	encode_u32_push (_decoded_len as u32, &mut _compress_buffer);
	
	let _encode_capacity = encode_capacity_max (_compress_buffer.len ()) .else_wrap (0x00bf84c9) ?;
	
	let mut _encode_buffer = Vec::with_capacity (_encode_capacity);
	encode (&_compress_buffer, &mut _encode_buffer) .else_wrap (0x080c7733) ?;
	
	if _encode_buffer.len () > ARMOR_ENCODED_SIZE_MAX {
		fail! (0xb894e943);
	}
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_encoded.extend_from_slice (&_encode_buffer);
	
	Ok (())
}




pub fn dearmor (_encoded : &[u8], _decoded : &mut Vec<u8>) -> ArmorResult {
	
	let _encoded_len = _encoded.len ();
	
	if _encoded_len > ARMOR_ENCODED_SIZE_MAX {
		fail! (0xe141a81a);
	}
	
	let _decode_capacity = decode_capacity_max (_encoded_len) .else_wrap (0x7321f5b4) ?;
	
	let mut _decode_buffer = Vec::with_capacity (_decode_capacity);
	decode (_encoded, &mut _decode_buffer) .else_wrap (0x6432ccd9) ?;
	
	let _decoded_len = decode_u32_pop (&mut _decode_buffer) .else_wrap (0xa8d32a02) ? as usize;
	
	if _decoded_len > ARMOR_DECODED_SIZE_MAX {
		fail! (0x433f5bb6);
	}
	
	let mut _decompress_buffer = Vec::with_capacity (_decoded_len);
	decompress (&_decode_buffer, &mut _decompress_buffer) .else_wrap (0x70f5d0b4) ?;
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_decoded.extend_from_slice (&_decompress_buffer);
	
	Ok (())
}



