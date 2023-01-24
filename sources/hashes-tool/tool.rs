

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;
use ::z_tokens_runtime::flags::*;

use crate::model::*;
use crate::hashes::*;
use crate::inputs::*;
use crate::format::*;




define_error! (pub MainError, result : MainResult);




const STDOUT_BUFFER_SIZE : usize = 8 * 1024;




pub fn main (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _format : Option<Format> = None;
	let mut _family : Option<Family> = None;
	let mut _output_size : Option<usize> = None;
	
	let mut _input_source : Option<InputSource> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0x0102258d) ?;
		
		_parser.refer (&mut _family)
				.metavar ("{algorithm}")
				.add_option (&["-a", "--algorithm"], ArgStoreOption, "(hashing algorithm)")
				.add_option (&["--md5"], ArgStoreConst (Some (Family::MD5)), "(use MD5)")
				.add_option (&["--sha1"], ArgStoreConst (Some (Family::SHA1)), "(use SHA1)")
				.add_option (&["--sha2"], ArgStoreConst (Some (Family::SHA2)), "(use SHA2)")
				.add_option (&["--sha3"], ArgStoreConst (Some (Family::SHA3)), "(use SHA3)")
				.add_option (&["--blake2"], ArgStoreConst (Some (Family::Blake2)), "(use Blake2s or Blake2b)")
				.add_option (&["--blake2s"], ArgStoreConst (Some (Family::Blake2s)), "(use Blake2s)")
				.add_option (&["--blake2b"], ArgStoreConst (Some (Family::Blake2b)), "(use Blake2b)")
				.add_option (&["--blake3"], ArgStoreConst (Some (Family::Blake3)), "(use Blake3)")
				.add_option (&["--git-sha1"], ArgStoreConst (Some (Family::GitSHA1)), "(use Git SHA1 flavour)")
				.add_option (&["--argon2"], ArgStoreConst (Some (Family::Argon2)), "(use one of Argon2 family) !!! EXPERIMENTAL !!!")
				.add_option (&["--argon2d"], ArgStoreConst (Some (Family::Argon2d)), "(use Argon2d) !!! EXPERIMENTAL !!!")
				.add_option (&["--argon2i"], ArgStoreConst (Some (Family::Argon2i)), "(use Argon2i) !!! EXPERIMENTAL !!!")
				.add_option (&["--argon2id"], ArgStoreConst (Some (Family::Argon2id)), "(use Argon2id) !!! EXPERIMENTAL !!!");
		
		_parser.refer (&mut _output_size)
				.metavar ("{size}")
				.add_option (&["-s", "--size"], ArgStoreOption, "(hash output size in bytes)")
				.add_option (&["--8b", "--1B"], ArgStoreConst (Some (1)), "(output 8 bits / 1 byte)")
				.add_option (&["--16b", "--2B"], ArgStoreConst (Some (2)), "(output 16 bits / 2 bytes)")
				.add_option (&["--32b", "--4B"], ArgStoreConst (Some (4)), "(output 32 bits / 4 bytes)")
				.add_option (&["--64b", "--8B"], ArgStoreConst (Some (8)), "(output 64 bits / 8 bytes)")
				.add_option (&["--128b", "--16B"], ArgStoreConst (Some (16)), "(output 128 bits / 16 bytes)")
				.add_option (&["--256b", "--32B"], ArgStoreConst (Some (32)), "(output 256 bits / 32 bytes)")
				.add_option (&["--512b", "--64B"], ArgStoreConst (Some (64)), "(output 512 bits / 64 bytes)");
		
		_parser.refer (&mut _input_source)
				.metavar ("{input}")
				.add_option (&["-i", "--stdin"], ArgStoreConst (Some (InputSource::Stdin)), "(read from stdin)")
				.add_option (&["-t", "--token"], ArgStoreOption, "(use this argument)");
		
		if execute_parser (_parser, _arguments) .else_wrap (0x88824ad0) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _format = _format.unwrap_or (Format::Hex);
	let (_algorithm, _output_size) = choose_algorithm (_family, _output_size) .else_wrap (0x01241ede) ?;
	
	let _input_source = _input_source.else_wrap (0x808b3d1e) ?;
	
	let mut _input : Box<dyn Input> = match _input_source {
			InputSource::Stdin =>
				Box::new (input_from_stdio () .else_wrap (0x211ceca5) ?),
			InputSource::String (ref _string) =>
				Box::new (input_from_string (_string) .else_wrap (0xc87afcd6) ?),
		};
	
	let _hash = hash (_algorithm, _output_size, &mut _input) .else_wrap (0x16112a03) ?;
	drop (_input);
	
	let mut _output = BufWriter::with_capacity (STDOUT_BUFFER_SIZE, stdout_locked ());
	format_hash (&_hash, &_format, &mut _output) .else_wrap (0x4990ace1) ?;
	writeln! (&mut _output) .else_wrap (0xf9e34569) ?;
	drop (_output.into_inner () .else_replace (0x8ab3f5e2) ?);
	
	Ok (ExitCode::SUCCESS)
}




#[ derive (Clone) ]
enum InputSource {
	Stdin,
	String (String),
}


impl FromStr for InputSource {
	
	type Err = ();
	
	fn from_str (_string : &str) -> Result<Self, ()> {
		Ok (InputSource::String (String::from (_string)))
	}
}


