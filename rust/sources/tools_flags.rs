

use crate::prelude::*;


pub(crate) use ::argparse::{
		ArgumentParser as ArgParser,
		Store as ArgStore,
		StoreTrue as ArgStoreTrue,
		StoreFalse as ArgStoreFalse,
		StoreConst as ArgStoreConst,
		StoreOption as ArgStoreOption,
	};




define_error! (pub FlagsError, result : FlagsResult);




pub struct RandomizerFlags {
	pub source : RandomizerSource,
}


#[ derive (Copy, Clone) ]
pub enum RandomizerSource {
	Os,
	Testing,
}


impl RandomizerFlags {
	
	pub fn new () -> FlagsResult<Self> {
		let _self = Self {
				source : RandomizerSource::Os,
			};
		Ok (_self)
	}
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		
		_parser.refer (&mut self.source)
				.add_option (&["--random-os"], ArgStoreConst (RandomizerSource::Os), "(use OS secure random generator)")
				.add_option (&["--random-testing"], ArgStoreConst (RandomizerSource::Testing), "(unsafe constant generator)");
		
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
	
	pub fn parser <'a> (&'a mut self, _parser : &mut ArgParser<'a>) -> FlagsResult {
		
		_parser.refer (&mut self.compact)
				.metavar ("{compact}")
				.add_option (&["-C"], ArgStoreConst (Some (true)), "(compact output, skip optional separators and groups)")
				.add_option (&["--compact"], ArgStoreOption, "");
		
		_parser.refer (&mut self.skip_separators_mandatory)
				.metavar ("{skip}")
				.add_option (&["--token-skip-mandatory-separators"], ArgStoreOption, "(skip token mandatory separators)");
		
		_parser.refer (&mut self.skip_separators_optional)
				.metavar ("{skip}")
				.add_option (&["--token-skip-optional-separators"], ArgStoreOption, "(skip token optional separators)");
		
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




pub fn execute_parser (_parser : ArgParser, _arguments : Vec<String>) -> FlagsResult<bool> {
	
	match _parser.parse (_arguments, &mut stdout_locked (), &mut stderr_locked ()) {
		Ok (()) =>
			Ok (false),
		Err (0) =>
			Ok (true),
		Err (_error) =>
			fail! (0x0f71ad86),
	}
}

