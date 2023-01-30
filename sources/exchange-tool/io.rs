

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;




define_error! (pub(crate) IoError, result : IoResult);




pub(crate) const STDOUT_BUFFER_SIZE : usize = 8 * 1024;




pub(crate) fn read_at_most (mut _stream : impl Read, _limit : usize) -> IoResult<Vec<u8>> {
	
	let mut _buffer = Vec::with_capacity (STDOUT_BUFFER_SIZE);
	
	// FIXME:  Actually impose limit!
	_stream.read_to_end (&mut _buffer) .else_wrap (0xb0ef2873) ?;
	
	if _buffer.len () > _limit {
		fail! (0x2d0cf0e1);
	}
	
	Ok (_buffer)
}


