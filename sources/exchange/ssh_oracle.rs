

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;

use crate::oracles::*;
use crate::ssh::*;




impl Oracle for SshWrapper {
	
	fn derive (&mut self, _namespace : Option<&'static str>, _input : &[u8], _output : &mut [u8; 32]) -> OracleResult {
		self.wrap (_namespace, _input, _output) .else_wrap (0x7d6e855b)
	}
	
	fn handle (&self) -> &OracleHandle {
		self.key (). handle ()
	}
}

