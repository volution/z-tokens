

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use ::std::eprintln;
use ::std::process::ExitCode;


use ::z_tokens_runtime_crypto::crates::rsa;
use ::z_tokens_runtime_crypto::crates::pkcs1;



mod crtk {
	
	#[ allow (unused_imports) ]
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
			mechanism::Mechanism,
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
	id : String,
	id_data : Option<Vec<u8>>,
	label : String,
	label_data : Option<Vec<u8>>,
	modulus_bits : usize,
	modulus_data : Vec<u8>,
	exponent_data : Vec<u8>,
	has_private : bool,
	has_public : bool,
	supports_decrypt : bool,
	supports_encrypt : bool,
	supports_sign : bool,
	supports_verify : bool,
	public_key : Option<rsa::RsaPublicKey>,
	public_key_pem : Option<String>,
}








pub fn main_pkcs11 (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	
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
	let _print_keys = false || _print_all;
	let _print_keys_pem = false || _print_all;
	
	
	let mut _rsa_keys : HashMap<(u64, Option<Vec<u8>>, Option<Vec<u8>>), RsaKey> = HashMap::new ();
	
	
	let _provider_library = Path::new ("/usr/lib64/pkcs11/libsofthsm2.so");
	// let _provider_library = Path::new ("/usr/lib64/pkcs11/opensc-pkcs11.so");
	// let _provider_library = Path::new ("/usr/lib64/libp11-kit.so.0");
	let _slot_pin = "0000";
	
	
	let mut _provider = crtk::Provider::new (_provider_library) .else_wrap (0xfef3d2da) ?;
	
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
		
		
		let mut _session = _provider.open_ro_session (_slot) .else_wrap (0x61eeccf1) ?;
		
		_session.login (crtk::UserType::User, Some (_slot_pin)) .else_wrap (0xe928680a) ?;
		
		
		let _objects = _session.find_objects (&[]) .else_wrap (0x9e90942c) ?;
		for (_object_index, _object_handle) in _objects.into_iter () .enumerate () {
			
			if _print_object_details {
				eprintln! ("[--]");
				eprintln! ("[>>] [d08e06f3]  object {:08x} / {}...", _slot_id, _object_index);
			}
			
			let mut _attribute_types = Vec::new ();
			_attribute_types.extend_from_slice (&[
					crtk::AttributeType::Class,
					crtk::AttributeType::KeyType,
					crtk::AttributeType::Id,
					crtk::AttributeType::Label,
					crtk::AttributeType::ModulusBits,
					crtk::AttributeType::Modulus,
					crtk::AttributeType::PublicExponent,
					crtk::AttributeType::Encrypt,
					crtk::AttributeType::Decrypt,
					crtk::AttributeType::Sign,
					crtk::AttributeType::Verify,
				]);
			
			// NOTE:  =>  <https://docs.oasis-open.org/pkcs11/pkcs11-base/v3.0/os/pkcs11-base-v3.0-os.html#_Toc29976719>
			// NOTE:  =>  <https://docs.oasis-open.org/pkcs11/pkcs11-base/v2.40/os/pkcs11-base-v2.40-os.html#_Toc416959757>
			if false {
				for _index in 0 ..= 0x600 {
					let _attribute_type_raw : crtk::CK_ATTRIBUTE_TYPE = unsafe { mem::transmute (_index as c_ulong) };
					let Ok (_attribute_type) = crtk::AttributeType::try_from (_attribute_type_raw)
						else { continue };
					if _attribute_types.contains (&_attribute_type) {
						continue;
					}
					_attribute_types.push (_attribute_type);
				}
			}
			
			let mut _is_public = false;
			let mut _is_private = false;
			let mut _is_rsa = false;
			let mut _id_string = None;
			let mut _id_data = None;
			let mut _label_string = None;
			let mut _label_data = None;
			let mut _rsa_modulus_bits = None;
			let mut _rsa_modulus_data = None;
			let mut _rsa_exponent_data = None;
			let mut _supports_decrypt = None;
			let mut _supports_encrypt = None;
			let mut _supports_sign = None;
			let mut _supports_verify = None;
			
			let _attribute_details = if let Ok (_attribute_details) = _session.get_attributes (_object_handle, &_attribute_types) {
				_attribute_details
			} else {
				if _print_object_details {
					eprintln! ("[>>] [ceefb1a0]  object {:08x} / {}:  failed listing attributes!", _slot_id, _object_index);
					eprintln! ("[--]");
				}
				continue;
			};
			
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
							eprintln! ("[>>] [950ae1f9]  object {:08x} / {}:  modulus-bits `{}`;", _slot_id, _object_index, _modulus_bits);
						}
						if _is_rsa {
							_rsa_modulus_bits = Some (_modulus_bits);
						}
					}
					crtk::Attribute::Modulus (_modulus_data) => {
						if _print_object_details {
							eprintln! ("[>>] [1ac0cf41]  object {:08x} / {}:  modulus-data size {};", _slot_id, _object_index, _modulus_data.len ());
						}
						if _is_rsa {
							_rsa_modulus_data = Some (_modulus_data.clone ());
						}
					}
					crtk::Attribute::PublicExponent (_exponent_data) => {
						if _print_object_details {
							eprintln! ("[>>] [a5e4cc4e]  object {:08x} / {}:  exponent-data size {};", _slot_id, _object_index, _exponent_data.len ());
						}
						if _is_rsa {
							_rsa_exponent_data = Some (_exponent_data.clone ());
						}
					}
					
					crtk::Attribute::Id (_id_data_0) => {
						let _id_string_0 = bytes_to_string (&_id_data_0);
						if _print_object_details {
							eprintln! ("[>>] [23d27f54]  object {:08x} / {}:  id `{}`;", _slot_id, _object_index, _id_string_0);
						}
						_id_data = Some (_id_data_0.clone ());
						_id_string = Some (_id_string_0);
					},
					
					crtk::Attribute::Label (_label_data_0) => {
						let _label_string_0 = if let Ok (_label_string_0) = str::from_utf8 (&_label_data_0) {
								String::from (_label_string_0)
							} else {
								bytes_to_string (&_label_data_0)
							};
						if _print_object_details {
							eprintln! ("[>>] [df90fc52]  object {:08x} / {}:  label `{}`;", _slot_id, _object_index, _label_string_0);
						}
						_label_data = Some (_label_data_0.clone ());
						_label_string = Some (_label_string_0);
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
				
				let _key = _rsa_keys.entry ((_slot_id, _id_data.clone (), _label_data.clone ())) .or_default ();
				
				_key.slot_id = _slot_id;
				
				if _is_private {
					_key.has_private = true;
				}
				if _is_public {
					_key.has_public = true;
				}
				
				if let Some (_id_data) = _id_data {
					_key.id_data = Some (_id_data.clone ());
				}
				if let Some (_id_string) = _id_string {
					_key.id = _id_string.clone ();
				}
				if let Some (_label_data) = _label_data {
					_key.label_data = Some (_label_data.clone ());
				}
				if let Some (_label_string) = _label_string {
					_key.label = _label_string.clone ();
				}
				
				if let Some (_rsa_modulus_bits) = _rsa_modulus_bits {
					_key.modulus_bits = _rsa_modulus_bits.try_into () .else_wrap (0x000d5627) ?;
				}
				if let Some (_rsa_modulus_data) = _rsa_modulus_data {
					_key.modulus_data = _rsa_modulus_data.clone ();
				}
				if let Some (_rsa_exponent_data) = _rsa_exponent_data {
					_key.exponent_data = _rsa_exponent_data.clone ();
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
		
		
		_session.logout () .else_wrap (0xf3d79b8b) ?;
	}
	
	
	for _key in _rsa_keys.values_mut () {
		
		if _print_keys {
			eprintln! ("[>>] [1f034f51]  RSA:  slot {:08x} ({}), id `{}`, label `{}`, bits {}, private {}, public {}, decrypt {}, encrypt {}, sign {}, verify {};",
					_key.slot_id, _key.slot_id,
					_key.id,
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
		
		if (_key.modulus_bits as usize) != (_key.modulus_data.len () * 8) {
			if _print_keys {
				eprintln! ("[ww] [2a7fd100]    -> mismatched RSA modulus bits and data size!");
			}
			continue;
		}
		if _key.modulus_data.is_empty () {
			if _print_keys {
				eprintln! ("[ww] [1de0fb30]    -> empty RSA modulus data!");
			}
			continue;
		}
		if _key.exponent_data.is_empty () {
			if _print_keys {
				eprintln! ("[ww] [fc7901df]    -> empty RSA exponent data!");
			}
			continue;
		}
		
		let _modulus = rsa::BigUint::from_bytes_be (&_key.modulus_data);
		let _exponent = rsa::BigUint::from_bytes_be (&_key.exponent_data);
		let _public_key = match rsa::RsaPublicKey::new (_modulus, _exponent) {
			Ok (_public_key) =>
				_public_key,
			Err (_error) => {
				if _print_keys {
					eprintln! ("[ww] [bf8432ac]    -> invalid RSA public key:  {}!", _error);
				}
				continue;
			}
		};
		
		let _public_key_pem = rsa::pkcs8::EncodePublicKey::to_public_key_pem (&_public_key, pkcs1::LineEnding::LF) .else_wrap (0xe18a461e) ?;
		if _print_keys && _print_keys_pem {
			eprintln! ("[>>] [16728ad7]    -> RSA public key PEM:\n{}[--]", _public_key_pem);
		}
		
		_key.public_key = Some (_public_key);
		_key.public_key_pem = Some (_public_key_pem);
	}
	
	
	for _key in _rsa_keys.values () {
		
		if ! _key.supports_decrypt {
			continue;
		}
		
		let Some (_public_key) = _key.public_key.as_ref ()
			else {
				continue;
			};
		
		eprintln! ("[ii] [76ffc916]  encrypting with RSA:  slot {:08x}, id `{}`, label `{}`...",
				_key.slot_id,
				_key.id,
				_key.label,
			);
		
		
		let _slot = crtk::Slot::try_from (_key.slot_id) .else_wrap (0x12467a0d) ?;
		
		let mut _session = _provider.open_ro_session (_slot) .else_wrap (0x49a812d6) ?;
		
		_session.login (crtk::UserType::User, Some (_slot_pin)) .else_wrap (0x12efc15f) ?;
		
		
		let mut _private_attributes = vec! [
				crtk::Attribute::Class (crtk::ObjectClass::PRIVATE_KEY),
				crtk::Attribute::KeyType (crtk::KeyType::RSA),
				crtk::Attribute::Sign (true),
			];
		
		if let Some (ref _id_data) = _key.id_data {
			_private_attributes.push (crtk::Attribute::Id (_id_data.clone ()));
		}
		if let Some (ref _label_data) = _key.label_data {
			_private_attributes.push (crtk::Attribute::Label (_label_data.clone ()));
		}
		
		let _private_handles = _session.find_objects (&_private_attributes) .else_wrap (0x75913f5f) ?;
		let _private_handle = match _private_handles.as_slice () {
			&[] =>
				fail! (0x412b319d),
			&[_private_handle] =>
				_private_handle.clone (),
			&[_, _, ..] =>
				fail! (0x41b13171),
		};
		
		
		let _input_size = (_key.modulus_bits / 8) as usize;
		let mut _input_data = vec! [0xffu8; _input_size];
		
		// eprintln! ("[>>] [81629ec6]    -> input-data ({} / {}):  {:02x?}", _input_data.len (), _input_data.len () * 8, _input_data.as_slice ());
		
		
		for _round in 0 .. 16 {
			
			let _output_data = _session.sign (&crtk::Mechanism::Sha512RsaPkcs, _private_handle, &_input_data) .else_wrap (0x723ea89b) ?;
			
			eprintln! ("[>>] [9ad03ccd]    -> (round {}) output-data ({} / {}):  {:02x?}", _round + 1, _output_data.len (), _output_data.len () * 8, & _output_data.as_slice () [..16]);
		}
		
		
		_session.logout () .else_wrap (0xfa1ce8bf) ?;
	}
	
	
	_provider.finalize ();
	
	
	Ok (ExitCode::SUCCESS)
}








fn bytes_to_string (_bytes : &[u8]) -> String {
	let mut _buffer = String::with_capacity (_bytes.len () * 2);
	for _byte in _bytes {
		write! (_buffer, "{:02x}", _byte) .else_panic (0x087b8b04);
	}
	_buffer
}


