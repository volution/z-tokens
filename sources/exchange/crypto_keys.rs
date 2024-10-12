

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use crate::coding::*;
use crate::oracles::*;


use ::z_tokens_runtime::{
		sensitive::drop,
	};


use ::z_tokens_runtime_crypto::{
		
		blake3_hash,
		blake3_hash_join,
		blake3_keyed_hash,
		
		generate_random,
		
		CryptographicMaterial as _,
		CryptographicInput as _,
	};


use ::z_tokens_runtime_crypto::crates::{
		
		x25519,
	};


use super::definitions::*;
use super::utilities::*;








pub(crate) fn derive_keys_phase_1 <'a> (
			_schema : &'static str,
			_private_keys : Vec<&x25519::StaticSecret>,
			_public_keys : Vec<&x25519::PublicKey>,
			_associated_inputs : Vec<InternalAssociatedInput<'a>>,
			_secret_inputs : Vec<InternalSecretInput<'a>>,
			_pin_inputs : Vec<InternalPinInput<'a>>,
			_seed_inputs : Vec<InternalSeedInput<'a>>,
			_ballast_inputs : Vec<InternalBallastInput<'a>>,
			_oracle_handles : Vec<InternalOracleHandle>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&str>,
			_encryption : bool,
		) -> CryptoResult<(
			InternalPartialKey,
			InternalAontKey,
			(InternalSecretMerge, Vec<InternalSecretInput<'a>>),
			(InternalPinMerge, Vec<InternalPinInput<'a>>),
			(InternalSeedMerge, Vec<InternalSeedInput<'a>>),
			(InternalBallastMerge, Vec<InternalBallastInput<'a>>),
			InternalOracleMerge,
			InternalOracleSorter,
			NonZeroU64,
		)>
{
	let _derivation_loops = _derivation_loops.map (NonZeroU64::get) .unwrap_or (1);
	let _derivation_loops_0 = NonZeroU64::new (_derivation_loops) .infallible (0x794c53db);
	
	let _namespace = _namespace.unwrap_or ("");
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive parameters hash...
	
	let _parameters_hash = blake3_hash (
			InternalParametersHash::wrap,
			CRYPTO_PARAMETERS_HASH_PURPOSE,
			&[],
			&[
				_schema.as_bytes (),
				& encode_u64_into (_derivation_loops),
				_namespace.as_bytes (),
			],
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive associated hashes...
	
	let mut _associated_hashes : Vec<_> = _associated_inputs.into_iter () .map (
			|_associated_input|
					blake3_hash (
							InternalAssociatedHash::wrap,
							CRYPTO_ASSOCIATED_HASH_PURPOSE,
							&[
								_parameters_hash.access (),
							],
							&[
								_associated_input.unwrap (),
							],
						)
		) .collect ();
	
	// NOTE:  associated data is not sorted or deduplicated, thus order is important!
	
	let _associated_merge = blake3_hash_join (
			InternalAssociatedMerge::wrap,
			CRYPTO_ASSOCIATED_MERGE_PURPOSE,
			_associated_hashes.iter () .map (InternalAssociatedHash::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive secret hashes...
	
	let mut _secret_pairs : Vec<_> = _secret_inputs.into_iter () .map (
			|_secret_input| {
					let _secret_hash = blake3_hash (
							InternalSecretHash::wrap,
							CRYPTO_SECRET_HASH_PURPOSE,
							&[
								_parameters_hash.access (),
							],
							&[
								_secret_input.access (),
							],
						);
					(_secret_hash, _secret_input)
				}
		) .collect ();
	
	_secret_pairs.sort_by (|_left, _right| InternalSecretHash::cmp_access (&_left.0, &_right.0));
	_secret_pairs.dedup_by (|_left, _right| InternalSecretHash::eq_access (&_left.0, &_right.0));
	
	let _secret_merge = blake3_hash_join (
			InternalSecretMerge::wrap,
			CRYPTO_SECRET_MERGE_PURPOSE,
			_secret_pairs.iter () .map (|_pair| &_pair.0) .map (InternalSecretHash::access),
		);
	
	let _secret_inputs = _secret_pairs.into_iter () .map (|_pair| _pair.1) .collect::<Vec<_>> ();
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive pin hashes...
	
	let mut _pin_pairs : Vec<_> = _pin_inputs.into_iter () .map (
			|_pin_input| {
					let _pin_hash = blake3_hash (
							InternalPinHash::wrap,
							CRYPTO_PIN_HASH_PURPOSE,
							&[
								_parameters_hash.access (),
							],
							&[
								_pin_input.access (),
							],
						);
					(_pin_hash, _pin_input)
				}
		) .collect ();
	
	_pin_pairs.sort_by (|_left, _right| InternalPinHash::cmp_access (&_left.0, &_right.0));
	_pin_pairs.dedup_by (|_left, _right| InternalPinHash::eq_access (&_left.0, &_right.0));
	
	let _pin_merge = blake3_hash_join (
			InternalPinMerge::wrap,
			CRYPTO_PIN_MERGE_PURPOSE,
			_pin_pairs.iter () .map (|_pair| &_pair.0) .map (InternalPinHash::access),
		);
	
	let _pin_inputs = _pin_pairs.into_iter () .map (|_pair| _pair.1) .collect::<Vec<_>> ();
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive seed hashes...
	
	let mut _seed_pairs : Vec<_> = _seed_inputs.into_iter () .map (
			|_seed_input| {
					let _seed_hash = blake3_hash (
							InternalSeedHash::wrap,
							CRYPTO_SEED_HASH_PURPOSE,
							&[
								_parameters_hash.access (),
							],
							&[
								_seed_input.access (),
							],
						);
					(_seed_hash, _seed_input)
				}
		) .collect ();
	
	_seed_pairs.sort_by (|_left, _right| InternalSeedHash::cmp_access (&_left.0, &_right.0));
	_seed_pairs.dedup_by (|_left, _right| InternalSeedHash::eq_access (&_left.0, &_right.0));
	
	let _seed_merge = blake3_hash_join (
			InternalSeedMerge::wrap,
			CRYPTO_SEED_MERGE_PURPOSE,
			_seed_pairs.iter () .map (|_pair| &_pair.0) .map (InternalSeedHash::access),
		);
	
	let _seed_inputs = _seed_pairs.into_iter () .map (|_pair| _pair.1) .collect::<Vec<_>> ();
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive ballast hashes...
	
	let mut _ballast_pairs : Vec<_> = _ballast_inputs.into_iter () .map (
			|_ballast_input| {
					let _ballast_hash = blake3_hash (
							InternalBallastHash::wrap,
							CRYPTO_BALLAST_HASH_PURPOSE,
							&[
								_parameters_hash.access (),
							],
							&[
								_ballast_input.access (),
							],
						);
					(_ballast_hash, _ballast_input)
				}
		) .collect ();
	
	_ballast_pairs.sort_by (|_left, _right| InternalBallastHash::cmp_access (&_left.0, &_right.0));
	_ballast_pairs.dedup_by (|_left, _right| InternalBallastHash::eq_access (&_left.0, &_right.0));
	
	let _ballast_merge = blake3_hash_join (
			InternalBallastMerge::wrap,
			CRYPTO_BALLAST_MERGE_PURPOSE,
			_ballast_pairs.iter () .map (|_pair| &_pair.0) .map (InternalBallastHash::access),
		);
	
	let _ballast_inputs = _ballast_pairs.into_iter () .map (|_pair| _pair.1) .collect::<Vec<_>> ();
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive oracle hashes...
	
	let _oracle_merge = blake3_hash_join (
			InternalOracleMerge::wrap,
			CRYPTO_ORACLE_MERGE_PURPOSE,
			_oracle_handles.iter () .map (InternalOracleHandle::access),
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive X25519 key...
	
	let _dhe_key = match (_private_keys.len (), _public_keys.len ()) {
		
		(0, 0) =>
			if _associated_hashes.is_empty () && _secret_inputs.is_empty () && _pin_inputs.is_empty () && _seed_inputs.is_empty () && _ballast_inputs.is_empty () && _oracle_handles.is_empty () {
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
			
			let _private_public_merge = blake3_hash_join (
					|_hash| _hash,
					CRYPTO_DHE_PUBLIC_MERGE_PURPOSE,
					_private_public_keys.iter () .map (|_key| _key.as_bytes ()),
				);
			let _public_merge = blake3_hash_join (
					|_hash| _hash,
					CRYPTO_DHE_PUBLIC_MERGE_PURPOSE,
					_public_keys.iter () .map (|_key| _key.as_bytes ()),
				);
			
			let (_senders_merge, _recipients_merge) = if _encryption {
				(_private_public_merge, _public_merge)
			} else {
				(_public_merge, _private_public_merge)
			};
			
			let _shared_merge = blake3_hash_join (
					|_hash| _hash,
					CRYPTO_DHE_SHARED_MERGE_PURPOSE,
					_shared.iter (),
				);
			
			let _dhe_key = blake3_hash (
					InternalDheKey::wrap,
					CRYPTO_DHE_KEY_PURPOSE,
					&[
						_parameters_hash.access (),
						&_senders_merge,
						&_recipients_merge,
						&_shared_merge,
					],
					&[],
				);
			
			_dhe_key
		}
	};
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive partial key (for the entire transaction)...
	
	let _partial_key = blake3_hash (
			InternalPartialKey::wrap,
			CRYPTO_PARTIAL_KEY_PURPOSE,
			&[
				_parameters_hash.access (),
				_associated_merge.access (),
				_oracle_merge.access (),
				_ballast_merge.access (),
				_seed_merge.access (),
				_dhe_key.access (),
				_secret_merge.access (),
				_pin_merge.access (),
			],
			&[],
		);
	
	drop! (_associated_merge);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive AONT key...
	
	let _aont_key = blake3_hash (
			InternalAontKey::wrap,
			CRYPTO_AONT_KEY_PURPOSE,
			&[
				_partial_key.access (),
			],
			&[],
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive oracles sorter...
	
	let _oracle_sorter = blake3_hash (
			InternalOracleSorter::wrap,
			CRYPTO_ORACLE_SORTER_PURPOSE,
			&[
				_partial_key.access (),
			],
			&[],
		);
	
	// --------------------------------------------------------------------------------
	
	Ok ((
			_partial_key,
			_aont_key,
			(_secret_merge, _secret_inputs),
			(_pin_merge, _pin_inputs),
			(_seed_merge, _seed_inputs),
			(_ballast_merge, _ballast_inputs),
			_oracle_merge, _oracle_sorter,
			_derivation_loops_0,
		))
}








pub(crate) fn derive_keys_phase_2 (
			_schema : &'static str,
			_partial_key : InternalPartialKey,
			_packet_salt : &InternalPacketSalt,
			_secret_pairs : (InternalSecretMerge, Vec<InternalSecretInput>),
			_pin_pairs : (InternalPinMerge, Vec<InternalPinInput>),
			_seed_pairs : (InternalSeedMerge, Vec<InternalSeedInput>),
			_ballast_pairs : (InternalBallastMerge, Vec<InternalBallastInput>),
			_oracles : (InternalOracleMerge, Vec<(&mut dyn Oracle, InternalOracleHandle)>),
			_derivation_loops : NonZeroU64,
		) -> CryptoResult<(
			InternalPacketKey,
			InternalEncryptionKey,
			InternalAuthenticationKey,
		)>
{
	let (_secret_merge, _secret_inputs) = _secret_pairs;
	let (_pin_merge, _pin_inputs) = _pin_pairs;
	let (_seed_merge, _seed_inputs) = _seed_pairs;
	let (_ballast_merge, _ballast_inputs) = _ballast_pairs;
	let (_oracle_merge, mut _oracles) = _oracles;
	
	// --------------------------------------------------------------------------------
	// NOTE:  initialize keys...
	
	let mut _packet_key = InternalPacketKey::wrap (_partial_key.unwrap ());
	let mut _oracle_key = InternalOracleOutput::wrap (_oracle_merge.unwrap ());
	let mut _ballast_key = InternalBallastKey::wrap (_ballast_merge.unwrap ());
	let mut _seed_key = InternalSeedKey::wrap (_seed_merge.unwrap ());
	let mut _secret_key = InternalSecretKey::wrap (_secret_merge.unwrap ());
	let mut _pin_key = InternalPinKey::wrap (_pin_merge.unwrap ());
	
	// --------------------------------------------------------------------------------
	// NOTE:  derivation loops...
	
	let _derivation_loops = _derivation_loops.get ();
	
	for _derivation_loop in 0 ..= _derivation_loops {
		
		// --------------------------------------------------------------------------------
		// NOTE:  derive packet key...
		
		_packet_key = blake3_hash (
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
			);
		
		if _derivation_loop == _derivation_loops {
			break;
		}
		
		// --------------------------------------------------------------------------------
		// NOTE:  call oracles...
		
		for (_oracle_wrapper, _oracle_handle) in _oracles.iter_mut () {
			
			let _oracle_input = blake3_hash (
					InternalOracleInput::wrap,
					CRYPTO_ORACLE_INPUT_PURPOSE,
					&[
						_packet_key.access (),
						_oracle_key.access (),
						_oracle_handle.access (),
					],
					&[],
				);
			
			let mut _oracle_output = InternalOracleOutput::zero ();
			_oracle_wrapper.derive (Some (_schema), _oracle_input.access (), _oracle_output.access_mut ()) .else_wrap (0xcc07e95e) ?;
			
			_oracle_key = blake3_hash (
					InternalOracleOutput::wrap,
					CRYPTO_ORACLE_KEY_PURPOSE,
					&[
						_oracle_input.access (),
						_oracle_output.access (),
					],
					&[],
				);
		}
		
		// --------------------------------------------------------------------------------
		// NOTE:  derive ballast argon hashes...
		
		let _ballast_count = _ballast_inputs.len ();
		
		for _ballast_input in _ballast_inputs.iter () {
			
			let _ballast_salt = blake3_hash (
					InternalBallastSalt::wrap,
					CRYPTO_BALLAST_SALT_PURPOSE,
					&[
						_packet_key.access (),
						_oracle_key.access (),
						_ballast_key.access (),
					],
					&[],
				);
			
			let _ballast_hash = blake3_keyed_hash (
					InternalBallastHash::wrap,
					_ballast_salt.access (),
					&[],
					&[
						_ballast_input.access (),
					]
				);
			
			let _ballast_argon = apply_argon_ballast (&_ballast_hash, &_ballast_salt, _ballast_count) ?;
			
			_ballast_key = blake3_hash (
					InternalBallastKey::wrap,
					CRYPTO_BALLAST_KEY_PURPOSE,
					&[
						_ballast_salt.access (),
						_ballast_hash.access (),
						_ballast_argon.access (),
					],
					&[],
				);
		}
		
		// --------------------------------------------------------------------------------
		// NOTE:  derive seed argon hashes...
		
		for _seed_input in _seed_inputs.iter () {
			
			let _seed_salt = blake3_hash (
					InternalSeedSalt::wrap,
					CRYPTO_SEED_SALT_PURPOSE,
					&[
						_packet_key.access (),
						_oracle_key.access (),
						_seed_key.access (),
					],
					&[],
				);
			
			let _seed_hash = blake3_keyed_hash (
					InternalSeedHash::wrap,
					_seed_salt.access (),
					&[],
					&[
						_seed_input.access (),
					]
				);
			
			_seed_key = blake3_hash (
					InternalSeedKey::wrap,
					CRYPTO_SEED_KEY_PURPOSE,
					&[
						_seed_salt.access (),
						_seed_hash.access (),
					],
					&[],
				);
		}
		
		// --------------------------------------------------------------------------------
		// NOTE:  derive secret argon hashes...
		
		for _secret_input in _secret_inputs.iter () {
			
			let _secret_salt = blake3_hash (
					InternalSecretSalt::wrap,
					CRYPTO_SECRET_SALT_PURPOSE,
					&[
						_packet_key.access (),
						_oracle_key.access (),
						_secret_key.access (),
					],
					&[],
				);
			
			let _secret_hash = blake3_keyed_hash (
					InternalSecretHash::wrap,
					_secret_salt.access (),
					&[],
					&[
						_secret_input.access (),
					]
				);
			
			let _secret_argon = apply_argon_secret (&_secret_hash, &_secret_salt) ?;
			
			_secret_key = blake3_hash (
					InternalSecretKey::wrap,
					CRYPTO_SECRET_KEY_PURPOSE,
					&[
						_secret_salt.access (),
						_secret_hash.access (),
						_secret_argon.access (),
					],
					&[],
				);
		}
		
		// --------------------------------------------------------------------------------
		// NOTE:  derive pin argon hashes...
		
		for _pin_input in _pin_inputs.iter () {
			
			let _pin_salt = blake3_hash (
					InternalPinSalt::wrap,
					CRYPTO_PIN_SALT_PURPOSE,
					&[
						_packet_key.access (),
						_oracle_key.access (),
						_pin_key.access (),
					],
					&[],
				);
			
			let _pin_hash = blake3_keyed_hash (
					InternalPinHash::wrap,
					_pin_salt.access (),
					&[],
					&[
						_pin_input.access (),
					]
				);
			
			let _pin_argon = apply_argon_pin (&_pin_hash, &_pin_salt) ?;
			
			_pin_key = blake3_hash (
					InternalPinKey::wrap,
					CRYPTO_PIN_KEY_PURPOSE,
					&[
						_pin_salt.access (),
						_pin_hash.access (),
						_pin_argon.access (),
					],
					&[],
				);
		}
		
		// --------------------------------------------------------------------------------
		// NOTE:  derive packet key...
		
		// NOTE:  This actually happens at the beginning of the next loop!
	}
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive encryption key...
	
	let _encryption_key = blake3_hash (
			InternalEncryptionKey::wrap,
			CRYPTO_ENCRYPTION_KEY_PURPOSE,
			&[
				_packet_key.access (),
			],
			&[],
		);
	
	// --------------------------------------------------------------------------------
	// NOTE:  derive authentication key...
	
	let _authentication_key = blake3_hash (
			InternalAuthenticationKey::wrap,
			CRYPTO_AUTHENTICATION_KEY_PURPOSE,
			&[
				_packet_key.access (),
			],
			&[],
		);
	
	// --------------------------------------------------------------------------------
	
	Ok ((_packet_key, _encryption_key, _authentication_key))
}








pub(crate) fn derive_packet_salt (
			_purpose : &'static str,
			_partial_key : &InternalPartialKey,
			_data : &[u8],
			_deterministic : bool,
		) -> CryptoResult<
			InternalPacketSalt
		>
{
	let _random_salt = if _deterministic {
			InternalPacketSalt::zero ()
		} else {
			generate_random (InternalPacketSalt::wrap)
		};
	
	let _packet_salt = blake3_hash (
			InternalPacketSalt::wrap,
			_purpose,
			&[
				_random_salt.access (),
				_partial_key.access (),
			],
			&[
				_data,
			],
		);
	
	Ok (_packet_salt)
}


