

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use crate::coding::*;


use ::z_tokens_runtime_crypto::{
		
		define_cryptographic_material,
		define_cryptographic_purpose,
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


pub(crate) const CRYPTO_ENCRYPTED_SCHEMA_SIZE : usize = 4;
pub(crate) const CRYPTO_ENCRYPTED_LENGTH_SIZE : usize = 4;
pub(crate) const CRYPTO_ENCRYPTED_PADDING_SIZE : usize = 256;
pub(crate) const CRYPTO_ENCRYPTED_SALT_SIZE : usize = InternalPacketSalt::SIZE;
pub(crate) const CRYPTO_ENCRYPTED_MAC_SIZE : usize = InternalAuthenticationMac::SIZE;


pub(crate) const CRYPTO_ENCRYPTED_HEADER_SIZE : usize = CRYPTO_ENCRYPTED_SCHEMA_SIZE + CRYPTO_ENCRYPTED_LENGTH_SIZE;
pub(crate) const CRYPTO_ENCRYPTED_TRAILER_SIZE : usize = CRYPTO_ENCRYPTED_SALT_SIZE + CRYPTO_ENCRYPTED_MAC_SIZE;


pub const CRYPTO_SCHEMA_V1_VALUE : u32 = 0xb7e8bc01;


pub(crate) const CRYPTO_SECRET_ARGON_M_COST : u32 = 64 * 1024;
pub(crate) const CRYPTO_SECRET_ARGON_T_COST : u32 = 4;
pub(crate) const CRYPTO_SECRET_ARGON_P_COST : u32 = 1;

pub(crate) const CRYPTO_PIN_ARGON_M_COST : u32 = 8 * 1024;
pub(crate) const CRYPTO_PIN_ARGON_T_COST : u32 = 1;
pub(crate) const CRYPTO_PIN_ARGON_P_COST : u32 = 1;

pub(crate) const CRYPTO_BALLAST_ARGON_M_COST : u32 = 256 * 1024;
pub(crate) const CRYPTO_BALLAST_ARGON_T_COST : u32 = 8;
pub(crate) const CRYPTO_BALLAST_ARGON_P_COST : u32 = 1;








define_cryptographic_material! (pub(crate) InternalDheKey, 32);
define_cryptographic_material! (pub(crate) InternalPqKey, 32);
define_cryptographic_material! (pub(crate) InternalPartialKey, 32);
define_cryptographic_material! (pub(crate) InternalAontKey, 32);
define_cryptographic_material! (pub(crate) InternalParametersHash, 32);

define_cryptographic_material! (pub(crate) InternalPacketSalt, 32);
define_cryptographic_material! (pub(crate) InternalPacketKey, 32);

define_cryptographic_material! (pub(crate) InternalEncryptionKey, 32);

define_cryptographic_material! (pub(crate) InternalAuthenticationKey, 32);
define_cryptographic_material! (pub(crate) InternalAuthenticationMac, 32);

define_cryptographic_material! (pub(crate) InternalAssociatedInput, input, slice);
define_cryptographic_material! (pub(crate) InternalAssociatedHash, 32);
define_cryptographic_material! (pub(crate) InternalAssociatedMerge, 32);

define_cryptographic_material! (pub(crate) InternalSecretInput, input, slice);
define_cryptographic_material! (pub(crate) InternalSecretHash, 32);
define_cryptographic_material! (pub(crate) InternalSecretMerge, 32);
define_cryptographic_material! (pub(crate) InternalSecretSalt, 32);
define_cryptographic_material! (pub(crate) InternalSecretArgon, 32);
define_cryptographic_material! (pub(crate) InternalSecretKey, 32);

define_cryptographic_material! (pub(crate) InternalPinInput, input, slice);
define_cryptographic_material! (pub(crate) InternalPinHash, 32);
define_cryptographic_material! (pub(crate) InternalPinMerge, 32);
define_cryptographic_material! (pub(crate) InternalPinSalt, 32);
define_cryptographic_material! (pub(crate) InternalPinArgon, 32);
define_cryptographic_material! (pub(crate) InternalPinKey, 32);

define_cryptographic_material! (pub(crate) InternalSeedInput, input, slice);
define_cryptographic_material! (pub(crate) InternalSeedHash, 32);
define_cryptographic_material! (pub(crate) InternalSeedMerge, 32);
define_cryptographic_material! (pub(crate) InternalSeedKey, 32);

define_cryptographic_material! (pub(crate) InternalBallastInput, input, slice);
define_cryptographic_material! (pub(crate) InternalBallastHash, 32);
define_cryptographic_material! (pub(crate) InternalBallastMerge, 32);
define_cryptographic_material! (pub(crate) InternalBallastSalt, 32);
define_cryptographic_material! (pub(crate) InternalBallastArgon, 32);
define_cryptographic_material! (pub(crate) InternalBallastKey, 32);

define_cryptographic_material! (pub(crate) InternalOracleHandle, 32);
define_cryptographic_material! (pub(crate) InternalOracleMerge, 32);
define_cryptographic_material! (pub(crate) InternalOracleSorter, 32);
define_cryptographic_material! (pub(crate) InternalOracleInput, 32);
define_cryptographic_material! (pub(crate) InternalOracleOutput, 32);
define_cryptographic_material! (pub(crate) InternalOracleKey, 32);

define_cryptographic_material! (pub(crate) InternalDecryptedData, input, slice);
define_cryptographic_material! (pub(crate) InternalEncryptedData, input, slice);

define_cryptographic_material! (pub(crate) InternalPasswordData, input, slice);
define_cryptographic_material! (pub(crate) InternalPasswordOutput, 32);




define_cryptographic_purpose! (pub(crate) CRYPTO_ENCRYPTION_SCHEMA_V1, encryption, schema_v1);
define_cryptographic_purpose! (pub(crate) CRYPTO_PASSWORD_SCHEMA_V1, password, schema_v1);

define_cryptographic_purpose! (pub(crate) CRYPTO_PARAMETERS_HASH_PURPOSE, encryption, parameters_hash);

define_cryptographic_purpose! (pub(crate) CRYPTO_DHE_PUBLIC_MERGE_PURPOSE, encryption, dhe_public_merge);
define_cryptographic_purpose! (pub(crate) CRYPTO_DHE_SHARED_MERGE_PURPOSE, encryption, dhe_shared_merge);
define_cryptographic_purpose! (pub(crate) CRYPTO_DHE_KEY_PURPOSE, encryption, dhe_key);

define_cryptographic_purpose! (pub(crate) CRYPTO_PARTIAL_KEY_PURPOSE, encryption, partial_key);
define_cryptographic_purpose! (pub(crate) CRYPTO_AONT_KEY_PURPOSE, encryption, aont_key);

define_cryptographic_purpose! (pub(crate) CRYPTO_PACKET_SALT_PURPOSE, encryption, packet_salt);
define_cryptographic_purpose! (pub(crate) CRYPTO_PACKET_KEY_PURPOSE, encryption, packet_key);
define_cryptographic_purpose! (pub(crate) CRYPTO_ENCRYPTION_KEY_PURPOSE, encryption, encryption_key);
define_cryptographic_purpose! (pub(crate) CRYPTO_AUTHENTICATION_KEY_PURPOSE, encryption, authentication_key);

define_cryptographic_purpose! (pub(crate) CRYPTO_ASSOCIATED_HASH_PURPOSE, encryption, associated_hash);
define_cryptographic_purpose! (pub(crate) CRYPTO_ASSOCIATED_MERGE_PURPOSE, encryption, associated_merge);

define_cryptographic_purpose! (pub(crate) CRYPTO_SECRET_HASH_PURPOSE, encryption, secret_hash);
define_cryptographic_purpose! (pub(crate) CRYPTO_SECRET_MERGE_PURPOSE, encryption, secret_merge);
define_cryptographic_purpose! (pub(crate) CRYPTO_SECRET_SALT_PURPOSE, encryption, secret_salt);
define_cryptographic_purpose! (pub(crate) CRYPTO_SECRET_KEY_PURPOSE, encryption, secret_key);

define_cryptographic_purpose! (pub(crate) CRYPTO_PIN_HASH_PURPOSE, encryption, pin_hash);
define_cryptographic_purpose! (pub(crate) CRYPTO_PIN_MERGE_PURPOSE, encryption, pin_merge);
define_cryptographic_purpose! (pub(crate) CRYPTO_PIN_SALT_PURPOSE, encryption, pin_salt);
define_cryptographic_purpose! (pub(crate) CRYPTO_PIN_KEY_PURPOSE, encryption, pin_key);

define_cryptographic_purpose! (pub(crate) CRYPTO_SEED_HASH_PURPOSE, encryption, seed_hash);
define_cryptographic_purpose! (pub(crate) CRYPTO_SEED_MERGE_PURPOSE, encryption, seed_merge);
define_cryptographic_purpose! (pub(crate) CRYPTO_SEED_KEY_PURPOSE, encryption, seed_key);

define_cryptographic_purpose! (pub(crate) CRYPTO_BALLAST_HASH_PURPOSE, encryption, ballast_hash);
define_cryptographic_purpose! (pub(crate) CRYPTO_BALLAST_MERGE_PURPOSE, encryption, ballast_merge);
define_cryptographic_purpose! (pub(crate) CRYPTO_BALLAST_SALT_PURPOSE, encryption, ballast_salt);
define_cryptographic_purpose! (pub(crate) CRYPTO_BALLAST_KEY_PURPOSE, encryption, ballast_key);

define_cryptographic_purpose! (pub(crate) CRYPTO_ORACLE_MERGE_PURPOSE, encryption, oracle_merge);
define_cryptographic_purpose! (pub(crate) CRYPTO_ORACLE_SORTER_PURPOSE, encryption, oracle_sorter);
define_cryptographic_purpose! (pub(crate) CRYPTO_ORACLE_INPUT_PURPOSE, encryption, oracle_input);
define_cryptographic_purpose! (pub(crate) CRYPTO_ORACLE_KEY_PURPOSE, encryption, oracle_key);

define_cryptographic_purpose! (pub(crate) CRYPTO_PASSWORD_SALT_PURPOSE, password, salt);
define_cryptographic_purpose! (pub(crate) CRYPTO_PASSWORD_OUTPUT_PURPOSE, password, output);


