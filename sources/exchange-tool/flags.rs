

use crate::prelude::*;




pub struct EncryptArguments {
	pub senders : MaterialSources<SenderPrivateKey>,
	pub recipients : MaterialSources<RecipientPublicKey>,
	pub shared : SharedKeysArguments,
	pub ssh_wrappers : SshWrappersArguments,
	pub deterministic : bool,
}


pub struct EncryptFlags {
	pub senders : SendersPrivateFlags,
	pub recipients : RecipientsPublicFlags,
	pub shared : SharedKeysFlags,
	pub ssh_wrappers : SshWrappersFlags,
	pub common : CommonFlags,
	pub deterministic : Option<bool>,
}




pub struct DecryptArguments {
	pub recipients : MaterialSources<RecipientPrivateKey>,
	pub senders : MaterialSources<SenderPublicKey>,
	pub shared : SharedKeysArguments,
	pub ssh_wrappers : SshWrappersArguments,
}


pub struct DecryptFlags {
	pub recipients : RecipientsPrivateFlags,
	pub senders : SendersPublicFlags,
	pub shared : SharedKeysFlags,
	pub ssh_wrappers : SshWrappersFlags,
	pub common : CommonFlags,
}




pub struct SharedKeysArguments {
	pub associated : MaterialSources<Associated>,
	pub secrets : MaterialSources<SharedSecret>,
	pub pins : MaterialSources<SharedPin>,
	pub seeds : MaterialSources<SharedSeed>,
	pub ballasts : MaterialSources<SharedBallast>,
}


pub struct SharedKeysFlags {
	pub associated : AssociatedFlags,
	pub secrets : SecretsFlags,
	pub pins : PinsFlags,
	pub seeds : SeedsFlags,
	pub ballasts : BallastsFlags,
}




pub struct CommonFlags {
	pub empty_is_missing : Option<bool>,
}




pub struct MaterialFlags {
	pub values : Vec<String>,
	pub from_environment : Vec<String>,
	pub from_file : Vec<String>,
	pub from_fd : Vec<u16>,
	pub from_stdin : Option<bool>,
}


pub struct MaterialSources<Material> {
	pub sources : Vec<MaterialSource<Material>>,
}

pub enum MaterialSource<Material> {
	Material (Material),
	StringValue (String),
	FromEnvironment (OsString),
	FromFile (PathBuf),
	FromFd (OwnedFd),
	FromStdin,
}




pub struct SendersPrivateFlags {
	pub materials : MaterialFlags,
}

pub struct SendersPublicFlags {
	pub materials : MaterialFlags,
}

pub struct RecipientsPrivateFlags {
	pub materials : MaterialFlags,
}

pub struct RecipientsPublicFlags {
	pub materials : MaterialFlags,
}


pub struct AssociatedFlags {
	pub materials : MaterialFlags,
}

pub struct SecretsFlags {
	pub materials : MaterialFlags,
}

pub struct PinsFlags {
	pub materials : MaterialFlags,
}

pub struct SeedsFlags {
	pub materials : MaterialFlags,
}

pub struct BallastsFlags {
	pub materials : MaterialFlags,
}




pub struct SshWrappersArguments {
}


pub struct SshWrappersFlags {
	pub materials : MaterialFlags,
}








impl MaterialFlags {
	
	pub fn new () -> FlagsResult<Self> {
		let _self = Self {
				values : Vec::new (),
				from_environment : Vec::new (),
				from_file : Vec::new (),
				from_fd : Vec::new (),
				from_stdin : None,
			};
		Ok (_self)
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
		
		_flags.define_multiple_flag_0 (&mut self.from_fd)
				.with_flag ((), _long_fd)
				.with_placeholder ("fd")
				.with_description ("from file-descriptor");
		
		_flags.define_switch_0 (&mut self.from_stdin)
				.with_flag ((), _long_stdin)
				.with_description ("from stdin");
		
		Ok (())
	}
}








impl SendersPrivateFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (_flags, 's', "sender", "sender-env", "sender-path", "sender-fd", "sender-stdin", "sender private key (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl SendersPublicFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (_flags, 's', "sender", "sender-env", "sender-path", "sender-fd", "sender-stdin", "sender public key (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl RecipientsPrivateFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (_flags, 'r', "recipient", "recipient-env", "recipient-path", "recipient-fd", "recipient-stdin", "recipient private key (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl RecipientsPublicFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (_flags, 'r', "recipient", "recipient-env", "recipient-path", "recipient-fd", "recipient-stdin", "recipient public key (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl AssociatedFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (_flags, 'a', "associated", "associated-env", "associated-path", "associated-fd", "associated-stdin", "associated data (multiple allowed, **order and duplicates are significant**)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl SecretsFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (_flags, 'x', "secret", "secret-env", "secret-path", "secret-fd", "secret-stdin", "shared secret, for additional security (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl PinsFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (_flags, 'e', "pin", "pin-env", "pin-path", "pin-fd", "pin-stdin", "shared PIN, for **WEAK** additional security (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl SeedsFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (_flags, 'e', "seed", "seed-env", "seed-path", "seed-fd", "seed-stdin", "shared seed, for additional security (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl BallastsFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (_flags, 'b', "ballast", "ballast-env", "ballast-path", "ballast-fd", "ballast-stdin", "shared ballast, for additional security (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl SshWrappersFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.materials.flags (_flags, 'S', "ssh-wrap", "ssh-wrap-env", "ssh-wrap-path", "ssh-wrap-fd", "ssh-wrap-stdin", "shared SSH agent key handle (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}








impl EncryptFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self {
				senders : SendersPrivateFlags::new () ?,
				recipients : RecipientsPublicFlags::new () ?,
				shared : SharedKeysFlags::new () ?,
				ssh_wrappers : SshWrappersFlags::new () ?,
				common : CommonFlags::new () ?,
				deterministic : None,
			})
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
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl DecryptFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self {
				recipients : RecipientsPrivateFlags::new () ?,
				senders : SendersPublicFlags::new () ?,
				shared : SharedKeysFlags::new () ?,
				ssh_wrappers : SshWrappersFlags::new () ?,
				common : CommonFlags::new () ?,
			})
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		self.recipients.flags (_flags) ?;
		self.senders.flags (_flags) ?;
		self.shared.flags (_flags) ?;
		self.ssh_wrappers.flags (_flags) ?;
		self.common.flags (_flags) ?;
		
		Ok (())
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl SharedKeysFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self {
				associated : AssociatedFlags::new () ?,
				secrets : SecretsFlags::new () ?,
				pins : PinsFlags::new () ?,
				seeds : SeedsFlags::new () ?,
				ballasts : BallastsFlags::new () ?,
			})
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		self.associated.flags (_flags) ?;
		self.secrets.flags (_flags) ?;
		self.pins.flags (_flags) ?;
		self.seeds.flags (_flags) ?;
		self.ballasts.flags (_flags) ?;
		Ok (())
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl CommonFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self {
				empty_is_missing : None,
			})
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		_flags.define_single_flag_0 (&mut self.empty_is_missing)
				.with_flag ((), "empty-is-missing")
				.with_placeholder ("bool")
				.with_description ("treat empty arguments as unspecified")
				.with_warning ("CAUTION");
		
		Ok (())
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}

