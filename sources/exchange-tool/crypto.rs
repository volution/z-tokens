

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use crate::keys::*;
use crate::coding::*;


use ::x25519_dalek as x25519;




define_error! (pub CryptoError, result : CryptoResult);




pub const CRYPTO_DECRYPTED_SIZE_MAX : usize = 128 * 1024 * 1024;
pub const CRYPTO_ENCRYPTED_SIZE_MAX : usize = CRYPTO_DECRYPTED_SIZE_MAX + 4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD;


pub const CRYPTO_ENCRYPTED_PADDING : usize = 256;
pub const CRYPTO_ENCRYPTED_OVERHEAD : usize = CRYPTO_ENCRYPTED_NONCE + CRYPTO_ENCRYPTED_MAC;
pub const CRYPTO_ENCRYPTED_NONCE : usize = 8;
pub const CRYPTO_ENCRYPTED_MAC : usize = 8;


static CRYPTO_ENCRYPTION_KEY_CONTEXT : &str = "z-tokens exchange encryption key (2023a)";
static CRYPTO_ENCRYPTION_NONCE_CONTEXT : &str = "z-tokens exchange encryption nonce (2023a)";
static CRYPTO_AUTHENTICATION_KEY_CONTEXT : &str = "z-tokens exchange authentication key (2023a)";








pub fn encrypt (_sender : &SenderPrivateKey, _recipient : &RecipientPublicKey, _decrypted : &[u8], _encrypted : &mut Vec<u8>) -> CryptoResult {
	
	let _decrypted_len = _decrypted.len ();
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x83d6c657);
	}
	
	let _compress_capacity = compress_capacity_max (_decrypted_len) .else_wrap (0x4198ca8b) ?;
	let _compress_capacity = _compress_capacity + 4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD;
	
	let mut _compress_buffer = Vec::with_capacity (_compress_capacity);
	compress (_decrypted, &mut _compress_buffer) .else_wrap (0xa9fadcdc) ?;
	
	{
		let mut _buffer = [0u8; 4];
		encode_u32 (_decrypted_len as u32, &mut _buffer);
		_compress_buffer.extend_from_slice (&_buffer);
	}
	
	{
		let _padding = CRYPTO_ENCRYPTED_PADDING - (_compress_buffer.len () % CRYPTO_ENCRYPTED_PADDING);
		assert! (_padding >= 1, "[0a1987ea]");
		assert! (_padding <= 256, "[d2c4f983]");
		let _padding = _padding as u8;
		for _ in 0 .. _padding {
			_compress_buffer.push (_padding);
		}
	}
	
	let mut _nonce = [0u8; CRYPTO_ENCRYPTED_NONCE];
	{
		use ::rand::RngCore as _;
		::rand::rngs::OsRng.fill_bytes (&mut _nonce);
	}
	
	let (_encrypt_key, _encrypt_nonce, _authentication_key) = derive_keys (&_sender.0.0, &_recipient.0.0, &_nonce) ?;
	
	apply_encryption (&_encrypt_key, &_encrypt_nonce, &mut _compress_buffer) ?;
	
	let mut _mac = [0u8; CRYPTO_ENCRYPTED_MAC];
	apply_authentication (&_authentication_key, &_compress_buffer, &mut _mac) ?;
	
	_compress_buffer.extend_from_slice (&_mac);
	
	_compress_buffer.extend_from_slice (&_nonce);
	
	let _encode_capacity = encode_capacity_max (_compress_buffer.len ()) .else_wrap (0x00bf84c9) ?;
	
	let mut _encode_buffer = Vec::with_capacity (_encode_capacity);
	encode (&_compress_buffer, &mut _encode_buffer) .else_wrap (0x5bc239f9) ?;
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_encrypted.extend_from_slice (&_encode_buffer);
	
	Ok (())
}




pub fn decrypt (_recipient : &RecipientPrivateKey, _sender : &SenderPublicKey, _encrypted : &[u8], _decrypted : &mut Vec<u8>) -> CryptoResult {
	
	let _encrypted_len = _encrypted.len ();
	
	if _encrypted_len > CRYPTO_ENCRYPTED_SIZE_MAX {
		fail! (0x5832104d);
	}
	
	let _decode_capacity = decode_capacity_max (_encrypted_len) .else_wrap (0xae545303) ?;
	
	let mut _decode_buffer = Vec::with_capacity (_decode_capacity);
	decode (_encrypted, &mut _decode_buffer) .else_wrap (0x10ff413a) ?;
	
	let mut _nonce = [0u8; CRYPTO_ENCRYPTED_NONCE];
	{
		let _decode_len = _decode_buffer.len ();
		if _decode_len < CRYPTO_ENCRYPTED_NONCE {
			fail! (0xbfead1cb);
		}
		_nonce.copy_from_slice (&_decode_buffer[(_decode_len - CRYPTO_ENCRYPTED_NONCE) .. _decode_len]);
		_decode_buffer.truncate (_decode_len - CRYPTO_ENCRYPTED_NONCE);
	}
	
	let (_encrypt_key, _encrypt_nonce, _authentication_key) = derive_keys (&_recipient.0.0, &_sender.0.0, &_nonce) ?;
	
	let mut _mac_expected = [0u8; CRYPTO_ENCRYPTED_MAC];
	{
		let _decode_len = _decode_buffer.len ();
		if _decode_len < CRYPTO_ENCRYPTED_MAC {
			fail! (0xb40635e3);
		}
		_mac_expected.copy_from_slice (&_decode_buffer[(_decode_len - CRYPTO_ENCRYPTED_MAC) .. _decode_len]);
		_decode_buffer.truncate (_decode_len - CRYPTO_ENCRYPTED_MAC);
	}
	
	let mut _mac_actual = [0u8; CRYPTO_ENCRYPTED_MAC];
	apply_authentication (&_authentication_key, &_decode_buffer, &mut _mac_actual) ?;
	
	if ! ::constant_time_eq::constant_time_eq (&_mac_actual, &_mac_expected) {
		fail! (0xad70c84c);
	}
	
	apply_encryption (&_encrypt_key, &_encrypt_nonce, &mut _decode_buffer) ?;
	
	{
		let _decode_len = _decode_buffer.len ();
		if _decode_len <= 1 {
			fail! (0x04d212d0);
		}
		
		let _padding = _decode_buffer[_decode_len - 1];
		if _padding < 1 {
			fail! (0x628e3a2b);
		}
		if _decode_len < (_padding as usize) {
			fail! (0xe17b846c);
		}
		for _padding_offset in 0 .. (_padding as usize) {
			let _padding_actual = _decode_buffer[_decode_len - _padding_offset - 1];
			if _padding_actual != _padding {
				fail! (0x1f66027e);
			}
		}
		
		_decode_buffer.truncate (_decode_len - (_padding as usize));
	}
	
	let mut _decrypted_len : usize;
	{
		let _decode_len = _decode_buffer.len ();
		if _decode_len < 4 {
			fail! (0x60af3a4c);
		}
		_decrypted_len = decode_u32_slice (&_decode_buffer[_decode_len - 4 ..]) as usize;
		_decode_buffer.truncate (_decode_len - 4);
	}
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x433f5bb6);
	}
	
	let mut _decompress_buffer = Vec::with_capacity (_decrypted_len);
	decompress (&_decode_buffer, &mut _decompress_buffer) .else_wrap (0xec71bc5c) ?;
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_decrypted.extend_from_slice (&_decompress_buffer);
	
	Ok (())
}








fn apply_encryption (_key : &[u8; 32], _nonce : &[u8; 32], _data : &mut [u8]) -> CryptoResult {
	
	use ::salsa20::cipher::KeyIvInit as _;
	use ::salsa20::cipher::StreamCipher as _;
	
	let _nonce = &_nonce[0 .. CRYPTO_ENCRYPTED_NONCE];
	
	let _key = ::salsa20::Key::from_slice (_key);
	let _nonce = ::salsa20::Nonce::from_slice (_nonce);
	
	let mut _cipher = ::salsa20::Salsa20::new (&_key, &_nonce);
	
	_cipher.try_apply_keystream (_data) .else_wrap (0x9c94d0d5) ?;
	
	Ok (())
}




fn apply_authentication (_key : &[u8; 32], _data : &[u8], _mac : &mut [u8; CRYPTO_ENCRYPTED_MAC]) -> CryptoResult {
	
	assert! (_mac.len () == CRYPTO_ENCRYPTED_MAC, "[bbcf8472]");
	
	let _hash =
			::blake3::Hasher::new_keyed (_key)
			.update (_data)
			.finalize ();
	
	_mac.copy_from_slice (& _hash.as_bytes () [0 .. CRYPTO_ENCRYPTED_MAC]);
	
	Ok (())
}




fn derive_keys (_private : &x25519::StaticSecret, _public : &x25519::PublicKey, _nonce : &[u8; CRYPTO_ENCRYPTED_NONCE]) -> CryptoResult<([u8; 32], [u8; 32], [u8; 32])> {
	
	let _shared = x25519::StaticSecret::diffie_hellman (_private, _public);
	let _shared = _shared.as_bytes ();
	
	let _encryption_key =
			::blake3::Hasher::new_derive_key (CRYPTO_ENCRYPTION_KEY_CONTEXT)
			.update (_shared)
			.update (_nonce)
			.finalize ()
			.into ();
	
	let _encryption_nonce =
			::blake3::Hasher::new_derive_key (CRYPTO_ENCRYPTION_NONCE_CONTEXT)
			.update (_shared)
			.update (_nonce)
			.finalize ()
			.into ();
	
	let _authentication_key =
			::blake3::Hasher::new_derive_key (CRYPTO_AUTHENTICATION_KEY_CONTEXT)
			.update (_shared)
			.update (_nonce)
			.finalize ()
			.into ();
	
	Ok ((_encryption_key, _encryption_nonce, _authentication_key))
}








fn encrypted_max_len (_decrypted_len : usize) -> CryptoResult<usize> {
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0xf486af87);
	}
	
	let _encrypted_len = ((((_decrypted_len + 4) / CRYPTO_ENCRYPTED_PADDING) + 1) * CRYPTO_ENCRYPTED_PADDING) + CRYPTO_ENCRYPTED_OVERHEAD;
	
	Ok (_encrypted_len)
}


fn decrypted_max_len (_encrypted_len : usize) -> CryptoResult<usize> {
	
	if _encrypted_len > CRYPTO_ENCRYPTED_SIZE_MAX {
		fail! (0x2f099ff9);
	}
	
	if _encrypted_len < (4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD) {
		fail! (0xafb25d04);
	}
	
	let _decrypted_len = _encrypted_len - (4 + CRYPTO_ENCRYPTED_OVERHEAD + CRYPTO_ENCRYPTED_PADDING);
	
	Ok (_decrypted_len)
}


