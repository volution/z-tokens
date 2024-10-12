

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use crate::crypto::*;
use crate::keys::*;
use crate::coding::*;
use crate::oracles::*;


use ::z_tokens_runtime_crypto::{
		
		blake3_hash,
		
		CryptographicMaterial as _,
		CryptographicInput as _,
	};


use ::z_tokens_runtime_crypto::crates::{
		
		x25519,
	};


use ::z_tokens_runtime::{
		
		sensitive::drop,
	};








pub(crate) fn password_backend (
			_private_keys : &[&x25519::StaticSecret],
			_public_keys : &[&x25519::PublicKey],
			_symmetric : &SymmetricParametersRawRefs,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : Vec<&mut dyn Oracle>,
			_send : bool,
		) -> CryptoResult
{
	let _private_keys = _private_keys.to_vec ();
	let _public_keys = _public_keys.to_vec ();
	
	let (_associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs) = wrap_associated_and_secrets_and_pins_inputs (_symmetric.associated, _symmetric.secrets, _symmetric.pins, _symmetric.seeds, _symmetric.ballasts) ?;
	let (_oracles, _oracle_handles) = wrap_oracles_phase_1 (_oracles) ?;
	let (_derivation_loops, _namespace) = (_symmetric.derivation_loops, _symmetric.namespace);
	
	let _password_data = InternalPasswordData::wrap (_password_data);
	let _password_data_len = _password_data.size ();
	
	if _password_data_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0xfa4d9417);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  deriving keys...
	
	let (_partial_key, _aont_key, _secret_data, _pin_data, _seed_data, _ballast_data, _oracle_merge, _oracle_sorter, _derivation_loops)
			= derive_keys_phase_1 (CRYPTO_PASSWORD_SCHEMA_V1, _private_keys, _public_keys, _associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs, _oracle_handles, _derivation_loops, _namespace, _send) ?;
	
	let _oracles = wrap_oracles_phase_2 (_oracles, _oracle_sorter) ?;
	
	drop! (_aont_key);
	
	// NOTE:  salting...
	
	let _packet_salt = derive_packet_salt (CRYPTO_PASSWORD_SALT_PURPOSE, &_partial_key, _password_data.access (), true) ?;
	
	drop! (_password_data);
	
	let (_packet_key, _encryption_key, _authentication_key)
			= derive_keys_phase_2 (CRYPTO_PASSWORD_SCHEMA_V1, _partial_key, &_packet_salt, _secret_data, _pin_data, _seed_data, _ballast_data, (_oracle_merge, _oracles), _derivation_loops) ?;
	
	drop! (_encryption_key, _authentication_key);
	
	let _password_output_0 = blake3_hash (
			InternalPasswordOutput::wrap,
			CRYPTO_PASSWORD_OUTPUT_PURPOSE,
			&[
				_packet_key.access (),
			],
			&[],
		);
	
	drop! (_packet_key);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_password_output.copy_from_slice (_password_output_0.access ());
	
	Ok (())
}








pub fn encrypt_with_raw (
			_senders : &[&SenderPrivateKey],
			_recipients : &[&RecipientPublicKey],
			_symmetric : &SymmetricParametersRawRefs,
			_decrypted : &[u8],
			_encrypted : &mut Vec<u8>,
			_oracles : Vec<&mut dyn Oracle>,
			_packet_salt_deterministic : bool,
		) -> CryptoResult
{
	let (_senders, _recipients) = wrap_senders_and_recipients_inputs (_senders, _recipients) ?;
	let (_associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs) = wrap_associated_and_secrets_and_pins_inputs (_symmetric.associated, _symmetric.secrets, _symmetric.pins, _symmetric.seeds, _symmetric.ballasts) ?;
	let (_oracles, _oracle_handles) = wrap_oracles_phase_1 (_oracles) ?;
	let (_derivation_loops, _namespace) = (_symmetric.derivation_loops, _symmetric.namespace);
	
	let _decrypted = InternalDecryptedData::wrap (_decrypted);
	let _decrypted_len = _decrypted.size ();
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  sanity check...
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x83d6c657);
	}
	
	let _compress_capacity = compress_capacity_max (_decrypted_len) .else_wrap (0x4198ca8b) ?;
	let _compress_capacity = _compress_capacity + CRYPTO_ENCRYPTED_HEADER_SIZE + CRYPTO_ENCRYPTED_PADDING_SIZE + CRYPTO_ENCRYPTED_TRAILER_SIZE;
	
	let mut _intermediate_buffer = Vec::with_capacity (_compress_capacity);
	
	// NOTE:  schema...
	
	encode_u32_push (CRYPTO_SCHEMA_V1_VALUE, &mut _intermediate_buffer);
	
	// NOTE:  length...
	
	encode_u32_push (_decrypted_len as u32, &mut _intermediate_buffer);
	
	// NOTE:  compressing...
	
	compress (_decrypted.access (), &mut _intermediate_buffer) .else_wrap (0xa9fadcdc) ?;
	
	if (_intermediate_buffer.len () + CRYPTO_ENCRYPTED_HEADER_SIZE) >= _decrypted_len {
		
		_intermediate_buffer.truncate (CRYPTO_ENCRYPTED_HEADER_SIZE);
		_intermediate_buffer.extend_from_slice (_decrypted.access ());
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	drop! (_decrypted);
	
	// NOTE:  padding...
	
	padding_push (CRYPTO_ENCRYPTED_HEADER_SIZE, CRYPTO_ENCRYPTED_PADDING_SIZE, &mut _intermediate_buffer);
	
	// NOTE:  deriving keys...
	
	let (_partial_key, _aont_key, _secret_data, _pin_data, _seed_data, _ballast_data, _oracle_merge, _oracle_sorter, _derivation_loops)
			= derive_keys_phase_1 (CRYPTO_ENCRYPTION_SCHEMA_V1, _senders, _recipients, _associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs, _oracle_handles, _derivation_loops, _namespace, true) ?;
	
	let _oracles = wrap_oracles_phase_2 (_oracles, _oracle_sorter) ?;
	
	// NOTE:  salting...
	
	let _packet_salt = derive_packet_salt (CRYPTO_PACKET_SALT_PURPOSE, &_partial_key, &_intermediate_buffer, _packet_salt_deterministic) ?;
	
	let (_packet_key, _encryption_key, _authentication_key)
			= derive_keys_phase_2 (CRYPTO_ENCRYPTION_SCHEMA_V1, _partial_key, &_packet_salt, _secret_data, _pin_data, _seed_data, _ballast_data, (_oracle_merge, _oracles), _derivation_loops) ?;
	
	drop! (_packet_key);
	
	// NOTE:  encrypting...
	
	apply_encryption (_encryption_key, &mut _intermediate_buffer) ?;
	
	// NOTE:  authenticating...
	
	let _mac = apply_authentication (_authentication_key, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (_mac.access ());
	
	drop! (_mac);
	
	// NOTE:  all-or-nothing...
	
	let mut _packet_salt = _packet_salt;
	apply_all_or_nothing_mangling (_aont_key, &mut _packet_salt, &_intermediate_buffer) ?;
	
	_intermediate_buffer.extend_from_slice (_packet_salt.access ());
	
	drop! (_packet_salt);
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  encoding...
	
	assert! (_intermediate_buffer.len () <= (_decrypted_len + CRYPTO_ENCRYPTED_HEADER_SIZE + CRYPTO_ENCRYPTED_PADDING_SIZE + CRYPTO_ENCRYPTED_TRAILER_SIZE), "[0e17b154]");
	
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








pub fn decrypt_with_raw (
			_recipients : &[&RecipientPrivateKey],
			_senders : &[&SenderPublicKey],
			_symmetric : &SymmetricParametersRawRefs,
			_encrypted : &[u8],
			_decrypted : &mut Vec<u8>,
			_oracles : Vec<&mut dyn Oracle>,
		) -> CryptoResult
{
	let (_recipients, _senders) = wrap_recipients_and_senders_inputs (_recipients, _senders) ?;
	let (_associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs) = wrap_associated_and_secrets_and_pins_inputs (_symmetric.associated, _symmetric.secrets, _symmetric.pins, _symmetric.seeds, _symmetric.ballasts) ?;
	let (_oracles, _oracle_handles) = wrap_oracles_phase_1 (_oracles) ?;
	let (_derivation_loops, _namespace) = (_symmetric.derivation_loops, _symmetric.namespace);
	
	let _encrypted = InternalEncryptedData::wrap (_encrypted);
	let _encrypted_len = _encrypted.size ();
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  sanity check...
	
	if _encrypted_len > CRYPTO_ENCRYPTED_SIZE_MAX {
		fail! (0x5832104d);
	}
	
	// NOTE:  decoding...
	
	let _decode_capacity = decode_capacity_max (_encrypted_len) .else_wrap (0xae545303) ?;
	
	let mut _intermediate_buffer = Vec::with_capacity (_decode_capacity);
	decode (_encrypted.access (), &mut _intermediate_buffer) .else_wrap (0x10ff413a) ?;
	
	if _intermediate_buffer.len () < (CRYPTO_ENCRYPTED_HEADER_SIZE + CRYPTO_ENCRYPTED_PADDING_SIZE + CRYPTO_ENCRYPTED_TRAILER_SIZE) {
		fail! (0x355aec97);
	}
	
	// NOTE:  schema...
	
	let _schema_value = decode_u32_slice (&_intermediate_buffer[..CRYPTO_ENCRYPTED_SCHEMA_SIZE]);
	
	if _schema_value != CRYPTO_SCHEMA_V1_VALUE {
		fail! (0xf64b2e28);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	drop! (_encrypted);
	
	// NOTE:  deriving keys...
	
	let (_partial_key, _aont_key, _secret_data, _pin_data, _seed_data, _ballast_data, _oracle_merge, _oracle_sorter, _derivation_loops)
			= derive_keys_phase_1 (CRYPTO_ENCRYPTION_SCHEMA_V1, _recipients, _senders, _associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs, _oracle_handles, _derivation_loops, _namespace, false) ?;
	
	let _oracles = wrap_oracles_phase_2 (_oracles, _oracle_sorter) ?;
	
	// NOTE:  all-or-nothing and salting...
	
	let _packet_salt = bytes_pop::<CRYPTO_ENCRYPTED_SALT_SIZE> (&mut _intermediate_buffer) .else_wrap (0x78ed3811) ?;
	let mut _packet_salt = InternalPacketSalt::wrap (_packet_salt);
	
	apply_all_or_nothing_mangling (_aont_key, &mut _packet_salt, &_intermediate_buffer) ?;
	
	// NOTE:  deriving keys...
	
	let (_packet_key, _encryption_key, _authentication_key)
			= derive_keys_phase_2 (CRYPTO_ENCRYPTION_SCHEMA_V1, _partial_key, &_packet_salt, _secret_data, _pin_data, _seed_data, _ballast_data, (_oracle_merge, _oracles), _derivation_loops) ?;
	
	drop! (_packet_key);
	drop! (_packet_salt);
	
	// NOTE:  authenticating...
	
	let _mac_expected = bytes_pop::<CRYPTO_ENCRYPTED_MAC_SIZE> (&mut _intermediate_buffer) .else_wrap (0x88084589) ?;
	let _mac_expected = InternalAuthenticationMac::wrap (_mac_expected);
	
	let _mac_actual = apply_authentication (_authentication_key, &_intermediate_buffer) ?;
	
	if ! InternalAuthenticationMac::compare_consume (_mac_actual, _mac_expected) {
		fail! (0xad70c84c);
	}
	
	// NOTE:  decrypting...
	
	apply_encryption (_encryption_key, &mut _intermediate_buffer) ?;
	
	// NOTE:  padding...
	
	padding_pop (CRYPTO_ENCRYPTED_HEADER_SIZE, CRYPTO_ENCRYPTED_PADDING_SIZE, &mut _intermediate_buffer) .else_wrap (0xbbdd100e) ?;
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  length...
	
	let _decrypted_len = decode_u32_slice (&_intermediate_buffer[CRYPTO_ENCRYPTED_SCHEMA_SIZE .. CRYPTO_ENCRYPTED_SCHEMA_SIZE + CRYPTO_ENCRYPTED_LENGTH_SIZE]) as usize;
	
	if _decrypted_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0x433f5bb6);
	}
	
	// NOTE:  decompressing...
	
	let mut _decompress_buffer = Vec::with_capacity (_decrypted_len);
	if _decrypted_len > (_intermediate_buffer.len () - CRYPTO_ENCRYPTED_HEADER_SIZE) {
		
		decompress (&_intermediate_buffer[CRYPTO_ENCRYPTED_HEADER_SIZE..], &mut _decompress_buffer) .else_wrap (0xec71bc5c) ?;
		
	} else {
		
		_decompress_buffer.extend_from_slice (&_intermediate_buffer[CRYPTO_ENCRYPTED_HEADER_SIZE..]);
	}
	
	_intermediate_buffer.truncate (CRYPTO_ENCRYPTED_HEADER_SIZE);
	
	if _decompress_buffer.len () != _decrypted_len {
		fail! (0x0610eb74);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  length...
	
	if _decrypted_len != decode_u32_pop (&mut _intermediate_buffer) .else_wrap (0x9d52a31c) ? as usize {
		panic! (unreachable, 0x01c6b8ec);
	}
	
	// NOTE:  schema...
	
	if _schema_value != decode_u32_pop (&mut _intermediate_buffer) .else_wrap (0x1555b2c2) ? {
		panic! (unreachable, 0x210d372d);
	}
	
	// NOTE:  finalizing...
	
	if ! _intermediate_buffer.is_empty () {
		fail! (0x7fc2ebf6);
	}
	
	// NOTE:  This last step is an overhead, but it ensures an all-or-nothing processing!
	_decrypted.extend_from_slice (&_decompress_buffer);
	
	Ok (())
}


