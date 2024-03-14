

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use crate::keys::*;
use crate::oracles::*;


use ::z_tokens_runtime_crypto::{
		
		blake3_keyed_hash,
		argon_derive,
		
		CryptographicMaterial as _,
	};


use ::z_tokens_runtime_crypto::crates::{
		
		chacha20,
		x25519,
	};


use super::definitions::*;








pub(crate) fn apply_encryption (_key : InternalEncryptionKey, _data : &mut [u8]) -> CryptoResult {
	
	use chacha20::cipher::KeyIvInit as _;
	use chacha20::cipher::StreamCipher as _;
	
	let _nonce = [0u8; 12];
	
	let _key = chacha20::Key::from (_key.unwrap ());
	let _nonce = chacha20::Nonce::from (_nonce);
	
	let mut _cipher = chacha20::ChaCha20::new (&_key, &_nonce);
	
	assert! (_data.len () >= CRYPTO_ENCRYPTED_HEADER_SIZE, "[c9e6989f]");
	
	_cipher.try_apply_keystream (&mut _data[CRYPTO_ENCRYPTED_SCHEMA_SIZE..]) .else_wrap (0x9c94d0d5) ?;
	
	Ok (())
}




pub(crate) fn apply_authentication (_key : InternalAuthenticationKey, _data : &[u8]) -> CryptoResult<InternalAuthenticationMac> {
	
	let _mac = blake3_keyed_hash (
			InternalAuthenticationMac::wrap,
			_key.access (),
			&[],
			&[
				_data,
			],
		);
	
	Ok (_mac)
}




pub(crate) fn apply_all_or_nothing_mangling (_key : InternalAontKey, _packet_salt : &mut InternalPacketSalt, _data : &[u8]) -> CryptoResult {
	
	const _SIZE : usize = InternalPacketSalt::SIZE;
	
	let _hash : [u8; _SIZE] = blake3_keyed_hash (
			|_hash| _hash,
			_key.access (),
			&[],
			&[
				_data,
			],
		);
	
	let _packet_salt = &mut _packet_salt.access_mut ();
	
	for _index in 0 .. _SIZE {
		_packet_salt[_index] ^= _hash[_index];
	}
	
	Ok (())
}








pub(crate) fn apply_argon_secret (_secret_hash : &InternalSecretHash, _secret_salt : &InternalSecretSalt) -> CryptoResult<InternalSecretArgon> {
	
	argon_derive (
			InternalSecretArgon::wrap,
			_secret_hash.access (),
			_secret_salt.access (),
			CRYPTO_SECRET_ARGON_M_COST,
			CRYPTO_SECRET_ARGON_T_COST,
			CRYPTO_SECRET_ARGON_P_COST,
		) .else_wrap (0xd26ab4d1)
}


pub(crate) fn apply_argon_pin (_pin_hash : &InternalPinHash, _pin_salt : &InternalPinSalt) -> CryptoResult<InternalPinArgon> {
	
	argon_derive (
			InternalPinArgon::wrap,
			_pin_hash.access (),
			_pin_salt.access (),
			CRYPTO_PIN_ARGON_M_COST,
			CRYPTO_PIN_ARGON_T_COST,
			CRYPTO_PIN_ARGON_P_COST,
		) .else_wrap (0xc396c6f6)
}


pub(crate) fn apply_argon_ballast (_ballast_hash : &InternalBallastHash, _ballast_salt : &InternalBallastSalt, _ballast_count : usize) -> CryptoResult<InternalBallastArgon> {
	
	let _ballast_count = _ballast_count as u32;
	
	let _m_cost_factor = _ballast_count;
	let _t_cost_factor = _ballast_count;
	let _p_cost_factor = 1;
	
	argon_derive (
			InternalBallastArgon::wrap,
			_ballast_hash.access (),
			_ballast_salt.access (),
			CRYPTO_BALLAST_ARGON_M_COST * _m_cost_factor,
			CRYPTO_BALLAST_ARGON_T_COST * _t_cost_factor,
			CRYPTO_BALLAST_ARGON_P_COST * _p_cost_factor,
		) .else_wrap (0x5e8b2b57)
}








pub(crate) fn wrap_senders_and_recipients_inputs <'a> (
			_senders : &'a [&'a SenderPrivateKey],
			_recipients : &'a [&'a RecipientPublicKey],
		) -> CryptoResult<(
			Vec<&'a x25519::StaticSecret>,
			Vec<&'a x25519::PublicKey>,
		)>
{
	let _private_keys = Vec::from (_senders) .into_iter () .map (SenderPrivateKey::access) .collect ();
	let _public_keys = Vec::from (_recipients) .into_iter () .map (RecipientPublicKey::access) .collect ();
	
	wrap_private_and_public_keys (_private_keys, _public_keys)
}


pub(crate) fn wrap_recipients_and_senders_inputs <'a> (
			_recipients : &'a [&'a RecipientPrivateKey],
			_senders : &'a [&'a SenderPublicKey],
		) -> CryptoResult<(
			Vec<&'a x25519::StaticSecret>,
			Vec<&'a x25519::PublicKey>,
		)>
{
	let _private_keys = Vec::from (_recipients) .into_iter () .map (RecipientPrivateKey::access) .collect ();
	let _public_keys = Vec::from (_senders) .into_iter () .map (SenderPublicKey::access) .collect ();
	
	wrap_private_and_public_keys (_private_keys, _public_keys)
}


pub(crate) fn wrap_private_and_public_keys <'a> (
			mut _private_keys : Vec<&'a x25519::StaticSecret>,
			mut _public_keys : Vec<&'a x25519::PublicKey>,
		) -> CryptoResult<(
			Vec<&'a x25519::StaticSecret>,
			Vec<&'a x25519::PublicKey>,
		)>
{
	debug_assert! (CRYPTO_X25519_COUNT_MAX <= (u32::MAX as usize), "[b370bd5b]");
	
	if _private_keys.len () > CRYPTO_X25519_COUNT_MAX {
		fail! (0x00bc509c);
	}
	if _public_keys.len () > CRYPTO_X25519_COUNT_MAX {
		fail! (0x7f1713ae);
	}
	
	_private_keys.sort_by (|_left, _right| Ord::cmp (_left.as_bytes (), _right.as_bytes ()));
	_private_keys.dedup_by (|_left, _right| PartialEq::eq (_left.as_bytes (), _right.as_bytes ()));
	
	_public_keys.sort_by (|_left, _right| Ord::cmp (_left.as_bytes (), _right.as_bytes ()));
	_public_keys.dedup_by (|_left, _right| PartialEq::eq (_left.as_bytes (), _right.as_bytes ()));
	
	Ok ((_private_keys, _public_keys))
}








pub(crate) fn wrap_associated_and_secrets_and_pins_inputs <'a> (
			_associated_inputs : &'a [&'a [u8]],
			_secret_inputs : &'a [&'a [u8]],
			_pin_inputs : &'a [&'a [u8]],
			_seed_inputs : &'a [&'a [u8]],
			_ballast_inputs : &'a [&'a [u8]],
		) -> CryptoResult<(
			Vec<InternalAssociatedInput<'a>>,
			Vec<InternalSecretInput<'a>>,
			Vec<InternalPinInput<'a>>,
			Vec<InternalSeedInput<'a>>,
			Vec<InternalBallastInput<'a>>,
		)>
{
	debug_assert! (CRYPTO_ASSOCIATED_COUNT_MAX <= (u32::MAX as usize), "[aa8fdcf2]");
	debug_assert! (CRYPTO_SECRET_COUNT_MAX <= (u32::MAX as usize), "[424cdca6]");
	debug_assert! (CRYPTO_PIN_COUNT_MAX <= (u32::MAX as usize), "[f1d98265]");
	debug_assert! (CRYPTO_SEED_COUNT_MAX <= (u32::MAX as usize), "[19367164]");
	debug_assert! (CRYPTO_BALLAST_COUNT_MAX <= (u32::MAX as usize), "[f5248ca4]");
	
	if _associated_inputs.len () > CRYPTO_ASSOCIATED_COUNT_MAX {
		fail! (0xa8b5584a);
	}
	if _secret_inputs.len () > CRYPTO_SECRET_COUNT_MAX {
		fail! (0x6eceb6e4);
	}
	if _pin_inputs.len () > CRYPTO_PIN_COUNT_MAX {
		fail! (0x8b060b37);
	}
	if _seed_inputs.len () > CRYPTO_SEED_COUNT_MAX {
		fail! (0xc1484864);
	}
	if _ballast_inputs.len () > CRYPTO_BALLAST_COUNT_MAX {
		fail! (0x618972f7);
	}
	
	let _associated_inputs = Vec::from (_associated_inputs) .into_iter () .map (InternalAssociatedInput::wrap) .collect ();
	let _secret_inputs = Vec::from (_secret_inputs) .into_iter () .map (InternalSecretInput::wrap) .collect ();
	let _pin_inputs = Vec::from (_pin_inputs) .into_iter () .map (InternalPinInput::wrap) .collect ();
	let _seed_inputs = Vec::from (_seed_inputs) .into_iter () .map (InternalSeedInput::wrap) .collect ();
	let _ballast_inputs = Vec::from (_ballast_inputs) .into_iter () .map (InternalBallastInput::wrap) .collect ();
	
	Ok ((_associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs))
}








pub(crate) fn wrap_oracles_phase_1 <'a> (
			_oracles : Vec<&'a mut dyn Oracle>,
		) -> CryptoResult<(
			Vec<&'a mut dyn Oracle>,
			Vec<InternalOracleHandle>,
		)>
{
	let _oracles_with_handles = wrap_oracles_internal (_oracles, None) ?;
	let (_oracles, _oracle_handles) = _oracles_with_handles.into_iter () .unzip ();
	Ok ((_oracles, _oracle_handles))
}


pub(crate) fn wrap_oracles_phase_2 <'a> (
			_oracles : Vec<&'a mut dyn Oracle>,
			_sorter : InternalOracleSorter,
		) -> CryptoResult<
			Vec<(&'a mut dyn Oracle, InternalOracleHandle)>,
		>
{
	let _oracles_with_handles = wrap_oracles_internal (_oracles, Some (_sorter)) ?;
	Ok (_oracles_with_handles)
}


pub(crate) fn wrap_oracles_internal <'a> (
			_oracles : Vec<&'a mut dyn Oracle>,
			_sorter : Option<InternalOracleSorter>,
		) -> CryptoResult<
			Vec<(&'a mut dyn Oracle, InternalOracleHandle)>,
		>
{
	debug_assert! (CRYPTO_ORACLE_COUNT_MAX <= (u32::MAX as usize), "[8d49c9e0]");
	
	if _oracles.len () > CRYPTO_ORACLE_COUNT_MAX {
		fail! (0x22fb37e2);
	}
	
	let mut _oracles_with_handles : Vec<_> = _oracles.into_iter ()
			.map (
				|_oracle| {
					let _oracle_handle = _oracle.handle () .as_raw ();
					let _oracle_handle = if let Some (_sorter) = &_sorter {
							blake3_keyed_hash (
									InternalOracleHandle::wrap,
									_sorter.access (),
									&[
										_oracle_handle,
									],
									&[],
								)
						} else {
							InternalOracleHandle::wrap_copy (_oracle_handle)
						};
					(_oracle, _oracle_handle)
				})
			.collect ();
	
	_oracles_with_handles.sort_by (|_left, _right| Ord::cmp (_left.1.access (), _right.1.access ()));
	_oracles_with_handles.dedup_by (|_left, _right| PartialEq::eq (_left.1.access (), _right.1.access ()));
	
	Ok (_oracles_with_handles)
}


