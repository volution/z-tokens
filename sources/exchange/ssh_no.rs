

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;

use crate::oracles::*;


use ::z_tokens_runtime::{
		memory::Rb,
	};








define_error! (pub SshError, result : SshResult);




pub struct SshWrapper {}
pub struct SshWrapperKey {}
pub struct SshWrapperAgent {}








impl SshWrapper {
	
	pub fn new (_key : SshWrapperKey, _agent : SshWrapperAgent) -> SshResult<SshWrapper> {
		panic! (unimplemented, 0xd1331652);
	}
	
	pub fn into_agent (self) -> SshResult<SshWrapperAgent> {
		panic! (unimplemented, 0x6bc7ddbd);
	}
	
	pub fn connect (_key : SshWrapperKey) -> SshResult<SshWrapper> {
		panic! (unimplemented, 0x0e2fae2c);
	}
	
	pub fn wrap (&mut self, _schema : Option<&'static str>, _input : &[u8], _output : &mut [u8; 32]) -> SshResult {
		panic! (unimplemented, 0x27e97836);
	}
	
	pub fn key (&self) -> &SshWrapperKey {
		panic! (unimplemented, 0x7dba805f);
	}
}




impl SshWrapperKey {
	
	pub fn encode (&self) -> SshResult<Rb<String>> {
		panic! (unimplemented, 0x017c106b);
	}
	
	pub fn decode_and_zeroize (_string : String) -> SshResult<Self> {
		panic! (unimplemented, 0x3524b90b);
	}
	
	pub fn decode (_string : &str) -> SshResult<Self> {
		panic! (unimplemented, 0xec629e9f);
	}
	
	pub fn handle (&self) -> &OracleHandle {
		panic! (unimplemented, 0x3c821ea8);
	}
	
	pub fn description (&self) -> SshResult<Rb<String>> {
		panic! (unimplemented, 0xbec36a24);
	}
	
	pub fn cmp (_left : &Self, _right : &Self) -> Ordering {
		panic! (unimplemented, 0x252e1474);
	}
}




impl SshWrapperAgent {
	
	pub fn connect () -> SshResult<SshWrapperAgent> {
		panic! (unimplemented, 0x24dc11d5);
	}
	
	pub fn keys (&mut self) -> SshResult<Vec<SshWrapperKey>> {
		panic! (unimplemented, 0x25a4327b);
	}
}


