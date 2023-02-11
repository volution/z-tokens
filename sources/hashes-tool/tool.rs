

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;
use ::z_tokens_runtime::flags::*;

use ::z_tokens_hashes::model::*;
use ::z_tokens_hashes::hashes::*;
use ::z_tokens_hashes::inputs::*;
use ::z_tokens_hashes::format::*;




define_error! (pub MainError, result : MainResult);




const STDOUT_BUFFER_SIZE : usize = 8 * 1024;




pub fn main_hash (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _format : Option<Format> = None;
	let mut _family : Option<Family> = None;
	let mut _output_size : Option<usize> = None;
	let mut _output_discard_right : Option<bool> = None;
	let mut _output_reversed : Option<bool> = None;
	
	let mut _input_sources : Vec<InputSource> = Vec::new ();
	let mut _inputs_canonicalize : Option<bool> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0x0102258d) ?;
		
		_parser.refer (&mut _family)
				.metavar ("{algorithm}")
				.add_option (&["-a", "--algorithm"], ArgStoreOption, "(hashing algorithm)")
				.add_option (&["--md5"], ArgStoreConst (Some (Family::MD5)), "(use MD5)")
				.add_option (&["--sha1"], ArgStoreConst (Some (Family::SHA1)), "(use SHA1)")
				.add_option (&["--sha2"], ArgStoreConst (Some (Family::SHA2)), "(use SHA2)")
				.add_option (&["--sha3"], ArgStoreConst (Some (Family::SHA3)), "(use SHA3)")
				.add_option (&["--shake128"], ArgStoreConst (Some (Family::Shake_128)), "(use Shake128)")
				.add_option (&["--shake256"], ArgStoreConst (Some (Family::Shake_256)), "(use Shake256)")
				.add_option (&["--blake2"], ArgStoreConst (Some (Family::Blake2)), "(use Blake2s or Blake2b)")
				.add_option (&["--blake2s"], ArgStoreConst (Some (Family::Blake2s)), "(use Blake2s)")
				.add_option (&["--blake2b"], ArgStoreConst (Some (Family::Blake2b)), "(use Blake2b)")
				.add_option (&["--blake3"], ArgStoreConst (Some (Family::Blake3)), "(use Blake3)")
				.add_option (&["--git-sha1"], ArgStoreConst (Some (Family::GitSHA1)), "(use Git SHA1 flavour)")
				.add_option (&["--git-sha2"], ArgStoreConst (Some (Family::GitSHA2)), "(use Git SHA2 flavour)")
				.add_option (&["--argon2"], ArgStoreConst (Some (Family::Argon2)), "(use one of Argon2 family) !!! EXPERIMENTAL !!!")
				.add_option (&["--argon2d"], ArgStoreConst (Some (Family::Argon2d)), "(use Argon2d) !!! EXPERIMENTAL !!!")
				.add_option (&["--argon2i"], ArgStoreConst (Some (Family::Argon2i)), "(use Argon2i) !!! EXPERIMENTAL !!!")
				.add_option (&["--argon2id"], ArgStoreConst (Some (Family::Argon2id)), "(use Argon2id) !!! EXPERIMENTAL !!!")
				.add_option (&["--siphash"], ArgStoreConst (Some (Family::SipHash)), "(use SipHash) !!! WEAK !!!")
				.add_option (&["--seahash"], ArgStoreConst (Some (Family::SeaHash)), "(use SeaHash) !!! WEAK !!!")
				.add_option (&["--highway"], ArgStoreConst (Some (Family::HighwayHash)), "(use HighwayHash) !!! WEAK !!!")
				.add_option (&["--xxhash"], ArgStoreConst (Some (Family::XxHash)), "(use xxHash) !!! WEAK !!!")
				.add_option (&["--xxh3"], ArgStoreConst (Some (Family::Xxh3)), "(use Xxh3) !!! WEAK and EXPERIMENTAL !!!")
				.add_option (&["--djb2"], ArgStoreConst (Some (Family::Djb2)), "(use djb2) !!! WEAK !!!")
				.add_option (&["--sdbm"], ArgStoreConst (Some (Family::SDBM)), "(use SDBM) !!! WEAK !!!")
				.add_option (&["--fnv1a"], ArgStoreConst (Some (Family::FNV1a)), "(use FNV1a) !!! WEAK !!!")
				.add_option (&["--crc"], ArgStoreConst (Some (Family::CRC)), "(use CRC) !!! WEAK !!!")
				.add_option (&["--crc32c"], ArgStoreConst (Some (Family::CRC32C)), "(use CRC32C) !!! WEAK !!!")
				.add_option (&["--adler"], ArgStoreConst (Some (Family::Adler)), "(use Adler) !!! WEAK !!!")
			;
		
		_parser.refer (&mut _output_size)
				.metavar ("{size}")
				.add_option (&["-s", "--size"], ArgStoreOption, "(hash output size in bytes)")
				.add_option (&["--8b", "--1B"], ArgStoreConst (Some (1)), "(output 8 bits / 1 byte)")
				.add_option (&["--16b", "--2B"], ArgStoreConst (Some (2)), "(output 16 bits / 2 bytes)")
				.add_option (&["--32b", "--4B"], ArgStoreConst (Some (4)), "(output 32 bits / 4 bytes)")
				.add_option (&["--64b", "--8B"], ArgStoreConst (Some (8)), "(output 64 bits / 8 bytes)")
				.add_option (&["--128b", "--16B"], ArgStoreConst (Some (16)), "(output 128 bits / 16 bytes)")
				.add_option (&["--256b", "--32B"], ArgStoreConst (Some (32)), "(output 256 bits / 32 bytes)")
				.add_option (&["--512b", "--64B"], ArgStoreConst (Some (64)), "(output 512 bits / 64 bytes)")
				.add_option (&["--1024b", "--128B"], ArgStoreConst (Some (128)), "(output 1024 bits / 128 bytes)")
				.add_option (&["--2048b", "--256B"], ArgStoreConst (Some (256)), "(output 2048 bits / 256 bytes)")
				.add_option (&["--4096b", "--512B"], ArgStoreConst (Some (512)), "(output 4096 bits / 512 bytes)")
				.add_option (&["--8192b", "--1024B"], ArgStoreConst (Some (1024)), "(output 8192 bits / 1024 bytes)")
			;
		
		_parser.refer (&mut _input_sources)
				.metavar ("{input}")
				.add_option (&["-t", "--token"], ArgPush, "(use this argument)")
				.add_option (&["-f", "--file"], ArgPushInputSourceFile, "(read from file)")
				// FIXME:  Flags and arguments aren't properly supported by `argparser`!
				// .add_option (&["-i", "--stdin"], ArgPushConst (InputSource::Stdin), "(read from stdin)")
				// .add_option (&["-e", "--empty"], ArgPushConst (InputSource::Empty), "(empty)")
			;
		
		_parser.refer (&mut _inputs_canonicalize)
				.metavar ("{canonicalize}")
				.add_option (&["-c", "--inputs-concatenate"], ArgStoreConst (Some (false)), "(concatenate inputs) (default for one input)")
				.add_option (&["-C", "--inputs-canonicalize"], ArgStoreConst (Some (true)), "(canonicalize inputs) (default for two or more inputs)");
		
		_parser.refer (&mut _output_discard_right)
				.metavar ("{alignment}")
				.add_option (&["--output-discard-right"], ArgStoreConst (Some (true)), "(if needed discard bytes from the right of the hash) (default)")
				.add_option (&["--output-discard-left"], ArgStoreConst (Some (false)), "(if needed discard bytes from the left of the hash)");
		
		_parser.refer (&mut _output_reversed)
				.metavar ("{reversed}")
				.add_option (&["--output-left-to-right"], ArgStoreConst (Some (false)), "(copy from left-to-right bytes from the hash) (default)")
				.add_option (&["--output-right-to-left", "--output-reversed"], ArgStoreConst (Some (true)), "(copy from right-to-left bytes from the hash)");
		
		if execute_parser (_parser, _arguments) .else_wrap (0x88824ad0) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _format = _format.unwrap_or (Format::Hex);
	let (_algorithm, _output_size) = choose_algorithm (_family, _output_size) .else_wrap (0x01241ede) ?;
	
	let _output_parameters = OutputParameters {
			size : _output_size,
			discard_right : _output_discard_right.unwrap_or (true),
			reversed : _output_reversed.unwrap_or (false),
		};
	
	let mut _inputs = _input_sources.into_iter () .map (InputSource::into_boxed_input) .collect::<Result<Vec<_>, _>> () ?;
	
	if _inputs.is_empty () {
		_inputs.push (InputSource::Stdin.into_boxed_input () ?)
	}
	
	let _inputs_canonicalize = _inputs_canonicalize.unwrap_or (_inputs.len () >= 2);
	
	let mut _input : Box<dyn Input> =
		if _inputs_canonicalize {
			Box::new (inputs_canonicalize (_inputs.into_iter ()) .else_wrap (0xed0348a4) ?)
		} else if _inputs.len () > 1 {
			Box::new (inputs_concatenate (_inputs.into_iter ()) .else_wrap (0xd6032117) ?)
		} else {
			_inputs.pop () .else_panic (0xadd28f1a)
		};
	
	let _hash = hash (_algorithm, &mut _input, &_output_parameters) .else_wrap (0x16112a03) ?;
	drop (_input);
	
	let mut _output = BufWriter::with_capacity (STDOUT_BUFFER_SIZE, stdout_locked ());
	format_hash (&_hash, &_format, &mut _output) .else_wrap (0x4990ace1) ?;
	writeln! (&mut _output) .else_wrap (0xf9e34569) ?;
	drop (_output.into_inner () .else_replace (0x8ab3f5e2) ?);
	
	Ok (ExitCode::SUCCESS)
}




#[ derive (Clone) ]
#[ allow (dead_code) ]
enum InputSource {
	Stdin,
	File (PathBuf),
	String (String),
	Empty,
}


impl InputSource {
	
	fn into_boxed_input (self) -> MainResult<Box<dyn Input>> {
		let _input : Box<dyn Input> = match self {
			InputSource::Stdin =>
				Box::new (input_from_stdio () .else_wrap (0x211ceca5) ?),
			InputSource::File (_path) =>
				Box::new (input_from_file (&_path) .else_wrap (0xa8211613) ?),
			InputSource::String (_string) =>
				Box::new (input_from_string_owned (_string) .else_wrap (0xc87afcd6) ?),
			InputSource::Empty =>
				Box::new (input_empty () .else_wrap (0xf3574630) ?),
		};
		Ok (_input)
	}
}


impl FromStr for InputSource {
	
	type Err = ();
	
	fn from_str (_string : &str) -> Result<Self, ()> {
		Ok (InputSource::String (String::from (_string)))
	}
}








struct ArgPushInputSourceFile;
struct ArgPushInputSourceFileAction <'a> (Rc<RefCell<&'a mut Vec<InputSource>>>);


impl argparse::action::TypedAction<Vec<InputSource>> for ArgPushInputSourceFile {
	
	fn bind <'a> (&self, _cell : Rc<RefCell<&'a mut Vec<InputSource>>>) -> argparse::action::Action<'a> {
		return argparse::action::Action::Single (Box::new (ArgPushInputSourceFileAction (_cell)));
	}
}


impl <'a> argparse::action::IArgAction for ArgPushInputSourceFileAction <'a> {
	
	fn parse_arg (&self, _argument : &str) -> argparse::action::ParseResult {
		let _cell : &RefCell<_> = self.0.borrow ();
		let mut _cell : RefMut<_> = _cell.borrow_mut ();
		_cell.push (InputSource::File (PathBuf::from (_argument)));
		argparse::action::ParseResult::Parsed
	}
}


