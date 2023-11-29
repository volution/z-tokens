

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use ::x25519_dalek as x25519;
use ::blake3::Hasher as Blake3;








define_error! (pub LowCryptoError, result : LowCryptoResult);








include! ("./crypto_macros.in");








pub trait CryptographicMaterial <const SIZE : usize> : Sized {
	
	fn consume (self) -> ();
	
	fn access (&self) -> &[u8; SIZE];
	
	fn access_slice (&self) -> &[u8] {
		Self::access (self) .as_slice ()
	}
	
	fn compare_access (_left : &Self, _right : &Self) -> bool {
		::constant_time_eq::constant_time_eq (_left.access (), _right.access ())
	}
	
	fn compare_consume (_left : Self, _right : Self) -> bool {
		let _outcome = Self::compare_access (&_left, &_right);
		_left.consume ();
		_right.consume ();
		_outcome
	}
	
	fn eq_access (_left : &Self, _right : &Self) -> bool {
		PartialEq::eq (_left.access (), _right.access ())
	}
	
	fn cmp_access (_left : &Self, _right : &Self) -> Ordering {
		Ord::cmp (_left.access (), _right.access ())
	}
}


pub trait CryptographicInput <'a> : Sized {
	
	fn consume (self) -> ();
	
	fn access (&self) -> &'a [u8];
	
	fn access_consume (self) -> &'a [u8] {
		let _material = self.access ();
		self.consume ();
		_material
	}
	
	fn size (&self) -> usize {
		self.access () .len ()
	}
	
	fn is_empty (&self) -> bool {
		self.access () .is_empty ()
	}
}








pub fn x25519_dhe <WC, WO> (
		_wrapper : WC,
		_purpose : &'static str,
		_private : &x25519::StaticSecret,
		_public : Option<&x25519::PublicKey>,
		_encryption : bool,
	) -> LowCryptoResult<WO>
	where
		WC : Fn ([u8; 32]) -> WO,
{
	let _private_public = x25519::PublicKey::from (_private);
	
	let _public = if let Some (_public) = _public {
			_public
		} else {
			&_private_public
		};
	
	let _dhe = x25519::StaticSecret::diffie_hellman (_private, _public);
	
	if ! _dhe.was_contributory () {
		fail! (0xa1ecea77);
	}
	
	let _dhe = _dhe.as_bytes ();
	
	let _sender_public = if _encryption { _private_public.as_bytes () } else { _public.as_bytes () };
	let _recipient_public = if _encryption { _public.as_bytes () } else { _private_public.as_bytes () };
	
	let _shared_key = blake3_derive_key (
			_wrapper,
			_purpose,
			&[
				_dhe,
				_sender_public,
				_recipient_public,
			],
			&[],
			None,
		);
	
	Ok (_shared_key)
}








pub fn blake3_derive_key <const NF : usize, const NV : usize, WC, WO> (
		_wrapper : WC,
		_purpose : &'static str,
		_fixed_elements : &[&[u8; 32]; NF],
		_variable_elements : &[&[u8]; NV],
		_index : Option<u32>,
	) -> WO
	where
		WC : Fn ([u8; 32]) -> WO,
{
	let mut _hasher = Blake3::new_derive_key (_purpose);
	
	blake3_update (&mut _hasher, _fixed_elements, _variable_elements, _index);
	
	let _hash : [u8; 32] = _hasher.finalize () .into ();
	
	let _wrapped = _wrapper (_hash);
	_wrapped
}


pub fn blake3_keyed_hash <const NF : usize, const NV : usize, WC, WO> (
		_wrapper : WC,
		_key : &[u8; 32],
		_fixed_elements : &[&[u8; 32]; NF],
		_variable_elements : &[&[u8]; NV],
		_index : Option<u32>,
	) -> WO
	where
		WC : Fn ([u8; 32]) -> WO,
{
	let mut _hasher = Blake3::new_keyed (_key);
	
	blake3_update (&mut _hasher, _fixed_elements, _variable_elements, _index);
	
	let _hash : [u8; 32] = _hasher.finalize () .into ();
	
	let _wrapped = _wrapper (_hash);
	_wrapped
}




pub fn blake3_update <const NF : usize, const NV : usize> (
		_hasher : &mut Blake3,
		_fixed_elements : &[&[u8; 32]; NF],
		_variable_elements : &[&[u8]; NV],
		_index : Option<u32>,
	) -> ()
{
	if let Some (_index) = _index {
		{
			let mut _bytes = [0u8; 4];
			use ::byteorder::ByteOrder as _;
			::byteorder::BigEndian::write_u32 (&mut _bytes, _index);
			_hasher.update (&_bytes);
		}
	}
	
	for _fixed_element in _fixed_elements {
		_hasher.update (_fixed_element.as_slice ());
	}
	
	for _variable_element in _variable_elements {
		
		let _size : u32 = _variable_element.len () .try_into () .else_panic (0xe5d3933d);
		
		{
			let mut _bytes = [0u8; 4];
			use ::byteorder::ByteOrder as _;
			::byteorder::BigEndian::write_u32 (&mut _bytes, _size);
			_hasher.update (&_bytes);
		}
		
		_hasher.update (_variable_element);
	}
}




pub fn blake3_derive_key_join <'a, WC, WO> (
		_wrapper : WC,
		_purpose : &'static str,
		_elements : impl Iterator<Item = &'a [u8; 32]>,
	) -> WO
	where
		WC : Fn ([u8; 32]) -> WO,
{
	let mut _hasher = Blake3::new_derive_key (_purpose);
	
	for _element in _elements {
		_hasher.update (_element);
	}
	
	let _hash : [u8; 32] = _hasher.finalize () .into ();
	
	let _wrapped = _wrapper (_hash);
	_wrapped
}








const ARGON_ALGORITHM : ::argon2::Algorithm = ::argon2::Algorithm::Argon2id;
const ARGON_VERSION : ::argon2::Version = ::argon2::Version::V0x13;
const ARGON_P_COST : u32 = 1;




pub fn argon_derive <WC, WO> (
		_wrapper : WC,
		_secret : &[u8; 32],
		_salt : &[u8; 32],
		_m_cost : u32,
		_t_cost : u32,
	) -> LowCryptoResult<WO>
	where
		WC : Fn ([u8; 32]) -> WO,
{
	let mut _output = [0u8; 32];
	
	let _parameters = ::argon2::Params::new (
				_m_cost,
				_t_cost,
				ARGON_P_COST,
				Some (_output.len ()),
			) .else_wrap (0xf2eebb0c) ?;
	
	let _hasher = ::argon2::Argon2::new (
				ARGON_ALGORITHM,
				ARGON_VERSION,
				_parameters,
			);
	
	_hasher.hash_password_into (
				_secret.as_slice (),
				_salt.as_slice (),
				&mut _output
			) .else_wrap (0xacae7396) ?;
	
	let _wrapped = _wrapper (_output);
	return Ok (_wrapped);
}








pub fn generate_random <WC, WO> (_wrapper : WC) -> WO
	where
		WC : Fn ([u8; 32]) -> WO,
{
	
	use ::rand::rngs::OsRng;
	use ::rand::RngCore as _;
	
	let mut _data = [0u8; 32];
	OsRng.fill_bytes (&mut _data);
	
	let _wrapped = _wrapper (_data);
	_wrapped
}








pub fn debug_material <const SIZE : usize> (_identifier : &str, _wrapper : &impl CryptographicMaterial<SIZE>) -> () {
	debug_bytes (_identifier, _wrapper.access ());
}


pub fn debug_bytes (_identifier : &str, _bytes : &[u8]) -> () {
	let mut _buffer = String::with_capacity (1024);
	_buffer.write_fmt (format_args! ("[>>] [a99accf0]  >>  {:-40}  >>  ", _identifier)) .else_panic (0xc3663c18);
	for _byte in _bytes {
		_buffer.write_fmt (format_args! ("{:02x}", *_byte)) .else_panic (0x5d9cb2c0);
	}
	_buffer.push ('\n');
	stderr_locked () .write (_buffer.as_bytes ()) .else_panic (0xe6d38156);
}


