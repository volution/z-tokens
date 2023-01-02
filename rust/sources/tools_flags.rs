

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
				.add_option (&["--random-testing"], ArgStoreConst (RandomizerSource::Testing), "!!! UNSAFE !!!");
		
		Ok (())
	}
	
	pub fn build (self) -> FlagsResult<Box<dyn Randomizer>> {
		let _randomizer : Box<dyn Randomizer> = match self.source {
			RandomizerSource::Os =>
				Box::new (RngRandomizer::from_os () .else_wrap (0x893f3ab5) ?),
			RandomizerSource::Testing =>
				Box::new (RngRandomizer::for_testing () .else_wrap (0x07578413) ?),
		};
		Ok (_randomizer)
	}
}


