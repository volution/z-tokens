

#![ allow (dead_code) ]




use crate::prelude::*;




pub(crate) struct EncryptArguments {
	pub senders : MaterialSources<SenderPrivateKey>,
	pub recipients : MaterialSources<RecipientPublicKey>,
	pub shared : SharedKeysArguments,
	pub ssh_wrappers : SshWrappersArguments,
	pub deterministic : bool,
}


pub(crate) struct EncryptFlags {
	pub senders : SendersPrivateFlags,
	pub recipients : RecipientsPublicFlags,
	pub shared : SharedKeysFlags,
	pub ssh_wrappers : SshWrappersFlags,
	pub common : CommonFlags,
	pub deterministic : Option<bool>,
}




pub(crate) struct DecryptArguments {
	pub recipients : MaterialSources<RecipientPrivateKey>,
	pub senders : MaterialSources<SenderPublicKey>,
	pub shared : SharedKeysArguments,
	pub ssh_wrappers : SshWrappersArguments,
}


pub(crate) struct DecryptFlags {
	pub recipients : RecipientsPrivateFlags,
	pub senders : SendersPublicFlags,
	pub shared : SharedKeysFlags,
	pub ssh_wrappers : SshWrappersFlags,
	pub common : CommonFlags,
}




pub(crate) struct PasswordArguments {
	pub senders : MaterialSources<SenderPrivateKey>,
	pub recipients : MaterialSources<RecipientPublicKey>,
	pub shared : SharedKeysArguments,
	pub ssh_wrappers : SshWrappersArguments,
}


pub(crate) struct PasswordFlags {
	pub senders : SendersPrivateFlags,
	pub recipients : RecipientsPublicFlags,
	pub shared : SharedKeysFlags,
	pub ssh_wrappers : SshWrappersFlags,
	pub common : CommonFlags,
}








pub(crate) struct SharedKeysArguments {
	pub associated : MaterialSources<Associated>,
	pub secrets : MaterialSources<SharedSecret>,
	pub pins : MaterialSources<SharedPin>,
	pub seeds : MaterialSources<SharedSeed>,
	pub ballasts : MaterialSources<SharedBallast>,
}


pub(crate) struct SharedKeysFlags {
	pub associated : AssociatedFlags,
	pub secrets : SecretsFlags,
	pub pins : PinsFlags,
	pub seeds : SeedsFlags,
	pub ballasts : BallastsFlags,
}



pub(crate) struct SshWrappersArguments {
	pub keys : MaterialSources<SshWrapperKey>,
}


pub(crate) struct SshWrappersFlags {
	pub keys : MaterialFlags,
}




pub(crate) struct CommonArguments {
	pub empty_is_missing : bool,
}


pub(crate) struct CommonFlags {
	pub empty_is_missing : Option<bool>,
}




pub(crate) struct MaterialFlags {
	pub values : Vec<OsString>,
	pub from_environment : Vec<OsString>,
	pub from_file : Vec<OsString>,
	#[ cfg (unix) ]
	pub from_fd : Vec<c_int>,
	pub from_stdin : Option<bool>,
	pub from_pinentry : Vec<String>,
}


pub(crate) struct MaterialSources<Material>
	where
		Material : MaterialValue,
{
	pub sources : Vec<MaterialSource<Material>>,
}


pub(crate) enum MaterialData {
	String (String),
	OsString (OsString),
	Bytes (Vec<u8>),
}


pub(crate) enum MaterialSource<Material>
	where
		Material : MaterialValue,
{
	Material (Material),
	FromString (OsString, bool),
	FromEnvironment (OsString, bool),
	FromFile (PathBuf, bool),
	#[ cfg (unix) ]
	FromFd (OwnedFd, bool),
	FromStdin (bool),
	FromPinentry (String, bool),
}


pub(crate) trait MaterialValue
	where
		Self : Sized,
{
	fn decode_string (_string : String) -> FlagsResult<Self>;
	
	fn decode_os_string (_string : OsString) -> FlagsResult<Self> {
		let _string = _string.into_string () .else_replace (0x5235a54a) ?;
		Self::decode_string (_string)
	}
	
	fn decode_bytes (_bytes : Vec<u8>) -> FlagsResult<Self> {
		let _string = String::from_utf8 (_bytes) .else_wrap (0x95e4ab54) ?;
		Self::decode_string (_string)
	}
	
	fn decode_data (_data : MaterialData) -> FlagsResult<Self> {
		match _data {
			MaterialData::String (_data) =>
				Self::decode_string (_data),
			MaterialData::OsString (_data) =>
				Self::decode_os_string (_data),
			MaterialData::Bytes (_data) =>
				Self::decode_bytes (_data),
		}
	}
}




pub(crate) struct SendersPrivateFlags {
	pub materials : MaterialFlags,
}

pub(crate) struct SendersPublicFlags {
	pub materials : MaterialFlags,
}

pub(crate) struct RecipientsPrivateFlags {
	pub materials : MaterialFlags,
}

pub(crate) struct RecipientsPublicFlags {
	pub materials : MaterialFlags,
}


pub(crate) struct AssociatedFlags {
	pub materials : MaterialFlags,
}

pub(crate) struct SecretsFlags {
	pub materials : MaterialFlags,
}

pub(crate) struct PinsFlags {
	pub materials : MaterialFlags,
}

pub(crate) struct SeedsFlags {
	pub materials : MaterialFlags,
}

pub(crate) struct BallastsFlags {
	pub materials : MaterialFlags,
}








impl MaterialFlags {
	
	pub fn new () -> Self {
		Self {
				values : Vec::new (),
				from_environment : Vec::new (),
				from_file : Vec::new (),
				#[ cfg (unix) ]
				from_fd : Vec::new (),
				from_stdin : None,
				from_pinentry : Vec::new (),
			}
	}
	
	pub fn flags <'a> (
				&'a mut self,
				_flags : &mut FlagsParserBuilder<'a>,
				_short_values : char,
				_long_values : &'static str,
				_long_environment : &'static str,
				_long_file : &'static str,
				_long_fd : &'static str,
				_long_stdin : &'static str,
				_long_pinentry : &'static str,
				_description : &'static str,
			) -> FlagsResult
	{
		_flags.define_multiple_flag_0 (&mut self.values)
				.with_flag (_short_values, _long_values)
				.with_placeholder ("string")
				.with_description (_description);
		
		_flags.define_multiple_flag_0 (&mut self.from_environment)
				.with_flag ((), _long_environment)
				.with_placeholder ("variable")
				.with_description ("from environment");
		
		_flags.define_multiple_flag_0 (&mut self.from_file)
				.with_flag ((), _long_file)
				.with_placeholder ("path")
				.with_description ("from file");
		
		#[ cfg (unix) ]
		_flags.define_multiple_flag_0 (&mut self.from_fd)
				.with_flag ((), _long_fd)
				.with_placeholder ("fd")
				.with_description ("from file-descriptor");
		
		_flags.define_switch_0 (&mut self.from_stdin)
				.with_flag ((), _long_stdin)
				.with_description ("from stdin");
		
		_flags.define_multiple_flag_0 (&mut self.from_pinentry)
				.with_flag ((), _long_pinentry)
				.with_placeholder ("prompt")
				.with_description ("via pinentry");
		
		Ok (())
	}
}








impl SendersPrivateFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				's',
				"sender", "sender-env", "sender-path", "sender-fd", "sender-stdin", "sender-pinentry",
				"sender private key (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<SenderPrivateKey>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for SenderPrivateKey {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0x979c45a5)
	}
}




impl SendersPublicFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				's',
				"sender", "sender-env", "sender-path", "sender-fd", "sender-stdin", "sender-pinentry",
				"sender public key (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<SenderPublicKey>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for SenderPublicKey {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0x75722db6)
	}
}




impl RecipientsPrivateFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				'r',
				"recipient", "recipient-env", "recipient-path", "recipient-fd", "recipient-stdin", "recipient-pinentry",
				"recipient private key (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<RecipientPrivateKey>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for RecipientPrivateKey {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0x283cd31f)
	}
}




impl RecipientsPublicFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				'r',
				"recipient", "recipient-env", "recipient-path", "recipient-fd", "recipient-stdin", "recipient-pinentry",
				"recipient public key (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<RecipientPublicKey>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for RecipientPublicKey {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0x8a455c2a)
	}
}




impl AssociatedFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				'a',
				"associated", "associated-env", "associated-path", "associated-fd", "associated-stdin", "associated-pinentry",
				"associated data (multiple allowed, **order and duplicates are significant**)",
			)
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<Associated>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for Associated {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_bytes (_string.into_bytes ())
	}
	
	fn decode_bytes (_bytes : Vec<u8>) -> FlagsResult<Self> {
		Ok (Self::new (_bytes))
	}
}




impl SecretsFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				'x',
				"secret", "secret-env", "secret-path", "secret-fd", "secret-stdin", "secret-pinentry",
				"shared secret, for additional security (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<SharedSecret>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for SharedSecret {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0xa3f7aa05)
	}
}




impl PinsFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				'e',
				"pin", "pin-env", "pin-path", "pin-fd", "pin-stdin", "pin-pinentry",
				"shared PIN, for **WEAK** additional security (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<SharedPin>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for SharedPin {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_bytes (_string.into_bytes ())
	}
	
	fn decode_bytes (_bytes : Vec<u8>) -> FlagsResult<Self> {
		Ok (Self::new (_bytes))
	}
}




impl SeedsFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				'e',
				"seed", "seed-env", "seed-path", "seed-fd", "seed-stdin", "seed-pinentry",
				"shared seed, for additional security (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<SharedSeed>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for SharedSeed {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0xe0add2f9)
	}
}




impl BallastsFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				'b',
				"ballast", "ballast-env", "ballast-path", "ballast-fd", "ballast-stdin", "ballast-pinentry",
				"shared ballast, for additional security (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<SharedBallast>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for SharedBallast {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0xfe1c8e20)
	}
}




impl SshWrappersFlags {
	
	pub fn new () -> Self {
		Self {
				keys : MaterialFlags::new (),
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.keys.flags (
				_flags,
				'S',
				"ssh-wrap", "ssh-wrap-env", "ssh-wrap-path", "ssh-wrap-fd", "ssh-wrap-stdin", "ssh-wrap-pinentry",
				"shared SSH agent key handle (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<SshWrappersArguments> {
		Ok (SshWrappersArguments {
				keys : self.keys.collect (_empty_is_missing) ?,
			})
	}
}


impl MaterialValue for SshWrapperKey {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0x1c720b43)
	}
}


impl SshWrappersArguments {
	
	pub fn wrappers (self) -> SshResult<Vec<SshWrapper>> {
		let _keys = self.keys.decode () .else_wrap (0x42f1fa1d) ?;
		_keys.into_iter () .map (SshWrapper::connect) .collect ()
	}
}








impl EncryptFlags {
	
	pub fn new () -> Self {
		Self {
				senders : SendersPrivateFlags::new (),
				recipients : RecipientsPublicFlags::new (),
				shared : SharedKeysFlags::new (),
				ssh_wrappers : SshWrappersFlags::new (),
				common : CommonFlags::new (),
				deterministic : None,
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.senders.flags (_flags) ?;
		self.recipients.flags (_flags) ?;
		self.shared.flags (_flags) ?;
		self.ssh_wrappers.flags (_flags) ?;
		self.common.flags (_flags) ?;
		
		_flags.define_switch_0 (&mut self.deterministic)
				.with_flag ((), "siv")
				.with_description ("deterministic output, based on SIV")
				.with_warning ("CAUTION");
		
		Ok (())
	}
	
	pub fn arguments (self) -> FlagsResult<EncryptArguments> {
		
		let _common = self.common.arguments () ?;
		let _senders = self.senders.arguments (_common.empty_is_missing) ?;
		let _recipients = self.recipients.arguments (_common.empty_is_missing) ?;
		let _shared = self.shared.arguments (_common.empty_is_missing) ?;
		let _ssh_wrappers = self.ssh_wrappers.arguments (_common.empty_is_missing) ?;
		
		Ok (EncryptArguments {
				senders : _senders,
				recipients : _recipients,
				shared : _shared,
				ssh_wrappers : _ssh_wrappers,
				deterministic : self.deterministic.unwrap_or (false),
			})
	}
}




impl DecryptFlags {
	
	pub fn new () -> Self {
		Self {
				recipients : RecipientsPrivateFlags::new (),
				senders : SendersPublicFlags::new (),
				shared : SharedKeysFlags::new (),
				ssh_wrappers : SshWrappersFlags::new (),
				common : CommonFlags::new (),
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.recipients.flags (_flags) ?;
		self.senders.flags (_flags) ?;
		self.shared.flags (_flags) ?;
		self.ssh_wrappers.flags (_flags) ?;
		self.common.flags (_flags) ?;
		
		Ok (())
	}
	
	pub fn arguments (self) -> FlagsResult<DecryptArguments> {
		
		let _common = self.common.arguments () ?;
		let _recipients = self.recipients.arguments (_common.empty_is_missing) ?;
		let _senders = self.senders.arguments (_common.empty_is_missing) ?;
		let _shared = self.shared.arguments (_common.empty_is_missing) ?;
		let _ssh_wrappers = self.ssh_wrappers.arguments (_common.empty_is_missing) ?;
		
		Ok (DecryptArguments {
				recipients : _recipients,
				senders : _senders,
				shared : _shared,
				ssh_wrappers : _ssh_wrappers,
			})
	}
}




impl PasswordFlags {
	
	pub fn new () -> Self {
		Self {
				senders : SendersPrivateFlags::new (),
				recipients : RecipientsPublicFlags::new (),
				shared : SharedKeysFlags::new (),
				ssh_wrappers : SshWrappersFlags::new (),
				common : CommonFlags::new (),
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.senders.flags (_flags) ?;
		self.recipients.flags (_flags) ?;
		self.shared.flags (_flags) ?;
		self.ssh_wrappers.flags (_flags) ?;
		self.common.flags (_flags) ?;
		
		Ok (())
	}
	
	pub fn arguments (self) -> FlagsResult<PasswordArguments> {
		
		let _common = self.common.arguments () ?;
		let _senders = self.senders.arguments (_common.empty_is_missing) ?;
		let _recipients = self.recipients.arguments (_common.empty_is_missing) ?;
		let _shared = self.shared.arguments (_common.empty_is_missing) ?;
		let _ssh_wrappers = self.ssh_wrappers.arguments (_common.empty_is_missing) ?;
		
		Ok (PasswordArguments {
				senders : _senders,
				recipients : _recipients,
				shared : _shared,
				ssh_wrappers : _ssh_wrappers,
			})
	}
}








impl SharedKeysFlags {
	
	pub fn new () -> Self {
		Self {
				associated : AssociatedFlags::new (),
				secrets : SecretsFlags::new (),
				pins : PinsFlags::new (),
				seeds : SeedsFlags::new (),
				ballasts : BallastsFlags::new (),
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.associated.flags (_flags) ?;
		self.secrets.flags (_flags) ?;
		self.pins.flags (_flags) ?;
		self.seeds.flags (_flags) ?;
		self.ballasts.flags (_flags) ?;
		Ok (())
	}
	
	pub fn arguments (self, _empty_is_missing : bool) -> FlagsResult<SharedKeysArguments> {
		let _associated = self.associated.arguments (_empty_is_missing) ?;
		let _secrets = self.secrets.arguments (_empty_is_missing) ?;
		let _pins = self.pins.arguments (_empty_is_missing) ?;
		let _seeds = self.seeds.arguments (_empty_is_missing) ?;
		let _ballasts = self.ballasts.arguments (_empty_is_missing) ?;
		Ok (SharedKeysArguments {
				associated : _associated,
				secrets : _secrets,
				pins : _pins,
				seeds : _seeds,
				ballasts : _ballasts,
			})
	}
}




impl CommonFlags {
	
	pub fn new () -> Self {
		Self {
				empty_is_missing : None,
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		_flags.define_single_flag_0 (&mut self.empty_is_missing)
				.with_flag ((), "empty-is-missing")
				.with_placeholder ("bool")
				.with_description ("treat empty arguments as unspecified")
				.with_warning ("CAUTION");
		
		Ok (())
	}
	
	pub fn arguments (&self) -> FlagsResult<CommonArguments> {
		Ok (CommonArguments {
				empty_is_missing : self.empty_is_missing.unwrap_or (false),
			})
	}
}








impl MaterialFlags {
	
	pub fn collect <Material> (&self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<Material>>
		where
			Material : MaterialValue,
	{
		
		let mut _sources = Vec::new ();
		
		for _value in self.values.iter () {
			if _empty_is_missing && _value.is_empty () {
				continue;
			}
			let _source = MaterialSource::FromString (_value.clone (), false);
			_sources.push (_source);
		}
		
		for _variable in self.from_environment.iter () {
			if _empty_is_missing && _variable.is_empty () {
				continue;
			}
			let _source = MaterialSource::FromEnvironment (_variable.clone (), _empty_is_missing);
			_sources.push (_source);
		}
		
		for _path in self.from_file.iter () {
			if _empty_is_missing && _path.is_empty () {
				continue;
			}
			let _source = MaterialSource::FromFile (PathBuf::from (_path), _empty_is_missing);
			_sources.push (_source);
		}
		
		#[ cfg (unix) ]
		for _descriptor in self.from_fd.iter () {
			let _descriptor = unsafe { OwnedFd::from_raw_fd (*_descriptor) };
			let _source = MaterialSource::FromFd (_descriptor, _empty_is_missing);
			_sources.push (_source);
		}
		
		if self.from_stdin.unwrap_or (false) {
			let _source = MaterialSource::FromStdin (_empty_is_missing);
			_sources.push (_source);
		}
		
		for _prompt in self.from_pinentry.iter () {
			if _empty_is_missing && _prompt.is_empty () {
				continue;
			}
			let _source = MaterialSource::FromPinentry (String::from (_prompt), _empty_is_missing);
			_sources.push (_source);
		}
		
		Ok (MaterialSources {
				sources : _sources,
			})
	}
}




impl <Material> MaterialSources<Material>
	where
		Material : MaterialValue,
{
	pub fn decode (self) -> FlagsResult<Vec<Material>> {
		self.decode_with (|_data| Ok (Some (Material::decode_data (_data) ?)))
	}
	
	pub fn decode_with <Decoder> (self, _decoder : Decoder) -> FlagsResult<Vec<Material>>
		where
			Decoder : Fn (MaterialData) -> FlagsResult<Option<Material>>,
	{
		let mut _values = Vec::with_capacity (self.sources.len ());
		for _source in self.sources.into_iter () {
			if let Some (_value) = _source.decode_with (&_decoder) ? {
				_values.push (_value);
			}
		}
		Ok (_values)
	}
}




impl <Material> MaterialSource<Material>
	where
		Material : MaterialValue,
{
	fn decode (self) -> FlagsResult<Option<Material>> {
		self.decode_with (|_data| Ok (Some (Material::decode_data (_data) ?)))
	}
	
	fn decode_with <Decoder> (self, _decoder : Decoder) -> FlagsResult<Option<Material>>
		where
			Decoder : Fn (MaterialData) -> FlagsResult<Option<Material>>,
	{
		fn _read_to_end (mut _stream : impl Read, _empty_is_missing : bool) -> FlagsResult<Option<MaterialData>> {
			let mut _bytes = Vec::new ();
			_stream.read_to_end (&mut _bytes) .else_wrap (0xb31a5f63) ?;
			if _empty_is_missing && _bytes.is_empty () {
				Ok (None)
			} else {
				Ok (Some (MaterialData::Bytes (_bytes)))
			}
		}
		
		let _data = match self {
			
			MaterialSource::Material (_value) =>
				return Ok (Some (_value)),
			
			MaterialSource::FromString (_data, _empty_is_missing) =>
				if _empty_is_missing && _data.is_empty () {
					None
				} else {
					Some (MaterialData::OsString (_data))
				}
			
			MaterialSource::FromEnvironment (_variable, _empty_is_missing) =>
				if let Some (_data) = var_os (_variable) {
					if _empty_is_missing && _data.is_empty () {
						None
					} else {
						Some (MaterialData::OsString (_data))
					}
				} else {
					if _empty_is_missing {
						None
					} else {
						fail! (0xae8eecbe);
					}
				}
			
			MaterialSource::FromFile (_path, _empty_is_missing) =>
				_read_to_end (File::open (_path) .else_wrap (0xd6609363) ?, _empty_is_missing) ?,
			
			#[ cfg (unix) ]
			MaterialSource::FromFd (_descriptor, _empty_is_missing) =>
				_read_to_end (File::from (_descriptor), _empty_is_missing) ?,
			
			MaterialSource::FromStdin (_empty_is_missing) =>
				_read_to_end (stdin_locked (), _empty_is_missing) ?,
			
			MaterialSource::FromPinentry (_prompt, _empty_is_missing) => {
				let _prompt = if ! _prompt.is_empty () {
						Some (_prompt.as_str ())
					} else {
						None
					};
				let _password = pinentry_password (_prompt, None, None) .else_wrap (0xe1aade49) ?;
				let Some (_password) = _password
					else {
						fail! (0x286a7d3c);
					};
				if _empty_is_missing && _password.is_empty () {
					None
				} else {
					Some (MaterialData::String (_password))
				}
			}
		};
		
		if let Some (_data) = _data {
			_decoder (_data)
		} else {
			Ok (None)
		}
	}
}


