

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use ::z_tokens_runtime::{
		memory::Rb,
		memory::RbList,
		sensitive::Sensitive,
	};








define_error! (pub SshError, result : SshResult);




pub struct SshWrapper {}
pub struct SshWrapperKey {}
pub struct SshWrapperAgent {}








impl SshWrapper {
	
	pub fn new (_key : impl Into<Rb<SshWrapperKey>>, _agent : SshWrapperAgent) -> SshResult<SshWrapper> {
		fail! (0xd1331652);
	}
	
	pub fn into_agent (self) -> SshResult<SshWrapperAgent> {
		fail! (0x6bc7ddbd);
	}
	
	pub fn connect (_key : impl Into<Rb<SshWrapperKey>>) -> SshResult<SshWrapper> {
		fail! (0x0e2fae2c);
	}
	
	pub fn wrap (&mut self, _input : &[u8], _output : &mut [u8; 32]) -> SshResult {
		fail! (0x27e97836);
	}
}




impl SshWrapperKey {
	
	pub fn encode (&self) -> SshResult<Rb<String>> {
		fail! (0x017c106b);
	}
	
	pub fn decode_and_zeroize (_string : String) -> SshResult<Self> {
		fail! (0x3524b90b);
	}
	
	pub fn decode (_string : &str) -> SshResult<Self> {
		fail! (0xec629e9f);
	}
	
	pub fn handle (&self) -> SshResult<Rb<String>> {
		fail! (0xbec36a24);
	}
}


impl Sensitive for SshWrapperKey {
	
	fn erase (&mut self) -> () {}
}




impl SshWrapperAgent {
	
	pub fn connect () -> SshResult<SshWrapperAgent> {
		fail! (0x24dc11d5);
	}
	
	pub fn keys (&mut self) -> SshResult<RbList<SshWrapperKey>> {
		fail! (0x25a4327b);
	}
}


