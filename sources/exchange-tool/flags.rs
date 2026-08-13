

#![ allow (dead_code) ]




use crate::prelude::*;




pub(crate) struct EncryptArguments {
	pub senders : MaterialSources<SenderPrivateKey>,
	pub recipients : MaterialSources<RecipientPublicKey>,
	pub shared : SharedKeysArguments,
	pub ssh_wrappers : SshWrappersArguments,
	pub common : CommonArguments,
	pub fingerprint : bool,
	pub deterministic : bool,
	pub randomness : MaterialSources<Randomness>,
}


pub(crate) struct EncryptFlags {
	pub senders : SendersPrivateFlags,
	pub recipients : RecipientsPublicFlags,
	pub shared : SharedKeysFlags,
	pub ssh_wrappers : SshWrappersFlags,
	pub common : CommonFlags,
	pub fingerprint : Option<bool>,
	pub deterministic : Option<bool>,
	pub randomness : MaterialFlags,
}




pub(crate) struct DecryptArguments {
	pub recipients : MaterialSources<RecipientPrivateKey>,
	pub senders : MaterialSources<SenderPublicKey>,
	pub shared : SharedKeysArguments,
	pub ssh_wrappers : SshWrappersArguments,
	pub common : CommonArguments,
	pub fingerprint : Option<Fingerprint>,
}


pub(crate) struct DecryptFlags {
	pub recipients : RecipientsPrivateFlags,
	pub senders : SendersPublicFlags,
	pub shared : SharedKeysFlags,
	pub ssh_wrappers : SshWrappersFlags,
	pub common : CommonFlags,
	pub fingerprint : DecryptFingerprintFlag,
}


#[ derive (Clone) ]
pub(crate) enum DecryptFingerprintFlag {
	Without,
	Expected (String),
}




pub(crate) struct PasswordArguments {
	pub inputs : InputsArguments,
	pub senders : MaterialSources<SenderPublicOrPrivateKey>,
	pub recipients : MaterialSources<RecipientPublicOrPrivateKey>,
	pub shared : SharedKeysArguments,
	pub ssh_wrappers : SshWrappersArguments,
	pub common : CommonArguments,
	pub fingerprint : Option<Option<Fingerprint>>,
}


pub(crate) struct PasswordFlags {
	pub inputs : InputsFlags,
	pub senders : SendersPublicOrPrivateFlags,
	pub recipients : RecipientsPublicOrPrivateFlags,
	pub shared : SharedKeysFlags,
	pub ssh_wrappers : SshWrappersFlags,
	pub common : CommonFlags,
	pub fingerprint : PasswordFingerprintFlag,
}


#[ derive (Clone) ]
pub(crate) enum PasswordFingerprintFlag {
	Without,
	Generate,
	Expected (String),
}




pub(crate) struct InputsArguments {
	pub inputs : MaterialSources<Vec<u8>>,
	pub canonicalize : Option<bool>,
}


pub(crate) struct InputsFlags {
	pub inputs : MaterialFlags,
	pub canonicalize : Option<bool>,
}








pub(crate) struct SharedKeysArguments {
	pub associated : AssociatedArguments,
	pub secrets : SecretsArguments,
	pub pins : PinsArguments,
	pub seeds : SeedsArguments,
	pub ballasts : BallastsArguments,
	pub derivation_loops : Option<NonZeroU64>,
}


pub(crate) struct SharedKeysFlags {
	pub associated : AssociatedFlags,
	pub secrets : SecretsFlags,
	pub pins : PinsFlags,
	pub seeds : SeedsFlags,
	pub ballasts : BallastsFlags,
	pub derivation_loops : Option<u64>,
}




pub(crate) struct SshWrappersArguments {
	pub keys : MaterialSources<SshWrapperKeyOrHandle>,
}


pub(crate) struct SshWrappersFlags {
	pub keys : MaterialFlags,
}




pub(crate) struct CommonArguments {
	pub namespace : Option<String>,
	pub empty_is_missing : bool,
}


pub(crate) struct CommonFlags {
	pub namespace : Option<String>,
	pub empty_is_missing : Option<bool>,
}




pub(crate) struct MaterialFlags {
	pub values : Vec<OsString>,
	pub from_environment : Vec<OsString>,
	pub from_file : Vec<OsString>,
	#[ cfg (target_family = "unix") ]
	pub from_fd : Vec<c_int>,
	pub from_stdin : Option<bool>,
	pub from_pinentry : Vec<String>,
	#[ cfg (all (target_os = "linux", target_env = "gnu")) ]
	pub from_lkkrs : Vec<String>,
	pub without : Option<bool>,
}


pub(crate) struct MaterialSources<Material>
	where
		Material : MaterialValue,
{
	pub sources : Vec<MaterialSource<Material>>,
	pub without : bool,
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
	#[ cfg (target_family = "unix") ]
	FromFd (OwnedFd, bool),
	FromStdin (bool),
	FromPinentry (String, bool),
	#[ cfg (all (target_os = "linux", target_env = "gnu")) ]
	FromLkkrs (String, bool),
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

pub(crate) struct SendersPublicOrPrivateFlags {
	pub materials : MaterialFlags,
}

pub(crate) struct RecipientsPrivateFlags {
	pub materials : MaterialFlags,
}

pub(crate) struct RecipientsPublicFlags {
	pub materials : MaterialFlags,
}

pub(crate) struct RecipientsPublicOrPrivateFlags {
	pub materials : MaterialFlags,
}




pub(crate) struct AssociatedArguments {
	pub tokens : MaterialSources<Associated>,
}

pub(crate) struct AssociatedFlags {
	pub materials : MaterialFlags,
}


pub(crate) struct SecretsArguments {
	pub encoded : MaterialSources<SharedSecret>,
	pub raw : MaterialSources<SharedSecretRaw>,
}

pub(crate) struct SecretsFlags {
	pub materials : MaterialFlags,
	pub materials_raw : MaterialFlags,
}


pub(crate) struct PinsArguments {
	pub tokens : MaterialSources<SharedPin>,
}

pub(crate) struct PinsFlags {
	pub materials : MaterialFlags,
}


pub(crate) struct SeedsArguments {
	pub encoded : MaterialSources<SharedSeed>,
	pub raw : MaterialSources<SharedSeedRaw>,
}

pub(crate) struct SeedsFlags {
	pub materials : MaterialFlags,
	pub materials_raw : MaterialFlags,
}


pub(crate) struct BallastsArguments {
	pub encoded : MaterialSources<SharedBallast>,
	pub raw : MaterialSources<SharedBallastRaw>,
}

pub(crate) struct BallastsFlags {
	pub materials : MaterialFlags,
	pub materials_raw : MaterialFlags,
}








impl MaterialFlags {
	
	pub fn new () -> Self {
		Self {
				values : Vec::new (),
				from_environment : Vec::new (),
				from_file : Vec::new (),
				#[ cfg (target_family = "unix") ]
				from_fd : Vec::new (),
				from_stdin : None,
				from_pinentry : Vec::new (),
				#[ cfg (all (target_os = "linux", target_env = "gnu")) ]
				from_lkkrs : Vec::new (),
				without : None,
			}
	}
	
	pub fn flags <'a> (
				&'a mut self,
				_flags : &mut FlagsParserBuilder<'a>,
				_short_values : Option<char>,
				_long_values : &'static str,
				_long_environment : &'static str,
				_long_file : &'static str,
				_long_fd : &'static str,
				_long_stdin : &'static str,
				_long_pinentry : &'static str,
				_long_lkkrs : &'static str,
				_long_without : Option<&'static str>,
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
		
		#[ cfg (target_family = "unix") ]
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
		
		#[ cfg (all (target_os = "linux", target_env = "gnu")) ]
		_flags.define_multiple_flag_0 (&mut self.from_lkkrs)
				.with_flag ((), _long_lkkrs)
				.with_placeholder ("selector")
				.with_description ("from Linux Kernel key-retention-service");
		
		if let Some (_long_without) = _long_without {
			_flags.define_switch_0 (&mut self.without)
					.with_flag ((), _long_without);
		}
		
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
				Some ('s'),
				"sender", "sender-env", "sender-path", "sender-fd", "sender-stdin", "sender-pinentry", "sender-lkkrs", Some ("without-sender"),
				"sender private key (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<SenderPrivateKey>> {
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
				Some ('s'),
				"sender", "sender-env", "sender-path", "sender-fd", "sender-stdin", "sender-pinentry", "sender-lkkrs", Some ("without-sender"),
				"sender public key (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<SenderPublicKey>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for SenderPublicKey {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0x75722db6)
	}
}




impl SendersPublicOrPrivateFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				Some ('s'),
				"sender", "sender-env", "sender-path", "sender-fd", "sender-stdin", "sender-pinentry", "sender-lkkrs", Some ("without-sender"),
				"sender public or private key (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<SenderPublicOrPrivateKey>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for SenderPublicOrPrivateKey {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0x764f3b24)
	}
}




impl RecipientsPrivateFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				Some ('r'),
				"recipient", "recipient-env", "recipient-path", "recipient-fd", "recipient-stdin", "recipient-pinentry", "recipient-lkkrs", Some ("without-recipient"),
				"recipient private key (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<RecipientPrivateKey>> {
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
				Some ('r'),
				"recipient", "recipient-env", "recipient-path", "recipient-fd", "recipient-stdin", "recipient-pinentry", "recipient-lkkrs", Some ("without-recipient"),
				"recipient public key (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<RecipientPublicKey>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for RecipientPublicKey {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0x8a455c2a)
	}
}




impl RecipientsPublicOrPrivateFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				Some ('r'),
				"recipient", "recipient-env", "recipient-path", "recipient-fd", "recipient-stdin", "recipient-pinentry", "recipient-lkkrs", Some ("without-recipient"),
				"recipient public or private key (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<MaterialSources<RecipientPublicOrPrivateKey>> {
		self.materials.collect (_empty_is_missing)
	}
}


impl MaterialValue for RecipientPublicOrPrivateKey {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0x9999c378)
	}
}




impl AssociatedArguments {
	
	pub fn load (self) -> FlagsResult<Vec<Associated>> {
		let _tokens = self.tokens.decode () ?;
		Ok (_tokens)
	}
}


impl AssociatedFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				Some ('a'),
				"associated", "associated-env", "associated-path", "associated-fd", "associated-stdin", "associated-pinentry", "associated-lkkrs", Some ("without-associated"),
				"associated data (multiple allowed, **order and duplicates are significant**)",
			)
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<AssociatedArguments> {
		let _tokens = self.materials.collect (_empty_is_missing) ?;
		Ok (AssociatedArguments {
				tokens : _tokens,
			})
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




impl SecretsArguments {
	
	pub fn load_merge (self) -> FlagsResult<Vec<Box<dyn SharedSecretTrait>>> {
		let _without = self.encoded.without || self.raw.without;
		let _encoded = self.encoded.decode () ?;
		let _raw = self.raw.decode () ?;
		let _merged = Iterator::chain (
				_encoded.into_iter () .map (|_value| Box::new (_value) as Box<dyn SharedSecretTrait>),
				_raw.into_iter () .map (|_value| Box::new (_value) as Box<dyn SharedSecretTrait>),
			);
		let _merged = _merged.collect::<Vec<_>> ();
		if _without && ! _merged.is_empty () {
			fail! (0x2eb1a6d8);
		}
		Ok (_merged)
	}
}


impl SecretsFlags {
	
	pub fn new () -> Self {
		Self {
				materials : MaterialFlags::new (),
				materials_raw : MaterialFlags::new (),
			 }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.materials.flags (
				_flags,
				Some ('x'),
				"secret", "secret-env", "secret-path", "secret-fd", "secret-stdin", "secret-pinentry", "secret-lkkrs", Some ("without-secret"),
				"shared secret, for additional security (multiple allowed, in any order, deduplicated)",
			) ?;
		
		self.materials_raw.flags (
				_flags,
				None,
				"raw-secret", "raw-secret-env", "raw-secret-path", "raw-secret-fd", "raw-secret-stdin", "raw-secret-pinentry", "raw-secret-lkkrs", None,
				"(raw) shared secret, for additional security (multiple allowed, in any order, deduplicated)",
			) ?;
		
		Ok (())
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<SecretsArguments> {
		let _encoded = self.materials.collect (_empty_is_missing) ?;
		let _raw = self.materials_raw.collect (_empty_is_missing) ?;
		Ok (SecretsArguments {
				encoded : _encoded,
				raw : _raw,
			})
	}
}


impl MaterialValue for SharedSecret {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0xa3f7aa05)
	}
}


impl MaterialValue for SharedSecretRaw {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_bytes (_string.into_bytes ())
	}
	
	fn decode_bytes (_bytes : Vec<u8>) -> FlagsResult<Self> {
		Ok (Self::new (_bytes))
	}
}




impl PinsArguments {
	
	pub fn load (self) -> FlagsResult<Vec<SharedPin>> {
		let _tokens = self.tokens.decode () ?;
		Ok (_tokens)
	}
}


impl PinsFlags {
	
	pub fn new () -> Self {
		Self { materials : MaterialFlags::new () }
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (
				_flags,
				Some ('e'),
				"pin", "pin-env", "pin-path", "pin-fd", "pin-stdin", "pin-pinentry", "pin-lkkrs", Some ("without-pin"),
				"shared PIN, for **WEAK** additional security (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<PinsArguments> {
		let _tokens = self.materials.collect (_empty_is_missing) ?;
		Ok (PinsArguments {
				tokens : _tokens,
			})
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




impl SeedsArguments {
	
	pub fn load_merge (self) -> FlagsResult<Vec<Box<dyn SharedSeedTrait>>> {
		let _without = self.encoded.without || self.raw.without;
		let _encoded = self.encoded.decode () ?;
		let _raw = self.raw.decode () ?;
		let _merged = Iterator::chain (
				_encoded.into_iter () .map (|_value| Box::new (_value) as Box<dyn SharedSeedTrait>),
				_raw.into_iter () .map (|_value| Box::new (_value) as Box<dyn SharedSeedTrait>),
			);
		let _merged = _merged.collect::<Vec<_>> ();
		if _without && ! _merged.is_empty () {
			fail! (0x6c5705ea);
		}
		Ok (_merged)
	}
}


impl SeedsFlags {
	
	pub fn new () -> Self {
		Self {
				materials : MaterialFlags::new (),
				materials_raw : MaterialFlags::new (),
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.materials.flags (
				_flags,
				Some ('e'),
				"seed", "seed-env", "seed-path", "seed-fd", "seed-stdin", "seed-pinentry", "seed-lkkrs", Some ("without-seed"),
				"shared seed, for additional security (multiple allowed, in any order, deduplicated)",
			) ?;
		
		self.materials_raw.flags (
				_flags,
				None,
				"raw-seed", "raw-seed-env", "raw-seed-path", "raw-seed-fd", "raw-seed-stdin", "raw-seed-pinentry", "raw-seed-lkkrs", None,
				"(raw) shared seed, for additional security (multiple allowed, in any order, deduplicated)",
			) ?;
		
		Ok (())
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<SeedsArguments> {
		let _encoded = self.materials.collect (_empty_is_missing) ?;
		let _raw = self.materials_raw.collect (_empty_is_missing) ?;
		Ok (SeedsArguments {
				encoded : _encoded,
				raw : _raw,
			})
	}
}


impl MaterialValue for SharedSeed {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0xe0add2f9)
	}
}


impl MaterialValue for SharedSeedRaw {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_bytes (_string.into_bytes ())
	}
	
	fn decode_bytes (_bytes : Vec<u8>) -> FlagsResult<Self> {
		Ok (Self::new (_bytes))
	}
}




impl BallastsArguments {
	
	pub fn load_merge (self) -> FlagsResult<Vec<Box<dyn SharedBallastTrait>>> {
		let _without = self.encoded.without || self.raw.without;
		let _encoded = self.encoded.decode () ?;
		let _raw = self.raw.decode () ?;
		let _merged = Iterator::chain (
				_encoded.into_iter () .map (|_value| Box::new (_value) as Box<dyn SharedBallastTrait>),
				_raw.into_iter () .map (|_value| Box::new (_value) as Box<dyn SharedBallastTrait>),
			);
		let _merged = _merged.collect::<Vec<_>> ();
		if _without && ! _merged.is_empty () {
			fail! (0x004eecfb);
		}
		Ok (_merged)
	}
}


impl BallastsFlags {
	
	pub fn new () -> Self {
		Self {
				materials : MaterialFlags::new (),
				materials_raw : MaterialFlags::new (),
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.materials.flags (
				_flags,
				Some ('b'),
				"ballast", "ballast-env", "ballast-path", "ballast-fd", "ballast-stdin", "ballast-pinentry", "ballast-lkkrs", Some ("without-ballast"),
				"shared ballast, for additional security (multiple allowed, in any order, deduplicated)",
			) ?;
		
		self.materials_raw.flags (
				_flags,
				None,
				"raw-ballast", "raw-ballast-env", "raw-ballast-path", "raw-ballast-fd", "raw-ballast-stdin", "raw-ballast-pinentry", "raw-ballast-lkkrs", None,
				"(raw) shared ballast, for additional security (multiple allowed, in any order, deduplicated)",
			) ?;
		
		Ok (())
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<BallastsArguments> {
		let _encoded = self.materials.collect (_empty_is_missing) ?;
		let _raw = self.materials_raw.collect (_empty_is_missing) ?;
		Ok (BallastsArguments {
				encoded : _encoded,
				raw : _raw,
			})
	}
}


impl MaterialValue for SharedBallast {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0xfe1c8e20)
	}
}


impl MaterialValue for SharedBallastRaw {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_bytes (_string.into_bytes ())
	}
	
	fn decode_bytes (_bytes : Vec<u8>) -> FlagsResult<Self> {
		Ok (Self::new (_bytes))
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
				Some ('S'),
				"ssh-wrap", "ssh-wrap-env", "ssh-wrap-path", "ssh-wrap-fd", "ssh-wrap-stdin", "ssh-wrap-pinentry", "ssh-wrap-lkkrs", Some ("without-ssh-wrap"),
				"shared SSH agent key handle (multiple allowed, in any order, deduplicated)",
			)
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<SshWrappersArguments> {
		Ok (SshWrappersArguments {
				keys : self.keys.collect (_empty_is_missing) ?,
			})
	}
}


impl MaterialValue for SshWrapperKeyOrHandle {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_and_zeroize (_string) .else_wrap (0x1c720b43)
	}
}


impl SshWrappersArguments {
	
	pub fn wrappers (self) -> SshResult<Vec<SshWrapper>> {
		let _keys = self.keys.decode () .else_wrap (0x42f1fa1d) ?;
		_keys.into_iter () .map (|_key| { let _agent = SshWrapperAgent::connect () ?; SshWrapper::new_with_key_or_handle (_key, _agent) }) .collect ()
	}
}








impl MaterialValue for Randomness {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Self::decode_bytes (_string.into_bytes ())
	}
	
	fn decode_bytes (_bytes : Vec<u8>) -> FlagsResult<Self> {
		Ok (Self::new (_bytes))
	}
}








impl InputsFlags {
	
	pub fn new () -> Self {
		Self {
				inputs : MaterialFlags::new (),
				canonicalize : None,
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.inputs.flags (
				_flags,
				Some ('i'),
				"input", "input-env", "input-path", "input-fd", "input-stdin", "input-pinentry", "input-lkkrs", Some ("without-input"),
				"inputs used in key derivation (multiple allowed, **order and duplicates are significant**)",
			) ?;
		
		{
			let _flag = _flags.define_complex (&mut self.canonicalize);
			_flag.define_switch_0 (true)
					.with_flag ((), "inputs-canonicalize")
					.with_description ("canonicalize inputs");
			_flag.define_switch_0 (false)
					.with_flag ((), "inputs-concatenate")
					.with_description ("concatenate inputs")
					.with_warning ("CAUTION");
		}
		
		Ok (())
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<InputsArguments> {
		Ok (InputsArguments {
				inputs : self.inputs.collect (_empty_is_missing) ?,
				canonicalize : self.canonicalize,
			})
	}
}


impl InputsArguments {
	
	pub fn load_data (self) -> FlagsResult<Vec<u8>> {
		
		let _without = self.inputs.without;
		
		let _inputs = self.inputs.decode () ?;
		let _inputs_count = _inputs.len ();
		
		if _inputs.is_empty () {
			if ! _without {
				fail! (0x603638f7);
			}
		}
		
		let _canonicalize = self.canonicalize.unwrap_or (_inputs_count > 1);
		
		let _data_size = _inputs.iter () .map (Vec::len) .sum::<usize> ();
		let _data_extra = if _canonicalize {
				mem::size_of::<u64> () * (1 + _inputs_count)
			} else {
				0
			};
		
		let mut _data = Vec::with_capacity (_data_size + _data_extra);
		
		if _canonicalize {
			_data.write_u64::<BigEndian> (_inputs_count.try_into () .infallible (0x0f81e826)) .infallible (0x43c8e08b);
		}
		for mut _input in _inputs.into_iter () {
			if _canonicalize {
				_data.write_u64::<BigEndian> (_input.len () .try_into () .infallible (0xf2a4f6a1)) .infallible (0x309615c5);
			}
			_data.append (&mut _input);
		}
		
		//  TODO!
		if false && _data.is_empty () {
			fail! (0x46811183);
		}
		
		Ok (_data)
	}
}








impl MaterialValue for Vec<u8> {
	
	fn decode_string (_string : String) -> FlagsResult<Self> {
		Ok (_string.into_bytes ())
	}
	
	fn decode_bytes (_bytes : Vec<u8>) -> FlagsResult<Self> {
		Ok (_bytes)
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
				fingerprint : None,
				deterministic : None,
				randomness : MaterialFlags::new (),
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.senders.flags (_flags) ?;
		self.recipients.flags (_flags) ?;
		self.shared.flags (_flags) ?;
		self.ssh_wrappers.flags (_flags) ?;
		
		{
			let _flag = _flags.define_complex (&mut self.fingerprint);
			_flag.define_switch_0 (true)
					.with_flag ((), "with-fingerprint")
					.with_description ("fingerprint input")
					.with_warning ("CAUTION");
			_flag.define_switch_0 (false)
					.with_flag ((), "without-fingerprint");
			_flag.define_flag_0 ()
					.with_flag ((), "fingerprint")
					.with_placeholder ("bool");
		}
		
		{
			let _flag = _flags.define_complex (&mut self.deterministic);
			_flag.define_switch_0 (true)
					.with_flag ((), "with-siv")
					.with_description ("deterministic output, based on SIV")
					.with_warning ("CAUTION");
			_flag.define_switch_0 (false)
					.with_flag ((), "without-siv");
			_flag.define_flag_0 ()
					.with_flag ((), "siv")
					.with_placeholder ("bool");
		}
		
		self.randomness.flags (
				_flags,
				None,
				"randomness", "randomness-env", "randomness-path", "randomness-fd", "randomness-stdin", "randomness-pinentry", "randomness-lkkrs", Some ("without-randomness"),
				"random data used to feed the encryption (multiple allowed, **order and duplicates are significant**)",
			) ?;
		
		self.common.flags (_flags) ?;
		
		Ok (())
	}
	
	pub fn arguments (self) -> FlagsResult<EncryptArguments> {
		
		let _common = self.common.collect () ?;
		
		let _senders = self.senders.collect (_common.empty_is_missing) ?;
		let _recipients = self.recipients.collect (_common.empty_is_missing) ?;
		let _shared = self.shared.collect (_common.empty_is_missing) ?;
		let _ssh_wrappers = self.ssh_wrappers.collect (_common.empty_is_missing) ?;
		
		let _randomness = self.randomness.collect (_common.empty_is_missing) ?;
		
		Ok (EncryptArguments {
				senders : _senders,
				recipients : _recipients,
				shared : _shared,
				ssh_wrappers : _ssh_wrappers,
				common : _common,
				fingerprint : self.fingerprint.unwrap_or (false),
				deterministic : self.deterministic.unwrap_or (false),
				randomness : _randomness,
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
				fingerprint : DecryptFingerprintFlag::Without,
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.recipients.flags (_flags) ?;
		self.senders.flags (_flags) ?;
		self.shared.flags (_flags) ?;
		self.ssh_wrappers.flags (_flags) ?;
		
		{
			let _flag = _flags.define_complex (&mut self.fingerprint);
			_flag.define_switch_0 (DecryptFingerprintFlag::Without)
					.with_flag ((), "without-fingerprint");
			_flag.define_flag_with_wrapper_0 (|_fingerprint| DecryptFingerprintFlag::Expected (_fingerprint))
					.with_flag ((), "fingerprint")
					.with_placeholder ("string")
					.with_description ("fingerprint generated during encryption");
		}
		
		self.common.flags (_flags) ?;
		
		Ok (())
	}
	
	pub fn arguments (self) -> FlagsResult<DecryptArguments> {
		
		let _common = self.common.collect () ?;
		
		let _recipients = self.recipients.collect (_common.empty_is_missing) ?;
		let _senders = self.senders.collect (_common.empty_is_missing) ?;
		let _shared = self.shared.collect (_common.empty_is_missing) ?;
		let _ssh_wrappers = self.ssh_wrappers.collect (_common.empty_is_missing) ?;
		
		let _fingerprint = match self.fingerprint {
				DecryptFingerprintFlag::Without =>
					None,
				DecryptFingerprintFlag::Expected (_fingerprint) =>
					Some (decode_fingerprint (&_fingerprint) .else_wrap (0xd593d621) ?),
			};
		
		Ok (DecryptArguments {
				recipients : _recipients,
				senders : _senders,
				shared : _shared,
				ssh_wrappers : _ssh_wrappers,
				common : _common,
				fingerprint : _fingerprint,
			})
	}
}


impl FlagValue for DecryptFingerprintFlag {}


impl FlagValueDisplay for DecryptFingerprintFlag {
	
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		match self {
			DecryptFingerprintFlag::Without =>
				write! (_formatter, "without") .else_wrap (0x60b29fac),
			DecryptFingerprintFlag::Expected (_fingerprint) =>
				write! (_formatter, "{}", _fingerprint) .else_wrap (0xe9117385),
		}
	}
}




impl PasswordFlags {
	
	pub fn new () -> Self {
		Self {
				inputs : InputsFlags::new (),
				senders : SendersPublicOrPrivateFlags::new (),
				recipients : RecipientsPublicOrPrivateFlags::new (),
				shared : SharedKeysFlags::new (),
				ssh_wrappers : SshWrappersFlags::new (),
				common : CommonFlags::new (),
				fingerprint : PasswordFingerprintFlag::Without,
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.inputs.flags (_flags) ?;
		self.senders.flags (_flags) ?;
		self.recipients.flags (_flags) ?;
		self.shared.flags (_flags) ?;
		self.ssh_wrappers.flags (_flags) ?;
		
		{
			let _flag = _flags.define_complex (&mut self.fingerprint);
			_flag.define_switch_0 (PasswordFingerprintFlag::Generate)
					.with_flag ((), "with-fingerprint")
					.with_description ("fingerprint input")
					.with_warning ("CAUTION");
			_flag.define_switch_0 (PasswordFingerprintFlag::Without)
					.with_flag ((), "without-fingerprint");
			_flag.define_flag_with_wrapper_0 (|_fingerprint| PasswordFingerprintFlag::Expected (_fingerprint))
					.with_flag ((), "fingerprint")
					.with_placeholder ("string")
					.with_description ("fingerprint generated previously");
		}
		
		self.common.flags (_flags) ?;
		
		Ok (())
	}
	
	pub fn arguments (self) -> FlagsResult<PasswordArguments> {
		
		let _common = self.common.collect () ?;
		
		let _senders = self.senders.collect (_common.empty_is_missing) ?;
		let _recipients = self.recipients.collect (_common.empty_is_missing) ?;
		let _shared = self.shared.collect (_common.empty_is_missing) ?;
		let _ssh_wrappers = self.ssh_wrappers.collect (_common.empty_is_missing) ?;
		
		let _inputs = self.inputs.collect (_common.empty_is_missing) ?;
		
		let _fingerprint = match self.fingerprint {
				PasswordFingerprintFlag::Without =>
					None,
				PasswordFingerprintFlag::Generate =>
					Some (None),
				PasswordFingerprintFlag::Expected (_fingerprint) =>
					Some (Some (decode_fingerprint (&_fingerprint) .else_wrap (0xec2c13df) ?)),
			};
		
		Ok (PasswordArguments {
				inputs : _inputs,
				senders : _senders,
				recipients : _recipients,
				shared : _shared,
				ssh_wrappers : _ssh_wrappers,
				common : _common,
				fingerprint : _fingerprint,
			})
	}
}


impl FlagValue for PasswordFingerprintFlag {}


impl FlagValueDisplay for PasswordFingerprintFlag {
	
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		match self {
			PasswordFingerprintFlag::Without =>
				write! (_formatter, "without") .else_wrap (0x6782be04),
			PasswordFingerprintFlag::Generate =>
				write! (_formatter, "generate") .else_wrap (0x3e2ea5ef),
			PasswordFingerprintFlag::Expected (_fingerprint) =>
				write! (_formatter, "{}", _fingerprint) .else_wrap (0xf51df6e1),
		}
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
				derivation_loops : None,
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.associated.flags (_flags) ?;
		self.secrets.flags (_flags) ?;
		self.pins.flags (_flags) ?;
		self.seeds.flags (_flags) ?;
		self.ballasts.flags (_flags) ?;
		
		_flags.define_single_flag_0 (&mut self.derivation_loops)
				.with_flag ((), "derivation-loops")
				.with_placeholder ("count")
				.with_description ("number of derivation loops");
		
		Ok (())
	}
	
	pub fn collect (self, _empty_is_missing : bool) -> FlagsResult<SharedKeysArguments> {
		
		let _associated = self.associated.collect (_empty_is_missing) ?;
		let _secrets = self.secrets.collect (_empty_is_missing) ?;
		let _pins = self.pins.collect (_empty_is_missing) ?;
		let _seeds = self.seeds.collect (_empty_is_missing) ?;
		let _ballasts = self.ballasts.collect (_empty_is_missing) ?;
		
		let _derivation_loops = match self.derivation_loops {
			Some (_loops) => NonZeroU64::new (_loops),
			None => None,
		};
		
		Ok (SharedKeysArguments {
				associated : _associated,
				secrets : _secrets,
				pins : _pins,
				seeds : _seeds,
				ballasts : _ballasts,
				derivation_loops : _derivation_loops,
			})
	}
}




impl CommonFlags {
	
	pub fn new () -> Self {
		Self {
				namespace : None,
				empty_is_missing : None,
			}
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		_flags.define_single_flag_0 (&mut self.namespace)
				.with_flag ((), "namespace")
				.with_placeholder ("string")
				.with_description ("token used for cryptography domain separation");
		
		{
			let _flag = _flags.define_complex (&mut self.empty_is_missing);
			_flag.define_switch_0 (true)
					.with_flag ((), "with-empty-is-missing")
					.with_description ("treat empty arguments as unspecified")
					.with_warning ("CAUTION");
			_flag.define_switch_0 (false)
					.with_flag ((), "without-empty-is-missing");
			_flag.define_flag_0 ()
					.with_flag ((), "empty-is-missing")
					.with_placeholder ("bool");
		}
		
		Ok (())
	}
	
	pub fn collect (&self) -> FlagsResult<CommonArguments> {
		Ok (CommonArguments {
				namespace : self.namespace.clone (),
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
		
		#[ cfg (target_family = "unix") ]
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
		
		#[ cfg (all (target_os = "linux", target_env = "gnu")) ]
		for _selector in self.from_lkkrs.iter () {
			if _empty_is_missing && _selector.is_empty () {
				continue;
			}
			let _source = MaterialSource::FromLkkrs (String::from (_selector), _empty_is_missing);
			_sources.push (_source);
		}
		
		Ok (MaterialSources {
				sources : _sources,
				without : self.without.unwrap_or (false),
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
		
		if self.without && ! _values.is_empty () {
			fail! (0x4b0f228d);
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
			
			#[ cfg (target_family = "unix") ]
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
			
			#[ cfg (all (target_os = "linux", target_env = "gnu")) ]
			MaterialSource::FromLkkrs (_selector, _empty_is_missing) => {
				let _data = lkkrs_key_read (_selector.as_str ()) .else_wrap (0xedf99975) ?;
				if let Some (_data) = _data {
					if _empty_is_missing && _data.is_empty () {
						None
					} else {
						Some (MaterialData::Bytes (_data))
					}
				} else {
					if _empty_is_missing {
						None
					} else {
						fail! (0x3a7a5ec1);
					}
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


