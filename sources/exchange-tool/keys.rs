

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use ::z_tokens_runtime::{
		memory::Rb,
		sensitive::SensitiveZeroize,
		sensitive::SensitiveIgnored,
		sensitive::zeroize_and_drop,
	};


use ::x25519_dalek as x25519;








define_error! (pub KeyEncodingError, result : KeyEncodingResult);
define_error! (pub KeyCreateError, result : KeyCreateResult);




pub struct SenderPrivateKey (pub(crate) Rb<SensitiveZeroize<x25519::StaticSecret>>);
pub struct SenderPublicKey (pub(crate) Rb<SensitiveIgnored<x25519::PublicKey>>);

pub struct RecipientPrivateKey (pub(crate) Rb<SensitiveZeroize<x25519::StaticSecret>>);
pub struct RecipientPublicKey (pub(crate) Rb<SensitiveIgnored<x25519::PublicKey>>);




pub const SENDER_PRIVATE_KEY_ENCODED_PREFIX : &str = "ztxsk";
pub const SENDER_PUBLIC_KEY_ENCODED_PREFIX : &str = "ztxsp";

pub const RECEIVER_PRIVATE_KEY_ENCODED_PREFIX : &str = "ztxrk";
pub const RECEIVER_PUBLIC_KEY_ENCODED_PREFIX : &str = "ztxrp";








impl SenderPrivateKey {
	
	pub fn decode_and_zeroize (_string : String) -> KeyEncodingResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> KeyEncodingResult<Self> {
		decode_sender_private_key (_string)
	}
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_sender_private_key (self)
	}
}


impl SenderPublicKey {
	
	pub fn decode_and_zeroize (_string : String) -> KeyEncodingResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> KeyEncodingResult<Self> {
		decode_sender_public_key (_string)
	}
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_sender_public_key (self)
	}
}




impl RecipientPrivateKey {
	
	pub fn decode_and_zeroize (_string : String) -> KeyEncodingResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> KeyEncodingResult<Self> {
		decode_recipient_private_key (_string)
	}
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_recipient_private_key (self)
	}
}


impl RecipientPublicKey {
	
	pub fn decode_and_zeroize (_string : String) -> KeyEncodingResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> KeyEncodingResult<Self> {
		decode_recipient_public_key (_string)
	}
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_recipient_public_key (self)
	}
}








pub fn decode_sender_private_key (_string : &str) -> KeyEncodingResult<SenderPrivateKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (SENDER_PRIVATE_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::StaticSecret::from (_key_data);
	zeroize_and_drop (_key_data);
	Ok (SenderPrivateKey (Rb::new (_key.into ())))
}


pub fn decode_sender_public_key (_string : &str) -> KeyEncodingResult<SenderPublicKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (SENDER_PUBLIC_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::PublicKey::from (_key_data);
	zeroize_and_drop (_key_data);
	Ok (SenderPublicKey (Rb::new (_key.into ())))
}


pub fn decode_recipient_private_key (_string : &str) -> KeyEncodingResult<RecipientPrivateKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (RECEIVER_PRIVATE_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::StaticSecret::from (_key_data);
	zeroize_and_drop (_key_data);
	Ok (RecipientPrivateKey (Rb::new (_key.into ())))
}


pub fn decode_recipient_public_key (_string : &str) -> KeyEncodingResult<RecipientPublicKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (RECEIVER_PUBLIC_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::PublicKey::from (_key_data);
	zeroize_and_drop (_key_data);
	Ok (RecipientPublicKey (Rb::new (_key.into ())))
}


fn decode_raw (_prefix : &str, _encoded : &str, _data : &mut [u8]) -> KeyEncodingResult {
	
	// FIXME:  Find a way to eliminate allocations!
	let (_prefix_actual, _bech_nibles, _bech_variant) = ::bech32::decode (_encoded) .else_wrap (0x2ba31a69) ?;
	
	if _prefix_actual != _prefix {
		fail! (0x4a4fe470);
	}
	if _bech_variant != ::bech32::Variant::Bech32m {
		fail! (0xcbd4e755);
	}
	
	let _data_actual : Vec<u8>;
	_data_actual = ::bech32::FromBase32::from_base32 (&_bech_nibles) .else_wrap (0x799c1726) ?;
	
	if _data_actual.len () != _data.len () {
		fail! (0xdce379e1);
	}
	
	_data.copy_from_slice (&_data_actual);
	
	let _bech_nibles : Vec<u8> = unsafe { mem::transmute (_bech_nibles) };
	zeroize_and_drop (_bech_nibles);
	zeroize_and_drop (_data_actual);
	
	Ok (())
}




pub fn encode_sender_private_key (_key : &SenderPrivateKey) -> KeyEncodingResult<Rb<String>> {
	let _key : &x25519::StaticSecret = &_key.0.0;
	let _bytes : &[u8; 32] = unsafe { mem::transmute (_key) };
	encode_raw (SENDER_PRIVATE_KEY_ENCODED_PREFIX, _bytes)
}


pub fn encode_sender_public_key (_key : &SenderPublicKey) -> KeyEncodingResult<Rb<String>> {
	let _bytes = _key.0.0.as_bytes ();
	encode_raw (SENDER_PUBLIC_KEY_ENCODED_PREFIX, _bytes)
}


pub fn encode_recipient_private_key (_key : &RecipientPrivateKey) -> KeyEncodingResult<Rb<String>> {
	let _key : &x25519::StaticSecret = &_key.0.0;
	let _bytes : &[u8; 32] = unsafe { mem::transmute (_key) };
	encode_raw (RECEIVER_PRIVATE_KEY_ENCODED_PREFIX, _bytes)
}


pub fn encode_recipient_public_key (_key : &RecipientPublicKey) -> KeyEncodingResult<Rb<String>> {
	let _bytes = _key.0.0.as_bytes ();
	encode_raw (RECEIVER_PUBLIC_KEY_ENCODED_PREFIX, _bytes)
}


fn encode_raw (_prefix : &str, _data : &[u8]) -> KeyEncodingResult<Rb<String>> {
	
	let _bech_nibles_capacity = _data.len () * 8 / 5 + 1;
	let _bech_string_capacity = _prefix.len () + 1 + _bech_nibles_capacity;
	
	let mut _bech_nibles = Vec::with_capacity (_bech_nibles_capacity);
	::bech32::ToBase32::write_base32 (&_data, &mut _bech_nibles) .else_replace (0xd5ea985b) ?;
	assert! (_bech_nibles_capacity == _bech_nibles.capacity (), "[5e22b060]  {} == {}", _bech_nibles_capacity, _bech_nibles.capacity ());
	
	let mut _bech_string = String::with_capacity (_bech_string_capacity);
	assert! (_bech_string_capacity == _bech_string.capacity (), "[9549d10e]  {} == {}", _bech_string_capacity, _bech_string.capacity ());
	
	::bech32::encode_to_fmt (&mut _bech_string, _prefix, &_bech_nibles, ::bech32::Variant::Bech32m) .else_wrap (0x9ee94010) ? .else_wrap (0x49c6b0af) ?;
	
	let _bech_nibles : Vec<u8> = unsafe { mem::transmute (_bech_nibles) };
	zeroize_and_drop (_bech_nibles);
	
	Ok (Rb::new (_bech_string))
}








pub fn create_sender_pair () -> KeyCreateResult<(SenderPrivateKey, SenderPublicKey)> {
	let (_private, _public) = create_x25519_pair_from_random () ?;
	let _private = SenderPrivateKey (Rb::new (_private.into ()));
	let _public = SenderPublicKey (Rb::new (_public.into ()));
	Ok ((_private, _public))
}


pub fn create_recipient_pair () -> KeyCreateResult<(RecipientPrivateKey, RecipientPublicKey)> {
	let (_private, _public) = create_x25519_pair_from_random () ?;
	let _private = RecipientPrivateKey (Rb::new (_private.into ()));
	let _public = RecipientPublicKey (Rb::new (_public.into ()));
	Ok ((_private, _public))
}


fn create_x25519_pair_from_random () -> KeyCreateResult<(x25519::StaticSecret, x25519::PublicKey)> {
	
	use ::rand::RngCore as _;
	let mut _bytes = [0u8; 32];
	::rand::rngs::OsRng.fill_bytes (&mut _bytes);
	
	let _private = x25519::StaticSecret::from (_bytes);
	let _public = x25519::PublicKey::from (&_private);
	
	Ok ((_private, _public))
}


