

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


const CRYPTO_ENCRYPTED_PADDING : usize = 256;
const CRYPTO_ENCRYPTED_OVERHEAD : usize = CRYPTO_ENCRYPTED_SALT + CRYPTO_ENCRYPTED_MAC;
const CRYPTO_ENCRYPTED_SALT : usize = InternalPacketSalt::SIZE;
const CRYPTO_ENCRYPTED_MAC : usize = InternalAuthenticationMac::SIZE;








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




const CRYPTO_SECRET_ARGON_M_COST : u32 = 512 * 1024;
const CRYPTO_SECRET_ARGON_T_COST : u32 = 8;

const CRYPTO_PIN_ARGON_M_COST : u32 = 128 * 1024;
const CRYPTO_PIN_ARGON_T_COST : u32 = 8;








pub fn encrypt (
			_sender : Option<&SenderPrivateKey>,
			_recipient : Option<&RecipientPublicKey>,
			_secret_input : Option<&[u8]>,
			_pin_input : Option<&[u8]>,
			_decrypted : &[u8],
			_encrypted : &mut Vec<u8>,
			_ssh_wrapper : Option<&mut SshWrapper>,
		) -> CryptoResult
{
	let _secret_input = _secret_input.map (InternalSecretInput::wrap);
	let _pin_input = _pin_input.map (InternalPinInput::wrap);
	
	let _decrypted = InternalDataDecrypted::wrap (_decrypted);
	let _decrypted_len = _decrypted.size ();
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x83d6c657);
	}
	
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
	
	let (_naive_key, _aont_key, (_secret_hash, _secret_exists), (_pin_hash, _pin_exists))
			= derive_keys_phase_1 (_sender, _recipient, _secret_input, _pin_input, true) ?;
	
	// NOTE:  salting...
	
	let mut _packet_salt = generate_random (InternalPacketSalt::wrap);
	
	let (_encryption_key, _authentication_key)
			= derive_keys_phase_2 (_naive_key, &_packet_salt, (_secret_hash, _secret_exists), (_pin_hash, _pin_exists), _ssh_wrapper) ?;
	
	// NOTE:  encryption...
	
	apply_encryption (_encryption_key, &mut _intermediate_buffer) ?;
	
	// NOTE:  authentication...
	
	let _mac = apply_authentication (_authentication_key, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (_mac.access ());
	
	// NOTE:  all-or-nothing...
	
	apply_all_or_nothing_mangling (_aont_key, &mut _packet_salt, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (_packet_salt.access ());
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  encoding...
	
	assert! (_intermediate_buffer.len () <= (_decrypted_len + 4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD), "[0e17b154]");
	
	let _encode_capacity = encode_capacity_max (_intermediate_buffer.len ()) .else_wrap (0x7f15a8ec) ?;
	
	let mut _encode_buffer = Vec::with_capacity (_encode_capacity);
	encode (&_intermediate_buffer, &mut _encode_buffer) .else_wrap (0x5bc239f9) ?;
	
	assert! (_encode_buffer.len () <= CRYPTO_ENCRYPTED_SIZE_MAX, "[bb3c2546]  {} <= {}", _encode_buffer.len (), CRYPTO_ENCRYPTED_SIZE_MAX);
	
	// NOTE:  finalizing...
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_encrypted.extend_from_slice (&_encode_buffer);
	
	Ok (())
}




pub fn decrypt (
			_recipient : Option<&RecipientPrivateKey>,
			_sender : Option<&SenderPublicKey>,
			_secret_input : Option<&[u8]>,
			_pin_input : Option<&[u8]>,
			_encrypted : &[u8],
			_decrypted : &mut Vec<u8>,
			_ssh_wrapper : Option<&mut SshWrapper>,
		) -> CryptoResult
{
	let _secret_input = _secret_input.map (InternalSecretInput::wrap);
	let _pin_input = _pin_input.map (InternalPinInput::wrap);
	
	let _encrypted = InternalDataEncrypted::wrap (_encrypted);
	let _encrypted_len = _encrypted.size ();
	
	if _encrypted_len > CRYPTO_ENCRYPTED_SIZE_MAX {
		fail! (0x5832104d);
	}
	
	// NOTE:  decoding...
	
	let _decode_capacity = decode_capacity_max (_encrypted_len) .else_wrap (0xae545303) ?;
	
	let mut _intermediate_buffer = Vec::with_capacity (_decode_capacity);
	decode (_encrypted.access (), &mut _intermediate_buffer) .else_wrap (0x10ff413a) ?;
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  deriving keys...
	
	let _sender = _sender.map (SenderPublicKey::access);
	let _recipient = _recipient.map (RecipientPrivateKey::access);
	
	let (_naive_key, _aont_key, (_secret_hash, _secret_exists), (_pin_hash, _pin_exists))
			= derive_keys_phase_1 (_recipient, _sender, _secret_input, _pin_input, false) ?;
	
	// NOTE:  all-or-nothing and salting...
	
	let _packet_salt = bytes_pop::<CRYPTO_ENCRYPTED_SALT> (&mut _intermediate_buffer) .else_wrap (0x78ed3811) ?;
	let mut _packet_salt = InternalPacketSalt::wrap (_packet_salt);
	
	apply_all_or_nothing_mangling (_aont_key, &mut _packet_salt, &_intermediate_buffer) ?;
	
	// NOTE:  deriving keys...
	
	let (_encryption_key, _authentication_key)
			= derive_keys_phase_2 (_naive_key, &_packet_salt, (_secret_hash, _secret_exists), (_pin_hash, _pin_exists), _ssh_wrapper) ?;
	
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
	
	// NOTE:  finalizing...
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_decrypted.extend_from_slice (&_decompress_buffer);
	
	Ok (())
}








fn apply_encryption (_key : InternalEncryptionKey, _data : &mut [u8]) -> CryptoResult {
	
	use ::chacha20::cipher::KeyIvInit as _;
	use ::chacha20::cipher::StreamCipher as _;
	
	let _nonce = [0u8; 12];
	
	let _key = ::chacha20::Key::from_slice (_key.access ());
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
			]);
	
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
			_secret_input : Option<InternalSecretInput>,
			_pin_input : Option<InternalPinInput>,
			_encryption : bool,
		) -> CryptoResult<(InternalNaiveKey, InternalAontKey, (InternalSecretHash, bool), (InternalPinHash, bool))>
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
	// NOTE:  derive secret hash (if exists)...
	
	let _secret_input = _secret_input.unwrap_or_else (InternalSecretInput::empty);
	let _secret_exists = ! _secret_input.is_empty ();
	
	let _secret_hash = blake3_derive_key (
			InternalSecretHash::wrap,
			CRYPTO_SECRET_HASH_CONTEXT,
			&[],
			&[
				_secret_input.access_consume (),
			]);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive pin hash (if exists)...
	
	let _pin_input = _pin_input.unwrap_or_else (InternalPinInput::empty);
	let _pin_exists = ! _pin_input.is_empty ();
	
	let _pin_hash = blake3_derive_key (
			InternalPinHash::wrap,
			CRYPTO_PIN_HASH_CONTEXT,
			&[],
			&[
				_pin_input.access_consume (),
			]);
	
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
			&[]);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive AONT key...
	
	let _aont_key = blake3_derive_key (
			InternalAontKey::wrap,
			CRYPTO_AONT_KEY_CONTEXT,
			&[
				_naive_key.access (),
			],
			&[]);
	
	// --------------------------------------------------------------------------------
	
	Ok ((_naive_key, _aont_key, (_secret_hash, _secret_exists), (_pin_hash, _pin_exists)))
}








fn derive_keys_phase_2 (
			_naive_key : InternalNaiveKey,
			_packet_salt : &InternalPacketSalt,
			_secret_hash : (InternalSecretHash, bool),
			_pin_hash : (InternalPinHash, bool),
			_ssh_wrapper : Option<&mut SshWrapper>,
		) -> CryptoResult<(InternalEncryptionKey, InternalAuthenticationKey)>
{
	let (_secret_hash, _secret_exists) = _secret_hash;
	let (_pin_hash, _pin_exists) = _pin_hash;
	
	// --------------------------------------------------------------------------------
	// NOTE:  call SSH wrapper (if exists)...
	
	let _ssh_wrap_key = if let Some (_ssh_wrapper) = _ssh_wrapper {
			
			let _ssh_wrap_input = blake3_derive_key (
					InternalSshWrapInput::wrap,
					CRYPTO_SSH_WRAP_INPUT_CONTEXT,
					&[
						_packet_salt.access (),
						_naive_key.access (),
					],
					&[]);
			
			// FIXME:  zeroize!
			let mut _ssh_wrap_output = [0u8; 32];
			_ssh_wrapper.wrap (_ssh_wrap_input.access (), &mut _ssh_wrap_output) .else_wrap (0xcc07e95e) ?;
			
			let _ssh_wrap_output = blake3_derive_key (
					InternalSshWrapOutput::wrap,
					CRYPTO_SSH_WRAP_OUTPUT_CONTEXT,
					&[
						_packet_salt.access (),
						_naive_key.access (),
						&_ssh_wrap_output,
					],
					&[]);
			
			_ssh_wrap_output
			
		} else {
			InternalSshWrapOutput::zero ()
		};
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive secret argon (if exists)...
	
	let _secret_key = if _secret_exists {
			
			let _secret_salt = blake3_derive_key (
					InternalSecretSalt::wrap,
					CRYPTO_SECRET_SALT_CONTEXT,
					&[
						_ssh_wrap_key.access (),
						_packet_salt.access (),
						_naive_key.access (),
					],
					&[]);
			
			let _secret_argon = apply_argon_secret (_secret_hash, _secret_salt) ?;
			
			let _secret_key = blake3_derive_key (
					InternalSecretKey::wrap,
					CRYPTO_SECRET_KEY_CONTEXT,
					&[
						_secret_argon.access (),
					],
					&[]);
			
			_secret_key
			
		} else {
			InternalSecretKey::wrap (_secret_hash.material)
		};
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive pin argon (if exists)...
	
	let _pin_key = if _pin_exists {
			
			let _pin_salt = blake3_derive_key (
					InternalPinSalt::wrap,
					CRYPTO_PIN_SALT_CONTEXT,
					&[
						_ssh_wrap_key.access (),
						_packet_salt.access (),
						_naive_key.access (),
					],
					&[]);
			
			let _pin_argon = apply_argon_pin (_pin_hash, _pin_salt) ?;
			
			let _pin_key = blake3_derive_key (
					InternalPinKey::wrap,
					CRYPTO_PIN_KEY_CONTEXT,
					&[
						_pin_argon.access (),
					],
					&[]);
			
			_pin_key
			
		} else {
			InternalPinKey::wrap (_pin_hash.material)
		};
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive wrapping key...
	
	let _wrapping_key = blake3_derive_key (
			InternalPacketKey::wrap,
			CRYPTO_PACKET_KEY_CONTEXT,
			&[
				_ssh_wrap_key.access (),
				_secret_key.access (),
				_pin_key.access (),
				_packet_salt.access (),
				_naive_key.access (),
			],
			&[]);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive encryption key...
	
	let _encryption_key = blake3_derive_key (
			InternalEncryptionKey::wrap,
			CRYPTO_ENCRYPTION_KEY_CONTEXT,
			&[
				_wrapping_key.access (),
			],
			&[]);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive authentication key...
	
	let _authentication_key = blake3_derive_key (
			InternalAuthenticationKey::wrap,
			CRYPTO_AUTHENTICATION_KEY_CONTEXT,
			&[
				_wrapping_key.access (),
			],
			&[]);
	
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


