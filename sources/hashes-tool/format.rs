

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;




define_error! (pub FormatError, result : FormatResult);




#[ derive (Copy, Clone) ]
pub enum Format {
	Hex,
}




pub fn format_hash (_hash : &[u8], _format : &Format, mut _stream : impl Write) -> FormatResult {
	
	match _format {
		Format::Hex => {
			for _byte in _hash {
				write! (&mut _stream, "{:0x}", _byte) .else_wrap (0xd3bc5f48) ?;
			}
			Ok (())
		}
	}
}


