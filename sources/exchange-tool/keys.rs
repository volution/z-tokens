

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use ::z_tokens_runtime::{
		memory::Rb,
		sensitive::Sensitive,
		sensitive::SensitiveZeroize,
		sensitive::SensitiveIgnored,
	};


use ::x25519_dalek as x25519;




define_error! (pub KeyEncodingError, result : KeyEncodingResult);
define_error! (pub KeyCreateError, result : KeyCreateResult);




pub struct SenderPrivateKey (Rb<SensitiveZeroize<x25519::StaticSecret>>);
pub struct SenderPublicKey (Rb<SensitiveIgnored<x25519::PublicKey>>);

pub struct ReceiverPrivateKey (Rb<SensitiveZeroize<x25519::StaticSecret>>);
pub struct ReceiverPublicKey (Rb<SensitiveIgnored<x25519::PublicKey>>);




impl SenderPrivateKey {
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_sender_private_key (self)
	}
}


impl SenderPublicKey {
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_sender_public_key (self)
	}
}




impl ReceiverPrivateKey {
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_receiver_private_key (self)
	}
}


impl ReceiverPublicKey {
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_receiver_public_key (self)
	}
}








pub const SENDER_PRIVATE_KEY_ENCODED_PREFIX : &str = "ztxsk";
pub const SENDER_PUBLIC_KEY_ENCODED_PREFIX : &str = "ztxsp";

pub const RECEIVER_PRIVATE_KEY_ENCODED_PREFIX : &str = "ztxrk";
pub const RECEIVER_PUBLIC_KEY_ENCODED_PREFIX : &str = "ztxrp";




pub fn decode_sender_private_key (_string : &str) -> KeyEncodingResult<SenderPrivateKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (SENDER_PRIVATE_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::StaticSecret::from (_key_data);
	Sensitive::erase (&mut SensitiveZeroize (_key_data));
	Ok (SenderPrivateKey (Rb::new (_key.into ())))
}


pub fn decode_sender_public_key (_string : &str) -> KeyEncodingResult<SenderPublicKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (SENDER_PUBLIC_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::PublicKey::from (_key_data);
	Sensitive::erase (&mut SensitiveZeroize (_key_data));
	Ok (SenderPublicKey (Rb::new (_key.into ())))
}


pub fn decode_receiver_private_key (_string : &str) -> KeyEncodingResult<ReceiverPrivateKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (RECEIVER_PRIVATE_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::StaticSecret::from (_key_data);
	Sensitive::erase (&mut SensitiveZeroize (_key_data));
	Ok (ReceiverPrivateKey (Rb::new (_key.into ())))
}


pub fn decode_receiver_public_key (_string : &str) -> KeyEncodingResult<ReceiverPublicKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (RECEIVER_PUBLIC_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::PublicKey::from (_key_data);
	Sensitive::erase (&mut SensitiveZeroize (_key_data));
	Ok (ReceiverPublicKey (Rb::new (_key.into ())))
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
	Sensitive::erase (&mut SensitiveZeroize (_data_actual));
	Sensitive::erase (&mut SensitiveZeroize (_bech_nibles));
	
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


pub fn encode_receiver_private_key (_key : &ReceiverPrivateKey) -> KeyEncodingResult<Rb<String>> {
	let _key : &x25519::StaticSecret = &_key.0.0;
	let _bytes : &[u8; 32] = unsafe { mem::transmute (_key) };
	encode_raw (RECEIVER_PRIVATE_KEY_ENCODED_PREFIX, _bytes)
}


pub fn encode_receiver_public_key (_key : &ReceiverPublicKey) -> KeyEncodingResult<Rb<String>> {
	let _bytes = _key.0.0.as_bytes ();
	encode_raw (RECEIVER_PUBLIC_KEY_ENCODED_PREFIX, _bytes)
}


fn encode_raw (_prefix : &str, _data : &[u8]) -> KeyEncodingResult<Rb<String>> {
	
	let _bech_nibles_capacity = _data.len () * 8 / 5 + 1;
	let _bech_string_capacity = _prefix.len () + 1 + _bech_nibles_capacity;
	
	let mut _bech_nibles = Vec::with_capacity (_bech_nibles_capacity);
	::bech32::ToBase32::write_base32 (&_data, &mut _bech_nibles) .else_replace (0xd5ea985b) ?;
	assert! (_bech_nibles_capacity == _bech_nibles.capacity (), "[5e22b060]");
	
	let mut _bech_string = String::with_capacity (_bech_string_capacity);
	assert! (_bech_string_capacity == _bech_string.capacity (), "[0x9549d10e]");
	
	::bech32::encode_to_fmt (&mut _bech_string, _prefix, &_bech_nibles, ::bech32::Variant::Bech32m) .else_wrap (0x9ee94010) ? .else_wrap (0x49c6b0af) ?;
	
	let _bech_nibles : Vec<u8> = unsafe { mem::transmute (_bech_nibles) };
	Sensitive::erase (&mut SensitiveZeroize (_bech_nibles));
	
	Ok (Rb::new (_bech_string))
}








pub fn create_sender_pair () -> KeyCreateResult<(SenderPrivateKey, SenderPublicKey)> {
	let (_private, _public) = create_x25519_pair_from_random () ?;
	let _private = SenderPrivateKey (Rb::new (_private.into ()));
	let _public = SenderPublicKey (Rb::new (_public.into ()));
	Ok ((_private, _public))
}


pub fn create_receiver_pair () -> KeyCreateResult<(ReceiverPrivateKey, ReceiverPublicKey)> {
	let (_private, _public) = create_x25519_pair_from_random () ?;
	let _private = ReceiverPrivateKey (Rb::new (_private.into ()));
	let _public = ReceiverPublicKey (Rb::new (_public.into ()));
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


