

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use crate::crates::pinentry;

use crate::crates::secrecy::ExposeSecret as _;




define_error! (pub PinentryError, result : PinentryResult);


pub(crate) const PINENTRY_DEFAULT_TITLE : &str = "[z-tokens]  Pinentry";
pub(crate) const PINENTRY_DEFAULT_DESCRIPTION : &str = "Please enter the password";
pub(crate) const PINENTRY_DEFAULT_PROMPT : &str = "Password";
pub(crate) const PINENTRY_DEFAULT_REPEAT_PROMPT : &str = "Repeat";
pub(crate) const PINENTRY_DEFAULT_REPEAT_ERROR : &str = "Matching inputs are required!";
pub(crate) const PINENTRY_DEFAULT_REQUIRED_ERROR : &str = "A non-empty input is required!";
pub(crate) const PINENTRY_DEFAULT_BUTTON_OK : &str = "Confirm";
pub(crate) const PINENTRY_DEFAULT_BUTTON_CANCEL : &str = "Cancel";
pub(crate) const PINENTRY_DEFAULT_TIMEOUT : u16 = 60;




pub fn pinentry_password (_description : Option<&str>, _prompt : Option<&str>, _repeat : Option<&str>) -> PinentryResult<Option<String>> {
	
	let mut _pinentry = pinentry::PassphraseInput::with_default_binary () .else_wrap (0xc3120b9e) ?;
	
	_pinentry.with_title (PINENTRY_DEFAULT_TITLE);
	_pinentry.with_description (PINENTRY_DEFAULT_DESCRIPTION);
	_pinentry.with_prompt (PINENTRY_DEFAULT_PROMPT);
	_pinentry.required (PINENTRY_DEFAULT_REQUIRED_ERROR);
	_pinentry.with_ok (PINENTRY_DEFAULT_BUTTON_OK);
	_pinentry.with_cancel (PINENTRY_DEFAULT_BUTTON_CANCEL);
	_pinentry.with_timeout (PINENTRY_DEFAULT_TIMEOUT);
	
	if let Some (_description) = _description {
		if ! _description.is_empty () {
			_pinentry.with_description (_description);
		}
	}
	if let Some (_prompt) = _prompt {
		if ! _prompt.is_empty () {
			_pinentry.with_prompt (_prompt);
		}
	}
	if let Some (_repeat) = _repeat {
		if ! _repeat.is_empty () {
			_pinentry.with_confirmation (_repeat, PINENTRY_DEFAULT_REPEAT_ERROR);
		} else {
			_pinentry.with_confirmation (PINENTRY_DEFAULT_REPEAT_PROMPT, PINENTRY_DEFAULT_REPEAT_ERROR);
		}
	}
	
	let _outcome = _pinentry.interact ();
	
	match _outcome {
		Ok (_password) => {
			let _password = _password.expose_secret ();
			if ! _password.is_empty () {
				Ok (Some (String::from (_password)))
			} else {
				Ok (None)
			}
		}
		Err (pinentry::Error::Cancelled) =>
			Ok (None),
		Err (pinentry::Error::Timeout) =>
			Err (PinentryError::new_with_code (0x88d91ae0)),
		Err (_) =>
			Err (PinentryError::new_with_code (0x7fb00962)),
	}
}


