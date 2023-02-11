

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;




define_error! (pub InputError, result : InputResult);




const BUFFER_SIZE_DEFAULT : usize = 128 * 1024;




pub trait Input {
	fn input (&mut self) -> InputResult<Option<&[u8]>>;
}




pub struct InputFromStream <Stream : Read> {
	buffer : BufReader<Stream>,
	consume_pending : usize,
}


impl <Stream : Read> Input for InputFromStream<Stream> {
	
	fn input (&mut self) -> InputResult<Option<&[u8]>> {
		if self.consume_pending > 0 {
			self.buffer.consume (self.consume_pending);
			self.consume_pending = 0;
		}
		let _data = self.buffer.fill_buf () .else_wrap (0x8cce4a89) ?;
		if ! _data.is_empty () {
			self.consume_pending = _data.len ();
			return Ok (Some (_data))
		} else {
			return Ok (None);
		}
	}
}




pub struct InputFromBytes <'a> {
	buffer : Option<&'a [u8]>,
}


impl <'a> Input for InputFromBytes <'a> {
	
	fn input (&mut self) -> InputResult<Option<&[u8]>> {
		if self.buffer.is_none () {
			return Ok (None);
		}
		let _data = self.buffer.take ();
		Ok (_data)
	}
}




impl <I : Input + ?Sized> Input for Box<I> {
	
	fn input (&mut self) -> InputResult<Option<&[u8]>> {
		self.as_mut () .input ()
	}
}


impl <I : Input + ?Sized> Input for &mut I {
	
	fn input (&mut self) -> InputResult<Option<&[u8]>> {
		(*self) .input ()
	}
}




pub fn input_from_stdio () -> InputResult<impl Input> {
	
	let _stream = stdin_locked ();
	
	input_from_stream (_stream)
}


pub fn input_from_file (_path : &Path) -> InputResult<impl Input> {
	
	let _stream = fs::File::open (_path) .else_wrap (0x425b5064) ?;
	
	input_from_stream (_stream)
}


pub fn input_from_string (_string : &str) -> InputResult<InputFromBytes> {
	
	input_from_bytes (_string.as_bytes ())
}


pub fn input_from_bytes (_bytes : &[u8]) -> InputResult<InputFromBytes> {
	
	let _input = InputFromBytes {
			buffer : Some (_bytes),
		};
	
	Ok (_input)
}


pub fn input_from_stream <Stream : Read> (_stream : Stream) -> InputResult<InputFromStream<Stream>> {
	
	let _buffer = BufReader::with_capacity (BUFFER_SIZE_DEFAULT, _stream);
	
	let _input = InputFromStream {
			buffer : _buffer,
			consume_pending : 0,
		};
	
	Ok (_input)
}


