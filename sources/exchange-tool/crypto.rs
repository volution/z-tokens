

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use crate::keys::*;
use crate::coding::*;


use ::x25519_dalek as x25519;








define_error! (pub CryptoError, result : CryptoResult);




pub const CRYPTO_DECRYPTED_SIZE_MAX : usize = 128 * 1024 * 1024;

pub const CRYPTO_ENCRYPTED_SIZE_MAX : usize =
		(
			(
				(
					CRYPTO_DECRYPTED_SIZE_MAX
					+ (CRYPTO_DECRYPTED_SIZE_MAX * COMPRESSION_OVERHEAD_FRACTION / COMPRESSION_OVERHEAD_DIVIDER) + COMPRESSION_OVERHEAD_EXTRA
					+ 4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD
				) / CODING_CHUNK_DECODED_SIZE
			) + 1
		) * (CODING_CHUNK_ENCODED_SIZE + 1);


pub const CRYPTO_ENCRYPTED_PADDING : usize = 255;
pub const CRYPTO_ENCRYPTED_OVERHEAD : usize = CRYPTO_ENCRYPTED_SALT + CRYPTO_ENCRYPTED_MAC;
pub const CRYPTO_ENCRYPTED_SALT : usize = 16;
pub const CRYPTO_ENCRYPTED_MAC : usize = 16;


static CRYPTO_ENCRYPTION_KEY_CONTEXT : &str = "z-tokens exchange encryption key (2023a)";
static CRYPTO_AUTHENTICATION_KEY_CONTEXT : &str = "z-tokens exchange authentication key (2023a)";
static CRYPTO_PIN_CONTEXT : &str = "z-tokens exchange pin (2023a)";








pub fn encrypt (_sender : &SenderPrivateKey, _recipient : &RecipientPublicKey, _decrypted : &[u8], _encrypted : &mut Vec<u8>, _pin : Option<&[u8]>) -> CryptoResult {
	
	let _decrypted_len = _decrypted.len ();
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x83d6c657);
	}
	
	let _compress_capacity = compress_capacity_max (_decrypted_len) .else_wrap (0x4198ca8b) ?;
	let _compress_capacity = _compress_capacity + 4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD;
	
	let mut _compress_buffer = Vec::with_capacity (_compress_capacity);
	compress (_decrypted, &mut _compress_buffer) .else_wrap (0xa9fadcdc) ?;
	
	encode_u32_push (_decrypted_len as u32, &mut _compress_buffer);
	
	padding_push (CRYPTO_ENCRYPTED_PADDING, &mut _compress_buffer);
	
	let mut _salt = generate_salt () ?;
	
	let (_encryption_key, _authentication_key) = derive_keys (&_sender.0.0, &_recipient.0.0, &_salt, _pin) ?;
	
	apply_encryption (&_encryption_key, &mut _compress_buffer) ?;
	
	let _mac = apply_authentication (&_authentication_key, &_compress_buffer) ?;
	
	_compress_buffer.extend_from_slice (&_mac);
	
	apply_all_or_nothing_mangling (&mut _salt, &_compress_buffer) ?;
	
	_compress_buffer.extend_from_slice (&_salt);
	
	let _encode_capacity = encode_capacity_max (_compress_buffer.len ()) .else_wrap (0x7f15a8ec) ?;
	
	let mut _encode_buffer = Vec::with_capacity (_encode_capacity);
	encode (&_compress_buffer, &mut _encode_buffer) .else_wrap (0x5bc239f9) ?;
	
	assert! (_encode_buffer.len () <= CRYPTO_ENCRYPTED_SIZE_MAX, "[bb3c2546]  {} <= {}", _encode_buffer.len (), CRYPTO_ENCRYPTED_SIZE_MAX);
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_encrypted.extend_from_slice (&_encode_buffer);
	
	Ok (())
}




pub fn decrypt (_recipient : &RecipientPrivateKey, _sender : &SenderPublicKey, _encrypted : &[u8], _decrypted : &mut Vec<u8>, _pin : Option<&[u8]>) -> CryptoResult {
	
	let _encrypted_len = _encrypted.len ();
	
	if _encrypted_len > CRYPTO_ENCRYPTED_SIZE_MAX {
		fail! (0x5832104d);
	}
	
	let _decode_capacity = decode_capacity_max (_encrypted_len) .else_wrap (0xae545303) ?;
	
	let mut _decode_buffer = Vec::with_capacity (_decode_capacity);
	decode (_encrypted, &mut _decode_buffer) .else_wrap (0x10ff413a) ?;
	
	let mut _salt = bytes_pop::<CRYPTO_ENCRYPTED_SALT> (&mut _decode_buffer) .else_wrap (0x78ed3811) ?;
	
	apply_all_or_nothing_mangling (&mut _salt, &_decode_buffer) ?;
	
	let (_encryption_key, _authentication_key) = derive_keys (&_recipient.0.0, &_sender.0.0, &_salt, _pin) ?;
	
	let _mac_expected = bytes_pop::<CRYPTO_ENCRYPTED_MAC> (&mut _decode_buffer) .else_wrap (0x88084589) ?;
	
	let _mac_actual = apply_authentication (&_authentication_key, &_decode_buffer) ?;
	
	if ! ::constant_time_eq::constant_time_eq (&_mac_actual, &_mac_expected) {
		fail! (0xad70c84c);
	}
	
	apply_encryption (&_encryption_key, &mut _decode_buffer) ?;
	
	padding_pop (CRYPTO_ENCRYPTED_PADDING, &mut _decode_buffer) .else_wrap (0xbbdd100e) ?;
	
	let _decrypted_len = decode_u32_pop (&mut _decode_buffer) .else_wrap (0xa8b8f7d8) ? as usize;
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x433f5bb6);
	}
	
	let mut _decompress_buffer = Vec::with_capacity (_decrypted_len);
	decompress (&_decode_buffer, &mut _decompress_buffer) .else_wrap (0xec71bc5c) ?;
	
	if _decompress_buffer.len () != _decrypted_len {
		fail! (0x0610eb74);
	}
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_decrypted.extend_from_slice (&_decompress_buffer);
	
	Ok (())
}








fn apply_encryption (_key : &[u8; 32], _data : &mut [u8]) -> CryptoResult {
	
	use ::salsa20::cipher::KeyIvInit as _;
	use ::salsa20::cipher::StreamCipher as _;
	
	let _nonce = [0u8; 8];
	
	let _key = ::salsa20::Key::from_slice (_key);
	let _nonce = ::salsa20::Nonce::from (_nonce);
	
	let mut _cipher = ::salsa20::Salsa20::new (&_key, &_nonce);
	
	_cipher.try_apply_keystream (_data) .else_wrap (0x9c94d0d5) ?;
	
	Ok (())
}




fn apply_authentication (_key : &[u8; 32], _data : &[u8]) -> CryptoResult<[u8; CRYPTO_ENCRYPTED_MAC]> {
	
	let _hash =
			::blake3::Hasher::new_keyed (_key)
			.update (_data)
			.finalize ();
	
	let mut _mac = [0u8; CRYPTO_ENCRYPTED_MAC];
	_mac.copy_from_slice (& _hash.as_bytes () [.. CRYPTO_ENCRYPTED_MAC]);
	
	Ok (_mac)
}




fn derive_keys (_private : &x25519::StaticSecret, _public : &x25519::PublicKey, _salt : &[u8; CRYPTO_ENCRYPTED_SALT], _pin : Option<&[u8]>) -> CryptoResult<([u8; 32], [u8; 32])> {
	
	let _shared = x25519::StaticSecret::diffie_hellman (_private, _public);
	let _shared = _shared.as_bytes ();
	
	let _pin : [u8; 32] =
			::blake3::Hasher::new_derive_key (CRYPTO_PIN_CONTEXT)
			.update (_pin.unwrap_or (&[]))
			.finalize ()
			.into ();
	
	let _encryption_key =
			::blake3::Hasher::new_derive_key (CRYPTO_ENCRYPTION_KEY_CONTEXT)
			.update (_shared)
			.update (_salt)
			.update (&_pin)
			.finalize ()
			.into ();
	
	let _authentication_key =
			::blake3::Hasher::new_derive_key (CRYPTO_AUTHENTICATION_KEY_CONTEXT)
			.update (_shared)
			.update (_salt)
			.update (&_pin)
			.finalize ()
			.into ();
	
	Ok ((_encryption_key, _authentication_key))
}




fn apply_all_or_nothing_mangling (_salt : &mut [u8; CRYPTO_ENCRYPTED_SALT], _data : &[u8]) -> CryptoResult {
	
	let _hash =
			::blake3::Hasher::new ()
			.update (_data)
			.finalize ();
	
	let _hash = _hash.as_bytes ();
	
	for _index in 0 .. CRYPTO_ENCRYPTED_SALT {
		_salt[_index] ^= _hash[_index];
	}
	
	Ok (())
}




fn generate_salt () -> CryptoResult<[u8; CRYPTO_ENCRYPTED_SALT]> {
	use ::rand::RngCore as _;
	let mut _salt = [0u8; CRYPTO_ENCRYPTED_SALT];
	::rand::rngs::OsRng.fill_bytes (&mut _salt);
	Ok (_salt)
}


