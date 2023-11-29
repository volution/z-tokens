

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


pub(crate) mod ssh {
	
	pub(crate) use ::russh_keys::{
		
			key::PublicKey,
			signature::Signature,
			agent::client::AgentClient,
			
			key::ED25519 as KEY_ED25519,
			key::SSH_RSA as KEY_RSA,
			
			key::ED25519 as SIG_ED25519,
			key::SSH_RSA as SIG_RSA_SHA1,
			key::RSA_SHA2_256 as SIG_RSA_SHA2_256,
			key::RSA_SHA2_512 as SIG_RSA_SHA2_512,
			
			key::SignatureHash,
			key::Name as AlgorithmName,
			
			PublicKeyBase64,
		};
}


use ::tokio::{
		runtime::{
			Runtime,
			Builder as RuntimeBuilder,
		},
		net::UnixStream,
	};


use ::z_tokens_runtime::{
		memory::Rb,
		sensitive::zeroize_and_drop,
		sensitive::drop,
	};

use crate::keys::{
		encode_raw,
		decode_raw_vec,
	};

use ::z_tokens_runtime::crypto::*;








define_error! (pub SshError, result : SshResult);




pub const SSH_WRAPPER_KEY_ENCODED_PREFIX : &str = "ztxws";


define_cryptographic_material! (InternalSshWrapInput, input, slice);
define_cryptographic_material! (InternalSshWrapKeyHash, 32);
define_cryptographic_material! (InternalSshWrapSchemaHash, 32);
define_cryptographic_material! (InternalSshWrapInputHash, 32);
define_cryptographic_material! (InternalSshWrapOutputHash, 32);


define_cryptographic_purpose! (SSH_WRAP_SCHEMA_UNDEFINED, ssh_wrap, schema_undefined);
define_cryptographic_purpose! (SSH_WRAP_KEY_HASH_PURPOSE, ssh_wrap, key_hash);
define_cryptographic_purpose! (SSH_WRAP_INPUT_HASH_PURPOSE, ssh_wrap, input_hash);
define_cryptographic_purpose! (SSH_WRAP_OUTPUT_HASH_PURPOSE, ssh_wrap, output_hash);








pub struct SshWrapper {
	key : SshWrapperKey,
	agent : SshWrapperAgent,
}




pub struct SshWrapperAgent {
	runtime : Runtime,
	client : Option<ssh::AgentClient<UnixStream>>,
}




pub struct SshWrapperKey (Rb<SshWrapperKeyInternals>);


struct SshWrapperKeyInternals {
	
	key_algorithm : KeyAlgorithm,
	signature_algorithm : SignatureAlgorithm,
	
	public_key : ssh::PublicKey,
	public_key_bytes : Vec<u8>,
	
	key_hash : InternalSshWrapKeyHash,
}




#[ derive (Copy, Clone, Eq, PartialEq) ]
#[ allow (non_camel_case_types) ]
pub(crate) enum KeyAlgorithm {
	Ed25519,
	RSA,
}


#[ derive (Copy, Clone, Eq, PartialEq) ]
#[ allow (non_camel_case_types) ]
pub(crate) enum SignatureAlgorithm {
	Ed25519,
	RSA_SHA1,
	RSA_SHA2_256,
	RSA_SHA2_512,
}


impl KeyAlgorithm {
	
	pub fn identifier (&self) -> &'static str {
		match self {
			KeyAlgorithm::Ed25519 => "key-SSH-Ed25519",
			KeyAlgorithm::RSA => "key-SSH-RSA",
		}
	}
	
	#[ allow (dead_code) ]
	pub fn ssh_name (&self) -> &'static str {
		match self {
			KeyAlgorithm::Ed25519 => ssh::KEY_ED25519.as_ref (),
			KeyAlgorithm::RSA => ssh::KEY_RSA.as_ref (),
		}
	}
}


impl SignatureAlgorithm {
	
	pub fn identifier (&self) -> &'static str {
		match self {
			SignatureAlgorithm::Ed25519 => "sig-SSH-Ed25519",
			SignatureAlgorithm::RSA_SHA1 => "sig-SSH-RSA-SHA1",
			SignatureAlgorithm::RSA_SHA2_256 => "sig-SSH-RSA-SHA2-256",
			SignatureAlgorithm::RSA_SHA2_512 => "sig-SSH-RSA-SHA2-512",
		}
	}
	
	#[ allow (dead_code) ]
	pub fn ssh_name (&self) -> &'static str {
		match self {
			SignatureAlgorithm::Ed25519 => ssh::SIG_ED25519.as_ref (),
			SignatureAlgorithm::RSA_SHA1 => ssh::SIG_RSA_SHA1.as_ref (),
			SignatureAlgorithm::RSA_SHA2_256 => ssh::SIG_RSA_SHA2_256.as_ref (),
			SignatureAlgorithm::RSA_SHA2_512 => ssh::SIG_RSA_SHA2_512.as_ref (),
		}
	}
}








impl SshWrapper {
	
	
	pub fn new (_key : SshWrapperKey, _agent : SshWrapperAgent) -> SshResult<SshWrapper> {
		let _key = _key.into ();
		let _self = SshWrapper {
				key : _key,
				agent : _agent,
			};
		Ok (_self)
	}
	
	
	pub fn into_agent (self) -> SshResult<SshWrapperAgent> {
		Ok (self.agent)
	}
	
	
	pub fn connect (_key : SshWrapperKey) -> SshResult<SshWrapper> {
		
		let _agent = SshWrapperAgent::connect () ?;
		Self::new (_key, _agent)
	}
	
	
	pub fn wrap (&mut self, _schema : Option<&'static str>, _input : &[u8], _output : &mut [u8; 32]) -> SshResult {
		
		let _input = InternalSshWrapInput::wrap (_input);
		
		let _key = self.key.0.deref ();
		
		let _key_hash = &_key.key_hash;
		
		let _schema_hash = blake3_derive_key (
				InternalSshWrapSchemaHash::wrap,
				_schema.unwrap_or (SSH_WRAP_SCHEMA_UNDEFINED),
				&[],
				&[],
				None,
			);
		
		let _input_hash = blake3_derive_key (
				InternalSshWrapInputHash::wrap,
				SSH_WRAP_INPUT_HASH_PURPOSE,
				&[
					_schema_hash.access (),
					_key_hash.access (),
				],
				&[
					_input.access (),
				],
				None,
			);
		
		let _outcome = {
			
			let _runtime = &mut self.agent.runtime;
			let mut _client = self.agent.client.take () .else_wrap (0xa5bc5a47) ?;
			
			let (_client, _outcome) = self.agent.runtime.block_on (async {
					_client.sign_request_signature (&_key.public_key, _input_hash.access ()) .await
				});
			
			self.agent.client = Some (_client);
			
			_outcome
		};
		
		let _signature = _outcome.else_wrap (0xe3badadd) ?;
		
		let (_signature_algorithm, _signature_bytes) = match _signature {
			ssh::Signature::Ed25519 (ref _bytes) =>
				(SignatureAlgorithm::Ed25519, _bytes.0.as_slice ()),
			ssh::Signature::RSA { hash : _hash, bytes : ref _bytes } =>
				match _hash {
					ssh::SignatureHash::SHA1 =>
						(SignatureAlgorithm::RSA_SHA1, _bytes.as_slice ()),
					ssh::SignatureHash::SHA2_256 =>
						(SignatureAlgorithm::RSA_SHA2_256, _bytes.as_slice ()),
					ssh::SignatureHash::SHA2_512 =>
						(SignatureAlgorithm::RSA_SHA2_512, _bytes.as_slice ()),
				}
		};
		
		if ! _key.public_key.verify_detached (_input_hash.access (), &_signature_bytes) {
			fail! (0x8fc8e73a);
		}
		
		if _signature_algorithm != _key.signature_algorithm {
			fail! (0x71bd7922);
		}
		
		drop! (_key);
		
		let _output_hash = blake3_derive_key (
				InternalSshWrapOutputHash::wrap,
				SSH_WRAP_OUTPUT_HASH_PURPOSE,
				&[
					_schema_hash.access (),
					_key_hash.access (),
					_input_hash.access (),
				],
				&[
					&_signature_bytes,
				],
				None,
			);
		
		_output.copy_from_slice (_output_hash.access ());
		
		Ok (())
	}
	
	
	pub fn handle (&self) -> SshResult<&[u8; 32]> {
		self.key.handle ()
	}
	
	
	pub fn cmp_by_keys (_left : &Self, _right : &Self) -> Ordering {
		
		let _left = _left.key.0.deref ();
		let _right = _right.key.0.deref ();
		
		Ordering::Equal
				.then_with (|| Ord::cmp (_left.key_algorithm.identifier (), _right.key_algorithm.identifier ()))
				.then_with (|| Ord::cmp (_left.signature_algorithm.identifier (), _right.signature_algorithm.identifier ()))
				.then_with (|| Ord::cmp (&_left.public_key_bytes, &_right.public_key_bytes))
	}
}








impl SshWrapperKey {
	
	
	pub fn encode (&self) -> SshResult<Rb<String>> {
		
		let _key = &self.0;
		
		let _key_algorithm_code = match _key.key_algorithm {
			KeyAlgorithm::Ed25519 => 1,
			KeyAlgorithm::RSA => 2,
		};
		
		let _signature_algorithm_code = match _key.signature_algorithm {
			SignatureAlgorithm::Ed25519 => 1,
			SignatureAlgorithm::RSA_SHA1 => 2,
			SignatureAlgorithm::RSA_SHA2_256 => 3,
			SignatureAlgorithm::RSA_SHA2_512 => 4,
		};
		
		let mut _serialized = Vec::with_capacity (_key.public_key_bytes.len () + 2);
		_serialized.extend_from_slice (&_key.public_key_bytes);
		_serialized.push (_key_algorithm_code);
		_serialized.push (_signature_algorithm_code);
		
		encode_raw (SSH_WRAPPER_KEY_ENCODED_PREFIX, &_serialized) .else_wrap (0xc28cbe1d)
	}
	
	
	pub fn decode_and_zeroize (_string : String) -> SshResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> SshResult<Self> {
		
		let mut _serialized = decode_raw_vec (SSH_WRAPPER_KEY_ENCODED_PREFIX, _string) .else_wrap (0x142d169b) ?;
		
		let _signature_algorithm_code = _serialized.pop () .else_wrap (0x3837da48) ?;
		let _key_algorithm_code = _serialized.pop () .else_wrap (0x428e8a31) ?;
		
		let _key_algorithm = match _key_algorithm_code {
			1 => KeyAlgorithm::Ed25519,
			2 => KeyAlgorithm::RSA,
			_ => fail! (0x561c2e7f),
		};
		
		let _signature_algorithm = match _signature_algorithm_code {
			1 => SignatureAlgorithm::Ed25519,
			2 => SignatureAlgorithm::RSA_SHA1,
			3 => SignatureAlgorithm::RSA_SHA2_256,
			4 => SignatureAlgorithm::RSA_SHA2_512,
			_ => fail! (0xacbf3148),
		};
		
		let _public_key = parse_public_key (&_key_algorithm, &_signature_algorithm, &_serialized) ?;
		
		let _key_hash = key_hash (&_key_algorithm, &_signature_algorithm, &_serialized);
		
		let _wrapper_key = SshWrapperKeyInternals {
				key_algorithm : _key_algorithm,
				signature_algorithm : _signature_algorithm,
				public_key_bytes : _serialized,
				public_key : _public_key,
				key_hash : _key_hash,
			};
		
		Ok (SshWrapperKey (Rb::new (_wrapper_key)))
	}
	
	
	pub fn handle (&self) -> SshResult<&[u8; 32]> {
		let _key = &self.0;
		Ok (_key.key_hash.access ())
	}
	
	
	pub fn description (&self) -> SshResult<Rb<String>> {
		let _key = &self.0;
		let _ssh_fingerprint = _key.public_key.fingerprint ();
		let _handle = format! ("[{}:{}:{}]", _key.key_algorithm.identifier (), _key.signature_algorithm.identifier (), _ssh_fingerprint);
		zeroize_and_drop (_ssh_fingerprint);
		Ok (Rb::new (_handle))
	}
}








impl SshWrapperAgent {
	
	
	pub fn connect () -> SshResult<SshWrapperAgent> {
		
		let mut _runtime =
				RuntimeBuilder::new_current_thread ()
				.enable_io ()
				.build ()
				.else_wrap (0x2159c9b5) ?;
		
		let mut _client = _runtime.block_on (async {
				ssh::AgentClient::connect_env () .await
			}) .else_wrap (0x67dc4fca) ?;
		
		let _wrapper = SshWrapperAgent {
				runtime : _runtime,
				client : Some (_client),
			};
		
		Ok (_wrapper)
	}
	
	
	pub fn keys (&mut self) -> SshResult<Vec<SshWrapperKey>> {
		
		let mut _client = self.client.take () .else_wrap (0xbe88e692) ?;
		
		let _public_keys = self.runtime.block_on (async {
				_client.request_identities () .await
			}) .else_wrap (0xbad56d04) ?;
		
		self.client = Some (_client);
		
		let mut _wrapper_keys = Vec::with_capacity (_public_keys.len ());
		
		for _public_key in _public_keys.into_iter () {
			
			let Some ((_key_algorithm, _signature_algorithms, _public_key_bytes)) = deconstruct_public_key (&_public_key)
				else {
					continue
				};
			
			for _signature_algorithm in _signature_algorithms.into_iter () {
				
				// NOTE:  (See the comment in `parse_public_key`!)
				let _public_key = parse_public_key (&_key_algorithm, &_signature_algorithm, &_public_key_bytes) ?;
				
				let _key_hash = key_hash (&_key_algorithm, &_signature_algorithm, &_public_key_bytes);
				
				let _wrapper_key = SshWrapperKeyInternals {
						
						key_algorithm : _key_algorithm.clone (),
						signature_algorithm : _signature_algorithm,
						
						public_key_bytes : _public_key_bytes.clone (),
						public_key : _public_key,
						
						key_hash : _key_hash,
					};
				
				let _wrapper_key = SshWrapperKey (Rb::new (_wrapper_key));
				
				_wrapper_keys.push (_wrapper_key);
			}
		}
		
		Ok (_wrapper_keys)
	}
}








fn key_hash (_key_algorithm : &KeyAlgorithm, _signature_algorithm : &SignatureAlgorithm, _public_key_bytes : &[u8]) -> InternalSshWrapKeyHash {
	
	let _key_hash = blake3_derive_key (
			InternalSshWrapKeyHash::wrap,
			SSH_WRAP_KEY_HASH_PURPOSE,
			&[],
			&[
				_key_algorithm.identifier () .as_bytes (),
				_signature_algorithm.identifier () .as_bytes (),
				_public_key_bytes,
			],
			None,
		);
	
	_key_hash
}








fn deconstruct_public_key (_public_key : &ssh::PublicKey) -> Option<(KeyAlgorithm, Vec<SignatureAlgorithm>, Vec<u8>)> {
	
	let (_key_algorithm, _signature_algorithms) = match ssh::AlgorithmName (_public_key.name ()) {
			
			ssh::SIG_ED25519 =>
				(KeyAlgorithm::Ed25519, vec![ SignatureAlgorithm::Ed25519 ]),
			
			// NOTE:  The `russh` library lies about the algorithm name... The public key is always `ssh-rsa`!
			ssh::SIG_RSA_SHA1 | ssh::SIG_RSA_SHA2_256 | ssh::SIG_RSA_SHA2_512 =>
				(KeyAlgorithm::RSA, vec! [
						SignatureAlgorithm::RSA_SHA1,
						SignatureAlgorithm::RSA_SHA2_256,
						SignatureAlgorithm::RSA_SHA2_512,
					]),
			
			_ =>
				return None,
		};
	
	let _public_key_bytes = ssh::PublicKeyBase64::public_key_bytes (_public_key);
	
	Some ((_key_algorithm, _signature_algorithms, _public_key_bytes))
}




fn parse_public_key (_key_algorithm : &KeyAlgorithm, _signature_algorithm : &SignatureAlgorithm, _bytes : &[u8]) -> SshResult<ssh::PublicKey> {
	
	// NOTE:  Although the key doesn't contain any signature related information,
	//        the `russh` library uses that to control the `ssh-agent` signature request...
	
	let _key = ssh::PublicKey::parse (_signature_algorithm.ssh_name () .as_bytes (), &_bytes) .else_wrap (0x536edcf6) ?;
	
	Ok (_key)
}


