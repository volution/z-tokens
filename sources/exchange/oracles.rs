

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use ::z_tokens_runtime::{
		memory::Rb,
	};




define_error! (pub OracleError, result : OracleResult);




pub trait Oracle {
	
	fn derive (&mut self, _schema : Option<&'static str>, _input : &[u8], _output : &mut [u8; 32]) -> OracleResult;
	
	fn handle (&self) -> &OracleHandle;
}


pub struct OracleHandle (Rb<[u8; 32]>);




impl OracleHandle {
	
	pub fn from_raw (_raw : &[u8; 32]) -> Self {
		Self (Rb::new_copy (_raw))
	}
	
	pub fn as_raw (&self) -> &[u8; 32] {
		self.0.deref ()
	}
}


