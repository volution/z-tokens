

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;
use ::z_tokens_runtime::flags::*;


use ::std::eprintln;




mod crtk {
	
	pub(super) use ::cryptoki::{
			context::Pkcs11 as Provider,
			context::CInitializeArgs as ProviderArguments,
			session::UserType as UserType,
			slot::Slot,
			object::AttributeType,
			object::Attribute,
			object::KeyType,
			object::ObjectClass,
			object::ObjectHandle,
		};
	
	pub(super) use ::cryptoki_sys::{
			CK_ATTRIBUTE_TYPE,
		};
}








define_error! (pub MainError, result : MainResult);




#[ derive (Default) ]
#[ derive (Debug) ]
struct RsaKey {
	slot_id : u64,
	label : String,
	modulus_bits : usize,
	has_private : bool,
	has_public : bool,
	supports_decrypt : bool,
	supports_encrypt : bool,
	supports_sign : bool,
	supports_verify : bool,
	handle_slot : Option<crtk::Slot>,
	handle_private : Option<crtk::ObjectHandle>,
	handle_public : Option<crtk::ObjectHandle>,
}








pub fn main (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	if _arguments.len () != 1 {
		fail! (0xb1bda4b1);
	}
	
	let _print_all = false;
	let _print_found = false || _print_all;
	let _print_provider_details = false || _print_all;
	let _print_slot_details = false || _print_all;
	let _print_token_details = false || _print_all;
	let _print_object_details = false || _print_all;
	let _print_mechanism_details = false || _print_all;
	let _print_keys = true || _print_all;
	
	
	let mut _rsa_keys : HashMap<(u64, String), RsaKey> = HashMap::new ();
	
	
	let mut _provider = crtk::Provider::new ("/usr/lib64/pkcs11/libsofthsm2.so") .else_wrap (0xfef3d2da) ?;
	
	_provider.initialize (crtk::ProviderArguments::OsThreads) .else_wrap (0x0f9cc555) ?;
	
	
	if _print_provider_details {
		let _details = _provider.get_library_info () .else_wrap (0x0a901adf) ?;
		eprintln! ("[>>] [ff2b0b40]  provider details: `{}` / `{}` / `{}` / `{}`;",
				_details.cryptoki_version (),
				_details.manufacturer_id (),
				_details.library_version (),
				_details.library_description (),
			);
	}
	
	
	let _slots = _provider.get_slots_with_initialized_token () .else_wrap (0x822faba2) ?;
	for _slot in _slots.into_iter () {
		let _slot_id = _slot.id ();
		
		if _print_slot_details {
			let _details = _provider.get_slot_info (_slot.clone ()) .else_wrap (0x3df0d6c1) ?;
			eprintln! ("[>>] [a12ecef2]  slot {:08x} ({}) details:  `{}` / `{}` / `{}` / `{}`  (present {}, removable {}, hardware {});",
					_slot_id, _slot_id,
					_details.manufacturer_id (),
					_details.hardware_version (),
					_details.firmware_version (),
					_details.slot_description (),
					_details.token_present (),
					_details.removable_device (),
					_details.hardware_slot (),
				);
		} else if _print_found {
			eprintln! ("[>>] [572578a8]  slot {:08x} ({});", _slot_id, _slot_id);
		}
		
		if _print_token_details {
			let _details = _provider.get_token_info (_slot.clone ()) .else_wrap (0xc372d438) ?;
			eprintln! ("[>>] [598f27ac]  token {:08x} ({}) details:  `{}` / `{}` / `{}` / `{}` / `{}` / `{}`  (login {}, pin {}, initialized {}, protected {}, rng {});",
					_slot_id, _slot_id,
					_details.manufacturer_id (),
					_details.hardware_version (),
					_details.firmware_version (),
					_details.model (),
					_details.label (),
					_details.serial_number (),
					_details.login_required (),
					_details.user_pin_initialized (),
					_details.token_initialized (),
					_details.write_protected (),
					_details.rng (),
				);
		}
		
		let _mechanisms = _provider.get_mechanism_list (_slot.clone ()) .else_wrap (0x70b7232f) ?;
		for _mechanism in _mechanisms.into_iter () {
			if _print_mechanism_details {
				let _details = _provider.get_mechanism_info (_slot.clone (), _mechanism.clone ()) .else_wrap (0x9ff3cc67) ?;
				let _usable =
						_details.encrypt () || _details.decrypt () ||
						_details.sign () || _details.verify () ||
						_details.wrap () || _details.unwrap ();
				if _usable {
					eprintln! ("[>>] [8464c934]  mechanism for {:08x} ({}) {}:  (encrypt {}, decrypt {}, sign {} ({}), verify {} ({}), wrap {}, unwrap {}, hardware {});",
							_slot_id, _slot_id, _mechanism,
							_details.encrypt (),
							_details.decrypt (),
							_details.sign (),
							_details.sign_recover (),
							_details.verify (),
							_details.verify_recover (),
							_details.wrap (),
							_details.unwrap (),
							_details.hardware (),
						);
				} else if _print_found {
					eprintln! ("[>>] [a3f98b15]  mechanism for {:08x} ({}) {};", _slot_id, _slot_id, _mechanism);
				}
			} else if _print_found {
				eprintln! ("[>>] [e88365ce]  mechanism for {:08x} ({}) {};", _slot_id, _slot_id, _mechanism);
			}
		}
		
		
		let mut _session = _provider.open_rw_session (_slot) .else_wrap (0x61eeccf1) ?;
		
		_session.login (crtk::UserType::User, Some ("0000")) .else_wrap (0xe928680a) ?;
		
		
		let _objects = _session.find_objects (&[]) .else_wrap (0x9e90942c) ?;
		for (_object_index, _object) in _objects.into_iter () .enumerate () {
			
			if _print_object_details {
				eprintln! ("[--]");
			}
			
			let mut _attribute_types = Vec::new ();
			_attribute_types.extend_from_slice (&[
					crtk::AttributeType::Class,
					crtk::AttributeType::KeyType,
					crtk::AttributeType::ModulusBits,
					crtk::AttributeType::Label,
					crtk::AttributeType::Encrypt,
					crtk::AttributeType::Decrypt,
					crtk::AttributeType::Sign,
					crtk::AttributeType::Verify,
					crtk::AttributeType::AllowedMechanisms,
				]);
			
			// NOTE:  =>  <https://docs.oasis-open.org/pkcs11/pkcs11-base/v3.0/os/pkcs11-base-v3.0-os.html#_Toc29976719>
			// NOTE:  =>  <https://docs.oasis-open.org/pkcs11/pkcs11-base/v2.40/os/pkcs11-base-v2.40-os.html#_Toc416959757>
			for _index in 0 ..= 0x600 {
				let _attribute_type_raw : crtk::CK_ATTRIBUTE_TYPE = unsafe { mem::transmute (_index as u64) };
				let Ok (_attribute_type) = crtk::AttributeType::try_from (_attribute_type_raw)
					else { continue };
				if _attribute_types.contains (&_attribute_type) {
					continue;
				}
				_attribute_types.push (_attribute_type);
			}
			
			let mut _is_public = false;
			let mut _is_private = false;
			let mut _is_rsa = false;
			let mut _label = None;
			let mut _rsa_modulus_bits = None;
			let mut _supports_decrypt = None;
			let mut _supports_encrypt = None;
			let mut _supports_sign = None;
			let mut _supports_verify = None;
			
			let _attribute_details = _session.get_attributes (_object, &_attribute_types) .else_wrap (0x4aa0392d) ?;
			for _attribute_details in _attribute_details.into_iter () {
				match _attribute_details {
					
					crtk::Attribute::Class (_class) => {
						if _print_object_details {
							eprintln! ("[>>] [1c1ceb65]  object {:08x} / {}:  class `{}`;", _slot_id, _object_index, _class);
						}
						match _class {
							crtk::ObjectClass::PRIVATE_KEY =>
								_is_private = true,
							crtk::ObjectClass::PUBLIC_KEY =>
								_is_public = true,
							_ =>
								(),
						}
					}
					
					crtk::Attribute::KeyType (_key_type) => {
						if _print_object_details {
							eprintln! ("[>>] [b5bbbf38]  object {:08x} / {}:  key-type `{}`;", _slot_id, _object_index, _key_type);
						}
						match _key_type {
							crtk::KeyType::RSA =>
								_is_rsa = true,
							_ =>
								(),
						}
					}
					
					crtk::Attribute::ModulusBits (_modulus_bits) => {
						if _print_object_details {
							eprintln! ("[>>] [909a50fc]  object {:08x} / {}:  modulus-bits `{}`;", _slot_id, _object_index, _modulus_bits);
						}
						_rsa_modulus_bits = Some (_modulus_bits);
					}
					
					crtk::Attribute::Label (_label_0) => {
						let _label_0 = str::from_utf8 (&_label_0) .else_wrap (0x9dbd9c6b) ?;
						let _label_0 = String::from (_label_0);
						if _print_object_details {
							eprintln! ("[>>] [df90fc52]  object {:08x} / {}:  label `{}`;", _slot_id, _object_index, _label_0);
						}
						_label = Some (_label_0);
					},
					
					crtk::Attribute::Encrypt (_supported) =>
						_supports_encrypt = Some (_supported),
					crtk::Attribute::Decrypt (_supported) =>
						_supports_decrypt = Some (_supported),
					crtk::Attribute::Sign (_supported) =>
						_supports_sign = Some (_supported),
					crtk::Attribute::Verify (_supported) =>
						_supports_verify = Some (_supported),
					
					crtk::Attribute::AllowedMechanisms (_mechanisms) =>
						if _print_object_details {
							for _mechanism in _mechanisms {
								eprintln! ("[>>] [5f09ed1d]  object {:08x} / {}:  mechanism `{}`;", _slot_id, _object_index, _mechanism);
							}
						}
					
					_ =>
						if _print_object_details {
							let _attribute_type = _attribute_details.attribute_type ();
							eprintln! ("[>>] [ec799c88]  object {:08x} / {}:  {}", _slot_id, _object_index, _attribute_type);
						}
				}
			}
			
			if _print_object_details {
				eprintln! ("[>>] [909a50fc]  object {:08x} / {}:  (decrypt {}, encrypt {}, sign {}, verify {});",
						_slot_id, _object_index,
						_supports_decrypt.unwrap_or (false), _supports_encrypt.unwrap_or (false),
						_supports_sign.unwrap_or (false), _supports_verify.unwrap_or (false),
					);
			}
			
			if _is_rsa {
				
				let _label = _label.else_wrap (0x3d5f177a) ?;
				let _key = _rsa_keys.entry ((_slot_id, _label.clone ())) .or_default ();
				
				_key.label = _label;
				
				_key.slot_id = _slot_id;
				_key.handle_slot = Some (_slot.clone ());
				if _is_private {
					_key.has_private = true;
					_key.handle_private = Some (_object);
				}
				if _is_public {
					_key.has_public = true;
					_key.handle_public = Some (_object);
				}
				
				if let Some (_rsa_modulus_bits) = _rsa_modulus_bits {
					_key.modulus_bits = _rsa_modulus_bits.try_into () .else_wrap (0x000d5627) ?;
				}
				if let Some (_supports_decrypt) = _supports_decrypt {
					_key.supports_decrypt = _key.supports_decrypt || _supports_decrypt;
				}
				if let Some (_supports_encrypt) = _supports_encrypt {
					_key.supports_encrypt = _key.supports_encrypt || _supports_encrypt;
				}
				if let Some (_supports_sign) = _supports_sign {
					_key.supports_sign = _key.supports_sign || _supports_sign;
				}
				if let Some (_supports_verify) = _supports_verify {
					_key.supports_verify = _key.supports_verify || _supports_verify;
				}
			}
			
			if _print_object_details {
				eprintln! ("[--]");
			}
		}
		
		
		_session.logout () .else_wrap (0xfa1ce8bf) ?;
	}
	
	
	_provider.finalize ();
	
	
	for _key in _rsa_keys.values () {
		eprintln! ("[>>] [1f034f51]  RSA:  slot {:08x} ({}), label `{}`, bits {}, private {}, public {}, decrypt {}, encrypt {}, sign {}, verify {};",
				_key.slot_id, _key.slot_id,
				_key.label,
				_key.modulus_bits,
				_key.has_private,
				_key.has_public,
				_key.supports_decrypt,
				_key.supports_encrypt,
				_key.supports_sign,
				_key.supports_verify,
			);
	}
	
	
	Ok (ExitCode::SUCCESS)
}


