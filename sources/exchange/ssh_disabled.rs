

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;

pub use crate::ssh_common::*;


use ::z_tokens_runtime::{
		memory::Rb,
	};








pub struct SshWrapperKey {
	handle : SshWrapperHandle,
}


pub struct SshWrapperAgent {}








impl SshWrapper {
	
	pub fn wrap (&mut self, _namespace : Option<&str>, _input : &[u8], _output : &mut [u8; 32]) -> SshResult {
		fail! (0x27e97836);
	}
}




impl SshWrapperKey {
	
	pub fn handle (&self) -> &SshWrapperHandle {
		&self.handle
	}
	
	pub fn encode (&self) -> SshResult<Rb<String>> {
		fail! (0x017c106b);
	}
	
	pub fn decode_and_zeroize (_string : String) -> SshResult<Self> {
		fail! (0x3524b90b);
	}
	
	pub fn decode (_string : &str) -> SshResult<Self> {
		fail! (0xec629e9f);
	}
	
	pub fn description (&self) -> SshResult<Rb<String>> {
		fail! (0xbec36a24);
	}
}




impl SshWrapperAgent {
	
	pub fn connect () -> SshResult<SshWrapperAgent> {
		fail! (0x24dc11d5);
	}
	
	pub fn keys (&mut self) -> SshResult<Vec<SshWrapperKey>> {
		fail! (0x25a4327b);
	}
}


