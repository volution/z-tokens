

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use ::z_tokens_runtime::memory::{
		Rb,
	};




define_error! (pub KeyEncodingError, result : KeyEncodingResult);
define_error! (pub KeyCreateError, result : KeyCreateResult);




pub struct SenderPrivateKey (Rb<[u8; 32]>);
pub struct SenderPublicKey (Rb<[u8; 32]>);

pub struct ReceiverPrivateKey (Rb<[u8; 32]>);
pub struct ReceiverPublicKey (Rb<[u8; 32]>);








pub fn decode_sender_private_key (_string : &str) -> KeyEncodingResult<SenderPrivateKey> {
	fail! (0x637d3066);
}

pub fn decode_sender_public_key (_string : &str) -> KeyEncodingResult<SenderPublicKey> {
	fail! (0x1f6cefe4);
}


pub fn decode_receiver_private_key (_string : &str) -> KeyEncodingResult<ReceiverPrivateKey> {
	fail! (0xb3dd2250);
}

pub fn decode_receiver_public_key (_string : &str) -> KeyEncodingResult<ReceiverPublicKey> {
	fail! (0xef847b91);
}




pub fn encode_sender_private_key (_key : &SenderPrivateKey) -> KeyEncodingResult<Rb<String>> {
	fail! (0x41942620);
}

pub fn encode_sender_public_key (_key : &SenderPublicKey) -> KeyEncodingResult<Rb<String>> {
	fail! (0xbb61a0c3);
}

pub fn encode_receiver_public_key (_key : &ReceiverPrivateKey) -> KeyEncodingResult<Rb<String>> {
	fail! (0xebf2cde3);
}

pub fn encode_receiver_public_key (_key : &ReceiverPublicKey) -> KeyEncodingResult<Rb<String>> {
	fail! (0xffd710d8);
}








pub fn create_sender_pair () -> KeyCreateResult<(SenderPrivateKey, SenderPublicKey)> {
	fail! (0x45c08ce8);
}


pub fn create_receiver_pair () -> KeyCreateResult<(ReceiverPrivateKey, ReceiverPublicKey)> {
	fail! (0xa4118197);
}


