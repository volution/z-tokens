

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
	pub from_stdin : Vec<()>,
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
				from_stdin : Vec::new (),
			};
		Ok (_self)
	}
	
	pub fn parser <'a> (
				&'a mut self,
				_parser : &mut ArgParser<'a>,
				_short_values : &'static str,
				_long_values : &'static str,
				_long_environment : &'static str,
				_long_file : &'static str,
				_long_fd : &'static str,
				_long_stdin : &'static str,
				_description : &'static str,
			) -> FlagsResult
	{
		
		_parser.refer (&mut self.values)
				.metavar ("{string}")
				.add_option (&[_short_values, _long_values], ArgPush, _description);
		
		_parser.refer (&mut self.from_environment)
				.metavar ("{path}")
				.add_option (&[_long_environment], ArgPush, "(from environment)");
		
		_parser.refer (&mut self.from_file)
				.metavar ("{path}")
				.add_option (&[_long_file], ArgPush, "(from file)");
		
		_parser.refer (&mut self.from_fd)
				.metavar ("{fd}")
				.add_option (&[_long_fd], ArgPush, "(from file-descriptor)");
		
		_parser.refer (&mut self.from_stdin)
				.metavar ("{bool}")
				.add_option (&[_long_stdin], ArgPushConst (()), "(from stdin)");
		
		Ok (())
	}
}








impl SendersPrivateFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.materials.parser (_parser, "-s", "--sender", "--sender-env", "--sender-path", "--sender-fd", "--sender-stdin", "(sender private key) (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl SendersPublicFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.materials.parser (_parser, "-s", "--sender", "--sender-env", "--sender-path", "--sender-fd", "--sender-stdin", "(sender public key) (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl RecipientsPrivateFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.materials.parser (_parser, "-r", "--recipient", "--recipient-env", "--recipient-path", "--recipient-fd", "--recipient-stdin", "(recipient private key) (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl RecipientsPublicFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.materials.parser (_parser, "-r", "--recipient", "--recipient-env", "--recipient-path", "--recipient-fd", "--recipient-stdin", "(recipient public key) (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl AssociatedFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.materials.parser (_parser, "-a", "--associated", "--associated-env", "--associated-path", "--associated-fd", "--associated-stdin", "(associated data) (multiple allowed, **order and duplicates are significant**)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl SecretsFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.materials.parser (_parser, "-x", "--secret", "--secret-env", "--secret-path", "--secret-fd", "--secret-stdin", "(shared secret, for additional security) (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl PinsFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.materials.parser (_parser, "-e", "--pin", "--pin-env", "--pin-path", "--pin-fd", "--pin-stdin", "(shared PIN, for **WEAK** additional security) (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl SeedsFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.materials.parser (_parser, "-e", "--seed", "--seed-env", "--seed-path", "--seed-fd", "--seed-stdin", "(shared seed, for additional security) (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl BallastsFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.materials.parser (_parser, "-b", "--ballast", "--ballast-env", "--ballast-path", "--ballast-fd", "--ballast-stdin", "(shared ballast, for additional security) (multiple allowed, in any order, deduplicated)")
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}




impl SshWrappersFlags {
	
	pub fn new () -> FlagsResult<Self> {
		Ok (Self { materials : MaterialFlags::new () ? })
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.materials.parser (_parser, "-S", "--ssh-wrap", "--ssh-wrap-env", "--ssh-wrap-path", "--ssh-wrap-fd", "--ssh-wrap-stdin", "(shared SSH agent key handle) (multiple allowed, in any order, deduplicated)")
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
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		
		self.senders.parser (_parser) ?;
		self.recipients.parser (_parser) ?;
		self.shared.parser (_parser) ?;
		self.ssh_wrappers.parser (_parser) ?;
		self.common.parser (_parser) ?;
		
		_parser.refer (&mut self.deterministic)
				.metavar ("{bool}")
				.add_option (&["--siv"], ArgStoreConst (Some (true)), "(deterministic output, based on SIV) (!!! CAUTION !!!)");
		
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
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		
		self.recipients.parser (_parser) ?;
		self.senders.parser (_parser) ?;
		self.shared.parser (_parser) ?;
		self.ssh_wrappers.parser (_parser) ?;
		self.common.parser (_parser) ?;
		
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
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		self.associated.parser (_parser) ?;
		self.secrets.parser (_parser) ?;
		self.pins.parser (_parser) ?;
		self.seeds.parser (_parser) ?;
		self.ballasts.parser (_parser) ?;
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
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		
		_parser.refer (&mut self.empty_is_missing)
				.metavar ("{bool}")
				.add_option (&["-M"], ArgStoreConst (Some (true)), "(treat empty arguments as unspecified) (!!! CAUTION !!!)")
				.add_option (&["--empty-is-missing"], ArgStoreOption, "");
		
		Ok (())
	}
	
	pub fn arguments (&self) -> FlagsResult<Option<()>> {
		Ok (None)
	}
}

