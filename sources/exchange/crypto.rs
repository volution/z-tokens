

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use crate::keys::*;
use crate::coding::*;
use crate::oracles::*;


use ::z_tokens_runtime::{
		sensitive::drop,
	};


use ::z_tokens_runtime_crypto::{
		
		blake3_derive_key,
		blake3_derive_key_join,
		blake3_keyed_hash,
		argon_derive,
		generate_random,
		
		CryptographicMaterial as _,
		CryptographicInput as _,
		
		define_cryptographic_material,
		define_cryptographic_purpose,
	};


use ::z_tokens_runtime_crypto::crates::{
		
		chacha20,
		x25519,
	};








define_error! (pub CryptoError, result : CryptoResult);




pub const CRYPTO_DECRYPTED_SIZE_MAX : usize = 128 * 1024 * 1024;

pub const CRYPTO_ENCRYPTED_SIZE_MAX : usize =
		(
			(
				(
					CRYPTO_DECRYPTED_SIZE_MAX
					+ CRYPTO_ENCRYPTED_HEADER_SIZE
					+ CRYPTO_ENCRYPTED_PADDING_SIZE
					+ CRYPTO_ENCRYPTED_TRAILER_SIZE
				) / CODING_CHUNK_DECODED_SIZE
				+ 1
			) / CODING_CHUNKS_PER_LINE
			+ 1
		) * (
			9 + 4 + 1
			+ CODING_CHUNKS_PER_LINE * (CODING_CHUNKS_PER_LINE + CODING_CHUNK_ENCODED_SIZE + 1)
		);


pub const CRYPTO_X25519_COUNT_MAX : usize = 1024;
pub const CRYPTO_ASSOCIATED_COUNT_MAX : usize = 1024;
pub const CRYPTO_SECRET_COUNT_MAX : usize = 1024;
pub const CRYPTO_PIN_COUNT_MAX : usize = 1024;
pub const CRYPTO_SEED_COUNT_MAX : usize = 1024;
pub const CRYPTO_BALLAST_COUNT_MAX : usize = 1024;
pub const CRYPTO_ORACLE_COUNT_MAX : usize = 1024;


const CRYPTO_ENCRYPTED_SCHEMA_SIZE : usize = 4;
const CRYPTO_ENCRYPTED_LENGTH_SIZE : usize = 4;
const CRYPTO_ENCRYPTED_PADDING_SIZE : usize = 256;
const CRYPTO_ENCRYPTED_SALT_SIZE : usize = InternalPacketSalt::SIZE;
const CRYPTO_ENCRYPTED_MAC_SIZE : usize = InternalAuthenticationMac::SIZE;


const CRYPTO_ENCRYPTED_HEADER_SIZE : usize = CRYPTO_ENCRYPTED_SCHEMA_SIZE + CRYPTO_ENCRYPTED_LENGTH_SIZE;
const CRYPTO_ENCRYPTED_TRAILER_SIZE : usize = CRYPTO_ENCRYPTED_SALT_SIZE + CRYPTO_ENCRYPTED_MAC_SIZE;


pub const CRYPTO_SCHEMA_V1_VALUE : u32 = 0xb7e8bc01;


const CRYPTO_SECRET_ARGON_M_COST : u32 = 256 * 1024;
const CRYPTO_SECRET_ARGON_T_COST : u32 = 2;

const CRYPTO_PIN_ARGON_M_COST : u32 = 32 * 1024;
const CRYPTO_PIN_ARGON_T_COST : u32 = 2;

const CRYPTO_SEED_ARGON_M_COST : u32 = 8;
const CRYPTO_SEED_ARGON_T_COST : u32 = 1;

const CRYPTO_BALLAST_ARGON_M_COST : u32 = 1024 * 1024;
const CRYPTO_BALLAST_ARGON_T_COST : u32 = 2;








define_cryptographic_material! (InternalDheKey, 32);
define_cryptographic_material! (InternalPartialKey, 32);
define_cryptographic_material! (InternalAontKey, 32);
define_cryptographic_material! (InternalSchemaHash, 32);

define_cryptographic_material! (InternalPacketSalt, 32);
define_cryptographic_material! (InternalPacketKey, 32);

define_cryptographic_material! (InternalEncryptionKey, 32);

define_cryptographic_material! (InternalAuthenticationKey, 32);
define_cryptographic_material! (InternalAuthenticationMac, 32);

define_cryptographic_material! (InternalAssociatedInput, input, slice);
define_cryptographic_material! (InternalAssociatedHash, 32);
define_cryptographic_material! (InternalAssociatedMerge, 32);

define_cryptographic_material! (InternalSecretInput, input, slice);
define_cryptographic_material! (InternalSecretHash, 32);
define_cryptographic_material! (InternalSecretMerge, 32);
define_cryptographic_material! (InternalSecretSalt, 32);
define_cryptographic_material! (InternalSecretArgon, 32);
define_cryptographic_material! (InternalSecretKey, 32);

define_cryptographic_material! (InternalPinInput, input, slice);
define_cryptographic_material! (InternalPinHash, 32);
define_cryptographic_material! (InternalPinMerge, 32);
define_cryptographic_material! (InternalPinSalt, 32);
define_cryptographic_material! (InternalPinArgon, 32);
define_cryptographic_material! (InternalPinKey, 32);

define_cryptographic_material! (InternalSeedInput, input, slice);
define_cryptographic_material! (InternalSeedHash, 32);
define_cryptographic_material! (InternalSeedMerge, 32);
define_cryptographic_material! (InternalSeedSalt, 32);
define_cryptographic_material! (InternalSeedArgon, 32);
define_cryptographic_material! (InternalSeedKey, 32);

define_cryptographic_material! (InternalBallastInput, input, slice);
define_cryptographic_material! (InternalBallastHash, 32);
define_cryptographic_material! (InternalBallastMerge, 32);
define_cryptographic_material! (InternalBallastSalt, 32);
define_cryptographic_material! (InternalBallastArgon, 32);
define_cryptographic_material! (InternalBallastKey, 32);

define_cryptographic_material! (InternalOracleHandle, 32);
define_cryptographic_material! (InternalOracleMerge, 32);
define_cryptographic_material! (InternalOracleSorter, 32);
define_cryptographic_material! (InternalOracleInput, 32);
define_cryptographic_material! (InternalOracleOutput, 32);
define_cryptographic_material! (InternalOracleKey, 32);

define_cryptographic_material! (InternalDecryptedData, input, slice);
define_cryptographic_material! (InternalEncryptedData, input, slice);

define_cryptographic_material! (InternalPasswordData, input, slice);
define_cryptographic_material! (InternalPasswordOutput, 32);




define_cryptographic_purpose! (CRYPTO_ENCRYPTION_SCHEMA_V1, encryption, schema_v1);
define_cryptographic_purpose! (CRYPTO_PASSWORD_SCHEMA_V1, password, schema_v1);

define_cryptographic_purpose! (CRYPTO_DHE_PUBLIC_MERGE_PURPOSE, encryption, dhe_public_merge);
define_cryptographic_purpose! (CRYPTO_DHE_SHARED_MERGE_PURPOSE, encryption, dhe_shared_merge);
define_cryptographic_purpose! (CRYPTO_DHE_KEY_PURPOSE, encryption, dhe_key);

define_cryptographic_purpose! (CRYPTO_PARTIAL_KEY_PURPOSE, encryption, partial_key);
define_cryptographic_purpose! (CRYPTO_AONT_KEY_PURPOSE, encryption, aont_key);

define_cryptographic_purpose! (CRYPTO_PACKET_SALT_PURPOSE, encryption, packet_salt);
define_cryptographic_purpose! (CRYPTO_PACKET_KEY_PURPOSE, encryption, packet_key);
define_cryptographic_purpose! (CRYPTO_ENCRYPTION_KEY_PURPOSE, encryption, encryption_key);
define_cryptographic_purpose! (CRYPTO_AUTHENTICATION_KEY_PURPOSE, encryption, authentication_key);

define_cryptographic_purpose! (CRYPTO_ASSOCIATED_HASH_PURPOSE, encryption, associated_hash);
define_cryptographic_purpose! (CRYPTO_ASSOCIATED_MERGE_PURPOSE, encryption, associated_merge);

define_cryptographic_purpose! (CRYPTO_SECRET_HASH_PURPOSE, encryption, secret_hash);
define_cryptographic_purpose! (CRYPTO_SECRET_MERGE_PURPOSE, encryption, secret_merge);
define_cryptographic_purpose! (CRYPTO_SECRET_SALT_PURPOSE, encryption, secret_salt);
define_cryptographic_purpose! (CRYPTO_SECRET_KEY_PURPOSE, encryption, secret_key);

define_cryptographic_purpose! (CRYPTO_PIN_HASH_PURPOSE, encryption, pin_hash);
define_cryptographic_purpose! (CRYPTO_PIN_MERGE_PURPOSE, encryption, pin_merge);
define_cryptographic_purpose! (CRYPTO_PIN_SALT_PURPOSE, encryption, pin_salt);
define_cryptographic_purpose! (CRYPTO_PIN_KEY_PURPOSE, encryption, pin_key);

define_cryptographic_purpose! (CRYPTO_SEED_HASH_PURPOSE, encryption, seed_hash);
define_cryptographic_purpose! (CRYPTO_SEED_MERGE_PURPOSE, encryption, seed_merge);
define_cryptographic_purpose! (CRYPTO_SEED_SALT_PURPOSE, encryption, seed_salt);
define_cryptographic_purpose! (CRYPTO_SEED_KEY_PURPOSE, encryption, seed_key);

define_cryptographic_purpose! (CRYPTO_BALLAST_HASH_PURPOSE, encryption, ballast_hash);
define_cryptographic_purpose! (CRYPTO_BALLAST_MERGE_PURPOSE, encryption, ballast_merge);
define_cryptographic_purpose! (CRYPTO_BALLAST_SALT_PURPOSE, encryption, ballast_salt);
define_cryptographic_purpose! (CRYPTO_BALLAST_KEY_PURPOSE, encryption, ballast_key);

define_cryptographic_purpose! (CRYPTO_ORACLE_MERGE_PURPOSE, encryption, oracle_merge);
define_cryptographic_purpose! (CRYPTO_ORACLE_SORTER_PURPOSE, encryption, oracle_sorter);
define_cryptographic_purpose! (CRYPTO_ORACLE_INPUT_PURPOSE, encryption, oracle_input);
define_cryptographic_purpose! (CRYPTO_ORACLE_OUTPUT_PURPOSE, encryption, oracle_output);

define_cryptographic_purpose! (CRYPTO_PASSWORD_SALT_PURPOSE, password, salt);
define_cryptographic_purpose! (CRYPTO_PASSWORD_OUTPUT_PURPOSE, password, output);








pub fn password <'a> (
			_senders : impl Iterator<Item = &'a SenderPrivateKey>,
			_recipients : impl Iterator<Item = &'a RecipientPublicKey>,
			_associated : impl Iterator<Item = &'a Associated>,
			_secret : impl Iterator<Item = &'a SharedSecret>,
			_pin : impl Iterator<Item = &'a SharedPin>,
			_seed : impl Iterator<Item = &'a SharedSeed>,
			_ballast : impl Iterator<Item = &'a SharedBallast>,
			_derivation_loops : Option<NonZeroU64>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
		) -> CryptoResult
{
	password_with_raw (
			& _senders.collect::<Vec<_>> (),
			& _recipients.collect::<Vec<_>> (),
			& _associated.map (Associated::access_bytes_slice) .collect::<Vec<_>> (),
			& _secret.map (SharedSecret::access_bytes_slice) .collect::<Vec<_>> (),
			& _pin.map (SharedPin::access_bytes_slice) .collect::<Vec<_>> (),
			& _seed.map (SharedSeed::access_bytes_slice) .collect::<Vec<_>> (),
			& _ballast.map (SharedBallast::access_bytes_slice) .collect::<Vec<_>> (),
			_derivation_loops,
			_password_data,
			_password_output,
			_oracles.collect (),
		)
}




pub fn password_with_raw (
			_senders : &[&SenderPrivateKey],
			_recipients : &[&RecipientPublicKey],
			_associated_inputs : &[&[u8]],
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_seed_inputs : &[&[u8]],
			_ballast_inputs : &[&[u8]],
			_derivation_loops : Option<NonZeroU64>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : Vec<&mut dyn Oracle>,
		) -> CryptoResult
{
	let (_senders, _recipients) = wrap_senders_and_recipients_inputs (_senders, _recipients) ?;
	let (_associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs) = wrap_associated_and_secrets_and_pins_inputs (_associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs) ?;
	let (_oracles, _oracle_handles) = wrap_oracles_phase_1 (_oracles) ?;
	
	let _password_data = InternalPasswordData::wrap (_password_data);
	let _password_data_len = _password_data.size ();
	
	if _password_data_len > CRYPTO_DECRYPTED_SIZE_MAX {
		fail! (0xfa4d9417);
	}
	
	// --------------------------------------------------------------------------------
	// --------------------------------------------------------------------------------
	
	// NOTE:  deriving keys...
	
	let (_partial_key, _aont_key, _secret_hashes, _pin_hashes, _seed_hashes, _ballast_hashes, _oracle_merge, _oracle_sorter)
			= derive_keys_phase_1 (CRYPTO_PASSWORD_SCHEMA_V1, _senders, _recipients, _associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs, _oracle_handles, true) ?;
	
	let _oracles = wrap_oracles_phase_2 (_oracles, _oracle_sorter) ?;
	
	drop! (_aont_key);
	
	// NOTE:  salting...
	
	let _packet_salt = blake3_derive_key (
			InternalPacketSalt::wrap,
			CRYPTO_PASSWORD_SALT_PURPOSE,
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
			= derive_keys_phase_2 (CRYPTO_PASSWORD_SCHEMA_V1, _partial_key, &_packet_salt, _secret_hashes, _pin_hashes, _seed_hashes, _ballast_hashes, (_oracle_merge, _oracles), _derivation_loops) ?;
	
	drop! (_encryption_key, _authentication_key);
	
	let _password_output_0 = blake3_derive_key (
			InternalPasswordOutput::wrap,
			CRYPTO_PASSWORD_OUTPUT_PURPOSE,
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








pub fn encrypt <'a> (
			_senders : impl Iterator<Item = &'a SenderPrivateKey>,
			_recipients : impl Iterator<Item = &'a RecipientPublicKey>,
			_associated : impl Iterator<Item = &'a Associated>,
			_secret : impl Iterator<Item = &'a SharedSecret>,
			_pin : impl Iterator<Item = &'a SharedPin>,
			_seed : impl Iterator<Item = &'a SharedSeed>,
			_ballast : impl Iterator<Item = &'a SharedBallast>,
			_derivation_loops : Option<NonZeroU64>,
			_decrypted : &[u8],
			_encrypted : &mut Vec<u8>,
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
			_packet_salt_deterministic : bool,
		) -> CryptoResult
{
	encrypt_with_raw (
			& _senders.collect::<Vec<_>> (),
			& _recipients.collect::<Vec<_>> (),
			& _associated.map (Associated::access_bytes_slice) .collect::<Vec<_>> (),
			& _secret.map (SharedSecret::access_bytes_slice) .collect::<Vec<_>> (),
			& _pin.map (SharedPin::access_bytes_slice) .collect::<Vec<_>> (),
			& _seed.map (SharedSeed::access_bytes_slice) .collect::<Vec<_>> (),
			& _ballast.map (SharedBallast::access_bytes_slice) .collect::<Vec<_>> (),
			_derivation_loops,
			_decrypted,
			_encrypted,
			_oracles.collect (),
			_packet_salt_deterministic,
		)
}




pub fn encrypt_with_raw (
			_senders : &[&SenderPrivateKey],
			_recipients : &[&RecipientPublicKey],
			_associated_inputs : &[&[u8]],
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_seed_inputs : &[&[u8]],
			_ballast_inputs : &[&[u8]],
			_derivation_loops : Option<NonZeroU64>,
			_decrypted : &[u8],
			_encrypted : &mut Vec<u8>,
			_oracles : Vec<&mut dyn Oracle>,
			_packet_salt_deterministic : bool,
		) -> CryptoResult
{
	let (_senders, _recipients) = wrap_senders_and_recipients_inputs (_senders, _recipients) ?;
	let (_associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs) = wrap_associated_and_secrets_and_pins_inputs (_associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs) ?;
	let (_oracles, _oracle_handles) = wrap_oracles_phase_1 (_oracles) ?;
	
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
	
	let (_partial_key, _aont_key, _secret_hashes, _pin_hashes, _seed_hashes, _ballast_hashes, _oracle_merge, _oracle_sorter)
			= derive_keys_phase_1 (CRYPTO_ENCRYPTION_SCHEMA_V1, _senders, _recipients, _associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs, _oracle_handles, true) ?;
	
	let _oracles = wrap_oracles_phase_2 (_oracles, _oracle_sorter) ?;
	
	// NOTE:  salting...
	
	let mut _packet_salt = if _packet_salt_deterministic {
			
			blake3_derive_key (
					InternalPacketSalt::wrap,
					CRYPTO_PACKET_SALT_PURPOSE,
					&[
						_partial_key.access (),
					],
					&[
						&_intermediate_buffer,
					],
					None,
				)
			
		} else {
			
			generate_random (InternalPacketSalt::wrap)
		};
	
	let (_packet_key, _encryption_key, _authentication_key)
			= derive_keys_phase_2 (CRYPTO_ENCRYPTION_SCHEMA_V1, _partial_key, &_packet_salt, _secret_hashes, _pin_hashes, _seed_hashes, _ballast_hashes, (_oracle_merge, _oracles), _derivation_loops) ?;
	
	drop! (_packet_key);
	
	// NOTE:  encrypting...
	
	apply_encryption (_encryption_key, &mut _intermediate_buffer) ?;
	
	// NOTE:  authenticating...
	
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








pub fn decrypt <'a> (
			_recipients : impl Iterator<Item = &'a RecipientPrivateKey>,
			_senders : impl Iterator<Item = &'a SenderPublicKey>,
			_associated : impl Iterator<Item = &'a Associated>,
			_secret : impl Iterator<Item = &'a SharedSecret>,
			_pin : impl Iterator<Item = &'a SharedPin>,
			_seed : impl Iterator<Item = &'a SharedSeed>,
			_ballast : impl Iterator<Item = &'a SharedBallast>,
			_derivation_loops : Option<NonZeroU64>,
			_encrypted : &[u8],
			_decrypted : &mut Vec<u8>,
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
		) -> CryptoResult
{
	decrypt_with_raw (
			& _recipients.collect::<Vec<_>> (),
			& _senders.collect::<Vec<_>> (),
			& _associated.map (Associated::access_bytes_slice) .collect::<Vec<_>> (),
			& _secret.map (SharedSecret::access_bytes_slice) .collect::<Vec<_>> (),
			& _pin.map (SharedPin::access_bytes_slice) .collect::<Vec<_>> (),
			& _seed.map (SharedSeed::access_bytes_slice) .collect::<Vec<_>> (),
			& _ballast.map (SharedBallast::access_bytes_slice) .collect::<Vec<_>> (),
			_derivation_loops,
			_encrypted,
			_decrypted,
			_oracles.collect (),
		)
}




pub fn decrypt_with_raw (
			_recipients : &[&RecipientPrivateKey],
			_senders : &[&SenderPublicKey],
			_associated_inputs : &[&[u8]],
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_seed_inputs : &[&[u8]],
			_ballast_inputs : &[&[u8]],
			_derivation_loops : Option<NonZeroU64>,
			_encrypted : &[u8],
			_decrypted : &mut Vec<u8>,
			_oracles : Vec<&mut dyn Oracle>,
		) -> CryptoResult
{
	let (_recipients, _senders) = wrap_recipients_and_senders_inputs (_recipients, _senders) ?;
	let (_associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs) = wrap_associated_and_secrets_and_pins_inputs (_associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs) ?;
	let (_oracles, _oracle_handles) = wrap_oracles_phase_1 (_oracles) ?;
	
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
	
	let (_partial_key, _aont_key, _secret_hashes, _pin_hashes, _seed_hashes, _ballast_hashes, _oracle_merge, _oracle_sorter)
			= derive_keys_phase_1 (CRYPTO_ENCRYPTION_SCHEMA_V1, _recipients, _senders, _associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs, _oracle_handles, false) ?;
	
	let _oracles = wrap_oracles_phase_2 (_oracles, _oracle_sorter) ?;
	
	// NOTE:  all-or-nothing and salting...
	
	let _packet_salt = bytes_pop::<CRYPTO_ENCRYPTED_SALT_SIZE> (&mut _intermediate_buffer) .else_wrap (0x78ed3811) ?;
	let mut _packet_salt = InternalPacketSalt::wrap (_packet_salt);
	
	apply_all_or_nothing_mangling (_aont_key, &mut _packet_salt, &_intermediate_buffer) ?;
	
	// NOTE:  deriving keys...
	
	let (_packet_key, _encryption_key, _authentication_key)
			= derive_keys_phase_2 (CRYPTO_ENCRYPTION_SCHEMA_V1, _partial_key, &_packet_salt, _secret_hashes, _pin_hashes, _seed_hashes, _ballast_hashes, (_oracle_merge, _oracles), _derivation_loops) ?;
	
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








fn apply_encryption (_key : InternalEncryptionKey, _data : &mut [u8]) -> CryptoResult {
	
	use chacha20::cipher::KeyIvInit as _;
	use chacha20::cipher::StreamCipher as _;
	
	let _nonce = [0u8; 12];
	
	let _key = chacha20::Key::from (_key.material);
	let _nonce = chacha20::Nonce::from (_nonce);
	
	let mut _cipher = chacha20::ChaCha20::new (&_key, &_nonce);
	
	assert! (_data.len () >= CRYPTO_ENCRYPTED_HEADER_SIZE, "[c9e6989f]");
	
	_cipher.try_apply_keystream (&mut _data[CRYPTO_ENCRYPTED_SCHEMA_SIZE..]) .else_wrap (0x9c94d0d5) ?;
	
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
			_schema : &'static str,
			_private_keys : Vec<&x25519::StaticSecret>,
			_public_keys : Vec<&x25519::PublicKey>,
			_associated_inputs : Vec<InternalAssociatedInput>,
			_secret_inputs : Vec<InternalSecretInput>,
			_pin_inputs : Vec<InternalPinInput>,
			_seed_inputs : Vec<InternalSeedInput>,
			_ballast_inputs : Vec<InternalBallastInput>,
			_oracle_handles : Vec<InternalOracleHandle>,
			_encryption : bool,
		) -> CryptoResult<(
			InternalPartialKey,
			InternalAontKey,
			(InternalSecretMerge, Vec<InternalSecretHash>),
			(InternalPinMerge, Vec<InternalPinHash>),
			(InternalSeedMerge, Vec<InternalSeedHash>),
			(InternalBallastMerge, Vec<InternalBallastHash>),
			InternalOracleMerge,
			InternalOracleSorter,
		)>
{
	// --------------------------------------------------------------------------------
	// NOTE:  derive schema hash...
	
	let _schema_hash = blake3_derive_key (
			InternalSchemaHash::wrap,
			_schema,
			&[],
			&[],
			None,
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive associated hashes...
	
	let mut _associated_hashes : Vec<_> = _associated_inputs.into_iter () .map (
			|_associated_input|
					blake3_derive_key (
							InternalAssociatedHash::wrap,
							CRYPTO_ASSOCIATED_HASH_PURPOSE,
							&[
								_schema_hash.access (),
							],
							&[
								_associated_input.access_consume (),
							],
							None,
						)
		) .collect ();
	
	// NOTE:  associated data is not sorted or deduplicated, thus order is important!
	
	let _associated_merge = blake3_derive_key_join (
			InternalAssociatedMerge::wrap,
			CRYPTO_ASSOCIATED_MERGE_PURPOSE,
			_associated_hashes.iter () .map (InternalAssociatedHash::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive secret hashes...
	
	let mut _secret_hashes : Vec<_> = _secret_inputs.into_iter () .map (
			|_secret_input|
					blake3_derive_key (
							InternalSecretHash::wrap,
							CRYPTO_SECRET_HASH_PURPOSE,
							&[
								_schema_hash.access (),
							],
							&[
								_secret_input.access_consume (),
							],
							None,
						)
		) .collect ();
	
	_secret_hashes.sort_by (InternalSecretHash::cmp_access);
	_secret_hashes.dedup_by (|_left, _right| InternalSecretHash::eq_access (_left, _right));
	
	let _secret_merge = blake3_derive_key_join (
			InternalSecretMerge::wrap,
			CRYPTO_SECRET_MERGE_PURPOSE,
			_secret_hashes.iter () .map (InternalSecretHash::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive pin hashes...
	
	let mut _pin_hashes : Vec<_> = _pin_inputs.into_iter () .map (
			|_pin_input|
					blake3_derive_key (
							InternalPinHash::wrap,
							CRYPTO_PIN_HASH_PURPOSE,
							&[
								_schema_hash.access (),
							],
							&[
								_pin_input.access_consume (),
							],
							None,
						)
		) .collect ();
	
	_pin_hashes.sort_by (InternalPinHash::cmp_access);
	_pin_hashes.dedup_by (|_left, _right| InternalPinHash::eq_access (_left, _right));
	
	let _pin_merge = blake3_derive_key_join (
			InternalPinMerge::wrap,
			CRYPTO_PIN_MERGE_PURPOSE,
			_pin_hashes.iter () .map (InternalPinHash::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive seed hashes...
	
	let mut _seed_hashes : Vec<_> = _seed_inputs.into_iter () .map (
			|_seed_input|
					blake3_derive_key (
							InternalSeedHash::wrap,
							CRYPTO_SEED_HASH_PURPOSE,
							&[
								_schema_hash.access (),
							],
							&[
								_seed_input.access_consume (),
							],
							None,
						)
		) .collect ();
	
	_seed_hashes.sort_by (InternalSeedHash::cmp_access);
	_seed_hashes.dedup_by (|_left, _right| InternalSeedHash::eq_access (_left, _right));
	
	let _seed_merge = blake3_derive_key_join (
			InternalSeedMerge::wrap,
			CRYPTO_SEED_MERGE_PURPOSE,
			_seed_hashes.iter () .map (InternalSeedHash::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive ballast hashes...
	
	let mut _ballast_hashes : Vec<_> = _ballast_inputs.into_iter () .map (
			|_ballast_input|
					blake3_derive_key (
							InternalBallastHash::wrap,
							CRYPTO_BALLAST_HASH_PURPOSE,
							&[
								_schema_hash.access (),
							],
							&[
								_ballast_input.access_consume (),
							],
							None,
						)
		) .collect ();
	
	_ballast_hashes.sort_by (InternalBallastHash::cmp_access);
	_ballast_hashes.dedup_by (|_left, _right| InternalBallastHash::eq_access (_left, _right));
	
	let _ballast_merge = blake3_derive_key_join (
			InternalBallastMerge::wrap,
			CRYPTO_BALLAST_MERGE_PURPOSE,
			_ballast_hashes.iter () .map (InternalBallastHash::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive oracle hashes...
	
	let _oracle_merge = blake3_derive_key_join (
			InternalOracleMerge::wrap,
			CRYPTO_ORACLE_MERGE_PURPOSE,
			_oracle_handles.iter () .map (InternalOracleHandle::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive X25519 DHE...
	
	let _dhe_key = match (_private_keys.len (), _public_keys.len ()) {
		
		(0, 0) =>
			if _associated_hashes.is_empty () && _secret_hashes.is_empty () && _pin_hashes.is_empty () && _seed_hashes.is_empty () && _ballast_hashes.is_empty () && _oracle_handles.is_empty () {
				fail! (0xa1de0167);
			} else {
				InternalDheKey::zero ()
			}
		
		(0, _) =>
			fail! (0x9cfadd5b),
		
		(_, _) => {
			
			let mut _private_public_keys = _private_keys.iter () .map (|_private_key| x25519::PublicKey::from (*_private_key)) .collect::<Vec<_>> ();
			_private_public_keys.sort_by (|_left, _right| Ord::cmp (_left.as_bytes (), _right.as_bytes ()));
			let _private_public_keys = _private_public_keys.iter () .collect::<Vec<_>> ();
			
			// NOTE:  If no recipient keys are specified, use sender public keys.
			let _public_keys = if _public_keys.is_empty () {
					&_private_public_keys
				} else {
					&_public_keys
				};
			
			let mut _shared = Vec::with_capacity (_private_keys.len () * _public_keys.len ());
			
			for _private_key in _private_keys {
				for _public_key in _public_keys {
					
					let _dhe = x25519::StaticSecret::diffie_hellman (_private_key, _public_key);
					
					if ! _dhe.was_contributory () {
						fail! (0xd00d13f7);
					}
					
					_shared.push (_dhe.to_bytes ());
				}
			}
			
			_shared.sort ();
			
			let _private_public_merge = blake3_derive_key_join (
					|_hash| _hash,
					CRYPTO_DHE_PUBLIC_MERGE_PURPOSE,
					_private_public_keys.iter () .map (|_key| _key.as_bytes ()),
				);
			let _public_merge = blake3_derive_key_join (
					|_hash| _hash,
					CRYPTO_DHE_PUBLIC_MERGE_PURPOSE,
					_public_keys.iter () .map (|_key| _key.as_bytes ()),
				);
			
			let (_senders_merge, _recipients_merge) = if _encryption {
				(_private_public_merge, _public_merge)
			} else {
				(_public_merge, _private_public_merge)
			};
			
			let _shared_merge = blake3_derive_key_join (
					|_hash| _hash,
					CRYPTO_DHE_SHARED_MERGE_PURPOSE,
					_shared.iter (),
				);
			
			let _dhe_key = blake3_derive_key_join (
					InternalDheKey::wrap,
					CRYPTO_DHE_KEY_PURPOSE,
					[
						_schema_hash.access (),
						&_senders_merge,
						&_recipients_merge,
						&_shared_merge,
					] .into_iter (),
				);
			
			_dhe_key
		}
	};
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive partial key (for the entire transaction)...
	
	let _partial_key = blake3_derive_key (
			InternalPartialKey::wrap,
			CRYPTO_PARTIAL_KEY_PURPOSE,
			&[
				_schema_hash.access (),
				_associated_merge.access (),
				_oracle_merge.access (),
				_ballast_merge.access (),
				_seed_merge.access (),
				_secret_merge.access (),
				_pin_merge.access (),
				_dhe_key.access (),
			],
			&[],
			None,
		);
	
	drop! (_associated_merge);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive AONT key...
	
	let _aont_key = blake3_derive_key (
			InternalAontKey::wrap,
			CRYPTO_AONT_KEY_PURPOSE,
			&[
				_partial_key.access (),
			],
			&[],
			None,
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive oracles sorter...
	
	let _oracle_sorter = blake3_derive_key (
			InternalOracleSorter::wrap,
			CRYPTO_ORACLE_SORTER_PURPOSE,
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
			(_secret_merge, _secret_hashes),
			(_pin_merge, _pin_hashes),
			(_seed_merge, _seed_hashes),
			(_ballast_merge, _ballast_hashes),
			_oracle_merge, _oracle_sorter,
		))
}








fn derive_keys_phase_2 (
			_schema : &'static str,
			_partial_key : InternalPartialKey,
			_packet_salt : &InternalPacketSalt,
			_secret_hash : (InternalSecretMerge, Vec<InternalSecretHash>),
			_pin_hash : (InternalPinMerge, Vec<InternalPinHash>),
			_seed_hash : (InternalSeedMerge, Vec<InternalSeedHash>),
			_ballast_hash : (InternalBallastMerge, Vec<InternalBallastHash>),
			_oracles : (InternalOracleMerge, Vec<(&mut dyn Oracle, InternalOracleHandle)>),
			_derivation_loops : Option<NonZeroU64>,
		) -> CryptoResult<(
			InternalPacketKey,
			InternalEncryptionKey,
			InternalAuthenticationKey,
		)>
{
	let (_secret_merge, _secret_hashes) = _secret_hash;
	let (_pin_merge, _pin_hashes) = _pin_hash;
	let (_seed_merge, _seed_hashes) = _seed_hash;
	let (_ballast_merge, _ballast_hashes) = _ballast_hash;
	let (_oracle_merge, mut _oracles) = _oracles;
	
	let _derivation_loops = _derivation_loops.map (NonZeroU64::get) .unwrap_or (1);
	
	// --------------------------------------------------------------------------------
	// NOTE:  initialize keys...
	
	let mut _packet_key = InternalPacketKey::wrap (_partial_key.material);
	let mut _oracle_key = InternalOracleOutput::wrap (_oracle_merge.material);
	let mut _ballast_key = InternalBallastKey::wrap (_ballast_merge.material);
	let mut _seed_key = InternalSeedKey::wrap (_seed_merge.material);
	let mut _secret_key = InternalSecretKey::wrap (_secret_merge.material);
	let mut _pin_key = InternalPinKey::wrap (_pin_merge.material);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derivation loops...
	
	for _derivation_loop in 0 ..= _derivation_loops {
		
		// --------------------------------------------------------------------------------
		// NOTE:  derive packet key...
		
		_packet_key = blake3_derive_key (
				InternalPacketKey::wrap,
				CRYPTO_PACKET_KEY_PURPOSE,
				&[
					_packet_salt.access (),
					_packet_key.access (),
					_oracle_key.access (),
					_ballast_key.access (),
					_seed_key.access (),
					_secret_key.access (),
					_pin_key.access (),
				],
				&[],
				Some (_derivation_loop),
			);
		
		if _derivation_loop == _derivation_loops {
			break;
		}
		
		// --------------------------------------------------------------------------------
		// NOTE:  call oracles...
		
		for (_oracle_wrapper, _oracle_handle) in _oracles.iter_mut () {
			
			let _oracle_input = blake3_derive_key (
					InternalOracleInput::wrap,
					CRYPTO_ORACLE_INPUT_PURPOSE,
					&[
						_packet_key.access (),
						_oracle_key.access (),
						_oracle_handle.access (),
					],
					&[],
					None,
				);
			
			let mut _oracle_output = InternalOracleOutput::zero ();
			_oracle_wrapper.derive (Some (_schema), _oracle_input.access (), &mut _oracle_output.material) .else_wrap (0xcc07e95e) ?;
			
			_oracle_key = blake3_derive_key (
					InternalOracleOutput::wrap,
					CRYPTO_ORACLE_OUTPUT_PURPOSE,
					&[
						_oracle_input.access (),
						_oracle_output.access (),
					],
					&[],
					None,
				);
		}
		
		// --------------------------------------------------------------------------------
		// NOTE:  derive ballast argon hashes...
		
		let _ballast_count = _ballast_hashes.len ();
		
		for _ballast_hash in _ballast_hashes.iter () {
			
			let _ballast_salt = blake3_derive_key (
					InternalBallastSalt::wrap,
					CRYPTO_BALLAST_SALT_PURPOSE,
					&[
						_packet_key.access (),
						_oracle_key.access (),
						_ballast_key.access (),
					],
					&[],
					None,
				);
			
			let _ballast_argon = apply_argon_ballast (_ballast_hash, &_ballast_salt, _ballast_count) ?;
			
			_ballast_key = blake3_derive_key (
					InternalBallastKey::wrap,
					CRYPTO_BALLAST_KEY_PURPOSE,
					&[
						_ballast_salt.access (),
						_ballast_argon.access (),
					],
					&[],
					None,
				);
		}
		
		// --------------------------------------------------------------------------------
		// NOTE:  derive seed argon hashes...
		
		for _seed_hash in _seed_hashes.iter () {
			
			let _seed_salt = blake3_derive_key (
					InternalSeedSalt::wrap,
					CRYPTO_SEED_SALT_PURPOSE,
					&[
						_packet_key.access (),
						_oracle_key.access (),
						_seed_key.access (),
					],
					&[],
					None,
				);
			
			let _seed_argon = apply_argon_seed (_seed_hash, &_seed_salt) ?;
			
			_seed_key = blake3_derive_key (
					InternalSeedKey::wrap,
					CRYPTO_SEED_KEY_PURPOSE,
					&[
						_seed_salt.access (),
						_seed_argon.access (),
					],
					&[],
					None,
				);
		}
		
		// --------------------------------------------------------------------------------
		// NOTE:  derive secret argon hashes...
		
		for _secret_hash in _secret_hashes.iter () {
			
			let _secret_salt = blake3_derive_key (
					InternalSecretSalt::wrap,
					CRYPTO_SECRET_SALT_PURPOSE,
					&[
						_packet_key.access (),
						_oracle_key.access (),
						_secret_key.access (),
					],
					&[],
					None,
				);
			
			let _secret_argon = apply_argon_secret (_secret_hash, &_secret_salt) ?;
			
			_secret_key = blake3_derive_key (
					InternalSecretKey::wrap,
					CRYPTO_SECRET_KEY_PURPOSE,
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
		
		for _pin_hash in _pin_hashes.iter () {
			
			let _pin_salt = blake3_derive_key (
					InternalPinSalt::wrap,
					CRYPTO_PIN_SALT_PURPOSE,
					&[
						_packet_key.access (),
						_oracle_key.access (),
						_pin_key.access (),
					],
					&[],
					None,
				);
			
			let _pin_argon = apply_argon_pin (_pin_hash, &_pin_salt) ?;
			
			_pin_key = blake3_derive_key (
					InternalPinKey::wrap,
					CRYPTO_PIN_KEY_PURPOSE,
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
		
		// NOTE:  This actually happens at the beginning of the next loop!
	}
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive encryption key...
	
	let _encryption_key = blake3_derive_key (
			InternalEncryptionKey::wrap,
			CRYPTO_ENCRYPTION_KEY_PURPOSE,
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
			CRYPTO_AUTHENTICATION_KEY_PURPOSE,
			&[
				_packet_key.access (),
			],
			&[],
			None,
		);
	
	// --------------------------------------------------------------------------------
	
	Ok ((_packet_key, _encryption_key, _authentication_key))
}








fn apply_argon_secret (_secret_hash : &InternalSecretHash, _secret_salt : &InternalSecretSalt) -> CryptoResult<InternalSecretArgon> {
	
	argon_derive (
			InternalSecretArgon::wrap,
			_secret_hash.access (),
			_secret_salt.access (),
			CRYPTO_SECRET_ARGON_M_COST,
			CRYPTO_SECRET_ARGON_T_COST,
		) .else_wrap (0xd26ab4d1)
}


fn apply_argon_pin (_pin_hash : &InternalPinHash, _pin_salt : &InternalPinSalt) -> CryptoResult<InternalPinArgon> {
	
	argon_derive (
			InternalPinArgon::wrap,
			_pin_hash.access (),
			_pin_salt.access (),
			CRYPTO_PIN_ARGON_M_COST,
			CRYPTO_PIN_ARGON_T_COST,
		) .else_wrap (0xc396c6f6)
}


fn apply_argon_seed (_seed_hash : &InternalSeedHash, _seed_salt : &InternalSeedSalt) -> CryptoResult<InternalSeedArgon> {
	
	argon_derive (
			InternalSeedArgon::wrap,
			_seed_hash.access (),
			_seed_salt.access (),
			CRYPTO_SEED_ARGON_M_COST,
			CRYPTO_SEED_ARGON_T_COST,
		) .else_wrap (0x9a61f747)
}


fn apply_argon_ballast (_ballast_hash : &InternalBallastHash, _ballast_salt : &InternalBallastSalt, _ballast_count : usize) -> CryptoResult<InternalBallastArgon> {
	
	let _ballast_count = _ballast_count as u32;
	
	let _m_cost_factor = _ballast_count;
	let _t_cost_factor = _ballast_count;
	
	argon_derive (
			InternalBallastArgon::wrap,
			_ballast_hash.access (),
			_ballast_salt.access (),
			CRYPTO_BALLAST_ARGON_M_COST * _m_cost_factor,
			CRYPTO_BALLAST_ARGON_T_COST * _t_cost_factor,
		) .else_wrap (0x5e8b2b57)
}








fn wrap_senders_and_recipients_inputs <'a> (
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


fn wrap_recipients_and_senders_inputs <'a> (
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


fn wrap_private_and_public_keys <'a> (
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








fn wrap_associated_and_secrets_and_pins_inputs <'a> (
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








fn wrap_oracles_phase_1 <'a> (
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


fn wrap_oracles_phase_2 <'a> (
			_oracles : Vec<&'a mut dyn Oracle>,
			_sorter : InternalOracleSorter,
		) -> CryptoResult<
			Vec<(&'a mut dyn Oracle, InternalOracleHandle)>,
		>
{
	let _oracles_with_handles = wrap_oracles_internal (_oracles, Some (_sorter)) ?;
	Ok (_oracles_with_handles)
}


fn wrap_oracles_internal <'a> (
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
									None,
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


