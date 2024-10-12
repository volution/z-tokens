

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
pub const SSH_WRAPPER_HANDLE_ENCODED_PREFIX : &str = "ztxoh";




pub struct SshWrapper {
	pub(crate) key : SshWrapperKey,
	pub(crate) agent : SshWrapperAgent,
}


pub struct SshWrapperHandle {
	pub(crate) handle : OracleHandle,
}


pub enum SshWrapperKeyOrHandle {
	Key (SshWrapperKey),
	Handle (SshWrapperHandle),
}








impl SshWrapper {
	
	pub fn new_with_key (_key : SshWrapperKey, _agent : SshWrapperAgent) -> SshResult<SshWrapper> {
		let _self = SshWrapper {
				key : _key,
				agent : _agent,
			};
		Ok (_self)
	}
	
	pub fn new_with_handle (_handle : SshWrapperHandle, mut _agent : SshWrapperAgent) -> SshResult<SshWrapper> {
		let Some (_key) = _agent.resolve_one (&_handle) ?
			else { fail! (0x73f6dec0) };
		Self::new_with_key (_key, _agent)
	}
	
	pub fn new_with_key_or_handle (_key_or_handle : SshWrapperKeyOrHandle, _agent : SshWrapperAgent) -> SshResult<SshWrapper> {
		match _key_or_handle {
			SshWrapperKeyOrHandle::Key (_key) => Self::new_with_key (_key, _agent),
			SshWrapperKeyOrHandle::Handle (_handle) => Self::new_with_handle (_handle, _agent),
		}
	}
	
	pub fn into_agent (self) -> SshResult<SshWrapperAgent> {
		Ok (self.agent)
	}
	
	pub fn key (&self) -> &SshWrapperKey {
		&self.key
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
	
	pub fn resolve_one (&mut self, _handle : &SshWrapperHandle) -> SshResult<Option<SshWrapperKey>> {
		for _key in self.keys () ? .into_iter () {
			if _key.handle () .as_raw () == _handle.as_raw () {
				return Ok (Some (_key));
			}
		}
		Ok (None)
	}
	
	pub fn resolve_all <'a> (&mut self, _handles : impl Iterator<Item = &'a SshWrapperHandle>) -> SshResult<Option<Vec<SshWrapperKey>>> {
		let _handles : Vec<_> = _handles.collect ();
		// FIXME:  Implement more efficiently!
		let mut _keys = Vec::with_capacity (_handles.len ());
		for _key in self.keys () ? .into_iter () {
			let mut _matched = false;
			for _handle in _handles.iter () {
				if _key.handle () .as_raw () == _handle.as_raw () {
					_matched = true;
					break;
				}
			}
			if _matched {
				_keys.push (_key);
			}
		}
		if _keys.len () == _handles.len () {
			Ok (Some (_keys))
		} else {
			Ok (None)
		}
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








impl SshWrapperKeyOrHandle {
	
	pub fn encode (&self) -> SshResult<Rb<String>> {
		match self {
			Self::Key (_key) => _key.encode (),
			Self::Handle (_handle) => _handle.encode (),
		}
	}
	
	pub fn decode_and_zeroize (_string : String) -> SshResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> SshResult<Self> {
		if _string.starts_with (SSH_WRAPPER_HANDLE_ENCODED_PREFIX) {
			Ok (Self::Handle (SshWrapperHandle::decode (_string) ?))
		} else if _string.starts_with (SSH_WRAPPER_KEY_ENCODED_PREFIX) {
			Ok (Self::Key (SshWrapperKey::decode (_string) ?))
		} else {
			fail! (0xd41808da);
		}
	}
}


