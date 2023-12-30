

use crate::prelude::*;




define_error! (pub MainError, result : MainResult);




pub struct RandomizerFlags {
	pub source : RandomizerSource,
}


#[ derive (Debug) ]
#[ derive (Copy, Clone) ]
pub enum RandomizerSource {
	Os,
	Testing,
}

impl FlagValue for RandomizerSource {}

impl FlagValueDisplay for RandomizerSource {
	fn display_value (&self, _formatter : &mut Formatter) -> FlagValueDisplayResult {
		Debug::fmt (self, _formatter) .else_wrap (0xd4eb1d49)
	}
}




impl RandomizerFlags {
	
	pub fn new () -> FlagsResult<Self> {
		let _self = Self {
				source : RandomizerSource::Os,
			};
		Ok (_self)
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		let _flag = _flags.define_complex (&mut self.source);
		_flag.define_switch_0 (RandomizerSource::Os)
				.with_flag ((), "random-os")
				.with_description ("use OS secure random generator")
				.with_default ("default");
		_flag.define_switch_0 (RandomizerSource::Testing)
				.with_flag ((), "random-testing")
				.with_description ("unsafe constant generator")
				.with_warning ("DO-NOT-USE");
		Ok (())
	}
	
	pub fn build (&self) -> FlagsResult<Box<dyn Randomizer>> {
		let _randomizer : Box<dyn Randomizer> = match self.source {
			RandomizerSource::Os =>
				Box::new (OsRandomizer::from_os () .else_wrap (0x893f3ab5) ?),
			RandomizerSource::Testing =>
				Box::new (SeedRandomizer::for_testing () .else_wrap (0x07578413) ?),
		};
		Ok (_randomizer)
	}
}




pub struct OutputFlags {
	pub compact : Option<bool>,
	pub skip_separators_mandatory : Option<bool>,
	pub skip_separators_optional : Option<bool>,
}


impl OutputFlags {
	
	pub fn new () -> FlagsResult<Self> {
		let _self = OutputFlags {
				compact : None,
				skip_separators_mandatory : None,
				skip_separators_optional : None,
			};
		Ok (_self)
	}
	
	pub fn flags <'a> (&'a mut self, _flags : &mut FlagsParserBuilder<'a>) -> FlagsResult {
		
		let _flag = _flags.define_complex (&mut self.compact);
		_flag.define_switch_0 (true)
				.with_flag ('C', ())
				.with_description ("compact output, skip optional separators and groups");
		_flag.define_flag_0 ()
				.with_flag ((), "compact")
				.with_placeholder ("boolean");
		
		_flags.define_single_flag_0 (&mut self.skip_separators_mandatory)
				.with_flag ((), "token-skip-mandatory-separators")
				.with_placeholder ("boolean")
				.with_description ("skip token mandatory separators");
		
		_flags.define_single_flag_0 (&mut self.skip_separators_optional)
				.with_flag ((), "token-skip-optional-separators")
				.with_placeholder ("boolean")
				.with_description ("skip token optional separators");
		
		Ok (())
	}
	
	pub fn build (&self) -> FlagsResult<OutputOptions> {
		
		let mut _options = OutputOptions::default ();
		
		if let Some (_compact) = self.compact {
			_options.output_separators_optional &= ! _compact;
		}
		if let Some (_skip) = self.skip_separators_mandatory {
			_options.output_separators_mandatory = ! _skip;
		}
		if let Some (_skip) = self.skip_separators_optional {
			_options.output_separators_optional = ! _skip;
		}
		
		Ok (_options)
	}
}


