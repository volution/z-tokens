

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;
use ::z_tokens_runtime_flags::*;

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
		let mut _flags = create_flags () .else_wrap (0x0102258d) ?;
		
		{
			let _flag = _flags.define_complex (&mut _family);
				
			_flag.define_flag ('a', "algorithm") .with_placeholder ("algorithm") .with_description ("hashing algorithm");
			
			_flag.define_switch ((), "md5", Family::MD5) .with_description ("use MD5");
				
			_flag.define_switch ((), "sha1", Family::SHA1) .with_description ("use SHA1");
			
			_flag.define_switch ((), "sha2", Family::SHA2) .with_description ("use one of SHA2");
			_flag.define_switch ((), "sha2-224", Family::SHA2_224) .with_description ("use SHA2-224");
			_flag.define_switch ((), "sha2-256", Family::SHA2_256) .with_description ("use SHA2-256");
			_flag.define_switch ((), "sha2-384", Family::SHA2_384) .with_description ("use SHA2-384");
			_flag.define_switch ((), "sha2-512", Family::SHA2_512) .with_description ("use SHA2-512");
			
			_flag.define_switch ((), "sha3", Family::SHA3) .with_description ("use one of SHA3");
			_flag.define_switch ((), "sha3-224", Family::SHA3_224) .with_description ("use SHA3-224");
			_flag.define_switch ((), "sha3-256", Family::SHA3_256) .with_description ("use SHA3-256");
			_flag.define_switch ((), "sha3-384", Family::SHA3_384) .with_description ("use SHA3-384");
			_flag.define_switch ((), "sha3-512", Family::SHA3_512) .with_description ("use SHA3-512");
			
			_flag.define_switch ((), "shake128", Family::Shake_128) .with_description ("use Shake128");
			_flag.define_switch ((), "shake256", Family::Shake_256) .with_description ("use Shake256");
			
			_flag.define_switch ((), "git-sha1", Family::GitSHA1) .with_description ("use Git SHA1 flavour");
			_flag.define_switch ((), "git-sha2", Family::GitSHA2) .with_description ("use Git SHA2 flavour");
			
			_flag.define_switch ((), "blake2", Family::Blake2) .with_description ("use Blake2s or Blake2b");
			_flag.define_switch ((), "blake2s", Family::Blake2s) .with_description ("use Blake2s");
			_flag.define_switch ((), "blake2b", Family::Blake2b) .with_description ("use Blake2b");
			
			_flag.define_switch ((), "blake3", Family::Blake3) .with_description ("use Blake3");
			
			_flag.define_switch ((), "siphash", Family::SipHash) .with_description ("use one of SipHash") .with_warning ("WEAK");
			_flag.define_switch ((), "siphash-64", Family::SipHash_64) .with_description ("use SipHash-64") .with_warning ("WEAK");
			_flag.define_switch ((), "siphash-128", Family::SipHash_128) .with_description ("use SipHash-128") .with_warning ("WEAK");
			
			_flag.define_switch ((), "seahash", Family::SeaHash) .with_description ("use SeaHash") .with_warning ("WEAK");
			
			_flag.define_switch ((), "highwayhash", Family::HighwayHash) .with_description ("use one of HighwayHash") .with_warning ("WEAK");
			_flag.define_switch ((), "highwayhash-64", Family::HighwayHash_64) .with_description ("use HighwayHash-64") .with_warning ("WEAK");
			_flag.define_switch ((), "highwayhash-128", Family::HighwayHash_128) .with_description ("use HighwayHash-128") .with_warning ("WEAK");
			_flag.define_switch ((), "highwayhash-256", Family::HighwayHash_256) .with_description ("use HighwayHash-256") .with_warning ("WEAK");
			
			_flag.define_switch ((), "xxhash", Family::XxHash) .with_description ("use one of xxHash") .with_warning ("WEAK");
			_flag.define_switch ((), "xxhash-32", Family::XxHash_32) .with_description ("use xxHash-32") .with_warning ("WEAK");
			_flag.define_switch ((), "xxhash-64", Family::XxHash_64) .with_description ("use xxHash-64") .with_warning ("WEAK");
			
			_flag.define_switch ((), "xxh3", Family::Xxh3) .with_description ("use Xxh3") .with_warning ("WEAK") .with_warning ("EXPERIMENTAL");
			_flag.define_switch ((), "xxh3-64", Family::Xxh3_64) .with_description ("use xxh3-64") .with_warning ("WEAK") .with_warning ("EXPERIMENTAL");
			_flag.define_switch ((), "xxh3-128", Family::Xxh3_128) .with_description ("use xxh3-128") .with_warning ("WEAK") .with_warning ("EXPERIMENTAL");
			
			_flag.define_switch ((), "djb2", Family::Djb2) .with_description ("use djb2") .with_warning ("WEAK");
			_flag.define_switch ((), "sdbm", Family::SDBM) .with_description ("use SDBM") .with_warning ("WEAK");
			
			_flag.define_switch ((), "fnv1a", Family::FNV1a) .with_description ("use one of FNV1a") .with_warning ("WEAK");
			_flag.define_switch ((), "fnv1a-32", Family::FNV1a_32) .with_description ("use FNV1a-32") .with_warning ("WEAK");
			_flag.define_switch ((), "fnv1a-64", Family::FNV1a_64) .with_description ("use FNV1a-64") .with_warning ("WEAK");
			
			_flag.define_switch ((), "crc", Family::CRC) .with_description ("use one of CRC") .with_warning ("WEAK");
			_flag.define_switch ((), "crc8", Family::CRC8) .with_description ("use CRC8") .with_warning ("WEAK");
			_flag.define_switch ((), "crc16", Family::CRC16) .with_description ("use CRC16") .with_warning ("WEAK");
			_flag.define_switch ((), "crc32", Family::CRC32) .with_description ("use CRC32") .with_warning ("WEAK");
			_flag.define_switch ((), "crc32c", Family::CRC32C) .with_description ("use CRC32C") .with_warning ("WEAK");
			_flag.define_switch ((), "crc64", Family::CRC64) .with_description ("use CRC64") .with_warning ("WEAK");
			
			_flag.define_switch ((), "adler", Family::Adler) .with_description ("use one of Adler") .with_warning ("WEAK");
			_flag.define_switch ((), "adler32", Family::Adler32) .with_description ("use Adler32") .with_warning ("WEAK");
			
			_flag.define_switch ((), "scrypt", Family::Scrypt) .with_description ("use Scrypt") .with_warning ("EXPERIMENTAL");
			
			_flag.define_switch ((), "argon2", Family::Argon2) .with_description ("use one of Argon2 family") .with_warning ("EXPERIMENTAL");
			_flag.define_switch ((), "argon2d", Family::Argon2d) .with_description ("use Argon2d") .with_warning ("EXPERIMENTAL");
			_flag.define_switch ((), "argon2i", Family::Argon2i) .with_description ("use Argon2i") .with_warning ("EXPERIMENTAL");
			_flag.define_switch ((), "argon2id", Family::Argon2id) .with_description ("use Argon2id") .with_warning ("EXPERIMENTAL");
		}
		
		{
			let _flag = _flags.define_complex (&mut _output_size);
			_flag.define_flag ('s', "size") .with_placeholder ("size") .with_description ("hash output size in bytes");
			_flag.define_switch ((), "8b", 1) .with_alias ((), "1B") .with_description ("output 8 bits / 1 byte");
			_flag.define_switch ((), "16b", 2) .with_alias ((), "2B") .with_description ("output 16 bits / 2 bytes");
			_flag.define_switch ((), "32b", 4) .with_alias ((), "4B") .with_description ("output 32 bits / 4 bytes");
			_flag.define_switch ((), "64b", 8) .with_alias ((), "8B") .with_description ("output 64 bits / 8 bytes");
			_flag.define_switch ((), "128b", 16) .with_alias ((), "16B") .with_description ("output 128 bits / 16 bytes");
			_flag.define_switch ((), "256b", 32) .with_alias ((), "32B") .with_description ("output 256 bits / 32 bytes");
			_flag.define_switch ((), "512b", 64) .with_alias ((), "64B") .with_description ("output 512 bits / 64 bytes");
			_flag.define_switch ((), "1024b", 128) .with_alias ((), "128B") .with_description ("output 1024 bits / 128 bytes");
			_flag.define_switch ((), "2048b", 256) .with_alias ((), "256B") .with_description ("output 2048 bits / 256 bytes");
			_flag.define_switch ((), "4096b", 512) .with_alias ((), "512B") .with_description ("output 4096 bits / 512 bytes");
			_flag.define_switch ((), "8192b", 1024) .with_alias ((), "1024B") .with_description ("output 8192 bits / 1024 bytes");
		}
		
		{
			let _flag = _flags.define_complex (&mut _input_sources);
			_flag.define_flag_with_wrapper ('t', "token", InputSource::String) .with_placeholder ("string") .with_description ("use this argument");
			_flag.define_flag_with_wrapper ('f', "file", |_path| InputSource::File (PathBuf::from (_path))) .with_placeholder ("path") .with_description ("read from file");
			_flag.define_switch ('i', "stdin", InputSource::Stdin) .with_description ("read from stdin");
			_flag.define_switch ('e', "empty", InputSource::Empty) .with_description ("empty");
		}
		
		{
			let _flag = _flags.define_complex (&mut _inputs_canonicalize);
			_flag.define_switch ('c', "inputs-concatenate", false) .with_description ("concatenate inputs") .with_default ("default for one input");
			_flag.define_switch ('C', "inputs-canonicalize", true) .with_description ("canonicalize inputs") .with_default ("default for two or more inputs");
		}
		
		{
			let _flag = _flags.define_complex (&mut _output_discard_right);
			_flag.define_switch ((), "output-discard-right", true) .with_description ("if needed discard bytes from the right of the hash") .with_default ("default");
			_flag.define_switch ((), "output-discard-left", false) .with_description ("if needed discard bytes from the left of the hash");
		}
		
		{
			let _flag = _flags.define_complex (&mut _output_reversed);
			_flag.define_switch ((), "output-left-to-right", false) .with_description ("copy from left-to-right bytes from the hash") .with_default ("default");
			_flag.define_switch ((), "output-right-to-left", true) .with_alias ((), "output-reversed") .with_description ("copy from right-to-left bytes from the hash");
		}
		
		if execute_flags (_flags, _arguments) .else_wrap (0x88824ad0) ? {
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


impl FlagValue for InputSource {}

impl FlagValueDisplay for InputSource {
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		fail! (0x69769807);
	}
}

impl FlagValueParsable for InputSource {
	fn parse_string (_string : String) -> FlagValueParseResult<Self> {
		fail! (0xa042006e);
	}
}


