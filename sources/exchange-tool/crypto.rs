

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


const CRYPTO_ENCRYPTED_PADDING : usize = 255;
const CRYPTO_ENCRYPTED_OVERHEAD : usize = CRYPTO_ENCRYPTED_SALT + CRYPTO_ENCRYPTED_MAC;
const CRYPTO_ENCRYPTED_SALT : usize = 32;
const CRYPTO_ENCRYPTED_MAC : usize = 32;








struct InternalDheKey ([u8; 32]);
struct InternalNaiveKey ([u8; 32]);
struct InternalAontKey ([u8; 32]);

struct InternalPacketSalt ([u8; 32]);
struct InternalPacketKey ([u8; 32]);

struct InternalEncryptionKey ([u8; 32]);

struct InternalAuthenticationKey ([u8; 32]);
struct InternalAuthenticationMac ([u8; 32]);

struct InternalSecretInput <'a> (&'a [u8]);
struct InternalSecretHash ([u8; 32]);
struct InternalSecretSalt ([u8; 32]);
struct InternalSecretArgon ([u8; 32]);
struct InternalSecretKey ([u8; 32]);

struct InternalPinInput <'a> (&'a [u8]);
struct InternalPinHash ([u8; 32]);
struct InternalPinSalt ([u8; 32]);
struct InternalPinArgon ([u8; 32]);
struct InternalPinKey ([u8; 32]);

struct InternalSshWrapInput ([u8; 32]);
struct InternalSshWrapOutput ([u8; 32]);

struct InternalDataDecrypted <'a> (&'a [u8]);
struct InternalDataEncrypted <'a> (&'a [u8]);




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
	let _secret_input = _secret_input.map (InternalSecretInput);
	let _secret_input = _secret_input.as_ref ();
	
	let _pin_input = _pin_input.map (InternalPinInput);
	let _pin_input = _pin_input.as_ref ();
	
	let _decrypted = InternalDataDecrypted (_decrypted);
	let _decrypted_len = _decrypted.0.len ();
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x83d6c657);
	}
	
	// NOTE:  compressing...
	
	let _compress_capacity = compress_capacity_max (_decrypted_len) .else_wrap (0x4198ca8b) ?;
	let _compress_capacity = _compress_capacity + 4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD;
	
	let mut _intermediate_buffer = Vec::with_capacity (_compress_capacity);
	compress (&_decrypted.0, &mut _intermediate_buffer) .else_wrap (0xa9fadcdc) ?;
	
	if _intermediate_buffer.len () >= _decrypted_len {
		
		_intermediate_buffer.clear ();
		_intermediate_buffer.extend_from_slice (&_decrypted.0);
	}
	
	// NOTE:  padding...
	
	encode_u32_push (_decrypted_len as u32, &mut _intermediate_buffer);
	
	padding_push (CRYPTO_ENCRYPTED_PADDING, &mut _intermediate_buffer);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  deriving keys...
	
	let _sender = _sender.map (|_key| &_key.0.0);
	let _recipient = _recipient.map (|_key| &_key.0.0);
	
	let (_naive_key, _aont_key, (_secret_hash, _secret_exists), (_pin_hash, _pin_exists))
			= derive_keys_phase_1 (_sender, _recipient, _secret_input, _pin_input, true) ?;
	
	let _sender = ();
	let _recipient = ();
	
	// NOTE:  salting...
	
	let mut _packet_salt = generate_random (InternalPacketSalt);
	
	let (_encryption_key, _authentication_key)
			= derive_keys_phase_2 (&_naive_key, &_packet_salt, (&_secret_hash, _secret_exists), (&_pin_hash, _pin_exists), _ssh_wrapper) ?;
	
	// NOTE:  encryption...
	
	apply_encryption (&_encryption_key, &mut _intermediate_buffer) ?;
	
	let _encryption_key = ();
	
	// NOTE:  authentication...
	
	let _mac = apply_authentication (&_authentication_key, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (&_mac.0);
	
	let _authentication_key = ();
	let _mac = ();
	
	// NOTE:  all-or-nothing...
	
	apply_all_or_nothing_mangling (&_aont_key, &mut _packet_salt, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (&_packet_salt.0);
	
	let _aont_key = ();
	let _packet_salt = ();
	
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
	let _secret_input = _secret_input.map (InternalSecretInput);
	let _secret_input = _secret_input.as_ref ();
	
	let _pin_input = _pin_input.map (InternalPinInput);
	let _pin_input = _pin_input.as_ref ();
	
	let _encrypted = InternalDataEncrypted (_encrypted);
	let _encrypted_len = _encrypted.0.len ();
	
	if _encrypted_len > CRYPTO_ENCRYPTED_SIZE_MAX {
		fail! (0x5832104d);
	}
	
	// NOTE:  decoding...
	
	let _decode_capacity = decode_capacity_max (_encrypted_len) .else_wrap (0xae545303) ?;
	
	let mut _intermediate_buffer = Vec::with_capacity (_decode_capacity);
	decode (&_encrypted.0, &mut _intermediate_buffer) .else_wrap (0x10ff413a) ?;
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  deriving keys...
	
	let _sender = _sender.map (|_key| &_key.0.0);
	let _recipient = _recipient.map (|_key| &_key.0.0);
	
	let (_naive_key, _aont_key, (_secret_hash, _secret_exists), (_pin_hash, _pin_exists))
			= derive_keys_phase_1 (_recipient, _sender, _secret_input, _pin_input, false) ?;
	
	let _sender = ();
	let _recipient = ();
	
	// NOTE:  all-or-nothing and salting...
	
	let _packet_salt = bytes_pop::<CRYPTO_ENCRYPTED_SALT> (&mut _intermediate_buffer) .else_wrap (0x78ed3811) ?;
	let mut _packet_salt = InternalPacketSalt (_packet_salt);
	
	apply_all_or_nothing_mangling (&_aont_key, &mut _packet_salt, &_intermediate_buffer) ?;
	
	let _aont_key = ();
	
	// NOTE:  deriving keys...
	
	let (_encryption_key, _authentication_key)
			= derive_keys_phase_2 (&_naive_key, &_packet_salt, (&_secret_hash, _secret_exists), (&_pin_hash, _pin_exists), _ssh_wrapper) ?;
	
	// NOTE:  authentication...
	
	let _mac_expected = bytes_pop::<CRYPTO_ENCRYPTED_MAC> (&mut _intermediate_buffer) .else_wrap (0x88084589) ?;
	let _mac_expected = InternalAuthenticationMac (_mac_expected);
	
	let _mac_actual = apply_authentication (&_authentication_key, &_intermediate_buffer) ?;
	
	if ! ::constant_time_eq::constant_time_eq (&_mac_actual.0, &_mac_expected.0) {
		fail! (0xad70c84c);
	}
	
	let _authentication_key = ();
	let _mac_expected = ();
	let _mac_actual = ();
	
	// NOTE:  decryption...
	
	apply_encryption (&_encryption_key, &mut _intermediate_buffer) ?;
	
	let _encryption_key = ();
	
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








fn apply_encryption (_key : &InternalEncryptionKey, _data : &mut [u8]) -> CryptoResult {
	
	use ::chacha20::cipher::KeyIvInit as _;
	use ::chacha20::cipher::StreamCipher as _;
	
	let _nonce = [0u8; 12];
	
	let _key = ::chacha20::Key::from_slice (&_key.0);
	let _nonce = ::chacha20::Nonce::from (_nonce);
	
	let mut _cipher = ::chacha20::ChaCha20::new (&_key, &_nonce);
	
	_cipher.try_apply_keystream (_data) .else_wrap (0x9c94d0d5) ?;
	
	Ok (())
}




fn apply_authentication (_key : &InternalAuthenticationKey, _data : &[u8]) -> CryptoResult<InternalAuthenticationMac> {
	
	let _mac = blake3_keyed_hash (
			InternalAuthenticationMac,
			&_key.0,
			&[],
			&[
				_data,
			]);
	
	Ok (_mac)
}




fn apply_all_or_nothing_mangling (_key : &InternalAontKey, _packet_salt : &mut InternalPacketSalt, _data : &[u8]) -> CryptoResult {
	
	const _SIZE : usize = mem::size_of::<InternalPacketSalt> ();
	
	let _hash : [u8; _SIZE] = blake3_keyed_hash (
			|_hash| _hash,
			&_key.0,
			&[],
			&[
				_data,
			],
		);
	
	for _index in 0 .. _SIZE {
		_packet_salt.0[_index] ^= _hash[_index];
	}
	
	Ok (())
}








fn derive_keys_phase_1 (
			_private : Option<&x25519::StaticSecret>,
			_public : Option<&x25519::PublicKey>,
			_secret_input : Option<&InternalSecretInput>,
			_pin_input : Option<&InternalPinInput>,
			_encryption : bool,
		) -> CryptoResult<(InternalNaiveKey, InternalAontKey, (InternalSecretHash, bool), (InternalPinHash, bool))>
{
	// --------------------------------------------------------------------------------
	// NOTE:  apply X25519 DHE...
	
	let _private = _private.else_wrap (0x70f91100) ?;
	
	let _dhe_key = x25519_dhe (
			InternalDheKey,
			CRYPTO_DHE_KEY_CONTEXT,
			_private,
			_public,
			_encryption,
		) ?;
	
	let _private = ();
	let _public = ();
	let _encryption = ();
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive secret hash (if exists)...
	
	let _secret_input = _secret_input.map (|_secret_input| _secret_input.0) .unwrap_or (&[]);
	let _secret_exists = ! _secret_input.is_empty ();
	
	let _secret_hash = blake3_derive_key (
			InternalSecretHash,
			CRYPTO_SECRET_HASH_CONTEXT,
			&[],
			&[
				_secret_input,
			]);
	
	let _secret_input = ();
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive pin hash (if exists)...
	
	let _pin_input = _pin_input.map (|_pin_input| _pin_input.0) .unwrap_or (&[]);
	let _pin_exists = ! _pin_input.is_empty ();
	
	let _pin_hash = blake3_derive_key (
			InternalPinHash,
			CRYPTO_PIN_HASH_CONTEXT,
			&[],
			&[
				_pin_input,
			]);
	
	let _pin_input = ();
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive naive key (for the entire transaction)...
	
	let _naive_key = blake3_derive_key (
			InternalNaiveKey,
			CRYPTO_NAIVE_KEY_CONTEXT,
			&[
				&_pin_hash.0,
				&_secret_hash.0,
				&_dhe_key.0,
			],
			&[]);
	
	let _pin_key = ();
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive AONT key...
	
	let _aont_key = blake3_derive_key (
			InternalAontKey,
			CRYPTO_AONT_KEY_CONTEXT,
			&[
				&_naive_key.0,
			],
			&[]);
	
	// --------------------------------------------------------------------------------
	
	Ok ((_naive_key, _aont_key, (_secret_hash, _secret_exists), (_pin_hash, _pin_exists)))
}








fn derive_keys_phase_2 (
			_naive_key : &InternalNaiveKey,
			_packet_salt : &InternalPacketSalt,
			_secret_hash : (&InternalSecretHash, bool),
			_pin_hash : (&InternalPinHash, bool),
			_ssh_wrapper : Option<&mut SshWrapper>,
		) -> CryptoResult<(InternalEncryptionKey, InternalAuthenticationKey)>
{
	let (_secret_hash, _secret_exists) = _secret_hash;
	let (_pin_hash, _pin_exists) = _pin_hash;
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive secret argon (if exists)...
	
	let _secret_key = if _secret_exists {
			
			let _secret_salt = blake3_derive_key (
					InternalSecretSalt,
					CRYPTO_SECRET_SALT_CONTEXT,
					&[
						&_pin_hash.0,
						&_packet_salt.0,
						&_naive_key.0,
					],
					&[]);
			
			let _secret_argon = apply_argon_secret (_secret_hash, &_secret_salt) ?;
			
			let _secret_key = blake3_derive_key (
					InternalSecretKey,
					CRYPTO_SECRET_KEY_CONTEXT,
					&[
						&_secret_argon.0,
					],
					&[]);
			
			_secret_key
			
		} else {
			InternalSecretKey (_secret_hash.0)
		};
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive pin argon (if exists)...
	
	let _pin_key = if _pin_exists {
			
			let _pin_salt = blake3_derive_key (
					InternalPinSalt,
					CRYPTO_PIN_SALT_CONTEXT,
					&[
						&_secret_hash.0,
						&_packet_salt.0,
						&_naive_key.0,
					],
					&[]);
			
			let _pin_argon = apply_argon_pin (_pin_hash, &_pin_salt) ?;
			
			let _pin_key = blake3_derive_key (
					InternalPinKey,
					CRYPTO_PIN_KEY_CONTEXT,
					&[
						&_pin_argon.0,
					],
					&[]);
			
			_pin_key
			
		} else {
			InternalPinKey (_pin_hash.0)
		};
	
	// --------------------------------------------------------------------------------
	// NOTE:  call SSH wrapper (if exists)...
	
	let _ssh_wrap_key = if let Some (_ssh_wrapper) = _ssh_wrapper {
			
			let _ssh_wrap_input = blake3_derive_key (
					InternalSshWrapInput,
					CRYPTO_SSH_WRAP_INPUT_CONTEXT,
					&[
						&_naive_key.0,
						&_packet_salt.0,
					],
					&[]);
			
			let mut _ssh_wrap_output = [0u8; 32];
			_ssh_wrapper.wrap (&_ssh_wrap_input.0, &mut _ssh_wrap_output) .else_wrap (0xcc07e95e) ?;
			
			let _ssh_wrap_output = blake3_derive_key (
					InternalSshWrapOutput,
					CRYPTO_SSH_WRAP_OUTPUT_CONTEXT,
					&[
						&_naive_key.0,
						&_packet_salt.0,
						&_ssh_wrap_output,
					],
					&[]);
			
			_ssh_wrap_output
			
		} else {
			InternalSshWrapOutput ([0u8; 32])
		};
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive wrapping key...
	
	let _wrapping_key = blake3_derive_key (
			InternalPacketKey,
			CRYPTO_PACKET_KEY_CONTEXT,
			&[
				&_naive_key.0,
				&_secret_key.0,
				&_pin_key.0,
				&_ssh_wrap_key.0,
				&_packet_salt.0,
			],
			&[]);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive encryption key...
	
	let _encryption_key = blake3_derive_key (
			InternalEncryptionKey,
			CRYPTO_ENCRYPTION_KEY_CONTEXT,
			&[
				&_wrapping_key.0,
			],
			&[]);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive authentication key...
	
	let _authentication_key = blake3_derive_key (
			InternalAuthenticationKey,
			CRYPTO_AUTHENTICATION_KEY_CONTEXT,
			&[
				&_wrapping_key.0,
			],
			&[]);
	
	// --------------------------------------------------------------------------------
	
	Ok ((_encryption_key, _authentication_key))
}








fn apply_argon_secret (_secret_hash : &InternalSecretHash, _secret_salt : &InternalSecretSalt) -> CryptoResult<InternalSecretArgon> {
	
	argon_derive (
			InternalSecretArgon,
			&_secret_hash.0,
			&_secret_salt.0,
			CRYPTO_SECRET_ARGON_M_COST,
			CRYPTO_SECRET_ARGON_T_COST,
		)
}


fn apply_argon_pin (_pin_hash : &InternalPinHash, _pin_salt : &InternalPinSalt) -> CryptoResult<InternalPinArgon> {
	
	argon_derive (
			InternalPinArgon,
			&_pin_hash.0,
			&_pin_salt.0,
			CRYPTO_PIN_ARGON_M_COST,
			CRYPTO_PIN_ARGON_T_COST,
		)
}


