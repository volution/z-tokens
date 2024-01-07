

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use crate::crates::keyutils::{
		Keyring,
		SpecialKeyring,
		keytypes,
	};




define_error! (pub KeyringError, result : KeyringResult);




pub fn lkkrs_key_read (_selector : &str) -> KeyringResult<Option<Vec<u8>>> {
	
	let (_keyring_id, _selector) = if _selector.starts_with ("@") {
		let _matches = &[
				("@t:", SpecialKeyring::Thread),
				("@p:", SpecialKeyring::Process),
				("@s:", SpecialKeyring::Session),
				("@u:", SpecialKeyring::User),
				("@us:", SpecialKeyring::UserSession),
				("@g:", SpecialKeyring::Group),
			];
		let (_keyring_id, _selector) = '_matched : {
			for (_prefix, _keyring_id) in _matches {
				if _selector.starts_with (_prefix) {
					let _selector = &_selector[_prefix.len () ..];
					break '_matched (*_keyring_id, _selector);
				}
			}
			fail! (0x8e34ac46);
		};
		(_keyring_id, _selector)
	} else {
		(SpecialKeyring::Session, _selector)
	};
	
	let (_type, _selector) = '_matched : {
		let _matches = &[
				("user:", keytypes::User),
			];
		for (_prefix, _type) in _matches {
			if _selector.starts_with (_prefix) {
				let _selector = &_selector[_prefix.len () ..];
				break '_matched (_type, _selector);
			}
		}
		fail! (0xb8805842);
	};
	
	// FIXME:  Add support for detecting missing keyring!
	let _keyring = Keyring::attach (_keyring_id) .else_wrap (0x765e5e71) ?;
	
	// FIXME:  Add support for detecting missing key!
	let _key = match _type {
		keytypes::User => {
			_keyring.search_for_key::<keytypes::User, _, _> (_selector, None) .else_wrap (0xe9e18313) ?
		}
	};
	
	let _payload = _key.read () .else_wrap (0xced88ebe) ?;
	
	Ok (Some (_payload))
}


