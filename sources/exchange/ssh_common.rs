

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;

use crate::oracles::*;
use crate::ssh::*;


use ::z_tokens_runtime::{
		memory::Rb,
		sensitive::zeroize_and_drop,
	};

use crate::keys::{
		encode_raw,
		decode_raw,
	};








define_error! (pub SshError, result : SshResult);




pub const SSH_WRAPPER_KEY_ENCODED_PREFIX : &str = "ztxws";
pub const SSH_WRAPPER_HANDLE_ENCODED_PREFIX : &str = "ztxwh";




pub struct SshWrapper {
	pub(crate) key : SshWrapperKey,
	pub(crate) agent : SshWrapperAgent,
}


pub struct SshWrapperHandle {
	pub(crate) handle : OracleHandle,
}








impl SshWrapper {
	
	pub fn new (_key : SshWrapperKey, _agent : SshWrapperAgent) -> SshResult<SshWrapper> {
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
	
	pub fn key (&self) -> &SshWrapperKey {
		&self.key
	}
}








impl SshWrapperHandle {
	
	pub fn encode (&self) -> SshResult<Rb<String>> {
		encode_raw (SSH_WRAPPER_HANDLE_ENCODED_PREFIX, self.as_raw ()) .else_wrap (0xccc87a84)
	}
	
	pub fn decode_and_zeroize (_string : String) -> SshResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> SshResult<Self> {
		let mut _raw = [0u8; 32];
		let _serialized = decode_raw (SSH_WRAPPER_HANDLE_ENCODED_PREFIX, _string, &mut _raw) .else_wrap (0xf7429b62) ?;
		Ok (SshWrapperHandle::from_raw (_raw))
	}
}


impl SshWrapperHandle {
	
	pub(crate) fn from_raw (_raw : [u8; 32]) -> Self {
		Self {
				handle : OracleHandle::from_raw (_raw),
			}
	}
	
	#[ allow (dead_code) ]
	pub(crate) fn copy_raw (_raw : &[u8; 32]) -> Self {
		Self {
				handle : OracleHandle::copy_raw (_raw),
			}
	}
	
	pub(crate) fn as_raw (&self) -> &[u8; 32] {
		self.handle.as_raw ()
	}
}




impl Oracle for SshWrapper {
	
	fn derive (&mut self, _namespace : Option<&'static str>, _input : &[u8], _output : &mut [u8; 32]) -> OracleResult {
		self.wrap (_namespace, _input, _output) .else_wrap (0x7d6e855b)
	}
	
	fn handle (&self) -> &OracleHandle {
		& self.key () .handle () .handle
	}
}




impl SshWrapperAgent {
	
	pub fn resolve (&mut self, _handle : &SshWrapperHandle) -> SshResult<Option<SshWrapperKey>> {
		for _key in self.keys () ? .into_iter () {
			if _key.handle () .as_raw () == _handle.as_raw () {
				return Ok (Some (_key));
			}
		}
		return Ok (None);
	}
}


