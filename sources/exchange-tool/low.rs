

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use ::x25519_dalek as x25519;
use ::blake3::Hasher as Blake3;


use crate::crypto::CryptoResult;
use crate::coding::encode_u32;








pub(crate) fn x25519_dhe <WC, WO> (
		_wrapper : WC,
		_context : &'static str,
		_private : &x25519::StaticSecret,
		_public : Option<&x25519::PublicKey>,
		_encryption : bool,
	) -> CryptoResult<WO>
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
		fail! (0xd00d13f7);
	}
	
	let _dhe = _dhe.as_bytes ();
	
	let _sender_public = if _encryption { _private_public.as_bytes () } else { _public.as_bytes () };
	let _receiver_public = if _encryption { _public.as_bytes () } else { _private_public.as_bytes () };
	
	let _shared_key = blake3_derive_key (
			_wrapper,
			_context,
			&[
				_dhe,
				_sender_public,
				_receiver_public,
			],
			&[]);
	
	Ok (_shared_key)
}




pub(crate) fn blake3_derive_key <const NF : usize, const NV : usize, WC, WO> (
		_wrapper : WC,
		_context : &'static str,
		_fixed_elements : &[&[u8; 32]; NF],
		_variable_elements : &[&[u8]; NV],
	) -> WO
	where
		WC : Fn ([u8; 32]) -> WO,
{
	let mut _hasher = Blake3::new_derive_key (_context);
	
	blake3_update (&mut _hasher, _fixed_elements, _variable_elements);
	
	let _hash : [u8; 32] = _hasher.finalize () .into ();
	
	let _wrapped = _wrapper (_hash);
	
	_wrapped
}


pub(crate) fn blake3_keyed_hash <const NF : usize, const NV : usize, WC, WO> (
		_wrapper : WC,
		_key : &[u8; 32],
		_fixed_elements : &[&[u8; 32]; NF],
		_variable_elements : &[&[u8]; NV],
	) -> WO
	where
		WC : Fn ([u8; 32]) -> WO,
{
	let mut _hasher = Blake3::new_keyed (_key);
	
	blake3_update (&mut _hasher, _fixed_elements, _variable_elements);
	
	let _hash : [u8; 32] = _hasher.finalize () .into ();
	
	let _wrapped = _wrapper (_hash);
	
	_wrapped
}


pub(crate) fn blake3_update <const NF : usize, const NV : usize> (
		_hasher : &mut Blake3,
		_fixed_elements : &[&[u8; 32]; NF],
		_variable_elements : &[&[u8]; NV],
	) -> ()
{
	for _fixed_element in _fixed_elements {
		_hasher.update (_fixed_element.as_slice ());
	}
	
	for _variable_element in _variable_elements {
		
		let mut _size_buffer = [0u8; 4];
		let _size : u32 = _variable_element.len () .try_into () .else_panic (0xe5d3933d);
		encode_u32 (_size, &mut _size_buffer);
		
		_hasher.update (&_size_buffer);
		
		_hasher.update (_variable_element);
	}
}




pub(crate) fn generate_random <WC, WO> (_wrapper : WC) -> WO
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


