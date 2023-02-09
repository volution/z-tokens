

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use crate::keys::*;
use crate::coding::*;
use crate::low::*;
use crate::macros::*;
use crate::ssh::SshWrapper;
use crate::ssh::SshResult;


use ::x25519_dalek as x25519;








define_error! (pub CryptoError, result : CryptoResult);




pub const CRYPTO_DECRYPTED_SIZE_MAX : usize = 128 * 1024 * 1024;

pub const CRYPTO_ENCRYPTED_SIZE_MAX : usize =
		(
			(
				(
					CRYPTO_DECRYPTED_SIZE_MAX
					+ 4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD
				) / CODING_CHUNK_DECODED_SIZE
				+ 1
			) / CODING_CHUNKS_PER_LINE
			+ 1
		) * (
			9 + 4 + 1
			+ CODING_CHUNKS_PER_LINE * (CODING_CHUNKS_PER_LINE + CODING_CHUNK_ENCODED_SIZE + 1)
		);


pub const CRYPTO_ASSOCIATED_COUNT_MAX : usize = 1024;
pub const CRYPTO_SECRET_COUNT_MAX : usize = 1024;
pub const CRYPTO_PIN_COUNT_MAX : usize = 1024;
pub const CRYPTO_ORACLE_COUNT_MAX : usize = 1024;




const CRYPTO_ENCRYPTED_PADDING : usize = 256;
const CRYPTO_ENCRYPTED_OVERHEAD : usize = CRYPTO_ENCRYPTED_SALT + CRYPTO_ENCRYPTED_MAC;
const CRYPTO_ENCRYPTED_SALT : usize = InternalPacketSalt::SIZE;
const CRYPTO_ENCRYPTED_MAC : usize = InternalAuthenticationMac::SIZE;




const CRYPTO_SECRET_ARGON_M_COST : u32 = 512 * 1024;
const CRYPTO_SECRET_ARGON_T_COST : u32 = 8;

const CRYPTO_PIN_ARGON_M_COST : u32 = 32 * 1024;
const CRYPTO_PIN_ARGON_T_COST : u32 = 4;








define_cryptographic_material! (InternalDheKey, 32);
define_cryptographic_material! (InternalPartialKey, 32);
define_cryptographic_material! (InternalAontKey, 32);

define_cryptographic_material! (InternalPacketSalt, 32);
define_cryptographic_material! (InternalPacketKey, 32);

define_cryptographic_material! (InternalEncryptionKey, 32);

define_cryptographic_material! (InternalAuthenticationKey, 32);
define_cryptographic_material! (InternalAuthenticationMac, 32);

define_cryptographic_material! (InternalAssociatedInput, input, slice);
define_cryptographic_material! (InternalAssociatedHash, 32);

define_cryptographic_material! (InternalSecretInput, input, slice);
define_cryptographic_material! (InternalSecretHash, 32);
define_cryptographic_material! (InternalSecretSalt, 32);
define_cryptographic_material! (InternalSecretArgon, 32);
define_cryptographic_material! (InternalSecretKey, 32);

define_cryptographic_material! (InternalPinInput, input, slice);
define_cryptographic_material! (InternalPinHash, 32);
define_cryptographic_material! (InternalPinSalt, 32);
define_cryptographic_material! (InternalPinArgon, 32);
define_cryptographic_material! (InternalPinKey, 32);

define_cryptographic_material! (InternalOracleHandle, 32);
define_cryptographic_material! (InternalOracleInput, 32);
define_cryptographic_material! (InternalOracleOutput, 32);
define_cryptographic_material! (InternalOracleKey, 32);

define_cryptographic_material! (InternalDecryptedData, input, slice);
define_cryptographic_material! (InternalEncryptedData, input, slice);

define_cryptographic_material! (InternalPasswordData, input, slice);
define_cryptographic_material! (InternalPasswordOutput, 32);




define_cryptographic_context! (CRYPTO_DHE_KEY_CONTEXT, encryption, dhe_key);
define_cryptographic_context! (CRYPTO_PARTIAL_KEY_CONTEXT, encryption, partial_key);
define_cryptographic_context! (CRYPTO_AONT_KEY_CONTEXT, encryption, aont_key);

define_cryptographic_context! (CRYPTO_PACKET_SALT_CONTEXT, encryption, packet_salt);
define_cryptographic_context! (CRYPTO_PACKET_KEY_CONTEXT, encryption, packet_key);
define_cryptographic_context! (CRYPTO_ENCRYPTION_KEY_CONTEXT, encryption, encryption_key);
define_cryptographic_context! (CRYPTO_AUTHENTICATION_KEY_CONTEXT, encryption, authentication_key);

define_cryptographic_context! (CRYPTO_ASSOCIATED_HASH_CONTEXT, encryption, associated_hash);

define_cryptographic_context! (CRYPTO_SECRET_HASH_CONTEXT, encryption, secret_hash);
define_cryptographic_context! (CRYPTO_SECRET_SALT_CONTEXT, encryption, secret_salt);
define_cryptographic_context! (CRYPTO_SECRET_KEY_CONTEXT, encryption, secret_key);

define_cryptographic_context! (CRYPTO_PIN_HASH_CONTEXT, encryption, pin_hash);
define_cryptographic_context! (CRYPTO_PIN_SALT_CONTEXT, encryption, pin_salt);
define_cryptographic_context! (CRYPTO_PIN_KEY_CONTEXT, encryption, pin_key);

define_cryptographic_context! (CRYPTO_ORACLE_HANDLE_CONTEXT, encryption, oracle_handle);
define_cryptographic_context! (CRYPTO_ORACLE_INPUT_CONTEXT, encryption, oracle_input);
define_cryptographic_context! (CRYPTO_ORACLE_OUTPUT_CONTEXT, encryption, oracle_output);

define_cryptographic_context! (CRYPTO_PASSWORD_SALT_CONTEXT, password, salt);
define_cryptographic_context! (CRYPTO_PASSWORD_OUTPUT_CONTEXT, password, output);








pub fn password (
			_sender : Option<&SenderPrivateKey>,
			_recipient : Option<&RecipientPublicKey>,
			_associated_inputs : &[&[u8]],
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_ssh_wrappers : Vec<&mut SshWrapper>,
		) -> CryptoResult
{
	let (_associated_inputs, _secret_inputs, _pin_inputs) = wrap_associated_and_secrets_and_pins_inputs (_associated_inputs, _secret_inputs, _pin_inputs) ?;
	let (_oracles, _oracle_handles) = wrap_oracles (_ssh_wrappers) ?;
	
	let _password_data = InternalPasswordData::wrap (_password_data);
	let _password_data_len = _password_data.size ();
	
	if _password_data_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0xfa4d9417);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  deriving keys...
	
	let _sender = _sender.map (SenderPrivateKey::access);
	let _recipient = _recipient.map (RecipientPublicKey::access);
	
	let (_partial_key, _aont_key, _secret_hashes, _pin_hashes, _oracle_hashes)
			= derive_keys_phase_1 (_sender, _recipient, _associated_inputs, _secret_inputs, _pin_inputs, _oracle_handles, true) ?;
	
	drop! (_sender, _recipient);
	drop! (_aont_key);
	
	// NOTE:  salting...
	
	let _packet_salt = blake3_derive_key (
			InternalPacketSalt::wrap,
			CRYPTO_PASSWORD_SALT_CONTEXT,
			&[
				_partial_key.access (),
			],
			&[
				_password_data.access (),
			],
			None,
		);
	
	drop! (_password_data);
	
	let (_packet_key, _encryption_key, _authentication_key)
			= derive_keys_phase_2 (_partial_key, &_packet_salt, _secret_hashes, _pin_hashes, (_oracles, _oracle_hashes)) ?;
	
	drop! (_encryption_key, _authentication_key);
	
	let _password_output_0 = blake3_derive_key (
			InternalPasswordOutput::wrap,
			CRYPTO_PASSWORD_OUTPUT_CONTEXT,
			&[
				_packet_key.access (),
			],
			&[],
			None,
		);
	
	drop! (_packet_key);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_password_output.copy_from_slice (_password_output_0.access ());
	
	Ok (())
}








pub fn encrypt (
			_sender : Option<&SenderPrivateKey>,
			_recipient : Option<&RecipientPublicKey>,
			_associated_inputs : &[&[u8]],
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_decrypted : &[u8],
			_encrypted : &mut Vec<u8>,
			_ssh_wrappers : Vec<&mut SshWrapper>,
			_packet_salt_deterministic : bool,
		) -> CryptoResult
{
	let (_associated_inputs, _secret_inputs, _pin_inputs) = wrap_associated_and_secrets_and_pins_inputs (_associated_inputs, _secret_inputs, _pin_inputs) ?;
	let (_oracles, _oracle_handles) = wrap_oracles (_ssh_wrappers) ?;
	
	let _decrypted = InternalDecryptedData::wrap (_decrypted);
	let _decrypted_len = _decrypted.size ();
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x83d6c657);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  compressing...
	
	let _compress_capacity = compress_capacity_max (_decrypted_len) .else_wrap (0x4198ca8b) ?;
	let _compress_capacity = _compress_capacity + 4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD;
	
	let mut _intermediate_buffer = Vec::with_capacity (_compress_capacity);
	compress (_decrypted.access (), &mut _intermediate_buffer) .else_wrap (0xa9fadcdc) ?;
	
	if _intermediate_buffer.len () >= _decrypted_len {
		
		_intermediate_buffer.clear ();
		_intermediate_buffer.extend_from_slice (_decrypted.access ());
	}
	
	// NOTE:  padding...
	
	encode_u32_push (_decrypted_len as u32, &mut _intermediate_buffer);
	
	padding_push (CRYPTO_ENCRYPTED_PADDING, &mut _intermediate_buffer);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  deriving keys...
	
	let _sender = _sender.map (SenderPrivateKey::access);
	let _recipient = _recipient.map (RecipientPublicKey::access);
	
	let (_partial_key, _aont_key, _secret_hashes, _pin_hashes, _oracle_hashes)
			= derive_keys_phase_1 (_sender, _recipient, _associated_inputs, _secret_inputs, _pin_inputs, _oracle_handles, true) ?;
	
	drop! (_sender, _recipient);
	
	// NOTE:  salting...
	
	let mut _packet_salt = if _packet_salt_deterministic {
			
			blake3_derive_key (
					InternalPacketSalt::wrap,
					CRYPTO_PACKET_SALT_CONTEXT,
					&[
						_partial_key.access (),
					],
					&[
						_decrypted.access (),
					],
					None,
				)
			
		} else {
			
			generate_random (InternalPacketSalt::wrap)
		};
	
	drop! (_decrypted);
	
	let (_packet_key, _encryption_key, _authentication_key)
			= derive_keys_phase_2 (_partial_key, &_packet_salt, _secret_hashes, _pin_hashes, (_oracles, _oracle_hashes)) ?;
	
	drop! (_packet_key);
	
	// NOTE:  encryption...
	
	apply_encryption (_encryption_key, &mut _intermediate_buffer) ?;
	
	// NOTE:  authentication...
	
	let _mac = apply_authentication (_authentication_key, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (_mac.access ());
	
	drop! (_mac);
	
	// NOTE:  all-or-nothing...
	
	apply_all_or_nothing_mangling (_aont_key, &mut _packet_salt, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (_packet_salt.access ());
	
	drop! (_packet_salt);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  encoding...
	
	assert! (_intermediate_buffer.len () <= (_decrypted_len + 4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD), "[0e17b154]");
	
	let _encode_capacity = encode_capacity_max (_intermediate_buffer.len ()) .else_wrap (0x7f15a8ec) ?;
	
	let mut _encode_buffer = Vec::with_capacity (_encode_capacity);
	encode (&_intermediate_buffer, &mut _encode_buffer) .else_wrap (0x5bc239f9) ?;
	
	assert! (_encode_buffer.len () <= CRYPTO_ENCRYPTED_SIZE_MAX, "[bb3c2546]  {} <= {}", _encode_buffer.len (), CRYPTO_ENCRYPTED_SIZE_MAX);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_encrypted.extend_from_slice (&_encode_buffer);
	
	Ok (())
}








pub fn decrypt (
			_recipient : Option<&RecipientPrivateKey>,
			_sender : Option<&SenderPublicKey>,
			_associated_inputs : &[&[u8]],
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_encrypted : &[u8],
			_decrypted : &mut Vec<u8>,
			_ssh_wrappers : Vec<&mut SshWrapper>,
		) -> CryptoResult
{
	let (_associated_inputs, _secret_inputs, _pin_inputs) = wrap_associated_and_secrets_and_pins_inputs (_associated_inputs, _secret_inputs, _pin_inputs) ?;
	let (_oracles, _oracle_handles) = wrap_oracles (_ssh_wrappers) ?;
	
	let _encrypted = InternalEncryptedData::wrap (_encrypted);
	let _encrypted_len = _encrypted.size ();
	
	if _encrypted_len > CRYPTO_ENCRYPTED_SIZE_MAX {
		fail! (0x5832104d);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  decoding...
	
	let _decode_capacity = decode_capacity_max (_encrypted_len) .else_wrap (0xae545303) ?;
	
	let mut _intermediate_buffer = Vec::with_capacity (_decode_capacity);
	decode (_encrypted.access (), &mut _intermediate_buffer) .else_wrap (0x10ff413a) ?;
	
	drop! (_encrypted);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  deriving keys...
	
	let _sender = _sender.map (SenderPublicKey::access);
	let _recipient = _recipient.map (RecipientPrivateKey::access);
	
	let (_partial_key, _aont_key, _secret_hashes, _pin_hashes, _oracle_hashes)
			= derive_keys_phase_1 (_recipient, _sender, _associated_inputs, _secret_inputs, _pin_inputs, _oracle_handles, false) ?;
	
	drop! (_sender, _recipient);
	
	// NOTE:  all-or-nothing and salting...
	
	let _packet_salt = bytes_pop::<CRYPTO_ENCRYPTED_SALT> (&mut _intermediate_buffer) .else_wrap (0x78ed3811) ?;
	let mut _packet_salt = InternalPacketSalt::wrap (_packet_salt);
	
	apply_all_or_nothing_mangling (_aont_key, &mut _packet_salt, &_intermediate_buffer) ?;
	
	// NOTE:  deriving keys...
	
	let (_packet_key, _encryption_key, _authentication_key)
			= derive_keys_phase_2 (_partial_key, &_packet_salt, _secret_hashes, _pin_hashes, (_oracles, _oracle_hashes)) ?;
	
	drop! (_packet_key);
	drop! (_packet_salt);
	
	// NOTE:  authentication...
	
	let _mac_expected = bytes_pop::<CRYPTO_ENCRYPTED_MAC> (&mut _intermediate_buffer) .else_wrap (0x88084589) ?;
	let _mac_expected = InternalAuthenticationMac::wrap (_mac_expected);
	
	let _mac_actual = apply_authentication (_authentication_key, &_intermediate_buffer) ?;
	
	if ! InternalAuthenticationMac::compare_consume (_mac_actual, _mac_expected) {
		fail! (0xad70c84c);
	}
	
	// NOTE:  decryption...
	
	apply_encryption (_encryption_key, &mut _intermediate_buffer) ?;
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  padding...
	
	padding_pop (CRYPTO_ENCRYPTED_PADDING, &mut _intermediate_buffer) .else_wrap (0xbbdd100e) ?;
	
	let _decrypted_len = decode_u32_pop (&mut _intermediate_buffer) .else_wrap (0xa8b8f7d8) ? as usize;
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x433f5bb6);
	}
	
	// NOTE:  decompressing...
	
	let _decompress_buffer = if _decrypted_len > _intermediate_buffer.len () {
		
		let mut _decompress_buffer = Vec::with_capacity (_decrypted_len);
		decompress (&_intermediate_buffer, &mut _decompress_buffer) .else_wrap (0xec71bc5c) ?;
		
		_decompress_buffer
	} else {
		_intermediate_buffer
	};
	
	if _decompress_buffer.len () != _decrypted_len {
		fail! (0x0610eb74);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_decrypted.extend_from_slice (&_decompress_buffer);
	
	Ok (())
}








fn apply_encryption (_key : InternalEncryptionKey, _data : &mut [u8]) -> CryptoResult {
	
	use ::chacha20::cipher::KeyIvInit as _;
	use ::chacha20::cipher::StreamCipher as _;
	
	let _nonce = [0u8; 12];
	
	let _key = ::chacha20::Key::from (_key.material);
	let _nonce = ::chacha20::Nonce::from (_nonce);
	
	let mut _cipher = ::chacha20::ChaCha20::new (&_key, &_nonce);
	
	_cipher.try_apply_keystream (_data) .else_wrap (0x9c94d0d5) ?;
	
	Ok (())
}




fn apply_authentication (_key : InternalAuthenticationKey, _data : &[u8]) -> CryptoResult<InternalAuthenticationMac> {
	
	let _mac = blake3_keyed_hash (
			InternalAuthenticationMac::wrap,
			_key.access (),
			&[],
			&[
				_data,
			],
			None,
		);
	
	Ok (_mac)
}




fn apply_all_or_nothing_mangling (_key : InternalAontKey, _packet_salt : &mut InternalPacketSalt, _data : &[u8]) -> CryptoResult {
	
	const _SIZE : usize = InternalPacketSalt::SIZE;
	
	let _hash : [u8; _SIZE] = blake3_keyed_hash (
			|_hash| _hash,
			_key.access (),
			&[],
			&[
				_data,
			],
			None,
		);
	
	let _packet_salt = &mut _packet_salt.material;
	
	for _index in 0 .. _SIZE {
		_packet_salt[_index] ^= _hash[_index];
	}
	
	Ok (())
}








fn derive_keys_phase_1 (
			_private : Option<&x25519::StaticSecret>,
			_public : Option<&x25519::PublicKey>,
			_associated_inputs : Vec<InternalAssociatedInput>,
			_secret_inputs : Vec<InternalSecretInput>,
			_pin_inputs : Vec<InternalPinInput>,
			_oracle_handles : Vec<InternalOracleHandle>,
			_encryption : bool,
		) -> CryptoResult<(
			InternalPartialKey,
			InternalAontKey,
			(InternalSecretHash, Vec<InternalSecretHash>),
			(InternalPinHash, Vec<InternalPinHash>),
			InternalOracleHandle,
		)>
{
	// --------------------------------------------------------------------------------
	// NOTE:  derive associated hashes...
	
	let mut _associated_hashes : Vec<_> = _associated_inputs.into_iter () .map (
			|_associated_input|
					blake3_derive_key (
							InternalAssociatedHash::wrap,
							CRYPTO_ASSOCIATED_HASH_CONTEXT,
							&[],
							&[
								_associated_input.access_consume (),
							],
							None,
						)
		) .collect ();
	
	// NOTE:  associated data is not sorted or deduplicated, thus order is important!
	
	let _associated_hash = blake3_derive_key_join (
			InternalAssociatedHash::wrap,
			CRYPTO_ASSOCIATED_HASH_CONTEXT,
			_associated_hashes.iter () .map (InternalAssociatedHash::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive secret hashes...
	
	let mut _secret_hashes : Vec<_> = _secret_inputs.into_iter () .map (
			|_secret_input|
					blake3_derive_key (
							InternalSecretHash::wrap,
							CRYPTO_SECRET_HASH_CONTEXT,
							&[],
							&[
								_secret_input.access_consume (),
							],
							None,
						)
		) .collect ();
	
	_secret_hashes.sort_by (InternalSecretHash::cmp_access);
	_secret_hashes.dedup_by (|_left, _right| InternalSecretHash::eq_access (_left, _right));
	
	let _secret_hash = blake3_derive_key_join (
			InternalSecretHash::wrap,
			CRYPTO_SECRET_HASH_CONTEXT,
			_secret_hashes.iter () .map (InternalSecretHash::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive pin hashes...
	
	let mut _pin_hashes : Vec<_> = _pin_inputs.into_iter () .map (
			|_pin_input|
					blake3_derive_key (
							InternalPinHash::wrap,
							CRYPTO_PIN_HASH_CONTEXT,
							&[],
							&[
								_pin_input.access_consume (),
							],
							None,
						)
		) .collect ();
	
	_pin_hashes.sort_by (InternalPinHash::cmp_access);
	_pin_hashes.dedup_by (|_left, _right| InternalPinHash::eq_access (_left, _right));
	
	let _pin_hash = blake3_derive_key_join (
			InternalPinHash::wrap,
			CRYPTO_PIN_HASH_CONTEXT,
			_pin_hashes.iter () .map (InternalPinHash::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive oracle hashes...
	
	let _oracle_hash = blake3_derive_key_join (
			InternalOracleHandle::wrap,
			CRYPTO_ORACLE_HANDLE_CONTEXT,
			_oracle_handles.iter () .map (InternalOracleHandle::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive X25519 DHE...
	
	let _dhe_key = if let Some (_private) = _private {
			
			x25519_dhe (
				InternalDheKey::wrap,
				CRYPTO_DHE_KEY_CONTEXT,
				_private,
				_public,
				_encryption,
			) ?
			
		} else {
			
			if _public.is_some () {
				fail! (0x884cbe55);
			}
			if _secret_hashes.is_empty () && _pin_hashes.is_empty () && _oracle_handles.is_empty () {
				fail! (0xa1de0167);
			}
			
			InternalDheKey::zero ()
		};
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive partial key (for the entire transaction)...
	
	let _partial_key = blake3_derive_key (
			InternalPartialKey::wrap,
			CRYPTO_PARTIAL_KEY_CONTEXT,
			&[
				_oracle_hash.access (),
				_secret_hash.access (),
				_pin_hash.access (),
				_associated_hash.access (),
				_dhe_key.access (),
			],
			&[],
			None,
		);
	
	drop! (_associated_hash);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive AONT key...
	
	let _aont_key = blake3_derive_key (
			InternalAontKey::wrap,
			CRYPTO_AONT_KEY_CONTEXT,
			&[
				_partial_key.access (),
			],
			&[],
			None,
		);
	
	// --------------------------------------------------------------------------------
	
	Ok ((
			_partial_key,
			_aont_key,
			(_secret_hash, _secret_hashes),
			(_pin_hash, _pin_hashes),
			_oracle_hash,
		))
}








fn derive_keys_phase_2 (
			_partial_key : InternalPartialKey,
			_packet_salt : &InternalPacketSalt,
			_secret_hash : (InternalSecretHash, Vec<InternalSecretHash>),
			_pin_hash : (InternalPinHash, Vec<InternalPinHash>),
			_oracles : (Vec<(&mut SshWrapper, InternalOracleHandle)>, InternalOracleHandle),
		) -> CryptoResult<(
			InternalPacketKey,
			InternalEncryptionKey,
			InternalAuthenticationKey,
		)>
{
	let (_secret_hash, _secret_hashes) = _secret_hash;
	let (_pin_hash, _pin_hashes) = _pin_hash;
	let (_oracles, _oracle_hashes) = _oracles;
	
	// --------------------------------------------------------------------------------
	// NOTE:  call SSH wrappers...
	
	let mut _oracle_key = InternalOracleOutput::wrap (_oracle_hashes.material);
	
	for (_oracle_wrapper, _oracle_handle) in _oracles.into_iter () {
		
		let _oracle_input = blake3_derive_key (
				InternalOracleInput::wrap,
				CRYPTO_ORACLE_INPUT_CONTEXT,
				&[
					_oracle_handle.access (),
					_oracle_key.access (),
					_packet_salt.access (),
					_partial_key.access (),
				],
				&[],
				None,
			);
		
		let mut _oracle_output = InternalOracleOutput::zero ();
		_oracle_wrapper.wrap (_oracle_input.access (), &mut _oracle_output.material) .else_wrap (0xcc07e95e) ?;
		
		_oracle_key = blake3_derive_key (
				InternalOracleOutput::wrap,
				CRYPTO_ORACLE_OUTPUT_CONTEXT,
				&[
					_oracle_input.access (),
					_oracle_output.access (),
				],
				&[],
				None,
			);
	}
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive secret argon hashes...
	
	let mut _secret_key = InternalSecretKey::wrap (_secret_hash.material);
	
	for _secret_hash in _secret_hashes.into_iter () {
		
		let _secret_salt = blake3_derive_key (
				InternalSecretSalt::wrap,
				CRYPTO_SECRET_SALT_CONTEXT,
				&[
					_secret_key.access (),
					_oracle_key.access (),
					_packet_salt.access (),
					_partial_key.access (),
				],
				&[],
				None,
			);
		
		let _secret_argon = apply_argon_secret (_secret_hash, &_secret_salt) ?;
		
		_secret_key = blake3_derive_key (
				InternalSecretKey::wrap,
				CRYPTO_SECRET_KEY_CONTEXT,
				&[
					_secret_salt.access (),
					_secret_argon.access (),
				],
				&[],
				None,
			);
	}
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive pin argon hashes...
	
	let mut _pin_key = InternalPinKey::wrap (_pin_hash.material);
	
	for _pin_hash in _pin_hashes.into_iter () {
		
		let _pin_salt = blake3_derive_key (
				InternalPinSalt::wrap,
				CRYPTO_PIN_SALT_CONTEXT,
				&[
					_pin_key.access (),
					_oracle_key.access (),
					_packet_salt.access (),
					_partial_key.access (),
				],
				&[],
				None,
			);
		
		let _pin_argon = apply_argon_pin (_pin_hash, &_pin_salt) ?;
		
		_pin_key = blake3_derive_key (
				InternalPinKey::wrap,
				CRYPTO_PIN_KEY_CONTEXT,
				&[
					_pin_salt.access (),
					_pin_argon.access (),
				],
				&[],
				None,
			);
	}
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive packet key...
	
	let _packet_key = blake3_derive_key (
			InternalPacketKey::wrap,
			CRYPTO_PACKET_KEY_CONTEXT,
			&[
				_oracle_key.access (),
				_secret_key.access (),
				_pin_key.access (),
				_packet_salt.access (),
				_partial_key.access (),
			],
			&[],
			None,
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive encryption key...
	
	let _encryption_key = blake3_derive_key (
			InternalEncryptionKey::wrap,
			CRYPTO_ENCRYPTION_KEY_CONTEXT,
			&[
				_packet_key.access (),
			],
			&[],
			None,
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive authentication key...
	
	let _authentication_key = blake3_derive_key (
			InternalAuthenticationKey::wrap,
			CRYPTO_AUTHENTICATION_KEY_CONTEXT,
			&[
				_packet_key.access (),
			],
			&[],
			None,
		);
	
	// --------------------------------------------------------------------------------
	
	Ok ((_packet_key, _encryption_key, _authentication_key))
}








fn apply_argon_secret (_secret_hash : InternalSecretHash, _secret_salt : &InternalSecretSalt) -> CryptoResult<InternalSecretArgon> {
	
	argon_derive (
			InternalSecretArgon::wrap,
			_secret_hash.access (),
			_secret_salt.access (),
			CRYPTO_SECRET_ARGON_M_COST,
			CRYPTO_SECRET_ARGON_T_COST,
		)
}


fn apply_argon_pin (_pin_hash : InternalPinHash, _pin_salt : &InternalPinSalt) -> CryptoResult<InternalPinArgon> {
	
	argon_derive (
			InternalPinArgon::wrap,
			_pin_hash.access (),
			_pin_salt.access (),
			CRYPTO_PIN_ARGON_M_COST,
			CRYPTO_PIN_ARGON_T_COST,
		)
}








fn wrap_associated_and_secrets_and_pins_inputs <'a> (
			_associated_inputs : &'a [&'a [u8]],
			_secret_inputs : &'a [&'a [u8]],
			_pin_inputs : &'a [&'a [u8]],
		) -> CryptoResult<(
			Vec<InternalAssociatedInput<'a>>,
			Vec<InternalSecretInput<'a>>,
			Vec<InternalPinInput<'a>>,
		)>
{
	debug_assert! (CRYPTO_ASSOCIATED_COUNT_MAX <= (u32::MAX as usize), "[aa8fdcf2]");
	debug_assert! (CRYPTO_SECRET_COUNT_MAX <= (u32::MAX as usize), "[424cdca6]");
	debug_assert! (CRYPTO_PIN_COUNT_MAX <= (u32::MAX as usize), "[f1d98265]");
	
	if _associated_inputs.len () > CRYPTO_ASSOCIATED_COUNT_MAX {
		fail! (0xa8b5584a);
	}
	if _secret_inputs.len () > CRYPTO_SECRET_COUNT_MAX {
		fail! (0x6eceb6e4);
	}
	if _pin_inputs.len () > CRYPTO_PIN_COUNT_MAX {
		fail! (0x8b060b37);
	}
	
	let _associated_inputs = Vec::from (_associated_inputs) .into_iter () .map (InternalAssociatedInput::wrap) .collect ();
	let _secret_inputs = Vec::from (_secret_inputs) .into_iter () .map (InternalSecretInput::wrap) .collect ();
	let _pin_inputs = Vec::from (_pin_inputs) .into_iter () .map (InternalPinInput::wrap) .collect ();
	
	Ok ((_associated_inputs, _secret_inputs, _pin_inputs))
}




fn wrap_oracles <'a> (
			_ssh_wrappers : Vec<&'a mut SshWrapper>,
		) -> CryptoResult<(
			Vec<(&'a mut SshWrapper, InternalOracleHandle)>,
			Vec<InternalOracleHandle>,
		)>
{
	debug_assert! (CRYPTO_ORACLE_COUNT_MAX <= (u32::MAX as usize), "[8d49c9e0]");
	
	if _ssh_wrappers.len () > CRYPTO_ORACLE_COUNT_MAX {
		fail! (0x22fb37e2);
	}
	
	let mut _oracles : Vec<_> = _ssh_wrappers.into_iter ()
			.map (
				|_ssh_wrapper| {
					let _ssh_wrapper_handle = _ssh_wrapper.handle () ?;
					let _oracle_handle = InternalOracleHandle::wrap_copy (_ssh_wrapper_handle);
					Ok ((_ssh_wrapper, _oracle_handle))
				})
			.collect::<SshResult<_>> () .else_wrap (0xa0911f9c) ?;
	
	_oracles.sort_by (|_left, _right| Ord::cmp (_left.1.access (), _right.1.access ()));
	_oracles.dedup_by (|_left, _right| PartialEq::eq (_left.1.access (), _right.1.access ()));
	
	let _oracle_handles = _oracles.iter () .map (|_pair| InternalOracleHandle::wrap_copy (_pair.1.access ())) .collect ();
	
	Ok ((_oracles, _oracle_handles))
}


