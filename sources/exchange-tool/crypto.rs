

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use crate::keys::*;


use ::z_tokens_runtime::{
		sensitive::zeroize_and_drop,
		sensitive::Zeroize as _,
	};


use ::x25519_dalek as x25519;




define_error! (pub CryptoError, result : CryptoResult);




pub const CRYPTO_PADDING : usize = 256;
pub const CRYPTO_OVERHEAD : usize = 32;

pub const CRYPTO_DECRYPTED_SIZE_MAX : usize = 128 * 1024 * 1024;
pub const CRYPTO_ENCRYPTED_SIZE_MAX : usize = CRYPTO_DECRYPTED_SIZE_MAX + CRYPTO_PADDING + CRYPTO_OVERHEAD;

pub(crate) const CRYPTO_NAMESPACE : &[u8] = b"f6eee60c50cdf7d06d97e65f739f0086913c7f36fa7e86bd4962aa17ed3f0f37";








pub fn encrypted_max_len (_decrypted_len : usize) -> CryptoResult<usize> {
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0xf486af87);
	}
	
	let _encrypted_len = (((_decrypted_len / CRYPTO_PADDING) + 1) * CRYPTO_PADDING) + CRYPTO_OVERHEAD;
	
	Ok (_encrypted_len)
}


pub fn decrypted_max_len (_encrypted_len : usize) -> CryptoResult<usize> {
	
	if _encrypted_len > CRYPTO_ENCRYPTED_SIZE_MAX {
		fail! (0x2f099ff9);
	}
	
	if _encrypted_len < (CRYPTO_PADDING + CRYPTO_OVERHEAD) {
		fail! (0xafb25d04);
	}
	
	let _decrypted_len = _encrypted_len - CRYPTO_PADDING - CRYPTO_OVERHEAD;
	
	Ok (_decrypted_len)
}








pub fn encrypt (_sender : &SenderPrivateKey, _recipient : &RecipientPublicKey, _decrypted : &[u8], _encrypted : &mut [u8]) -> CryptoResult {
	
	_encrypted.zeroize ();
	
	let _encrypted_len = encrypted_max_len (_decrypted.len ()) ?;
	
	_encrypted.zeroize ();
	
	if _encrypted.len () != _encrypted_len {
		fail! (0x3ad1b2ab);
	}
	
	fail! (0xd43596df);
}


pub fn decrypt (_recipient : &RecipientPrivateKey, _sender : &SenderPublicKey, _encrypted : &[u8], _decrypted : &mut [u8]) -> CryptoResult<usize> {
	
	_decrypted.zeroize ();
	
	let _decrypted_len = decrypted_max_len (_encrypted.len ()) ?;
	
	if _decrypted.len () < _decrypted_len {
		fail! (0xe2a75aca);
	}
	
	fail! (0xf07847bf);
}

