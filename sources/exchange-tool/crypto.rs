

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use crate::keys::*;
use crate::coding::*;
use crate::low::*;
use crate::macros::*;
use crate::ssh::SshWrapper;


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


pub const CRYPTO_SECRET_COUNT_MAX : usize = 1024;
pub const CRYPTO_PIN_COUNT_MAX : usize = 1024;
pub const CRYPTO_SSH_WRAPPER_COUNT_MAX : usize = 1024;




const CRYPTO_ENCRYPTED_PADDING : usize = 256;
const CRYPTO_ENCRYPTED_OVERHEAD : usize = CRYPTO_ENCRYPTED_SALT + CRYPTO_ENCRYPTED_MAC;
const CRYPTO_ENCRYPTED_SALT : usize = InternalPacketSalt::SIZE;
const CRYPTO_ENCRYPTED_MAC : usize = InternalAuthenticationMac::SIZE;




const CRYPTO_SECRET_ARGON_M_COST : u32 = 512 * 1024;
const CRYPTO_SECRET_ARGON_T_COST : u32 = 8;

const CRYPTO_PIN_ARGON_M_COST : u32 = 32 * 1024;
const CRYPTO_PIN_ARGON_T_COST : u32 = 4;








define_cryptographic_material! (InternalDheKey, 32);
define_cryptographic_material! (InternalNaiveKey, 32);
define_cryptographic_material! (InternalAontKey, 32);

define_cryptographic_material! (InternalPacketSalt, 32);
define_cryptographic_material! (InternalPacketKey, 32);

define_cryptographic_material! (InternalEncryptionKey, 32);

define_cryptographic_material! (InternalAuthenticationKey, 32);
define_cryptographic_material! (InternalAuthenticationMac, 32);

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

define_cryptographic_material! (InternalSshWrapInput, 32);
define_cryptographic_material! (InternalSshWrapOutput, 32);

define_cryptographic_material! (InternalDataDecrypted, input, slice);
define_cryptographic_material! (InternalDataEncrypted, input, slice);




define_cryptographic_context! (CRYPTO_DHE_KEY_CONTEXT, encryption, dhe_key);
define_cryptographic_context! (CRYPTO_NAIVE_KEY_CONTEXT, encryption, naive_key);
define_cryptographic_context! (CRYPTO_AONT_KEY_CONTEXT, encryption, aont_key);

define_cryptographic_context! (CRYPTO_PACKET_KEY_CONTEXT, encryption, packet_key);
define_cryptographic_context! (CRYPTO_ENCRYPTION_KEY_CONTEXT, encryption, encryption_key);
define_cryptographic_context! (CRYPTO_AUTHENTICATION_KEY_CONTEXT, encryption, authentication_key);

define_cryptographic_context! (CRYPTO_SECRET_HASH_CONTEXT, encryption, secret_hash);
define_cryptographic_context! (CRYPTO_SECRET_SALT_CONTEXT, encryption, secret_salt);
define_cryptographic_context! (CRYPTO_SECRET_KEY_CONTEXT, encryption, secret_key);

define_cryptographic_context! (CRYPTO_PIN_HASH_CONTEXT, encryption, pin_hash);
define_cryptographic_context! (CRYPTO_PIN_SALT_CONTEXT, encryption, pin_salt);
define_cryptographic_context! (CRYPTO_PIN_KEY_CONTEXT, encryption, pin_key);

define_cryptographic_context! (CRYPTO_SSH_WRAP_INPUT_CONTEXT, encryption, ssh_wrap_input);
define_cryptographic_context! (CRYPTO_SSH_WRAP_OUTPUT_CONTEXT, encryption, ssh_wrap_output);








pub fn encrypt (
			_sender : Option<&SenderPrivateKey>,
			_recipient : Option<&RecipientPublicKey>,
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_decrypted : &[u8],
			_encrypted : &mut Vec<u8>,
			_ssh_wrappers : Vec<&mut SshWrapper>,
		) -> CryptoResult
{
	let (_secret_inputs, _pin_inputs) = wrap_secrets_and_pins_inputs (_secret_inputs, _pin_inputs) ?;
	let _ssh_wrappers = wrap_ssh_wrappers (_ssh_wrappers) ?;
	
	let _decrypted = InternalDataDecrypted::wrap (_decrypted);
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
	
	drop! (_decrypted);
	
	// NOTE:  padding...
	
	encode_u32_push (_decrypted_len as u32, &mut _intermediate_buffer);
	
	padding_push (CRYPTO_ENCRYPTED_PADDING, &mut _intermediate_buffer);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  deriving keys...
	
	let _sender = _sender.map (SenderPrivateKey::access);
	let _recipient = _recipient.map (RecipientPublicKey::access);
	
	let (_naive_key, _aont_key, _secret_hashes, _pin_hashes)
			= derive_keys_phase_1 (_sender, _recipient, _secret_inputs, _pin_inputs, true) ?;
	
	drop! (_sender, _recipient);
	
	// NOTE:  salting...
	
	let mut _packet_salt = generate_random (InternalPacketSalt::wrap);
	
	let (_encryption_key, _authentication_key)
			= derive_keys_phase_2 (_naive_key, &_packet_salt, _secret_hashes, _pin_hashes, _ssh_wrappers) ?;
	
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
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_encrypted : &[u8],
			_decrypted : &mut Vec<u8>,
			_ssh_wrappers : Vec<&mut SshWrapper>,
		) -> CryptoResult
{
	let (_secret_inputs, _pin_inputs) = wrap_secrets_and_pins_inputs (_secret_inputs, _pin_inputs) ?;
	let _ssh_wrappers = wrap_ssh_wrappers (_ssh_wrappers) ?;
	
	let _encrypted = InternalDataEncrypted::wrap (_encrypted);
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
	
	let (_naive_key, _aont_key, _secret_hashes, _pin_hashes)
			= derive_keys_phase_1 (_recipient, _sender, _secret_inputs, _pin_inputs, false) ?;
	
	drop! (_sender, _recipient);
	
	// NOTE:  all-or-nothing and salting...
	
	let _packet_salt = bytes_pop::<CRYPTO_ENCRYPTED_SALT> (&mut _intermediate_buffer) .else_wrap (0x78ed3811) ?;
	let mut _packet_salt = InternalPacketSalt::wrap (_packet_salt);
	
	apply_all_or_nothing_mangling (_aont_key, &mut _packet_salt, &_intermediate_buffer) ?;
	
	// NOTE:  deriving keys...
	
	let (_encryption_key, _authentication_key)
			= derive_keys_phase_2 (_naive_key, &_packet_salt, _secret_hashes, _pin_hashes, _ssh_wrappers) ?;
	
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
			_secret_inputs : Vec<InternalSecretInput>,
			_pin_inputs : Vec<InternalPinInput>,
			_encryption : bool,
		) -> CryptoResult<(InternalNaiveKey, InternalAontKey, (InternalSecretHash, Vec<InternalSecretHash>), (InternalPinHash, Vec<InternalPinHash>))>
{
	// --------------------------------------------------------------------------------
	// NOTE:  apply X25519 DHE...
	
	let _private = _private.else_wrap (0x70f91100) ?;
	
	let _dhe_key = x25519_dhe (
			InternalDheKey::wrap,
			CRYPTO_DHE_KEY_CONTEXT,
			_private,
			_public,
			_encryption,
		) ?;
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive secret hashes...
	
	let mut _secret_hashes : Vec<_> = _secret_inputs.into_iter () .enumerate () .map (
			|(_secret_index, _secret_input)|
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
	
	let mut _pin_hashes : Vec<_> = _pin_inputs.into_iter () .enumerate () .map (
			|(_pin_index, _pin_input)|
					blake3_derive_key (
							InternalPinHash::wrap,
							CRYPTO_SECRET_HASH_CONTEXT,
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
	// NOTE:  derive naive key (for the entire transaction)...
	
	let _naive_key = blake3_derive_key (
			InternalNaiveKey::wrap,
			CRYPTO_NAIVE_KEY_CONTEXT,
			&[
				_secret_hash.access (),
				_pin_hash.access (),
				_dhe_key.access (),
			],
			&[],
			None,
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive AONT key...
	
	let _aont_key = blake3_derive_key (
			InternalAontKey::wrap,
			CRYPTO_AONT_KEY_CONTEXT,
			&[
				_naive_key.access (),
			],
			&[],
			None,
		);
	
	// --------------------------------------------------------------------------------
	
	Ok ((_naive_key, _aont_key, (_secret_hash, _secret_hashes), (_pin_hash, _pin_hashes)))
}








fn derive_keys_phase_2 (
			_naive_key : InternalNaiveKey,
			_packet_salt : &InternalPacketSalt,
			_secret_hash : (InternalSecretHash, Vec<InternalSecretHash>),
			_pin_hash : (InternalPinHash, Vec<InternalPinHash>),
			_ssh_wrappers : Vec<&mut SshWrapper>,
		) -> CryptoResult<(InternalEncryptionKey, InternalAuthenticationKey)>
{
	let (_secret_hash, _secret_hashes) = _secret_hash;
	let (_pin_hash, _pin_hashes) = _pin_hash;
	
	let _secret_exists = ! _secret_hashes.is_empty ();
	let _pin_exists = ! _pin_hashes.is_empty ();
	
	// --------------------------------------------------------------------------------
	// NOTE:  call SSH wrapper (if exists)...
	
	let mut _ssh_wrap_key = InternalSshWrapOutput::zero ();
	
	for _ssh_wrapper in _ssh_wrappers.into_iter () {
		
		let _ssh_wrap_input = blake3_derive_key (
				InternalSshWrapInput::wrap,
				CRYPTO_SSH_WRAP_INPUT_CONTEXT,
				&[
					_ssh_wrap_key.access (),
					_packet_salt.access (),
					_naive_key.access (),
				],
				&[],
				None,
			);
		
		_ssh_wrap_key.consume ();
		
		// FIXME:  zeroize!
		let mut _ssh_wrap_output = [0u8; 32];
		_ssh_wrapper.wrap (_ssh_wrap_input.access (), &mut _ssh_wrap_output) .else_wrap (0xcc07e95e) ?;
		
		_ssh_wrap_key = blake3_derive_key (
				InternalSshWrapOutput::wrap,
				CRYPTO_SSH_WRAP_OUTPUT_CONTEXT,
				&[
					_packet_salt.access (),
					_naive_key.access (),
					&_ssh_wrap_output,
				],
				&[],
				None,
			);
	}
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive secret argon (if exists)...
	
	let mut _secret_key = InternalSecretKey::wrap (_secret_hash.material);
	
	for _secret_hash in _secret_hashes.into_iter () {
		
		let _secret_salt = blake3_derive_key (
				InternalSecretSalt::wrap,
				CRYPTO_SECRET_SALT_CONTEXT,
				&[
					_secret_key.access (),
					_ssh_wrap_key.access (),
					_packet_salt.access (),
					_naive_key.access (),
				],
				&[],
				None,
			);
		
		_secret_key.consume ();
		
		let _secret_argon = apply_argon_secret (_secret_hash, _secret_salt) ?;
		
		_secret_key = blake3_derive_key (
				InternalSecretKey::wrap,
				CRYPTO_SECRET_KEY_CONTEXT,
				&[
					_secret_argon.access (),
				],
				&[],
				None,
			);
	}
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive pin argon (if exists)...
	
	let mut _pin_key = InternalPinKey::wrap (_pin_hash.material);
	
	for _pin_hash in _pin_hashes.into_iter () {
		
		let _pin_salt = blake3_derive_key (
				InternalPinSalt::wrap,
				CRYPTO_PIN_SALT_CONTEXT,
				&[
					_pin_key.access (),
					_ssh_wrap_key.access (),
					_packet_salt.access (),
					_naive_key.access (),
				],
				&[],
				None,
			);
		
		_pin_key.consume ();
		
		let _pin_argon = apply_argon_pin (_pin_hash, _pin_salt) ?;
		
		_pin_key = blake3_derive_key (
				InternalPinKey::wrap,
				CRYPTO_PIN_KEY_CONTEXT,
				&[
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
				_ssh_wrap_key.access (),
				_secret_key.access (),
				_pin_key.access (),
				_packet_salt.access (),
				_naive_key.access (),
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
	
	Ok ((_encryption_key, _authentication_key))
}








fn apply_argon_secret (_secret_hash : InternalSecretHash, _secret_salt : InternalSecretSalt) -> CryptoResult<InternalSecretArgon> {
	
	argon_derive (
			InternalSecretArgon::wrap,
			_secret_hash.access (),
			_secret_salt.access (),
			CRYPTO_SECRET_ARGON_M_COST,
			CRYPTO_SECRET_ARGON_T_COST,
		)
}


fn apply_argon_pin (_pin_hash : InternalPinHash, _pin_salt : InternalPinSalt) -> CryptoResult<InternalPinArgon> {
	
	argon_derive (
			InternalPinArgon::wrap,
			_pin_hash.access (),
			_pin_salt.access (),
			CRYPTO_PIN_ARGON_M_COST,
			CRYPTO_PIN_ARGON_T_COST,
		)
}




fn wrap_secrets_and_pins_inputs <'a> (
			_secret_inputs : &'a [&'a [u8]],
			_pin_inputs : &'a [&'a [u8]],
		) -> CryptoResult<(Vec<InternalSecretInput<'a>>, Vec<InternalPinInput<'a>>)>
{
	debug_assert! (CRYPTO_SECRET_COUNT_MAX <= (u32::MAX as usize), "[424cdca6]");
	debug_assert! (CRYPTO_PIN_COUNT_MAX <= (u32::MAX as usize), "[f1d98265]");
	
	if _secret_inputs.len () > CRYPTO_SECRET_COUNT_MAX {
		fail! (0x6eceb6e4);
	}
	if _pin_inputs.len () > CRYPTO_PIN_COUNT_MAX {
		fail! (0x8b060b37);
	}
	
	let _secret_inputs = Vec::from (_secret_inputs) .into_iter () .map (InternalSecretInput::wrap) .collect ();
	let _pin_inputs = Vec::from (_pin_inputs) .into_iter () .map (InternalPinInput::wrap) .collect ();
	
	Ok ((_secret_inputs, _pin_inputs))
}


fn wrap_ssh_wrappers <'a> (
			_ssh_wrappers : Vec<&'a mut SshWrapper>,
		) -> CryptoResult<Vec<&'a mut SshWrapper>>
{
	debug_assert! (CRYPTO_SSH_WRAPPER_COUNT_MAX <= (u32::MAX as usize), "[8d49c9e0]");
	
	if _ssh_wrappers.len () > CRYPTO_SSH_WRAPPER_COUNT_MAX {
		fail! (0x22fb37e2);
	}
	
	let mut _ssh_wrappers : Vec<_> = _ssh_wrappers.into_iter () .collect ();
	
	_ssh_wrappers.sort_by (|_left, _right| SshWrapper::cmp_by_keys (*_left, *_right));
	_ssh_wrappers.dedup_by (|_left, _right| SshWrapper::cmp_by_keys (*_left, *_right) == Ordering::Equal);
	
	Ok (_ssh_wrappers)
}


