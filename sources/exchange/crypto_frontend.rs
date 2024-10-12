

use ::z_tokens_runtime::preludes::std_plus_extras::*;


use crate::crypto::*;
use crate::keys::*;
use crate::oracles::*;








pub fn password_symmetric <'a> (
			_associated : impl Iterator<Item = &'a Associated>,
			_secret : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pin : impl Iterator<Item = &'a SharedPin>,
			_seed : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballast : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&str>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
		) -> CryptoResult
{
	password_symmetric_with_raw (
			& _associated.map (Associated::access_bytes_slice) .collect::<Vec<_>> (),
			& _secret.map (SharedSecretTrait::access_bytes_slice) .collect::<Vec<_>> (),
			& _pin.map (SharedPin::access_bytes_slice) .collect::<Vec<_>> (),
			& _seed.map (SharedSeedTrait::access_bytes_slice) .collect::<Vec<_>> (),
			& _ballast.map (SharedBallastTrait::access_bytes_slice) .collect::<Vec<_>> (),
			_derivation_loops,
			_namespace,
			_password_data,
			_password_output,
			_oracles.collect (),
		)
}


pub fn password_symmetric_with_raw (
			_associated_inputs : &[&[u8]],
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_seed_inputs : &[&[u8]],
			_ballast_inputs : &[&[u8]],
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&str>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : Vec<&mut dyn Oracle>,
		) -> CryptoResult
{
	password_backend (Vec::new (), Vec::new (), _associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs, _derivation_loops, _namespace, _password_data, _password_output, _oracles, true)
}




pub fn password_send <'a> (
			_senders : impl Iterator<Item = &'a SenderPrivateKey>,
			_recipients : impl Iterator<Item = &'a RecipientPublicKey>,
			_associated : impl Iterator<Item = &'a Associated>,
			_secret : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pin : impl Iterator<Item = &'a SharedPin>,
			_seed : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballast : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&str>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
		) -> CryptoResult
{
	password_send_with_raw (
			& _senders.collect::<Vec<_>> (),
			& _recipients.collect::<Vec<_>> (),
			& _associated.map (Associated::access_bytes_slice) .collect::<Vec<_>> (),
			& _secret.map (SharedSecretTrait::access_bytes_slice) .collect::<Vec<_>> (),
			& _pin.map (SharedPin::access_bytes_slice) .collect::<Vec<_>> (),
			& _seed.map (SharedSeedTrait::access_bytes_slice) .collect::<Vec<_>> (),
			& _ballast.map (SharedBallastTrait::access_bytes_slice) .collect::<Vec<_>> (),
			_derivation_loops,
			_namespace,
			_password_data,
			_password_output,
			_oracles.collect (),
		)
}


pub fn password_send_with_raw (
			_senders : &[&SenderPrivateKey],
			_recipients : &[&RecipientPublicKey],
			_associated_inputs : &[&[u8]],
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_seed_inputs : &[&[u8]],
			_ballast_inputs : &[&[u8]],
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&str>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : Vec<&mut dyn Oracle>,
		) -> CryptoResult
{
	let (_senders, _recipients) = wrap_senders_and_recipients_inputs (_senders, _recipients) ?;
	password_backend (_senders, _recipients, _associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs, _derivation_loops, _namespace, _password_data, _password_output, _oracles, true)
}




pub fn password_receive <'a> (
			_recipients : impl Iterator<Item = &'a RecipientPrivateKey>,
			_senders : impl Iterator<Item = &'a SenderPublicKey>,
			_associated : impl Iterator<Item = &'a Associated>,
			_secret : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pin : impl Iterator<Item = &'a SharedPin>,
			_seed : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballast : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&str>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
		) -> CryptoResult
{
	password_receive_with_raw (
			& _recipients.collect::<Vec<_>> (),
			& _senders.collect::<Vec<_>> (),
			& _associated.map (Associated::access_bytes_slice) .collect::<Vec<_>> (),
			& _secret.map (SharedSecretTrait::access_bytes_slice) .collect::<Vec<_>> (),
			& _pin.map (SharedPin::access_bytes_slice) .collect::<Vec<_>> (),
			& _seed.map (SharedSeedTrait::access_bytes_slice) .collect::<Vec<_>> (),
			& _ballast.map (SharedBallastTrait::access_bytes_slice) .collect::<Vec<_>> (),
			_derivation_loops,
			_namespace,
			_password_data,
			_password_output,
			_oracles.collect (),
		)
}


pub fn password_receive_with_raw (
			_recipients : &[&RecipientPrivateKey],
			_senders : &[&SenderPublicKey],
			_associated_inputs : &[&[u8]],
			_secret_inputs : &[&[u8]],
			_pin_inputs : &[&[u8]],
			_seed_inputs : &[&[u8]],
			_ballast_inputs : &[&[u8]],
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&str>,
			_password_data : &[u8],
			_password_output : &mut [u8; 32],
			_oracles : Vec<&mut dyn Oracle>,
		) -> CryptoResult
{
	let (_recipients, _senders) = wrap_recipients_and_senders_inputs (_recipients, _senders) ?;
	password_backend (_recipients, _senders, _associated_inputs, _secret_inputs, _pin_inputs, _seed_inputs, _ballast_inputs, _derivation_loops, _namespace, _password_data, _password_output, _oracles, false)
}








pub fn encrypt <'a> (
			_senders : impl Iterator<Item = &'a SenderPrivateKey>,
			_recipients : impl Iterator<Item = &'a RecipientPublicKey>,
			_associated : impl Iterator<Item = &'a Associated>,
			_secret : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pin : impl Iterator<Item = &'a SharedPin>,
			_seed : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballast : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&str>,
			_decrypted : &[u8],
			_encrypted : &mut Vec<u8>,
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
			_packet_salt_deterministic : bool,
		) -> CryptoResult
{
	encrypt_with_raw (
			& _senders.collect::<Vec<_>> (),
			& _recipients.collect::<Vec<_>> (),
			& _associated.map (Associated::access_bytes_slice) .collect::<Vec<_>> (),
			& _secret.map (SharedSecretTrait::access_bytes_slice) .collect::<Vec<_>> (),
			& _pin.map (SharedPin::access_bytes_slice) .collect::<Vec<_>> (),
			& _seed.map (SharedSeedTrait::access_bytes_slice) .collect::<Vec<_>> (),
			& _ballast.map (SharedBallastTrait::access_bytes_slice) .collect::<Vec<_>> (),
			_derivation_loops,
			_namespace,
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
			_secret : impl Iterator<Item = &'a dyn SharedSecretTrait>,
			_pin : impl Iterator<Item = &'a SharedPin>,
			_seed : impl Iterator<Item = &'a dyn SharedSeedTrait>,
			_ballast : impl Iterator<Item = &'a dyn SharedBallastTrait>,
			_derivation_loops : Option<NonZeroU64>,
			_namespace : Option<&str>,
			_encrypted : &[u8],
			_decrypted : &mut Vec<u8>,
			_oracles : impl Iterator<Item = &'a mut dyn Oracle>,
		) -> CryptoResult
{
	decrypt_with_raw (
			& _recipients.collect::<Vec<_>> (),
			& _senders.collect::<Vec<_>> (),
			& _associated.map (Associated::access_bytes_slice) .collect::<Vec<_>> (),
			& _secret.map (SharedSecretTrait::access_bytes_slice) .collect::<Vec<_>> (),
			& _pin.map (SharedPin::access_bytes_slice) .collect::<Vec<_>> (),
			& _seed.map (SharedSeedTrait::access_bytes_slice) .collect::<Vec<_>> (),
			& _ballast.map (SharedBallastTrait::access_bytes_slice) .collect::<Vec<_>> (),
			_derivation_loops,
			_namespace,
			_encrypted,
			_decrypted,
			_oracles.collect (),
		)
}


