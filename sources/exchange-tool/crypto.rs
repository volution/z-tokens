

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use crate::keys::*;


use ::z_tokens_runtime::{
		sensitive::zeroize_and_drop,
		sensitive::Zeroize as _,
	};


use ::x25519_dalek as x25519;




define_error! (pub CryptoError, result : CryptoResult);

define_error! (CompressionError, result : CompressionResult);




pub const CRYPTO_PADDING : usize = 256;
pub const CRYPTO_OVERHEAD : usize = 64 + 4;

pub const CRYPTO_DECRYPTED_SIZE_MAX : usize = 128 * 1024 * 1024;
pub const CRYPTO_ENCRYPTED_SIZE_MAX : usize = CRYPTO_DECRYPTED_SIZE_MAX + CRYPTO_PADDING + CRYPTO_OVERHEAD;

pub(crate) const CRYPTO_NAMESPACE : &[u8] = b"f6eee60c50cdf7d06d97e65f739f0086913c7f36fa7e86bd4962aa17ed3f0f37";








pub fn encrypt (_sender : &SenderPrivateKey, _recipient : &RecipientPublicKey, _decrypted : &[u8], _encrypted : &mut Vec<u8>) -> CryptoResult {
	
	// FIXME:  On `Err` things aren't zeroized!
	
	let _encrypt_max_len = encrypted_max_len (_decrypted.len ()) ?;
	
	let mut _compress_buffer = Vec::with_capacity (_decrypted.len () + BROTLI_BLOCK);
	compress (_decrypted, &mut _compress_buffer) .else_wrap (0xa9fadcdc) ?;
	
	let mut _encrypt_buffer = Vec::with_capacity (_encrypt_max_len);
	_encrypt_buffer.extend_from_slice (&_compress_buffer);
	
	let _decompressed_len = _decrypted.len ();
	{
		let mut _buffer = [0u8; 4];
		use ::byteorder::ByteOrder as _;
		::byteorder::BigEndian::write_u32 (&mut _buffer, _decompressed_len as u32);
		_encrypt_buffer.extend_from_slice (&_buffer);
	}
	
	{
		let _padding = CRYPTO_PADDING - (_encrypt_buffer.len () % CRYPTO_PADDING);
		assert! (_padding >= 1, "[0a1987ea]");
		assert! (_padding <= 256, "[d2c4f983]");
		let _padding = _padding as u8;
		for _ in 0 .. _padding {
			_encrypt_buffer.push (_padding);
		}
	}
	
	let _shared = _sender.0.0.diffie_hellman (&_recipient.0.0);
	
	apply_salsa20 (&_shared, &mut _encrypt_buffer) ?;
	
	// ...
	
	let mut _encode_buffer = Vec::new ();
	_encode_buffer.resize ((_encrypt_buffer.len () / 5 + 1) * 7, 0);
	let _encode_buffer_size =
			::bs58::encode (&_encrypt_buffer)
			.with_alphabet (::bs58::Alphabet::BITCOIN)
			.into (_encode_buffer.as_mut_slice ())
			.else_wrap (0xafe90906) ?;
	_encode_buffer.truncate (_encode_buffer_size);
	
	_encrypted.extend_from_slice (&_encode_buffer);
	
	zeroize_and_drop (_encrypt_buffer);
	zeroize_and_drop (_compress_buffer);
	
	Ok (())
}




pub fn decrypt (_recipient : &RecipientPrivateKey, _sender : &SenderPublicKey, _encrypted : &[u8], _decrypted : &mut Vec<u8>) -> CryptoResult {
	
	// FIXME:  On `Err` things aren't zeroized!
	
	let _decrypt_max_len = decrypted_max_len (_encrypted.len ()) ?;
	
	let mut _decrypt_buffer = Vec::new ();
	_decrypt_buffer.resize ((_encrypted.len () / 5 + 1) * 4, 0);
	let _decrypt_buffer_size =
			::bs58::decode (_encrypted)
			.with_alphabet (::bs58::Alphabet::BITCOIN)
			.into (_decrypt_buffer.as_mut_slice ())
			.else_wrap (0x5bd4757f) ?;
	_decrypt_buffer.truncate (_decrypt_buffer_size);
	
	let _shared = _recipient.0.0.diffie_hellman (&_sender.0.0);
	
	apply_salsa20 (&_shared, &mut _decrypt_buffer) ?;
	
	{
		let _padding = * _decrypt_buffer.last () .unwrap ();
		assert! (_padding >= 1, "[a296d085]");
		for _ in 0 .. _padding {
			let _padding_actual = _decrypt_buffer.pop () .unwrap ();
			if _padding_actual != _padding {
				fail! (0x1f66027e);
			}
		}
	}
	
	let mut _decompress_len : usize;
	{
		let _buffer = &_decrypt_buffer[_decrypt_buffer.len () - 4 ..];
		use ::byteorder::ByteOrder as _;
		_decompress_len = ::byteorder::BigEndian::read_u32 (_buffer) as usize;
		_decrypt_buffer.truncate (_decrypt_buffer.len () - 4);
	}
	
	let mut _decompress_buffer = Vec::with_capacity (_decompress_len);
	decompress (&_decrypt_buffer, &mut _decompress_buffer) .else_wrap (0x0a2de8ec) ?;
	
	_decrypted.extend_from_slice (&_decompress_buffer);
	
	zeroize_and_drop (_decrypt_buffer);
	zeroize_and_drop (_decompress_buffer);
	
	Ok (())
}




fn apply_salsa20 (_shared : &x25519::SharedSecret, _data : &mut [u8]) -> CryptoResult {
	
	use ::salsa20::cipher::KeyIvInit as _;
	use ::salsa20::cipher::StreamCipher as _;
	
	let _key = ::salsa20::Key::from_slice (_shared.as_bytes ());
	
	// FIXME!
	let _nonce = ::salsa20::Nonce::from ([0u8; 8]);
	
	let mut _cipher = ::salsa20::Salsa20::new (&_key, &_nonce);
	
	_cipher.try_apply_keystream (_data) .else_wrap (0x9c94d0d5) ?;
	
	Ok (())
}








fn encrypted_max_len (_decrypted_len : usize) -> CryptoResult<usize> {
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0xf486af87);
	}
	
	let _encrypted_len = (((_decrypted_len / CRYPTO_PADDING) + 1) * CRYPTO_PADDING) + CRYPTO_OVERHEAD;
	
	Ok (_encrypted_len)
}


fn decrypted_max_len (_encrypted_len : usize) -> CryptoResult<usize> {
	
	if _encrypted_len > CRYPTO_ENCRYPTED_SIZE_MAX {
		fail! (0x2f099ff9);
	}
	
//	if _encrypted_len < (CRYPTO_PADDING + CRYPTO_OVERHEAD) {
//		fail! (0xafb25d04);
//	}
	
//	let _decrypted_len = _encrypted_len - CRYPTO_PADDING - CRYPTO_OVERHEAD;
	let _decrypted_len = _encrypted_len;
	
	Ok (_decrypted_len)
}








fn compress (_data : &[u8], _buffer : &mut Vec<u8>) -> CompressionResult {
	
	_buffer.zeroize ();
	let _buffer_capacity = _buffer.capacity ();
	
	let mut _encoder = ::brotli::CompressorWriter::new (_buffer, BROTLI_BLOCK, BROTLI_Q, BROTLI_LGWIN);
	_encoder.write_all (_data) .else_wrap (0x7ea342b9) ?;
	_encoder.flush () .else_wrap (0xb5560900) ?;
	let _buffer = _encoder.into_inner ();
	
	assert! (_buffer.capacity () == _buffer_capacity, "[af54fcfc]");
	
	Ok (())
}


fn decompress (_data : &[u8], _buffer : &mut Vec<u8>) -> CompressionResult {
	
	_buffer.zeroize ();
	let _buffer_capacity = _buffer.capacity ();
	
	let mut _decoder = ::brotli::Decompressor::new (_data, BROTLI_BLOCK);
	_decoder.read_to_end (_buffer) .else_wrap (0xf20a0822) ?;
	
	assert! (_buffer.capacity () == _buffer_capacity, "[630ddcba]");
	
	Ok (())
}


const BROTLI_Q : u32 = 9;
const BROTLI_LGWIN : u32 = 24;
const BROTLI_BLOCK : usize = 16 * 1024;

