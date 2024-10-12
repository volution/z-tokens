

#![ allow (dead_code) ]




use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use ::z_tokens_runtime::{
		memory::Rb,
		sensitive::zeroize_and_drop,
	};

use ::z_tokens_runtime_codings::crates::bech32;

use ::z_tokens_runtime_random::crates::rand::{
		RngCore as _,
		rngs::OsRng,
	};


use ::z_tokens_runtime_crypto::crates::{
		x25519,
	};








define_error! (pub KeyEncodingError, result : KeyEncodingResult);
define_error! (pub KeyCreateError, result : KeyCreateResult);




pub struct SenderPrivateKey (Rb<x25519::StaticSecret>);
pub struct SenderPublicKey (Rb<x25519::PublicKey>);

pub struct RecipientPrivateKey (Rb<x25519::StaticSecret>);
pub struct RecipientPublicKey (Rb<x25519::PublicKey>);

pub struct SharedSecret (Rb<[u8; 32]>);
pub struct SharedSeed (Rb<[u8; 32]>);
pub struct SharedBallast (Rb<[u8; 32]>);

pub struct SharedSecretRaw (Rb<Vec<u8>>);
pub struct SharedSeedRaw (Rb<Vec<u8>>);
pub struct SharedBallastRaw (Rb<Vec<u8>>);

pub struct Associated (Rb<Vec<u8>>);
pub struct SharedPin (Rb<Vec<u8>>);

pub struct PasswordOutput (Rb<[u8; 32]>);


pub enum SenderPublicOrPrivateKey {
	Public (SenderPublicKey),
	Private (SenderPrivateKey),
}

pub enum RecipientPublicOrPrivateKey {
	Public (RecipientPublicKey),
	Private (RecipientPrivateKey),
}




pub trait SharedSecretTrait {
	fn access_bytes_slice (&self) -> &[u8];
}

pub trait SharedSeedTrait {
	fn access_bytes_slice (&self) -> &[u8];
}

pub trait SharedBallastTrait {
	fn access_bytes_slice (&self) -> &[u8];
}




pub const SENDER_PRIVATE_KEY_ENCODED_PREFIX : &str = "ztxsk";
pub const SENDER_PUBLIC_KEY_ENCODED_PREFIX : &str = "ztxsp";

pub const RECIPIENT_PRIVATE_KEY_ENCODED_PREFIX : &str = "ztxrk";
pub const RECIPIENT_PUBLIC_KEY_ENCODED_PREFIX : &str = "ztxrp";

pub const SHARED_SECRET_ENCODED_PREFIX : &str = "ztxcs";
pub const SHARED_SEED_ENCODED_PREFIX : &str = "ztxsd";
pub const SHARED_BALLAST_ENCODED_PREFIX : &str = "ztxbl";








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
	
	pub fn to_recipient (&self) -> RecipientPrivateKey {
		RecipientPrivateKey (self.0.clone ())
	}
	
	pub(crate) fn access (&self) -> &x25519::StaticSecret {
		&self.0
	}
	
	pub(crate) fn access_bytes (&self) -> &[u8; 32] {
		let _key : &x25519::StaticSecret = self.access ();
		let _bytes : &[u8; 32] = unsafe { mem::transmute (_key) };
		_bytes
	}
	
	pub(crate) fn access_bytes_slice (&self) -> &[u8] {
		self.access_bytes () .as_slice ()
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
	
	pub fn to_recipient (&self) -> RecipientPublicKey {
		RecipientPublicKey (self.0.clone ())
	}
	
	pub(crate) fn access (&self) -> &x25519::PublicKey {
		&self.0
	}
	
	pub(crate) fn access_bytes (&self) -> &[u8; 32] {
		self.access () .as_bytes ()
	}
	
	pub(crate) fn access_bytes_slice (&self) -> &[u8] {
		self.access_bytes () .as_slice ()
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
	
	pub fn to_sender (&self) -> SenderPrivateKey {
		SenderPrivateKey (self.0.clone ())
	}
	
	pub(crate) fn access (&self) -> &x25519::StaticSecret {
		&self.0
	}
	
	pub(crate) fn access_bytes (&self) -> &[u8; 32] {
		let _key : &x25519::StaticSecret = self.access ();
		let _bytes : &[u8; 32] = unsafe { mem::transmute (_key) };
		_bytes
	}
	
	pub(crate) fn access_bytes_slice (&self) -> &[u8] {
		self.access_bytes () .as_slice ()
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
	
	pub fn to_sender (&self) -> SenderPublicKey {
		SenderPublicKey (self.0.clone ())
	}
	
	pub(crate) fn access (&self) -> &x25519::PublicKey {
		&self.0
	}
	
	pub(crate) fn access_bytes (&self) -> &[u8; 32] {
		self.access () .as_bytes ()
	}
	
	pub(crate) fn access_bytes_slice (&self) -> &[u8] {
		self.access_bytes () .as_slice ()
	}
}








impl SenderPublicOrPrivateKey {
	
	pub fn decode_and_zeroize (_string : String) -> KeyEncodingResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> KeyEncodingResult<Self> {
		if _string.starts_with (SENDER_PUBLIC_KEY_ENCODED_PREFIX) {
			Ok (Self::Public (SenderPublicKey::decode (_string) ?))
		} else if _string.starts_with (SENDER_PRIVATE_KEY_ENCODED_PREFIX) {
			Ok (Self::Private (SenderPrivateKey::decode (_string) ?))
		} else {
			fail! (0x431002d1);
		}
	}
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		match self {
			Self::Public (_key) => _key.encode (),
			Self::Private (_key) => _key.encode (),
		}
	}
	
	pub fn to_recipient (&self) -> RecipientPublicOrPrivateKey {
		match self {
			Self::Public (_key) => RecipientPublicOrPrivateKey::Public (_key.to_recipient ()),
			Self::Private (_key) => RecipientPublicOrPrivateKey::Private (_key.to_recipient ()),
		}
	}
}




impl RecipientPublicOrPrivateKey {
	
	pub fn decode_and_zeroize (_string : String) -> KeyEncodingResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> KeyEncodingResult<Self> {
		if _string.starts_with (RECIPIENT_PUBLIC_KEY_ENCODED_PREFIX) {
			Ok (Self::Public (RecipientPublicKey::decode (_string) ?))
		} else if _string.starts_with (RECIPIENT_PRIVATE_KEY_ENCODED_PREFIX) {
			Ok (Self::Private (RecipientPrivateKey::decode (_string) ?))
		} else {
			fail! (0xbc0246d3);
		}
	}
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		match self {
			Self::Public (_key) => _key.encode (),
			Self::Private (_key) => _key.encode (),
		}
	}
	
	pub fn to_sender (&self) -> SenderPublicOrPrivateKey {
		match self {
			Self::Public (_key) => SenderPublicOrPrivateKey::Public (_key.to_sender ()),
			Self::Private (_key) => SenderPublicOrPrivateKey::Private (_key.to_sender ()),
		}
	}
}








impl SharedSecret {
	
	pub fn decode_and_zeroize (_string : String) -> KeyEncodingResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> KeyEncodingResult<Self> {
		decode_shared_secret (_string)
	}
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_shared_secret (self)
	}
	
	pub fn access_bytes (&self) -> &[u8; 32] {
		&self.0
	}
	
	pub fn access_bytes_slice (&self) -> &[u8] {
		self.access_bytes () .as_slice ()
	}
}




impl SharedSeed {
	
	pub fn decode_and_zeroize (_string : String) -> KeyEncodingResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> KeyEncodingResult<Self> {
		decode_shared_seed (_string)
	}
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_shared_seed (self)
	}
	
	pub fn access_bytes (&self) -> &[u8; 32] {
		&self.0
	}
	
	pub fn access_bytes_slice (&self) -> &[u8] {
		self.access_bytes () .as_slice ()
	}
}




impl SharedBallast {
	
	pub fn decode_and_zeroize (_string : String) -> KeyEncodingResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> KeyEncodingResult<Self> {
		decode_shared_ballast (_string)
	}
	
	pub fn encode (&self) -> KeyEncodingResult<Rb<String>> {
		encode_shared_ballast (self)
	}
	
	pub fn access_bytes (&self) -> &[u8; 32] {
		&self.0
	}
	
	pub fn access_bytes_slice (&self) -> &[u8] {
		self.access_bytes () .as_slice ()
	}
}




impl SharedSecretRaw {
	
	pub fn new (_data : Vec<u8>) -> Self {
		Self (Rb::new (_data))
	}
	
	pub fn access_bytes_slice (&self) -> &[u8] {
		&self.0
	}
}


impl SharedSeedRaw {
	
	pub fn new (_data : Vec<u8>) -> Self {
		Self (Rb::new (_data))
	}
	
	pub fn access_bytes_slice (&self) -> &[u8] {
		&self.0
	}
}


impl SharedBallastRaw {
	
	pub fn new (_data : Vec<u8>) -> Self {
		Self (Rb::new (_data))
	}
	
	pub fn access_bytes_slice (&self) -> &[u8] {
		&self.0
	}
}




impl SharedSecretTrait for SharedSecret {
	fn access_bytes_slice (&self) -> &[u8] { self.access_bytes_slice () }
}
impl SharedSecretTrait for SharedSecretRaw {
	fn access_bytes_slice (&self) -> &[u8] { self.access_bytes_slice () }
}


impl SharedSeedTrait for SharedSeed {
	fn access_bytes_slice (&self) -> &[u8] { self.access_bytes_slice () }
}
impl SharedSeedTrait for SharedSeedRaw {
	fn access_bytes_slice (&self) -> &[u8] { self.access_bytes_slice () }
}


impl SharedBallastTrait for SharedBallast {
	fn access_bytes_slice (&self) -> &[u8] { self.access_bytes_slice () }
}
impl SharedBallastTrait for SharedBallastRaw {
	fn access_bytes_slice (&self) -> &[u8] { self.access_bytes_slice () }
}







impl Associated {
	
	pub fn new (_data : Vec<u8>) -> Self {
		Self (Rb::new (_data))
	}
	
	pub fn access_bytes_slice (&self) -> &[u8] {
		&self.0
	}
}


impl SharedPin {
	
	pub fn new (_data : Vec<u8>) -> Self {
		Self (Rb::new (_data))
	}
	
	pub fn access_bytes_slice (&self) -> &[u8] {
		&self.0
	}
}




impl PasswordOutput {
	
	pub fn new (_data : [u8; 32]) -> Self {
		Self (Rb::new (_data))
	}
	
	pub fn access_bytes (&self) -> &[u8; 32] {
		&self.0
	}
	
	pub fn access_bytes_slice (&self) -> &[u8] {
		self.access_bytes () .as_slice ()
	}
}








pub fn decode_sender_private_key (_string : &str) -> KeyEncodingResult<SenderPrivateKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (SENDER_PRIVATE_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::StaticSecret::from (_key_data);
	zeroize_and_drop (_key_data);
	Ok (SenderPrivateKey (Rb::new (_key)))
}


pub fn decode_sender_public_key (_string : &str) -> KeyEncodingResult<SenderPublicKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (SENDER_PUBLIC_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::PublicKey::from (_key_data);
	zeroize_and_drop (_key_data);
	Ok (SenderPublicKey (Rb::new (_key)))
}


pub fn decode_recipient_private_key (_string : &str) -> KeyEncodingResult<RecipientPrivateKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (RECIPIENT_PRIVATE_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::StaticSecret::from (_key_data);
	zeroize_and_drop (_key_data);
	Ok (RecipientPrivateKey (Rb::new (_key)))
}


pub fn decode_recipient_public_key (_string : &str) -> KeyEncodingResult<RecipientPublicKey> {
	let mut _key_data = [0u8; 32];
	decode_raw (RECIPIENT_PUBLIC_KEY_ENCODED_PREFIX, _string, &mut _key_data) ?;
	let _key = x25519::PublicKey::from (_key_data);
	zeroize_and_drop (_key_data);
	Ok (RecipientPublicKey (Rb::new (_key)))
}


pub fn decode_shared_secret (_string : &str) -> KeyEncodingResult<SharedSecret> {
	let mut _key_data = [0u8; 32];
	decode_raw (SHARED_SECRET_ENCODED_PREFIX, _string, &mut _key_data) ?;
	Ok (SharedSecret (Rb::new (_key_data)))
}


pub fn decode_shared_seed (_string : &str) -> KeyEncodingResult<SharedSeed> {
	let mut _key_data = [0u8; 32];
	decode_raw (SHARED_SEED_ENCODED_PREFIX, _string, &mut _key_data) ?;
	Ok (SharedSeed (Rb::new (_key_data)))
}


pub fn decode_shared_ballast (_string : &str) -> KeyEncodingResult<SharedBallast> {
	let mut _key_data = [0u8; 32];
	decode_raw (SHARED_BALLAST_ENCODED_PREFIX, _string, &mut _key_data) ?;
	Ok (SharedBallast (Rb::new (_key_data)))
}


pub(crate) fn decode_raw (_prefix : &str, _encoded : &str, _data : &mut [u8]) -> KeyEncodingResult {
	
	let _data_actual = decode_raw_vec (_prefix, _encoded) ?;
	
	if _data_actual.len () != _data.len () {
		fail! (0xdce379e1);
	}
	
	_data.copy_from_slice (&_data_actual);
	
	zeroize_and_drop (_data_actual);
	
	Ok (())
}


pub(crate) fn decode_raw_vec (_prefix : &str, _encoded : &str) -> KeyEncodingResult<Vec<u8>> {
	
	// FIXME:  Find a way to eliminate allocations!
	
	use bech32::{
			Bech32m,
			primitives::decode::CheckedHrpstring,
		};
	
	let _decoded = CheckedHrpstring::new::<Bech32m> (_encoded) .else_wrap (0x2ba31a69) ?;
	
	let _prefix_actual = _decoded.hrp ();
	if _prefix_actual.as_str () != _prefix {
		fail! (0x4a4fe470);
	}
	
	let _data = _decoded.byte_iter () .collect ();
	
	Ok (_data)
}




pub fn encode_sender_private_key (_key : &SenderPrivateKey) -> KeyEncodingResult<Rb<String>> {
	let _bytes = _key.access_bytes ();
	encode_raw (SENDER_PRIVATE_KEY_ENCODED_PREFIX, _bytes)
}


pub fn encode_sender_public_key (_key : &SenderPublicKey) -> KeyEncodingResult<Rb<String>> {
	let _bytes = _key.access_bytes ();
	encode_raw (SENDER_PUBLIC_KEY_ENCODED_PREFIX, _bytes)
}


pub fn encode_recipient_private_key (_key : &RecipientPrivateKey) -> KeyEncodingResult<Rb<String>> {
	let _bytes = _key.access_bytes ();
	encode_raw (RECIPIENT_PRIVATE_KEY_ENCODED_PREFIX, _bytes)
}


pub fn encode_recipient_public_key (_key : &RecipientPublicKey) -> KeyEncodingResult<Rb<String>> {
	let _bytes = _key.access_bytes ();
	encode_raw (RECIPIENT_PUBLIC_KEY_ENCODED_PREFIX, _bytes)
}


pub fn encode_shared_secret (_key : &SharedSecret) -> KeyEncodingResult<Rb<String>> {
	let _bytes = _key.access_bytes ();
	encode_raw (SHARED_SECRET_ENCODED_PREFIX, _bytes)
}


pub fn encode_shared_seed (_key : &SharedSeed) -> KeyEncodingResult<Rb<String>> {
	let _bytes = _key.access_bytes ();
	encode_raw (SHARED_SEED_ENCODED_PREFIX, _bytes)
}


pub fn encode_shared_ballast (_key : &SharedBallast) -> KeyEncodingResult<Rb<String>> {
	let _bytes = _key.access_bytes ();
	encode_raw (SHARED_BALLAST_ENCODED_PREFIX, _bytes)
}


pub(crate) fn encode_raw (_prefix : &str, _data : &[u8]) -> KeyEncodingResult<Rb<String>> {
	
	// FIXME:  Find a way to eliminate allocations!
	
	use bech32::{
			Bech32m,
			Hrp,
			encode_lower,
		};
	
	let _prefix = Hrp::parse (_prefix) .else_wrap (0x21b86a70) ?;
	
	let _encoded = encode_lower::<Bech32m> (_prefix, _data) .else_wrap (0x852b53d0) ?;
	
	Ok (Rb::new (_encoded))
}








pub fn create_sender_pair () -> KeyCreateResult<(SenderPrivateKey, SenderPublicKey)> {
	let (_private, _public) = create_x25519_pair_from_random () ?;
	let _private = SenderPrivateKey (Rb::new (_private));
	let _public = SenderPublicKey (Rb::new (_public));
	Ok ((_private, _public))
}


pub fn create_recipient_pair () -> KeyCreateResult<(RecipientPrivateKey, RecipientPublicKey)> {
	let (_private, _public) = create_x25519_pair_from_random () ?;
	let _private = RecipientPrivateKey (Rb::new (_private));
	let _public = RecipientPublicKey (Rb::new (_public));
	Ok ((_private, _public))
}


fn create_x25519_pair_from_random () -> KeyCreateResult<(x25519::StaticSecret, x25519::PublicKey)> {
	
	let mut _bytes = [0u8; 32];
	OsRng.fill_bytes (&mut _bytes);
	
	let _private = x25519::StaticSecret::from (_bytes);
	let _public = x25519::PublicKey::from (&_private);
	
	Ok ((_private, _public))
}




pub fn create_shared_secret () -> KeyCreateResult<SharedSecret> {
	
	let mut _bytes = [0u8; 32];
	OsRng.fill_bytes (&mut _bytes);
	
	Ok (SharedSecret (Rb::new (_bytes)))
}


pub fn create_shared_seed () -> KeyCreateResult<SharedSeed> {
	
	let mut _bytes = [0u8; 32];
	OsRng.fill_bytes (&mut _bytes);
	
	Ok (SharedSeed (Rb::new (_bytes)))
}


pub fn create_shared_ballast () -> KeyCreateResult<SharedBallast> {
	
	let mut _bytes = [0u8; 32];
	OsRng.fill_bytes (&mut _bytes);
	
	Ok (SharedBallast (Rb::new (_bytes)))
}




pub fn create_shared_pin () -> KeyCreateResult<Rb<String>> {
	
	let mut _bytes = [0u8; 8];
	OsRng.fill_bytes (&mut _bytes);
	
	let _pin : u64 = unsafe { mem::transmute (_bytes) };
	let _pin = _pin % 10_000_000_000;
	
	let _pin = format! ("{:010}", _pin);
	
	Ok (Rb::new (_pin))
}


