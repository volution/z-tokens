

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use crate::keys::*;
use crate::coding::*;
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
const CRYPTO_ENCRYPTED_SALT : usize = 16;
const CRYPTO_ENCRYPTED_MAC : usize = 16;


define_cryptographic_context! (CRYPTO_ENCRYPTION_KEY_CONTEXT, encryption, encryption_key);
define_cryptographic_context! (CRYPTO_AUTHENTICATION_KEY_CONTEXT, encryption, authentication_key);
define_cryptographic_context! (CRYPTO_SHARED_KEY_CONTEXT, encryption, shared_key);
define_cryptographic_context! (CRYPTO_BASE_KEY_CONTEXT, encryption, base_key);
define_cryptographic_context! (CRYPTO_AONT_KEY_CONTEXT, encryption, aont_key);
define_cryptographic_context! (CRYPTO_SECRET_SALT_CONTEXT, encryption, secret_salt);
define_cryptographic_context! (CRYPTO_SECRET_KEY_CONTEXT, encryption, secret_key);
define_cryptographic_context! (CRYPTO_PIN_SALT_CONTEXT, encryption, pin_salt);
define_cryptographic_context! (CRYPTO_PIN_KEY_CONTEXT, encryption, pin_key);
define_cryptographic_context! (CRYPTO_SSH_WRAP_INPUT_CONTEXT, encryption, ssh_wrap_input);
define_cryptographic_context! (CRYPTO_SSH_WRAP_OUTPUT_CONTEXT, encryption, ssh_wrap_output);


const CRYPTO_SECRET_ARGON_ALGORITHM : ::argon2::Algorithm = ::argon2::Algorithm::Argon2id;
const CRYPTO_SECRET_ARGON_VERSION : ::argon2::Version = ::argon2::Version::V0x13;

const CRYPTO_SECRET_ARGON_M_COST : u32 = 512 * 1024;
const CRYPTO_SECRET_ARGON_T_COST : u32 = 8;
const CRYPTO_SECRET_ARGON_P_COST : u32 = 1;


const CRYPTO_PIN_ARGON_ALGORITHM : ::argon2::Algorithm = ::argon2::Algorithm::Argon2id;
const CRYPTO_PIN_ARGON_VERSION : ::argon2::Version = ::argon2::Version::V0x13;

const CRYPTO_PIN_ARGON_M_COST : u32 = 128 * 1024;
const CRYPTO_PIN_ARGON_T_COST : u32 = 8;
const CRYPTO_PIN_ARGON_P_COST : u32 = 1;








pub fn encrypt (
			_sender : Option<&SenderPrivateKey>,
			_recipient : Option<&RecipientPublicKey>,
			_secret : Option<&[u8]>,
			_pin : Option<&[u8]>,
			_decrypted : &[u8],
			_encrypted : &mut Vec<u8>,
			_ssh_wrapper : Option<&mut SshWrapper>,
		) -> CryptoResult
{
	let _decrypted_len = _decrypted.len ();
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x83d6c657);
	}
	
	// NOTE:  compressing...
	
	let _compress_capacity = compress_capacity_max (_decrypted_len) .else_wrap (0x4198ca8b) ?;
	let _compress_capacity = _compress_capacity + 4 + CRYPTO_ENCRYPTED_PADDING + CRYPTO_ENCRYPTED_OVERHEAD;
	
	let mut _intermediate_buffer = Vec::with_capacity (_compress_capacity);
	compress (_decrypted, &mut _intermediate_buffer) .else_wrap (0xa9fadcdc) ?;
	
	if _intermediate_buffer.len () >= _decrypted_len {
		
		_intermediate_buffer.clear ();
		_intermediate_buffer.extend_from_slice (_decrypted);
	}
	
	// NOTE:  wrapping...
	
	encode_u32_push (_decrypted_len as u32, &mut _intermediate_buffer);
	
	padding_push (CRYPTO_ENCRYPTED_PADDING, &mut _intermediate_buffer);
	
	// NOTE:  deriving keys...
	
	let _sender = _sender.map (|_key| &_key.0.0);
	let _recipient = _recipient.map (|_key| &_key.0.0);
	
	let (_base_key, _aont_key) = derive_keys_phase_1 (_sender, _recipient, _secret, _pin, true) ?;
	
	let mut _salt = generate_salt () ?;
	
	let (_encryption_key, _authentication_key) = derive_keys_phase_2 (&_base_key, &_salt, _ssh_wrapper) ?;
	
	// NOTE:  authenticated encryption...
	
	apply_encryption (&_encryption_key, &mut _intermediate_buffer) ?;
	
	let _mac = apply_authentication (&_authentication_key, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (&_mac);
	
	// NOTE:  all-or-nothing...
	
	apply_all_or_nothing_mangling (&_aont_key, &mut _salt, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (&_salt);
	
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
			_secret : Option<&[u8]>,
			_pin : Option<&[u8]>,
			_encrypted : &[u8],
			_decrypted : &mut Vec<u8>,
			_ssh_wrapper : Option<&mut SshWrapper>,
		) -> CryptoResult
{
	let _encrypted_len = _encrypted.len ();
	
	if _encrypted_len > CRYPTO_ENCRYPTED_SIZE_MAX {
		fail! (0x5832104d);
	}
	
	// NOTE:  decoding...
	
	let _decode_capacity = decode_capacity_max (_encrypted_len) .else_wrap (0xae545303) ?;
	
	let mut _intermediate_buffer = Vec::with_capacity (_decode_capacity);
	decode (_encrypted, &mut _intermediate_buffer) .else_wrap (0x10ff413a) ?;
	
	// NOTE:  deriving keys...
	
	let _sender = _sender.map (|_key| &_key.0.0);
	let _recipient = _recipient.map (|_key| &_key.0.0);
	
	let (_base_key, _aont_key) = derive_keys_phase_1 (_recipient, _sender, _secret, _pin, false) ?;
	
	// NOTE:  all-or-nothing...
	
	let mut _salt = bytes_pop::<CRYPTO_ENCRYPTED_SALT> (&mut _intermediate_buffer) .else_wrap (0x78ed3811) ?;
	
	apply_all_or_nothing_mangling (&_aont_key, &mut _salt, &_intermediate_buffer) ?;
	
	// NOTE:  deriving keys...
	
	let (_encryption_key, _authentication_key) = derive_keys_phase_2 (&_base_key, &_salt, _ssh_wrapper) ?;
	
	// NOTE:  authenticated encryption...
	
	let _mac_expected = bytes_pop::<CRYPTO_ENCRYPTED_MAC> (&mut _intermediate_buffer) .else_wrap (0x88084589) ?;
	
	let _mac_actual = apply_authentication (&_authentication_key, &_intermediate_buffer) ?;
	
	if ! ::constant_time_eq::constant_time_eq (&_mac_actual, &_mac_expected) {
		fail! (0xad70c84c);
	}
	
	apply_encryption (&_encryption_key, &mut _intermediate_buffer) ?;
	
	// NOTE:  unwrapping...
	
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








fn apply_encryption (_key : &[u8; 32], _data : &mut [u8]) -> CryptoResult {
	
	use ::chacha20::cipher::KeyIvInit as _;
	use ::chacha20::cipher::StreamCipher as _;
	
	let _nonce = [0u8; 12];
	
	let _key = ::chacha20::Key::from_slice (_key);
	let _nonce = ::chacha20::Nonce::from (_nonce);
	
	let mut _cipher = ::chacha20::ChaCha20::new (&_key, &_nonce);
	
	_cipher.try_apply_keystream (_data) .else_wrap (0x9c94d0d5) ?;
	
	Ok (())
}




fn apply_authentication (_key : &[u8; 32], _data : &[u8]) -> CryptoResult<[u8; CRYPTO_ENCRYPTED_MAC]> {
	
	let _hash =
			::blake3::Hasher::new_keyed (_key)
			.update (_data)
			.finalize ();
	
	let mut _mac = [0u8; CRYPTO_ENCRYPTED_MAC];
	_mac.copy_from_slice (& _hash.as_bytes () [.. CRYPTO_ENCRYPTED_MAC]);
	
	Ok (_mac)
}




fn derive_keys_phase_1 (
			_private : Option<&x25519::StaticSecret>,
			_public : Option<&x25519::PublicKey>,
			_secret : Option<&[u8]>,
			_pin : Option<&[u8]>,
			_encryption : bool,
		) -> CryptoResult<([u8; 32], [u8; 32])>
{
	let _private = _private.else_wrap (0x70f91100) ?;
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
	
	let _shared_key : [u8; 32] =
			::blake3::Hasher::new_derive_key (CRYPTO_SHARED_KEY_CONTEXT)
			.update (_dhe)
			.update (_sender_public)
			.update (_receiver_public)
			.finalize ()
			.into ();
	
	let _secret_salt : [u8; 32] =
			::blake3::Hasher::new_derive_key (CRYPTO_SECRET_SALT_CONTEXT)
			.update (&_shared_key)
			.finalize ()
			.into ();
	
	let _pin_salt : [u8; 32] =
			::blake3::Hasher::new_derive_key (CRYPTO_PIN_SALT_CONTEXT)
			.update (&_shared_key)
			.finalize ()
			.into ();
	
	let _secret = apply_argon_secret (_secret.map (|_secret| (_secret, &_secret_salt))) ?;
	
	let _pin = apply_argon_pin (_pin.map (|_pin| (_pin, &_pin_salt))) ?;
	
	let _secret : [u8; 32] =
			::blake3::Hasher::new_derive_key (CRYPTO_SECRET_KEY_CONTEXT)
			.update (&_secret)
			.finalize ()
			.into ();
	
	let _pin : [u8; 32] =
			::blake3::Hasher::new_derive_key (CRYPTO_PIN_KEY_CONTEXT)
			.update (&_pin)
			.finalize ()
			.into ();
	
	let _base_key : [u8; 32] =
			::blake3::Hasher::new_derive_key (CRYPTO_BASE_KEY_CONTEXT)
			.update (&_pin)
			.update (&_secret)
			.update (&_shared_key)
			.finalize ()
			.into ();
	
	let _aont_key : [u8; 32] =
			::blake3::Hasher::new_derive_key (CRYPTO_AONT_KEY_CONTEXT)
			.update (&_base_key)
			.finalize ()
			.into ();
	
	Ok ((_base_key, _aont_key))
}




fn derive_keys_phase_2 (
			_base_key : &[u8; 32],
			_salt : &[u8; CRYPTO_ENCRYPTED_SALT],
			_ssh_wrapper : Option<&mut SshWrapper>,
		) -> CryptoResult<([u8; 32], [u8; 32])>
{
	let _wrapped_key = if let Some (_ssh_wrapper) = _ssh_wrapper {
		
		let _wrap_input : [u8; 32] =
				::blake3::Hasher::new_derive_key (CRYPTO_SSH_WRAP_INPUT_CONTEXT)
				.update (_base_key)
				.update (_salt)
				.finalize ()
				.into ();
		
		let mut _wrap_output = [0u8; 32];
		_ssh_wrapper.wrap (&_wrap_input, &mut _wrap_output) .else_wrap (0xcc07e95e) ?;
		
		let _wrap_output : [u8; 32] =
				::blake3::Hasher::new_derive_key (CRYPTO_SSH_WRAP_OUTPUT_CONTEXT)
				.update (_base_key)
				.update (_salt)
				.update (&_wrap_output)
				.finalize ()
				.into ();
		
		Some (_wrap_output)
	} else {
		None
	};
	
	let _wrapped_key = _wrapped_key.as_ref () .unwrap_or (_base_key);
	
	let _encryption_key : [u8; 32] =
			::blake3::Hasher::new_derive_key (CRYPTO_ENCRYPTION_KEY_CONTEXT)
			.update (_wrapped_key)
			.update (_salt)
			.finalize ()
			.into ();
	
	let _authentication_key : [u8; 32] =
			::blake3::Hasher::new_derive_key (CRYPTO_AUTHENTICATION_KEY_CONTEXT)
			.update (_wrapped_key)
			.update (_salt)
			.finalize ()
			.into ();
	
	Ok ((_encryption_key, _authentication_key))
}




fn apply_all_or_nothing_mangling (_key : &[u8; 32], _salt : &mut [u8; CRYPTO_ENCRYPTED_SALT], _data : &[u8]) -> CryptoResult {
	
	let _hash =
			::blake3::Hasher::new_keyed (_key)
			.update (_data)
			.finalize ();
	
	let _hash = _hash.as_bytes ();
	
	for _index in 0 .. CRYPTO_ENCRYPTED_SALT {
		_salt[_index] ^= _hash[_index];
	}
	
	Ok (())
}




fn apply_argon_secret (_secret_and_salt : Option<(&[u8], &[u8; 32])>) -> CryptoResult<[u8; 32]> {
	
	let mut _output = [0u8; 32];
	
	let Some ((_secret, _salt)) = _secret_and_salt
		else {
			return Ok (_output);
		};
	
	if _secret.is_empty () {
		return Ok (_output);
	}
	
	let _parameters = ::argon2::Params::new (
				CRYPTO_SECRET_ARGON_M_COST,
				CRYPTO_SECRET_ARGON_T_COST,
				CRYPTO_SECRET_ARGON_P_COST,
				Some (_output.len ()),
			) .else_wrap (0xf2eebb0c) ?;
	
	let _hasher = ::argon2::Argon2::new (
				CRYPTO_SECRET_ARGON_ALGORITHM,
				CRYPTO_SECRET_ARGON_VERSION,
				_parameters,
			);
	
	_hasher.hash_password_into (_secret, _salt, &mut _output) .else_wrap (0xacae7396) ?;
	
	Ok (_output)
}


fn apply_argon_pin (_pin_and_salt : Option<(&[u8], &[u8; 32])>) -> CryptoResult<[u8; 32]> {
	
	let mut _output = [0u8; 32];
	
	let Some ((_pin, _salt)) = _pin_and_salt
		else {
			return Ok (_output);
		};
	
	if _pin.is_empty () {
		return Ok (_output);
	}
	
	let _parameters = ::argon2::Params::new (
				CRYPTO_PIN_ARGON_M_COST,
				CRYPTO_PIN_ARGON_T_COST,
				CRYPTO_PIN_ARGON_P_COST,
				Some (_output.len ()),
			) .else_wrap (0x23aba478) ?;
	
	let _hasher = ::argon2::Argon2::new (
				CRYPTO_PIN_ARGON_ALGORITHM,
				CRYPTO_PIN_ARGON_VERSION,
				_parameters,
			);
	
	_hasher.hash_password_into (_pin, _salt, &mut _output) .else_wrap (0x23a4154f) ?;
	
	Ok (_output)
}




fn generate_salt () -> CryptoResult<[u8; CRYPTO_ENCRYPTED_SALT]> {
	use ::rand::RngCore as _;
	let mut _salt = [0u8; CRYPTO_ENCRYPTED_SALT];
	::rand::rngs::OsRng.fill_bytes (&mut _salt);
	Ok (_salt)
}


