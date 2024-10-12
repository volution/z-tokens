

use ::z_tokens_runtime::preludes::std_plus_extras::*;


use crate::crypto::*;
use crate::keys::*;
use crate::oracles::*;








pub struct SymmetricParametersRawRefs <'a> {
	pub associated : &'a [&'a [u8]],
	pub secrets : &'a [&'a [u8]],
	pub pins : &'a [&'a [u8]],
	pub seeds : &'a [&'a [u8]],
	pub ballasts : &'a [&'a [u8]],
	pub derivation_loops : Option<NonZeroU64>,
	pub namespace : Option<&'a str>,
}




pub struct SymmetricParametersRawBoxed <'a> {
	pub associated : Box<[&'a [u8]]>,
	pub secrets : Box<[&'a [u8]]>,
	pub pins : Box<[&'a [u8]]>,
	pub seeds : Box<[&'a [u8]]>,
	pub ballasts : Box<[&'a [u8]]>,
	pub derivation_loops : Option<NonZeroU64>,
	pub namespace : Option<&'a str>,
}


impl <'a> SymmetricParametersRawBoxed<'a> {
	
	pub fn new (
			_associated : impl Iterator<Item = &'a Associated>,
			_secrets : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pins : impl Iterator<Item = &'a SharedPin>,
			_seeds : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballasts : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&'a str>,
		) -> Self
	{
		Self {
				associated : _associated.map (Associated::access_bytes_slice) .collect::<Vec<_>> () .into_boxed_slice (),
				secrets : _secrets.map (SharedSecretTrait::access_bytes_slice) .collect::<Vec<_>> () .into_boxed_slice (),
				pins : _pins.map (SharedPin::access_bytes_slice) .collect::<Vec<_>> () .into_boxed_slice (),
				seeds : _seeds.map (SharedSeedTrait::access_bytes_slice) .collect::<Vec<_>> () .into_boxed_slice (),
				ballasts : _ballasts.map (SharedBallastTrait::access_bytes_slice) .collect::<Vec<_>> () .into_boxed_slice (),
				derivation_loops : _derivation_loops,
				namespace : _namespace,
			}
	}
	
	pub fn refs (&'a self) -> SymmetricParametersRawRefs<'a> {
		SymmetricParametersRawRefs {
				associated : &self.associated,
				secrets : &self.secrets,
				pins : &self.pins,
				seeds : &self.seeds,
				ballasts : &self.ballasts,
				derivation_loops : self.derivation_loops,
				namespace : self.namespace,
			}
	}
}








pub fn password_symmetric <'a> (
			_associated : impl Iterator<Item = &'a Associated>,
			_secrets : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pins : impl Iterator<Item = &'a SharedPin>,
			_seeds : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballasts : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&'a str>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
		) -> CryptoResult
{
	let _symmetric = SymmetricParametersRawBoxed::new (_associated, _secrets, _pins, _seeds, _ballasts, _derivation_loops, _namespace);
	password_symmetric_with_raw (
			& _symmetric.refs (),
			_password_data,
			_password_output,
			_oracles.collect (),
		)
}


pub fn password_symmetric_with_raw (
			_symmetric : &SymmetricParametersRawRefs,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : Vec<&mut dyn Oracle>,
		) -> CryptoResult
{
	password_backend (&[], &[], _symmetric, _password_data, _password_output, _oracles, true)
}




pub fn password_send <'a> (
			_senders : impl Iterator<Item = &'a SenderPrivateKey>,
			_recipients : impl Iterator<Item = &'a RecipientPublicKey>,
			_associated : impl Iterator<Item = &'a Associated>,
			_secrets : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pins : impl Iterator<Item = &'a SharedPin>,
			_seeds : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballasts : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&'a str>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
		) -> CryptoResult
{
	let _symmetric = SymmetricParametersRawBoxed::new (_associated, _secrets, _pins, _seeds, _ballasts, _derivation_loops, _namespace);
	password_send_with_raw (
			& _senders.collect::<Vec<_>> (),
			& _recipients.collect::<Vec<_>> (),
			& _symmetric.refs (),
			_password_data,
			_password_output,
			_oracles.collect (),
		)
}


pub fn password_send_with_raw (
			_senders : &[&SenderPrivateKey],
			_recipients : &[&RecipientPublicKey],
			_symmetric : &SymmetricParametersRawRefs,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : Vec<&mut dyn Oracle>,
		) -> CryptoResult
{
	let (_senders, _recipients) = wrap_senders_and_recipients_inputs (_senders, _recipients) ?;
	password_backend (&_senders, &_recipients, _symmetric, _password_data, _password_output, _oracles, true)
}




pub fn password_receive <'a> (
			_recipients : impl Iterator<Item = &'a RecipientPrivateKey>,
			_senders : impl Iterator<Item = &'a SenderPublicKey>,
			_associated : impl Iterator<Item = &'a Associated>,
			_secrets : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pins : impl Iterator<Item = &'a SharedPin>,
			_seeds : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballasts : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&'a str>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
		) -> CryptoResult
{
	let _symmetric = SymmetricParametersRawBoxed::new (_associated, _secrets, _pins, _seeds, _ballasts, _derivation_loops, _namespace);
	password_receive_with_raw (
			& _recipients.collect::<Vec<_>> (),
			& _senders.collect::<Vec<_>> (),
			& _symmetric.refs (),
			_password_data,
			_password_output,
			_oracles.collect (),
		)
}


pub fn password_receive_with_raw (
			_recipients : &[&RecipientPrivateKey],
			_senders : &[&SenderPublicKey],
			_symmetric : &SymmetricParametersRawRefs,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : Vec<&mut dyn Oracle>,
		) -> CryptoResult
{
	let (_recipients, _senders) = wrap_recipients_and_senders_inputs (_recipients, _senders) ?;
	password_backend (&_recipients, &_senders, _symmetric, _password_data, _password_output, _oracles, false)
}








pub fn encrypt <'a> (
			_senders : impl Iterator<Item = &'a SenderPrivateKey>,
			_recipients : impl Iterator<Item = &'a RecipientPublicKey>,
			_associated : impl Iterator<Item = &'a Associated>,
			_secrets : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pins : impl Iterator<Item = &'a SharedPin>,
			_seeds : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballasts : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&'a str>,
			_decrypted : &[u8],
			_encrypted : &mut Vec<u8>,
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
			_packet_salt_deterministic : bool,
		) -> CryptoResult
{
	let _symmetric = SymmetricParametersRawBoxed::new (_associated, _secrets, _pins, _seeds, _ballasts, _derivation_loops, _namespace);
	encrypt_with_raw (
			& _senders.collect::<Vec<_>> (),
			& _recipients.collect::<Vec<_>> (),
			& _symmetric.refs (),
			_decrypted,
			_encrypted,
			_oracles.collect (),
			_packet_salt_deterministic,
		)
}








pub fn decrypt <'a> (
			_recipients : impl Iterator<Item = &'a RecipientPrivateKey>,
			_senders : impl Iterator<Item = &'a SenderPublicKey>,
			_associated : impl Iterator<Item = &'a Associated>,
			_secrets : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pins : impl Iterator<Item = &'a SharedPin>,
			_seeds : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballasts : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&'a str>,
			_encrypted : &[u8],
			_decrypted : &mut Vec<u8>,
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
		) -> CryptoResult
{
	let _symmetric = SymmetricParametersRawBoxed::new (_associated, _secrets, _pins, _seeds, _ballasts, _derivation_loops, _namespace);
	decrypt_with_raw (
			& _recipients.collect::<Vec<_>> (),
			& _senders.collect::<Vec<_>> (),
			& _symmetric.refs (),
			_encrypted,
			_decrypted,
			_oracles.collect (),
		)
}


