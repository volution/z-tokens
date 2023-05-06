

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;








define_error! (pub EncodingError, result : EncodingResult);

define_error! (pub(crate) CompressionError, result : CompressionResult);




pub(crate) const CODING_CHUNK_DECODED_SIZE : usize = 8;
pub(crate) const CODING_CHUNK_ENCODED_SIZE : usize = 13;

pub(crate) const CODING_CHUNKS_PER_LINE : usize = 5;
pub(crate) const CODING_LINES_PER_BLOCK : usize = 20;




// FIXME:  https://github.com/google/brotli/issues/274
const COMPRESSION_OVERHEAD_FRACTION : usize = 1;
const COMPRESSION_OVERHEAD_DIVIDER : usize = 16;
const COMPRESSION_OVERHEAD_EXTRA : usize = 1024;


const COMPRESSION_BROTLI_Q : u32 = 9;
const COMPRESSION_BROTLI_LGWIN : u32 = 24;

const COMPRESSION_BROTLI_BLOCK : usize = 128 * 1024;








pub fn encode (_decoded : &[u8], _buffer : &mut Vec<u8>) -> EncodingResult {
	
	let _buffer_capacity = _buffer.capacity ();
	
	let mut _decode_buffer = [0u8; CODING_CHUNK_DECODED_SIZE + 1];
	let mut _encode_buffer = [0u8; CODING_CHUNK_ENCODED_SIZE];
	
	let mut _offset : u32 = 0;
	let mut _crc_buffer = [0u8; 4];
	let _decoded_len = _decoded.len ();
	
	let _encode_lines_count =
			if _decoded_len > 0 {
				(_decoded_len / (CODING_CHUNK_DECODED_SIZE * CODING_CHUNKS_PER_LINE))
				+ if (_decoded_len % (CODING_CHUNK_DECODED_SIZE * CODING_CHUNKS_PER_LINE)) == 0 { 0 } else { 1 }
			} else {
				0
			};
	let _encode_line_width =
			if _encode_lines_count >= 2 {
				(_encode_lines_count.ilog10 () as usize) + 1
			} else {
				0
			};
	
	if _encode_line_width > 0 {
		_buffer.push (b'\n');
	}
	
	let mut _encode_size_last = 0;
	let mut _encode_line_index = 0;
	for (_index, _decoded_chunk) in _decoded.chunks (CODING_CHUNK_DECODED_SIZE) .enumerate () {
		
		let _decoded_chunk_len = _decoded_chunk.len ();
		_decode_buffer[.. _decoded_chunk_len].copy_from_slice (_decoded_chunk);
		
		_offset += _decoded_chunk_len as u32;
		encode_u32 (_offset, &mut _crc_buffer);
		
		let mut _crc = ::crc_any::CRCu8::crc8 ();
		_crc.digest (_decoded_chunk);
		_crc.digest (&_crc_buffer);
		
		if (_offset as usize) == _decoded_len {
			encode_u32 (u32::MAX, &mut _crc_buffer);
			_crc.digest (&_crc_buffer);
		}
		
		_decode_buffer[_decoded_chunk_len] = _crc.get_crc ();
		
		if (_index % CODING_CHUNKS_PER_LINE) == 0 {
			if _index > 0 {
				_buffer.push (b'\n');
				_encode_line_index += 1;
			}
			if _encode_line_width > 0 {
				if (_encode_line_index > 0) && (_encode_line_index % CODING_LINES_PER_BLOCK) == 0 {
					_buffer.push (b'\n');
				}
				if (_encode_line_index + 1) < _encode_lines_count {
					write! (_buffer, "#{0:01$},  ", _encode_line_index + 1, _encode_line_width) .else_panic (0xa7e364d5);
				} else {
					write! (_buffer, "#{0:01$}.  ", _encode_line_index + 1, _encode_line_width) .else_panic (0x83426d64);
				}
			}
		} else {
			if _index > 0 {
				for _ in _encode_size_last ..= CODING_CHUNK_ENCODED_SIZE {
					_buffer.push (b' ');
				}
			}
		}
		
		let _encode_size =
				::bs58::encode (&_decode_buffer[..= _decoded_chunk_len])
				.with_alphabet (::bs58::Alphabet::BITCOIN)
				.into (_encode_buffer.as_mut_slice ())
				.else_wrap (0xafe90906) ?;
		
		_buffer.extend_from_slice (&_encode_buffer[.. _encode_size]);
		
		_encode_size_last = _encode_size;
	}
	
	if _encode_line_width > 0 {
		_buffer.push (b'\n');
	}
	
	if _decoded_len > 0 {
		_buffer.push (b'\n');
		assert! ((_encode_line_index + 1) == _encode_lines_count, "[27c3972a] {} {}", _encode_line_index, _encode_lines_count);
	} else {
		assert! (_encode_lines_count == 0, "[544feca9]");
		assert! (_encode_line_index == 0, "[0c3a15c8]");
	}
	
	assert! (_buffer.capacity () == _buffer_capacity, "[9360b1c0]  {} == {}", _buffer.capacity (), _buffer_capacity);
	
	Ok (())
}




pub fn decode (_encoded : &[u8], _buffer : &mut Vec<u8>) -> EncodingResult {
	
	let _buffer_capacity = _buffer.capacity ();
	
	let mut _decode_buffer = [0u8; CODING_CHUNK_DECODED_SIZE + 1];
	
	let mut _offset : u32 = 0;
	let mut _crc_buffer = [0u8; 4];
	
	let mut _end_of_chunks = false;
	for _encoded_chunk in _encoded.split (u8::is_ascii_whitespace) {
		
		if _encoded_chunk.is_empty () {
			continue;
		}
		
		if _encoded_chunk[0] == b'#' {
			continue;
		}
		
		if _end_of_chunks {
			fail! (0x379115d2);
		}
		
		let _decode_size =
				::bs58::decode (_encoded_chunk)
				.with_alphabet (::bs58::Alphabet::BITCOIN)
				.into (_decode_buffer.as_mut_slice ())
				.else_wrap (0x5bd4757f) ?;
		
		if _decode_size <= 1 {
			fail! (0xd5487640);
		}
		let _crc_expected = _decode_buffer[_decode_size - 1];
		
		_offset += _decode_size as u32 - 1;
		encode_u32 (_offset, &mut _crc_buffer);
		
		let mut _crc = ::crc_any::CRCu8::crc8 ();
		_crc.digest (&_decode_buffer[.. (_decode_size - 1)]);
		_crc.digest (&_crc_buffer);
		
		if _crc_expected != _crc.get_crc () {
			
			encode_u32 (u32::MAX, &mut _crc_buffer);
			_crc.digest (&_crc_buffer);
			
			if _crc_expected != _crc.get_crc () {
				fail! (0xa4ba6e58);
			} else {
				_end_of_chunks = true;
			}
		}
		
		_buffer.extend_from_slice (&_decode_buffer[.. (_decode_size - 1)]);
	}
	
	// NOTE:  Sometimes (1 in 256) the CRC will succeed without requiring the end-of-chunks application.
	//if ! _end_of_chunks {
	//	fail! (0x924663bd);
	//}
	
	assert! (_buffer.capacity () == _buffer_capacity, "[6d624ac5]  {} == {}", _buffer.capacity (), _buffer_capacity);
	
	Ok (())
}




pub fn encode_capacity_max (_decoded_len : usize) -> EncodingResult<usize> {
	
	let _chunks = (_decoded_len / CODING_CHUNK_DECODED_SIZE) + 1;
	
	// NOTE:  Let's assume we write at most 10 million lines.
	let _encoded_len = _chunks * (CODING_CHUNK_ENCODED_SIZE + 1 + 9 + 4);
	
	Ok (_encoded_len)
}


pub fn decode_capacity_max (_encoded_len : usize) -> EncodingResult<usize> {
	
	// NOTE:  Some tokens are shorter.
	// NOTE:  We don't take into account line numbers, thus the resulting number is an overestimation.
	let _chunks = (_encoded_len / (CODING_CHUNK_ENCODED_SIZE - 2 + 1)) + 1;
	
	let _decoded_len = _chunks * CODING_CHUNK_DECODED_SIZE;
	
	Ok (_decoded_len)
}








pub(crate) fn compress (_data : &[u8], _buffer : &mut Vec<u8>) -> CompressionResult {
	
	let _buffer_capacity = _buffer.capacity ();
	
	let mut _encoder = ::brotli::CompressorWriter::new (_buffer, COMPRESSION_BROTLI_BLOCK, COMPRESSION_BROTLI_Q, COMPRESSION_BROTLI_LGWIN);
	
	_encoder.write_all (_data) .else_wrap (0x7ea342b9) ?;
	_encoder.flush () .else_wrap (0xb5560900) ?;
	let _buffer = _encoder.into_inner ();
	
	assert! (_buffer.capacity () == _buffer_capacity, "[af54fcfc]  {} == {}", _buffer.capacity (), _buffer_capacity);
	
	Ok (())
}




pub(crate) fn decompress (_data : &[u8], _buffer : &mut Vec<u8>) -> CompressionResult {
	
	let _buffer_capacity = _buffer.capacity ();
	
	let mut _decoder = ::brotli::Decompressor::new (_data, COMPRESSION_BROTLI_BLOCK);
	
	_decoder.read_to_end (_buffer) .else_wrap (0xf20a0822) ?;
	
	assert! (_buffer.capacity () == _buffer_capacity, "[630ddcba]  {} == {}", _buffer.capacity (), _buffer_capacity);
	
	Ok (())
}




pub(crate) fn compress_capacity_max (_uncompressed_len : usize) -> CompressionResult<usize> {
		
	Ok (_uncompressed_len + (_uncompressed_len * COMPRESSION_OVERHEAD_FRACTION / COMPRESSION_OVERHEAD_DIVIDER) + COMPRESSION_OVERHEAD_EXTRA)
}








#[ allow (dead_code) ]
pub(crate) fn encode_u32_into (_value : u32) -> [u8; 4] {
	let mut _buffer = [0u8; 4];
	encode_u32 (_value, &mut _buffer);
	_buffer
}

pub(crate) fn encode_u32 (_value : u32, _buffer : &mut [u8; 4]) -> () {
	encode_u32_slice (_value, _buffer.as_mut_slice ())
}

#[ allow (dead_code) ]
#[ must_use ]
pub(crate) fn decode_u32 (_buffer : &[u8; 4]) -> u32 {
	decode_u32_slice (_buffer.as_slice ())
}


pub(crate) fn encode_u32_slice (_value : u32, _buffer : &mut [u8]) -> () {
	use ::byteorder::ByteOrder as _;
	::byteorder::BigEndian::write_u32 (_buffer, _value);
}

#[ must_use ]
pub(crate) fn decode_u32_slice (_buffer : &[u8]) -> u32 {
	use ::byteorder::ByteOrder as _;
	::byteorder::BigEndian::read_u32 (_buffer)
}


pub(crate) fn encode_u32_push (_value : u32, _buffer : &mut Vec<u8>) -> () {
	_buffer.push (0); _buffer.push (0); _buffer.push (0); _buffer.push (0);
	let _buffer_len = _buffer.len ();
	encode_u32_slice (_value, &mut _buffer[_buffer_len - 4 ..]);
}

#[ must_use ]
pub(crate) fn decode_u32_pop (_buffer : &mut Vec<u8>) -> Option<u32> {
	
	let _buffer_len = _buffer.len ();
	if _buffer_len < 4 {
		return None;
	}
	
	let _value = decode_u32_slice (&_buffer[_buffer_len - 4 ..]);
	
	_buffer.truncate (_buffer_len - 4);
	
	Some (_value)
}








#[ must_use ]
pub(crate) fn bytes_pop <const SIZE : usize> (_buffer : &mut Vec<u8>) -> Option<[u8; SIZE]> {
	
	let _buffer_len = _buffer.len ();
	if _buffer_len < SIZE {
		return None;
	}
	
	let mut _bytes = [0u8; SIZE];
	_bytes.copy_from_slice (&_buffer[(_buffer_len - SIZE) ..]);
	
	_buffer.truncate (_buffer_len - SIZE);
	
	Some (_bytes)
}








pub(crate) fn padding_push (_offset : usize, _alignment : usize, _buffer : &mut Vec<u8>) -> () {
	
	let _buffer_len = _buffer.len ();
	
	assert! (_offset <= _buffer_len, "[ca72a353]");
	
	assert! (_alignment > 0, "[cdc52ac7]");
	assert! (_alignment <= 256, "[9d23d229]");
	
	let _padding = _alignment - ((_buffer_len - _offset) % _alignment);
	
	assert! (_padding >= 1, "[0a1987ea]");
	assert! (_padding <= _alignment, "[d2c4f983]");
	
	for _index in 0 .. _padding {
		let _remaining = _index as u8;
		_buffer.push (_remaining);
	}
	
	assert! (_buffer.len () == (_buffer_len + _padding), "[e295c14b]");
	assert! (((_buffer.len () - _offset) % _alignment) == 0, "[4471a66f]");
}


pub(crate) fn padding_pop (_offset : usize, _alignment : usize, _buffer : &mut Vec<u8>) -> EncodingResult {
	
	let _buffer_len = _buffer.len ();
	
	assert! (_alignment > 0, "[cbf1cbaf]");
	assert! (_alignment <= 256, "[a5b18bae]");
	
	if _offset > _buffer_len {
		fail! (0x3c92d18e);
	}
	if (_buffer_len - _offset) < _alignment {
		fail! (0x04d212d0);
	}
	if ((_buffer_len - _offset) % _alignment) != 0 {
		fail! (0x25bfe610);
	}
	
	let _padding = 1 + _buffer[_buffer_len - 1] as usize;
	
	if _padding > _alignment {
		fail! (0x1d74fcde);
	}
	if (_buffer_len - _offset) < _padding {
		fail! (0xe17b846c);
	}
	
	for _index in 0 .. _padding {
		let _remaining_expected = _index as u8;
		let _remaining_actual = _buffer[_buffer_len - _padding + _index];
		if _remaining_actual != _remaining_expected {
			fail! (0x1f66027e);
		}
	}
	
	_buffer.truncate (_buffer_len - _padding);
	
	Ok (())
}


