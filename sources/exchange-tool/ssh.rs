

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use ::russh_keys::{
		key::PublicKey as SshPublicKey,
		signature::Signature as SshSignature,
		agent::client::AgentClient as SshAgentClient,
		PublicKeyBase64 as _,
	};


use ::tokio::{
		runtime::{
			Runtime,
			Builder as RuntimeBuilder,
		},
		net::UnixStream,
	};


use ::z_tokens_runtime::{
		memory::Rb,
		memory::RbList,
		sensitive::Sensitive,
		sensitive::SensitiveIgnored,
		sensitive::zeroize_and_drop,
	};

use crate::keys::{
		encode_raw,
		decode_raw_vec,
	};








define_error! (pub SshError, result : SshResult);




pub const SSH_WRAPPER_KEY_ENCODED_PREFIX : &str = "ztxws";


const SSH_WRAP_KEY_HASH_CONTEXT : &str = "z-tokens exchange ssh wrap key hash (2023a)";
const SSH_WRAP_INPUT_HASH_CONTEXT : &str = "z-tokens exchange ssh wrap input hash (2023a)";
const SSH_WRAP_OUTPUT_HASH_CONTEXT : &str = "z-tokens exchange ssh wrap output hash (2023a)";




pub struct SshWrapper {
	key : Rb<SshWrapperKey>,
	agent : SshWrapperAgent,
}


pub struct SshWrapperKey {
	public_key : Rb<SensitiveIgnored<SshPublicKey>>,
}


pub struct SshWrapperAgent {
	runtime : Runtime,
	client : Option<SshAgentClient<UnixStream>>,
}








impl SshWrapper {
	
	
	pub fn new (_key : impl Into<Rb<SshWrapperKey>>, _agent : SshWrapperAgent) -> SshResult<SshWrapper> {
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
	
	
	pub fn connect (_key : impl Into<Rb<SshWrapperKey>>) -> SshResult<SshWrapper> {
		
		let _agent = SshWrapperAgent::connect () ?;
		Self::new (_key, _agent)
	}
	
	
	pub fn wrap (&mut self, _input : &[u8], _output : &mut [u8; 32]) -> SshResult {
		
		let _key = &self.key.public_key.0;
		let _key_algorithm = _key.name ();
		let _key_serialized = _key.public_key_bytes ();
		
		let _runtime = &mut self.agent.runtime;
		let mut _client = self.agent.client.take () .else_wrap (0xa5bc5a47) ?;
		
		let _key_hash : [u8; 32] =
				::blake3::Hasher::new_derive_key (SSH_WRAP_KEY_HASH_CONTEXT)
				.update (_key_algorithm.as_bytes ())
				.update (&[0u8])
				.update (&_key_serialized)
				.finalize ()
				.into ();
		
		let _input_hash : [u8; 32] =
				::blake3::Hasher::new_derive_key (SSH_WRAP_INPUT_HASH_CONTEXT)
				.update (&_key_hash)
				.update (_input)
				.finalize ()
				.into ();
		
		let (_client, _outcome) = self.agent.runtime.block_on (async {
				_client.sign_request_signature (_key, &_input_hash) .await
			});
		
		self.agent.client = Some (_client);
		
		let _signature = _outcome.else_wrap (0xe3badadd) ?;
		let _signature : &[u8] = match _signature {
			SshSignature::Ed25519 (ref _bytes) => &_bytes.0,
			SshSignature::RSA { hash : _, bytes : ref _bytes } => &_bytes,
		};
		
		if ! _key.verify_detached (&_input_hash, &_signature) {
			fail! (0x8fc8e73a);
		}
		
		let _output_hash : [u8; 32] =
				::blake3::Hasher::new_derive_key (SSH_WRAP_OUTPUT_HASH_CONTEXT)
				.update (&_key_hash)
				.update (&_input_hash)
				.update (&_signature)
				.finalize ()
				.into ();
		
		_output.copy_from_slice (&_output_hash);
		
		Ok (())
	}
}








impl SshWrapperKey {
	
	
	pub fn encode (&self) -> SshResult<Rb<String>> {
		
		let _key : &SshPublicKey = &self.public_key.0;
		
		let _algorithm = _key.name ();
		let mut _serialized = _key.public_key_bytes ();
		
		let _code = match _algorithm {
			"ssh-ed25519" => 1,
			"ssh-rsa" => 2,
			_rsa if _rsa.starts_with ("rsa-") => 2,
			_ => fail! (0x3cb34ad5),
		};
		
		_serialized.push (_code);
		
		encode_raw (SSH_WRAPPER_KEY_ENCODED_PREFIX, &_serialized) .else_wrap (0xc28cbe1d)
	}
	
	
	pub fn decode_and_zeroize (_string : String) -> SshResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> SshResult<Self> {
		
		let mut _serialized = decode_raw_vec (SSH_WRAPPER_KEY_ENCODED_PREFIX, _string) .else_wrap (0x142d169b) ?;
		let _code = _serialized.pop () .else_wrap (0x428e8a31) ?;
		
		let _algorithm = match _code {
			1 => "ssh-ed25519",
			2 => "ssh-rsa",
			_ => fail! (0xaec55807),
		};
		
		let _key = SshPublicKey::parse (_algorithm.as_bytes (), &_serialized) .else_wrap (0x536edcf6) ?;
		
		let _wrapper_key = SshWrapperKey {
				public_key : Rb::new (_key.clone () .into ()),
			};
		
		Ok (_wrapper_key)
	}
	
	
	pub fn handle (&self) -> SshResult<Rb<String>> {
		let _key : &SshPublicKey = &self.public_key.0;
		let _algorithm = _key.name ();
		let _fingerprint = _key.fingerprint ();
		let _handle = format! ("{}:{}", _algorithm, _fingerprint);
		zeroize_and_drop (_fingerprint);
		Ok (Rb::new (_handle))
	}
}


impl Sensitive for SshWrapperKey {
	
	fn erase (&mut self) -> () {
		self.public_key.erase ();
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
				SshAgentClient::connect_env () .await
			}) .else_wrap (0x67dc4fca) ?;
		
		let _wrapper = SshWrapperAgent {
				runtime : _runtime,
				client : Some (_client),
			};
		
		Ok (_wrapper)
	}
	
	
	pub fn keys (&mut self) -> SshResult<RbList<SshWrapperKey>> {
		
		let mut _client = self.client.take () .else_wrap (0xbe88e692) ?;
		
		let _keys = self.runtime.block_on (async {
				_client.request_identities () .await
			}) .else_wrap (0xbad56d04) ?;
		
		self.client = Some (_client);
		
		let mut _wrapper_keys = Vec::with_capacity (_keys.len ());
		
		for _key in _keys {
			let _algorithm = _key.name ();
			if ! ((_algorithm == "ssh-ed25519") || (_algorithm == "ssh-rsa") || _algorithm.starts_with ("rsa-")) {
				continue;
			}
			
			let _wrapper_key = SshWrapperKey {
					public_key : Rb::new (_key.clone () .into ()),
				};
			
			_wrapper_keys.push (_wrapper_key);
		}
		
		let _wrapper_keys = RbList::from_vec (_wrapper_keys);
		
		Ok (_wrapper_keys)
	}
}


