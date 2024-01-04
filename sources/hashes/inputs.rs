

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;




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




pub struct InputEmpty;


impl Input for InputEmpty {
	
	fn input (&mut self) -> InputResult<Option<&[u8]>> {
		return Ok (None);
	}
}




pub struct InputFromBytesBoxes {
	stack : Vec<Box<[u8]>>,
	should_pop : bool,
}


impl InputFromBytesBoxes {
	
	pub fn from_vec (_bytes : impl Into<Vec<u8>>) -> Self {
		Self {
				stack : vec! [_bytes.into () .into_boxed_slice ()],
				should_pop : false,
			}
	}
	
	pub fn from_iter_of_vec (_bytes : impl Iterator<Item = Vec<u8>>) -> Self {
		let mut _stack = _bytes.map (Vec::into_boxed_slice) .collect::<Vec<_>> ();
		_stack.reverse ();
		Self {
				stack : _stack,
				should_pop : false,
			}
	}
}


impl Input for InputFromBytesBoxes {
	
	fn input (&mut self) -> InputResult<Option<&[u8]>> {
		if self.should_pop {
			self.should_pop = false;
			self.stack.pop ();
		}
		let Some (_bytes) = self.stack.last ()
			else {
				return Ok (None);
			};
		self.should_pop = true;
		return Ok (Some (&_bytes));
	}
}




pub struct InputFromBytesSlices <'a> {
	stack : Vec<&'a [u8]>,
	should_pop : bool,
}


impl <'a> InputFromBytesSlices<'a> {
	
	pub fn from_slice (_bytes : impl Into<&'a [u8]>) -> Self {
		Self {
				stack : vec! [_bytes.into ()],
				should_pop : false,
			}
	}
	
	pub fn from_iter_of_slice (_bytes : impl Iterator<Item = &'a [u8]>) -> Self {
		let mut _stack = _bytes.collect::<Vec<_>> ();
		_stack.reverse ();
		Self {
				stack : _stack,
				should_pop : false,
			}
	}
}


impl <'a> Input for InputFromBytesSlices<'a> {
	
	fn input (&mut self) -> InputResult<Option<&[u8]>> {
		if self.should_pop {
			self.should_pop = false;
			self.stack.pop ();
		}
		let Some (_bytes) = self.stack.last ()
			else {
				return Ok (None);
			};
		self.should_pop = true;
		return Ok (Some (_bytes));
	}
}




pub struct InputFromConcatenation <I : Input> {
	stack : Vec<I>,
	should_pop : bool,
}


impl <I : Input> InputFromConcatenation<I> {
	
	pub fn new (_inputs : impl Iterator<Item = I>) -> Self {
		let mut _stack = _inputs.collect::<Vec<_>> ();
		_stack.reverse ();
		Self {
				stack : _stack,
				should_pop : false,
			}
	}
}


impl <I : Input> Input for InputFromConcatenation<I> {
	
	fn input (&mut self) -> InputResult<Option<&[u8]>> {
		if self.should_pop {
			self.should_pop = false;
			self.stack.pop ();
		}
		let Some (_head) = self.stack.last_mut ()
			else {
				return Ok (None);
			};
		let Some (_data) = _head.input () ?
			else {
				self.should_pop = true;
				return Ok (Some (&[]));
			};
		return Ok (Some (_data));
	}
}




pub struct InputFromCanonicalization <I : Input> {
	stack : Vec<I>,
	should_pop : bool,
	should_size : bool,
	size_value : usize,
	size_buffer : [u8; 8],
}


impl <I : Input> InputFromCanonicalization<I> {
	
	pub fn new (_inputs : impl Iterator<Item = I>) -> Self {
		let mut _stack = _inputs.collect::<Vec<_>> ();
		_stack.reverse ();
		let _stack_size = _stack.len ();
		Self {
				stack : _stack,
				should_pop : false,
				size_value : _stack_size,
				should_size : true,
				size_buffer : [0u8; 8],
			}
	}
}


impl <I : Input> Input for InputFromCanonicalization<I> {
	
	fn input (&mut self) -> InputResult<Option<&[u8]>> {
		if self.should_size {
			self.should_size = false;
			use ::byteorder::ByteOrder as _;
			::byteorder::BigEndian::write_u64 (&mut self.size_buffer, self.size_value.try_into () .else_wrap (0x19f2d2bd) ?);
			self.size_value = 0;
			return Ok (Some (&self.size_buffer));
		}
		if self.should_pop {
			self.should_pop = false;
			self.stack.pop ();
		}
		let Some (_head) = self.stack.last_mut ()
			else {
				return Ok (None);
			};
		let Some (_data) = _head.input () ?
			else {
				self.should_pop = true;
				self.should_size = true;
				return Ok (Some (&[]));
			};
		self.size_value += _data.len ();
		return Ok (Some (_data));
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




pub fn inputs_concatenate <I : Input> (_inputs : impl Iterator<Item = I>) -> InputResult<impl Input> {
	let _input = InputFromConcatenation::new (_inputs);
	Ok (_input)
}


pub fn inputs_canonicalize <I : Input> (_inputs : impl Iterator<Item = I>) -> InputResult<impl Input> {
	let _input = InputFromCanonicalization::new (_inputs);
	Ok (_input)
}


pub fn input_empty () -> InputResult<impl Input> {
	
	Ok (InputEmpty)
}


pub fn input_from_string_owned (_string : String) -> InputResult<impl Input> {
	
	input_from_bytes_owned (_string.into_bytes ())
}


pub fn input_from_bytes_owned (_bytes : Vec<u8>) -> InputResult<impl Input> {
	
	let _input = InputFromBytesBoxes::from_vec (_bytes);
	
	Ok (_input)
}


pub fn input_from_string_slice <'a> (_string : &'a str) -> InputResult<impl Input + 'a> {
	
	input_from_bytes_slice (_string.as_bytes ())
}


pub fn input_from_bytes_slice <'a> (_bytes : &'a [u8]) -> InputResult<impl Input + 'a> {
	
	let _input = InputFromBytesSlices::from_slice (_bytes);
	
	Ok (_input)
}


pub fn input_from_stdio () -> InputResult<impl Input> {
	
	let _stream = stdin_locked ();
	
	input_from_stream (_stream)
}


pub fn input_from_file (_path : &Path) -> InputResult<impl Input> {
	
	let _stream = fs::File::open (_path) .else_wrap (0x425b5064) ?;
	
	input_from_stream (_stream)
}


pub fn input_from_stream <Stream : Read> (_stream : Stream) -> InputResult<impl Input> {
	
	let _buffer = BufReader::with_capacity (BUFFER_SIZE_DEFAULT, _stream);
	
	let _input = InputFromStream {
			buffer : _buffer,
			consume_pending : 0,
		};
	
	Ok (_input)
}


