

use ::vrl_errors::*;




#[ cfg (feature = "zt-runtime-allocator") ]
use ::z_tokens_runtime::allocator;


pub(crate) use ::std::process::ExitCode;


pub(crate) const IO_BUFFER_SIZE : usize = 4 * 1024;




define_error! (pub MainError, result : MainResult);


